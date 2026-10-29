"""Reads this sheet's Bills Receivable / Bills Payable answers by LAB-COMMON C4, streaming, so an answer of tens of
megabytes is never held whole.

Run with `python -I read.py bills <answer> [<rows>]` from this folder. Exit 0 = the answer parsed and holds no STATUS
element, 1 = it failed (the line printed says why; the sheet treats it as STOP), 2 = the command line is wrong.

`python -I read.py compare <whole answer> <part answer> <first day>` reads both answers the same way and compares them
bill for bill. A bill is its whole row: BILLDATE, BILLREF, BILLPARTY, BILLCL, BILLDUE and BILLOVERDUE, each trimmed except the
names, compared as text, as a multiset. It prints counts only: the part's rows, the part's rows dated before the first day,
the whole's rows dated from the first day, how many of those are in the part, and how many rows each holds that the other
does not. `subset yes` means the part is exactly the whole's rows dated from the first day; `subset no` means it is not.
BILLDATE is read as Tally writes it, day-month-year with a three-letter month and a two-digit year (`1-Apr-25`); a date in
any other form fails (exit 1).

The answer is the FLAT grammar (protocol reference, native outstandings): under ENVELOPE a BILLFIXED element is followed by
sibling BILLCL, BILLDUE and BILLOVERDUE elements, with no row wrapper. Success carries no STATUS anywhere; a STATUS element
only appears on failure; an empty result is a bare ENVELOPE. Any other element under ENVELOPE, any BILLFIXED field other
than BILLDATE, BILLREF and BILLPARTY, and a BILLCL, BILLDUE or BILLOVERDUE count that differs from the BILLFIXED count fail (the
protocol parser refuses the same). Printed: the encoded size, the number of BILLFIXED, BILLCL,
BILLDUE and BILLOVERDUE elements, the exact sum of BILLCL (its sign shows the side), the sums of its negative and positive
values, and how many BILLCL are empty. `rows`, if given, must equal the number of BILLFIXED, else exit 1.

Decoding is C4's: UTF-16LE with or without a byte-order mark; only the characters XML forbids (0 to 8, 11, 12, 14 to 31)
are dropped, raw or as character references. No name or value is printed except those counts and sums.
"""
import codecs
import re
import sys
import xml.etree.ElementTree as ET
from decimal import Decimal, InvalidOperation

CHUNK = 1 << 20
FORBIDDEN_RAW = re.compile("[\x00-\x08\x0b\x0c\x0e-\x1f]")
REFERENCE = re.compile(r"&#(x[0-9A-Fa-f]+|[0-9]+);")
SEEN = ("BILLFIXED", "BILLCL", "BILLDUE", "BILLOVERDUE")
FIXED_FIELDS = ("BILLDATE", "BILLREF", "BILLPARTY")


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
    """Yields (event, element, depth path) for the whole answer; returns the size read through `events.size`."""
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
    events.size = size


def check_bills(path, rows):
    counts = dict.fromkeys(SEEN, 0)
    total = negative = positive = Decimal(0)
    empty = 0
    root = None
    for event, element, path_now in events(path):
        if event == "start" and len(path_now) == 1:
            root = element.tag
            if root != "ENVELOPE":
                raise Failed("the root is {}, not ENVELOPE".format(root))
        if event != "end" or len(path_now) < 2 or path_now[0] != "ENVELOPE":
            continue
        if element.tag == "STATUS":
            raise Failed("a STATUS element is present (the report's failure marker): {!r}".format(element.text or ""))
        if len(path_now) == 3:
            if path_now[1] != "BILLFIXED" or element.tag not in FIXED_FIELDS:
                raise Failed("an unexpected element {} under {}".format(element.tag, path_now[1]))
            continue
        if len(path_now) != 2:
            raise Failed("an element nested deeper than the grammar allows: {}".format("/".join(path_now)))
        if element.tag not in counts:
            raise Failed("an unexpected element under ENVELOPE: {}".format(element.tag))
        counts[element.tag] += 1
        if element.tag == "BILLCL":
            text = (element.text or "").strip()
            if text == "":
                empty += 1
            else:
                try:
                    value = Decimal(text)
                except InvalidOperation:
                    raise Failed("a BILLCL is not a number: {!r}".format(text))
                total += value
                if value < 0:
                    negative += value
                else:
                    positive += value
        element.clear()
    if root is None:
        raise Failed("no root element")
    for tag in SEEN[1:]:
        if counts[tag] != counts["BILLFIXED"]:
            raise Failed("{} {} is not the BILLFIXED {}: a row lacks a scalar or holds a second".format(
                tag, counts[tag], counts["BILLFIXED"]))
    if rows is not None and counts["BILLFIXED"] != rows:
        raise Failed("BILLFIXED {} is not the {} expected".format(counts["BILLFIXED"], rows))
    return "bills ok bytes {} BILLFIXED {} BILLCL {} BILLDUE {} BILLOVERDUE {} empty_BILLCL {} sum {} negative {} positive {}".format(
        events.size, counts["BILLFIXED"], counts["BILLCL"], counts["BILLDUE"], counts["BILLOVERDUE"], empty,
        total, negative, positive)


