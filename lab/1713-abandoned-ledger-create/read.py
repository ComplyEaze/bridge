"""Reads this sheet's answers by LAB-COMMON C4, streaming, so a BUSY answer of hundreds of megabytes is never held whole.

Run with `python -I read.py <mode> ...` from this folder. Exit 0 = the check passed, 1 = it failed (the line printed
says why; the sheet treats it as STOP), 2 = the command line is wrong.

  import  <answer> <created>                  an import answer: every counter present, CREATED as given, the others
                                              0, no LINEERROR; prints LASTVCHID and LASTMID
  marks   <answer> [<company-guid>]           MARKS: exactly one COMPANY row (with that GUID, if given), its GUID
                                              and ALTMSTID present (one absent = FAIL); ALTVCHID is printed, and
                                              may be absent: THE COMPANY holds no voucher, and Tally sends no
                                              ALTVCHID until one exists (measured at S1, 28 Oct)
  ledgers <answer> <company-guid> [<probe>]   LEDGERS: every row of THE COMPANY, `Lab Bulk Expense` present once;
                                              prints probe=present (with GUID, parent, ALTERID) or probe=absent
  busy    <answer> <company-guid> <rows>      BUSY: VOUCHER rows counted by structure, each GUID THE COMPANY's
  plan    <m0-bytes> <d0-seconds>             M0's decision: how many more bulk files keep BUSY at 300,000,000 bytes

Decoding is C4's: UTF-16LE with or without a byte-order mark; only the characters XML forbids (0 to 8, 11, 12, 14 to
31) are dropped, raw or as character references; rows are the children of ENVELOPE/BODY/DATA/COLLECTION (the CMPINFO
count lines are not rows); names are never trimmed and are printed with repr, so a trailing line break shows.
"""
import codecs
import math
import re
import sys
import xml.etree.ElementTree as ET

CHUNK = 1 << 20
FORBIDDEN_RAW = re.compile("[\x00-\x08\x0b\x0c\x0e-\x1f]")
REFERENCE = re.compile(r"&#(x[0-9A-Fa-f]+|[0-9]+);")
ROW_PATH = ("ENVELOPE", "BODY", "DATA", "COLLECTION")
COUNTERS = ("CREATED", "ALTERED", "DELETED", "LASTVCHID", "LASTMID", "COMBINED", "IGNORED", "ERRORS", "CANCELLED",
            "EXCEPTIONS")
ZERO = ("ALTERED", "DELETED", "COMBINED", "IGNORED", "ERRORS", "CANCELLED", "EXCEPTIONS")
# Where an import answer carries its counters. A native create answers with a bare RESPONSE root (seen at S3 on 28 Oct:
# 538 B, CREATED 2, no ENVELOPE); the enveloped IMPORTRESULT form is kept. Exactly one block must be present.
RESULT_BLOCKS = (("ENVELOPE", "BODY", "DATA", "IMPORTRESULT"), ("RESPONSE",))
BUSY_TARGET = 300_000_000
M0_ROWS = 24_000
PER_FILE = 250


class Failed(Exception):
    pass


def forbidden(code):
    return code <= 8 or code in (11, 12) or 14 <= code <= 31


def drop_reference(match):
    text = match.group(1)
    code = int(text[1:], 16) if text[0] == "x" else int(text)
    return "" if forbidden(code) else match.group(0)


def clean(text):
    return REFERENCE.sub(drop_reference, FORBIDDEN_RAW.sub("", text))


