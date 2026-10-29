#!/usr/bin/env python3
"""Writes this sheet's requests from the two committed templates, and names the product's census span.

The templates are the product's own requests, byte for byte, as captured live on 26 Oct 2026 and committed on master as
src-tauri/crates/bridge-tally-protocol/tests/fixtures/agent/spanned-census-two-books/ (o-c2.request: the voucher census
spanned to an AlterID range; o-m.request: the company high-water read). Only four things vary, each admitted by a closed rule:
  - the company text (SVCURRENTCOMPANY): read from a file on the lab PC (UTF-8; a byte-order mark that Notepad adds is dropped), XML-escaped ONCE as text with the product's escape set
    (& < > " ' CR LF); it never appears in this sheet, in an argument or in anything printed;
  - the two dates (they land inside a quoted $$Date:"..." formula literal, where escaping does not protect, so only eight digits
    forming a real calendar date are admitted, and the from-date may not be after the to-date; any day of the month, as a
    licensed TallyPrime accepts and the product sends; this sheet runs in licensed mode only);
  - the span (lo, hi]: non-negative integers, lo < hi, and hi - lo at most 8192, the product's own census capacity.
`aligned` prints the span the product itself sends for a given AlterID: (k * 8192, min((k + 1) * 8192, mark)], the k-th of
`census_spans` in agent_voucher_window.rs, and with --after the next one. It prints the two bounds: they stay on the screen.
Nothing read from Tally chooses the report, the collection, the fetch list or the formula.

  python -I gen_census.py --check
  python -I gen_census.py census --company-file F --from 20250401 --to 20260331 --lo N --hi N --out PATH
  python -I gen_census.py marks --company-file F --out PATH
  python -I gen_census.py aligned --mark V --at N         (the product span holding AlterID N)
  python -I gen_census.py aligned --mark V --after HI     (the product span after (.., HI])

`census` and `marks` print only the size in bytes of the file written. `aligned` prints `lo hi` only.
"""
import argparse
import datetime
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CENSUS_TEMPLATE = os.path.join(HERE, "templates", "census-spanned.utf16le.xml")
MARKS_TEMPLATE = os.path.join(HERE, "templates", "high-water.utf16le.xml")
T_COMPANY = "BRIDGE OUTSTANDINGS LAB"
T_FROM, T_TO, T_LO, T_HI = "20250420", "20260630", 0, 21
FORMULA = "$AlterID &gt; %d AND $AlterID &lt;= %d"
PRODUCT_CAPACITY = 8192            # CENSUS_WIRE_BYTES_PER_VOUCHER 4 KiB against the 32 MiB cap (agent_voucher_window.rs)
CAP_BYTES = 32 * 1024 * 1024       # the transport's response cap


def escape_text(value):
    return (value.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace('"', "&quot;")
            .replace("'", "&apos;").replace("\r", "&#13;").replace("\n", "&#10;"))


def template(path):
    data = open(path, "rb").read()
    if data[:2] != b"\xff\xfe":
        raise SystemExit("a template has no UTF-16LE byte-order mark")
    return data[2:].decode("utf-16-le")


def once(text, old, new):
    if text.count(old) != 1:
        raise SystemExit("the template is not the shape this script was written for")
    return text.replace(old, new)


def date(value):
    if not re.fullmatch(r"[0-9]{8}", value):
        raise SystemExit("a date is not eight digits")
    try:
        datetime.date(int(value[:4]), int(value[4:6]), int(value[6:]))
    except ValueError:
        raise SystemExit("a date is not a real calendar date")
    return value


def window(frm, to):
    frm, to = date(frm), date(to)
    if frm > to:
        raise SystemExit("the from-date is after the to-date")
    return frm, to


def aligned(mark, at=None, after=None):
    if mark < 1 or (at is None) == (after is None):
        raise SystemExit("aligned needs a mark of at least 1 and exactly one of --at and --after")
    if at is not None:
        if not 1 <= at <= mark:
            raise SystemExit("--at is not an AlterID of the book (1 to the mark)")
        lo = (at - 1) // PRODUCT_CAPACITY * PRODUCT_CAPACITY
    else:
        if after % PRODUCT_CAPACITY != 0 or after >= mark:
            raise SystemExit("--after is not the upper bound of a product span below the mark")
        lo = after
    return lo, min(lo + PRODUCT_CAPACITY, mark)


def company(name):
    if not name or len(name) > 200 or any(ord(c) < 32 and c not in "\r\n" for c in name) or "\x00" in name:
        raise SystemExit("the company text is empty, too long or holds a control character")
    return name