MONTHS = {name: index for index, name in enumerate(
    ("Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"), 1)}
BILL_DATE = re.compile(r"([0-9]{1,2})-([A-Z][a-z]{2})-([0-9]{2})")


def bill_day(text):
    """BILLDATE as YYYYMMDD; two-digit years are 20yy (the books are 2025-26)."""
    match = BILL_DATE.fullmatch(text)
    if not match or match.group(2) not in MONTHS:
        raise Failed("a BILLDATE is not in the form Tally writes: {!r}".format(text))
    return "20{}{:02d}{:02d}".format(match.group(3), MONTHS[match.group(2)], int(match.group(1)))


def bill_rows(path):
    """Every row of a flat Bills answer, as (BILLDATE as YYYYMMDD, the whole row), after `check_bills` has passed it."""
    rows = []
    fixed = None
    scalars = []
    for event, element, path_now in events(path):
        if event != "end" or len(path_now) < 2 or path_now[0] != "ENVELOPE":
            continue
        if len(path_now) == 3:
            # A BILLFIXED field: read with its BILLFIXED, whose end comes after its fields'.
            continue
        if element.tag == "BILLFIXED":
            if fixed is not None:
                raise Failed("a BILLFIXED before the last row's scalars")
            fixed = {child.tag: child.text or "" for child in element}
            scalars = []
        elif element.tag in SEEN[1:]:
            if fixed is None:
                raise Failed("a {} before any BILLFIXED".format(element.tag))
            scalars.append((element.text or "").strip())
            if len(scalars) == 3:
                day = bill_day((fixed.get("BILLDATE") or "").strip())
                rows.append((day, ((fixed.get("BILLDATE") or "").strip(), (fixed.get("BILLREF") or "").strip(),
                                   fixed.get("BILLPARTY") or "", tuple(scalars))))
                fixed = None
        element.clear()
    if fixed is not None:
        raise Failed("the last row has fewer than three scalars")
    return rows


def compare(whole_path, part_path, first_day):
    if not re.fullmatch(r"20[0-9]{6}", first_day):
        raise Failed("the first day is not YYYYMMDD: {!r}".format(first_day))
    check_bills(whole_path, None)
    check_bills(part_path, None)
    from collections import Counter
    whole = bill_rows(whole_path)
    part = bill_rows(part_path)
    expected = Counter(row for day, row in whole if day >= first_day)
    got = Counter(row for _, row in part)
    before = sum(1 for day, _ in part if day < first_day)
    matched = sum((expected & got).values())
    only_part = sum((got - expected).values())
    only_whole = sum((expected - got).values())
    return "compare ok whole_rows {} part_rows {} part_rows_before_first_day {} whole_rows_from_first_day {} matched {} only_in_part {} only_in_whole {} subset {}".format(
        len(whole), len(part), before, sum(expected.values()), matched, only_part, only_whole,
        "yes" if only_part == 0 and only_whole == 0 else "no")


def main(argv):
    if len(argv) in (2, 3) and argv[0] == "bills":
        return check_bills(argv[1], int(argv[2]) if len(argv) == 3 else None)
    if len(argv) == 4 and argv[0] == "compare":
        return compare(argv[1], argv[2], argv[3])
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