def events(path):
    """Yields (event, element, depth path) for the whole answer; an answer that does not parse raises Failed."""
    parser = ET.XMLPullParser(events=("start", "end"))
    decoder = codecs.getincrementaldecoder("utf-16-le")()
    stack = []
    carry = ""
    first = True
    size = 0
    with open(path, "rb") as handle:
        while True:
            data = handle.read(CHUNK)
            size += len(data)
            final = not data
            text = carry + decoder.decode(data, final=final)
            if first and text:
                if text.startswith("﻿"):
                    text = text[1:]
                first = False
            carry = ""
            if not final:
                # A character reference may be split between chunks: keep an unfinished one for the next chunk.
                amp = text.rfind("&", max(0, len(text) - 16))
                if amp >= 0 and ";" not in text[amp:]:
                    carry, text = text[amp:], text[:amp]
            try:
                parser.feed(clean(text))
                if final:
                    parser.close()
                ready = list(parser.read_events())
            except ET.ParseError as error:
                raise Failed("the answer does not parse: {}".format(error))
            for event, element in ready:
                if event == "start":
                    stack.append(element.tag)
                    yield event, element, tuple(stack)
                else:
                    yield event, element, tuple(stack)
                    stack.pop()
            if final:
                break
    if size == 0:
        raise Failed("the answer is empty")


def rows(path, tag):
    """Yields each row element of `tag` when it is complete; it is cleared afterwards to keep memory flat."""
    parents = []
    for event, element, path_now in events(path):
        if event == "start" and path_now == ROW_PATH:
            parents.append(element)
        if event == "end" and path_now == ROW_PATH + (tag,):
            yield element
            element.clear()
            for parent in parents:
                parent.clear()


def number(element, tag):
    text = element.findtext(tag)
    return None if text is None else text.strip()


def shown(value):
    """A field that is missing prints as `absent`, never as `None` or as an empty string."""
    return "absent" if value is None or value == "" else value


def check_import(path, created):
    seen = {}
    results = 0
    line_errors = []
    for event, element, path_now in events(path):
        if event != "end":
            continue
        if element.tag == "LINEERROR":
            line_errors.append(repr(element.text or ""))
        if path_now in RESULT_BLOCKS:
            results += 1
        if path_now[:-1] in RESULT_BLOCKS:
            if element.tag in COUNTERS:
                if element.tag in seen:
                    raise Failed("counter {} appears twice".format(element.tag))
                seen[element.tag] = (element.text or "").strip()
    if results != 1:
        raise Failed("{} result blocks (IMPORTRESULT or a RESPONSE root), expected 1".format(results))
    missing = [name for name in COUNTERS if name not in seen]
    if missing:
        raise Failed("counters missing: {}".format(",".join(missing)))
    if line_errors:
        raise Failed("LINEERROR {}".format(" ".join(line_errors)))
    if seen["CREATED"] != str(created):
        raise Failed("CREATED {}, expected {}".format(seen["CREATED"], created))
    nonzero = ["{} {}".format(name, seen[name]) for name in ZERO if seen[name] != "0"]
    if nonzero:
        raise Failed("not 0: {}".format(", ".join(nonzero)))
    return "import ok CREATED {} LASTVCHID {} LASTMID {}".format(seen["CREATED"], seen["LASTVCHID"], seen["LASTMID"])


def check_marks(path, guid):
    found = []
    for row in rows(path, "COMPANY"):
        found.append((number(row, "GUID"), number(row, "ALTVCHID"), number(row, "ALTMSTID")))
    if len(found) != 1:
        raise Failed("{} COMPANY rows, expected exactly 1".format(len(found)))
    row_guid, altvchid, altmstid = found[0]
    if guid is not None and row_guid != guid:
        raise Failed("the COMPANY row's GUID is {}, not THE COMPANY's".format(row_guid))
    # ALTVCHID is not required: the company holds no voucher until S4's bulk Journals, and Tally sends no ALTVCHID
    # until a voucher exists (seen at S1). No decision of this sheet reads ALTVCHID.
    missing = [tag for tag, value in (("GUID", row_guid), ("ALTMSTID", altmstid))
               if shown(value) == "absent"]
    if missing:
        raise Failed("the COMPANY row has {} absent".format(", ".join(missing)))
    return "marks ok GUID {} ALTVCHID {} ALTMSTID {}".format(row_guid, shown(altvchid), altmstid)


