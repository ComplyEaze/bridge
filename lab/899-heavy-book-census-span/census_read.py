"""Reads one census answer by LAB-COMMON C4, streaming, and prints counts only.

Run with `python -I census_read.py <answer> <lo> <hi>`; exit 0 = the answer parsed and holds no LINEERROR and no STATUS other than 1, 1 = it did not
(the line printed says why, in fixed words that never repeat the span's bounds; the sheet treats it as STOP), 2 = the command line is wrong.

Rows are the VOUCHER elements under ENVELOPE/BODY/DATA/COLLECTION (the CMPINFO count lines are not rows). Printed: the size in
bytes, the number of rows, how many carry GUID, ALTERID and DATE (a missing one is counted), how many rows have an ALTERID not in
(lo, hi], how many GUIDs repeat, and the smallest and largest ALTERID as OFFSETS from lo (an absolute id is a figure of the book
and is never printed). No GUID, date or name is printed.

Decoding is C4's: UTF-16LE with or without a byte-order mark; only the characters XML forbids (0 to 8, 11, 12, 14 to 31) are
dropped, raw or as character references.
"""
import codecs
import re
import sys
import xml.etree.ElementTree as ET

CHUNK = 1 << 20
FORBIDDEN_RAW = re.compile("[\x00-\x08\x0b\x0c\x0e-\x1f]")
REFERENCE = re.compile(r"&#(x[0-9A-Fa-f]+|[0-9]+);")
ROW_PATH = ("ENVELOPE", "BODY", "DATA", "COLLECTION")


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
            if final and size == 0:
                raise Failed("the answer is empty")
            text = carry + decoder.decode(data, final=final)
            if first and text:
                if text.startswith("﻿"):
                    text = text[1:]
                first = False
            carry = ""
            if not final:
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
    events.size = size


def read(path, lo, hi):
    rows = missing = outside = 0
    seen = set()
    repeats = 0
    low = high = None
    parents = []
    for event, element, path_now in events(path):
        if event == "start" and path_now == ROW_PATH:
            parents.append(element)
        if event == "end" and element.tag == "LINEERROR":
            raise Failed("a LINEERROR element is present")
        if event == "end" and element.tag == "STATUS" and (element.text or "").strip() != "1":
            raise Failed("a STATUS element other than 1 is present: {!r}".format(element.text or ""))
        if event == "end" and path_now == ROW_PATH + ("VOUCHER",):
            rows += 1
            guid = (element.findtext("GUID") or "").strip()
            alter = (element.findtext("ALTERID") or "").strip()
            when = (element.findtext("DATE") or "").strip()
            if not guid or not alter.isdigit() or not when:
                missing += 1
            if alter.isdigit():
                value = int(alter)
                if not (lo < value <= hi):
                    outside += 1
                low = value if low is None else min(low, value)
                high = value if high is None else max(high, value)
            if guid:
                if guid in seen:
                    repeats += 1
                seen.add(guid)
            element.clear()
            for parent in parents:
                parent.clear()
    return "census ok bytes {} rows {} rows_missing_a_field {} rows_outside_span {} repeated_guids {} min_alterid_offset {} max_alterid_offset {}".format(
        events.size, rows, missing, outside, repeats,
        "none" if low is None else low - lo, "none" if high is None else high - lo)


def main(argv):
    if len(argv) == 3:
        return read(argv[0], int(argv[1]), int(argv[2]))
    return None


if __name__ == "__main__":
    try:
        result = main(sys.argv[1:])
    except Failed as error:
        print("FAIL: {}".format(error))
        sys.exit(1)
    except (OSError, ValueError):
        # A fixed line, never the exception's text: it could carry the span's bounds or a path.
        print("FAIL: the answer or the arguments could not be read")
        sys.exit(1)
    if result is None:
        print(__doc__)
        sys.exit(2)
    print(result)
    sys.exit(0)
