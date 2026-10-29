"""Reads this sheet's Bills Receivable / Bills Payable answers by LAB-COMMON C4, streaming, so an answer of tens of
megabytes is never held whole.

Run with `python -I read.py bills <answer> [<rows>]` from this folder. Exit 0 = the answer parsed and holds no STATUS
element, 1 = it failed (the line printed says why; the sheet treats it as STOP), 2 = the command line is wrong.

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


def main(argv):
    if len(argv) in (2, 3) and argv[0] == "bills":
        return check_bills(argv[1], int(argv[2]) if len(argv) == 3 else None)
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