def check_ledgers(path, guid, probe):
    count = 0
    bulk_expense = 0
    probes = []
    for row in rows(path, "LEDGER"):
        count += 1
        name = row.get("NAME")
        if number(row, "BRIDGECOMPANYGUID") != guid:
            raise Failed("ledger {} carries company GUID {}".format(repr(name), number(row, "BRIDGECOMPANYGUID")))
        if name == "Lab Bulk Expense":
            bulk_expense += 1
        if name is not None and name.startswith("Lab Probe"):
            probes.append((name, number(row, "GUID"), row.findtext("PARENT"), number(row, "ALTERID")))
    if count == 0:
        raise Failed("no LEDGER rows")
    if bulk_expense != 1:
        raise Failed("`Lab Bulk Expense` appears {} times, expected 1".format(bulk_expense))
    listed = " ".join("{}(len {})".format(repr(name), len(name)) for name, _, _, _ in probes) or "none"
    line = "ledgers ok rows {}; Lab Probe ledgers: {}".format(count, listed)
    if probe is not None:
        mine = [entry for entry in probes if entry[0] == probe]
        if len(mine) > 1:
            raise Failed("{} appears {} times".format(repr(probe), len(mine)))
        if mine:
            _, row_guid, parent, alterid = mine[0]
            line += "; probe=present {} GUID {} PARENT {} ALTERID {}".format(
                repr(probe), shown(row_guid), "absent" if parent is None else repr(parent), shown(alterid))
        else:
            line += "; probe=absent {}".format(repr(probe))
    return line


def check_busy(path, guid, expected):
    count = 0
    for row in rows(path, "VOUCHER"):
        count += 1
        row_guid = number(row, "GUID") or ""
        if not row_guid.startswith(guid):
            raise Failed("voucher row {} has GUID {}, not of THE COMPANY".format(count, repr(row_guid)))
    if count != expected:
        raise Failed("{} VOUCHER rows, expected {}".format(count, expected))
    return "busy ok rows {}".format(count)


def plan(m0_bytes, d0_seconds):
    if not 0 < m0_bytes <= 400_000_000:
        raise Failed("M0's size {} is not between 1 and 400,000,000 bytes".format(m0_bytes))
    if not (math.isfinite(d0_seconds) and d0_seconds > 0):
        raise Failed("D0 {} is not a time above 0 s".format(d0_seconds))
    per_row = m0_bytes / M0_ROWS
    if d0_seconds >= 60:
        return "plan: D0 {} s is 60 s or more: send no more bulk files; go to S5".format(d0_seconds)
    extra = 0
    while extra < 96 and (96 + extra + 1) * PER_FILE * per_row <= BUSY_TARGET:
        extra += 1
    predicted = int((96 + extra) * PER_FILE * per_row)
    if extra == 0:
        return "plan: {:.0f} bytes a row; one more file would pass 300,000,000 bytes: send none; go to S5".format(
            per_row)
    return "plan: {:.0f} bytes a row; send bulk-097 to bulk-{:03d} ({} files); BUSY then predicted {} bytes, {} rows".format(
        per_row, 96 + extra, extra, predicted, (96 + extra) * PER_FILE)


def main(argv):
    if len(argv) < 2:
        return None
    mode, rest = argv[0], argv[1:]
    if mode == "import" and len(rest) == 2:
        return check_import(rest[0], int(rest[1]))
    if mode == "marks" and len(rest) in (1, 2):
        return check_marks(rest[0], rest[1] if len(rest) == 2 else None)
    if mode == "ledgers" and len(rest) in (2, 3):
        return check_ledgers(rest[0], rest[1], rest[2] if len(rest) == 3 else None)
    if mode == "busy" and len(rest) == 3:
        return check_busy(rest[0], rest[1], int(rest[2]))
    if mode == "plan" and len(rest) == 2:
        return plan(int(rest[0]), float(rest[1]))
    return None


if __name__ == "__main__":
    try:
        result = main(sys.argv[1:])
    except Failed as error:
        print("FAIL: {}".format(error))
        sys.exit(1)
    except (OSError, ValueError) as error:
        print("FAIL: {}".format(error))
        sys.exit(1)
    if result is None:
        print(__doc__)
        sys.exit(2)
    print(result)
    sys.exit(0)