def build_census(name, frm, to, lo, hi):
    text = template(CENSUS_TEMPLATE)
    if not (isinstance(lo, int) and isinstance(hi, int) and 0 <= lo < hi and hi - lo <= PRODUCT_CAPACITY):
        raise SystemExit("the span is outside the product's capacity")
    text = once(text, "<SVCURRENTCOMPANY>%s</SVCURRENTCOMPANY>" % T_COMPANY, "<SVCURRENTCOMPANY>%s</SVCURRENTCOMPANY>" % escape_text(name))
    if text.count(T_FROM) != 2 or text.count(T_TO) != 2:
        raise SystemExit("the template is not the shape this script was written for")
    text = text.replace(T_FROM, frm).replace(T_TO, to)
    text = once(text, FORMULA % (T_LO, T_HI), FORMULA % (lo, hi))
    return b"\xff\xfe" + text.encode("utf-16-le")


def build_marks(name):
    text = template(MARKS_TEMPLATE)
    text = once(text, "<SVCURRENTCOMPANY>%s</SVCURRENTCOMPANY>" % T_COMPANY, "<SVCURRENTCOMPANY>%s</SVCURRENTCOMPANY>" % escape_text(name))
    return b"\xff\xfe" + text.encode("utf-16-le")


def check():
    c = open(CENSUS_TEMPLATE, "rb").read()
    m = open(MARKS_TEMPLATE, "rb").read()
    assert build_census(T_COMPANY, T_FROM, T_TO, T_LO, T_HI) == c, "the census template is not reproduced"
    assert build_marks(T_COMPANY) == m, "the marks template is not reproduced"
    odd = build_census('A & B "C"', "20250401", "20260331", 100, 350)
    text = odd[2:].decode("utf-16-le")
    assert "A &amp; B &quot;C&quot;" in text and "&amp;amp;" not in text, "the company text is not escaped exactly once"
    assert "$AlterID &gt; 100 AND $AlterID &lt;= 350" in text and text.count("20250401") == 2 and text.count("20260331") == 2
    import xml.etree.ElementTree as ET
    ET.fromstring(text)
    day = build_census("X", *window("20250805", "20250805"), 0, 8192)[2:].decode("utf-16-le")
    assert day.count("20250805") == 4 and "$AlterID &gt; 0 AND $AlterID &lt;= 8192" in day, "a one-day window is not rendered"
    for bad in (("20250806", "20250805"), ("20250231", "20250301"), ("2025080", "20250805")):
        try:
            window(*bad)
        except SystemExit:
            continue
        raise AssertionError("a bad window was admitted")
    assert aligned(29900, at=3737) == (0, 8192) and aligned(29900, at=8192) == (0, 8192)
    assert aligned(29900, at=8193) == (8192, 16384) and aligned(29900, after=24576) == (24576, 29900)
    assert aligned(2000, at=250) == (0, 2000)
    for args in ({"at": 0}, {"at": 29901}, {"after": 100}, {"after": 32768}, {}):
        try:
            aligned(29900, **args)
        except SystemExit:
            continue
        raise AssertionError("a bad span request was admitted")
    print("gen_census: ok (the census and marks templates reproduced byte for byte; an ampersand and quotes escaped once; the span, dates, a one-day window, the product-aligned spans and parse checked)")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    sub = parser.add_subparsers(dest="mode")
    c = sub.add_parser("census")
    c.add_argument("--company-file", required=True)
    c.add_argument("--from", dest="frm", required=True)
    c.add_argument("--to", required=True)
    c.add_argument("--lo", type=int, required=True)
    c.add_argument("--hi", type=int, required=True)
    c.add_argument("--out", required=True)
    m = sub.add_parser("marks")
    m.add_argument("--company-file", required=True)
    m.add_argument("--out", required=True)
    a = sub.add_parser("aligned")
    a.add_argument("--mark", type=int, required=True)
    a.add_argument("--at", type=int)
    a.add_argument("--after", type=int)
    args = parser.parse_args()
    if args.check:
        check()
        return 0
    if args.mode == "aligned":
        print("%d %d" % aligned(args.mark, args.at, args.after))
        return 0
    if args.mode in ("census", "marks"):
        name = company(open(args.company_file, encoding="utf-8-sig").read().rstrip("\r\n"))
        data = build_census(name, *window(args.frm, args.to), args.lo, args.hi) if args.mode == "census" else build_marks(name)
        with open(args.out, "xb") as handle:
            handle.write(data)
        print("request written (%d bytes)" % len(data))
        return 0
    print(__doc__)
    return 2


if __name__ == "__main__":
    sys.exit(main())
