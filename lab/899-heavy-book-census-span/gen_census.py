#!/usr/bin/env python3
"""Writes this sheet's requests from the two committed templates, and sizes the next span.

The templates are the product's own requests, byte for byte, as captured live on 26 Oct 2026 and committed on master as
src-tauri/crates/bridge-tally-protocol/tests/fixtures/agent/spanned-census-two-books/ (o-c2.request: the voucher census
spanned to an AlterID range; o-m.request: the company high-water read). Only four things vary, each admitted by a closed rule:
  - the company text (SVCURRENTCOMPANY): read from a file on the lab PC (UTF-8; a byte-order mark that Notepad adds is dropped), XML-escaped ONCE as text with the product's escape set
    (& < > " ' CR LF); it never appears in this sheet, in an argument or in anything printed;
  - the two dates (they land inside a quoted $$Date:"..." formula literal, where escaping does not protect, so only eight digits
    forming a real calendar date are admitted; the from-day must be 1 or 2 and the to-day 31, the product's window rule);
  - the span (lo, hi]: non-negative integers, lo < hi, and hi - lo at most 8192, the product's own census capacity.
Nothing read from Tally chooses the report, the collection, the fetch list or the formula.

  python -I gen_census.py --check
  python -I gen_census.py census --company-file F --from 20250401 --to 20260331 --lo N --hi N --out PATH
  python -I gen_census.py marks --company-file F --out PATH
  python -I gen_census.py plan --prev-rows R --prev-width W --prev-bytes B --prev-seconds T --target ROWS [--room N]

`census` and `marks` print only the size in bytes of the file written. `plan` prints numbers only.
"""
import argparse
import datetime
import math
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
BYTES_LIMIT = CAP_BYTES // 2       # a span is planned so that its estimated bytes, with BYTES_MARGIN, fit half the cap
BYTES_MARGIN = 1.25
SECONDS_LIMIT = 15.0               # planned, against the product's 20 s request limit
SECONDS_MARGIN = 1.5


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


def date(value, day_ok):
    if not re.fullmatch(r"[0-9]{8}", value):
        raise SystemExit("a date is not eight digits")
    try:
        parsed = datetime.date(int(value[:4]), int(value[4:6]), int(value[6:]))
    except ValueError:
        raise SystemExit("a date is not a real calendar date")
    if parsed.day not in day_ok:
        raise SystemExit("a date has a day outside the window rule")
    return value


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
    print("gen_census: ok (the census and marks templates reproduced byte for byte; an ampersand and quotes escaped once; the span, dates and parse checked)")


def plan(a):
    reasons = []
    if a.prev_seconds > 20:
        print("STOP: the previous span took more than 20 s")
        return 1
    if a.prev_rows < 50 or a.prev_width < 1:
        print("STOP: the previous span holds too few rows to size the next one")
        return 1
    rho = a.prev_rows / a.prev_width
    per_row = a.prev_bytes / a.prev_rows          # includes the fixed envelope: an upper estimate for a larger span
    seconds_per_row = a.prev_seconds / a.prev_rows
    by_bytes = int(BYTES_LIMIT / (per_row * BYTES_MARGIN))
    by_seconds = int(SECONDS_LIMIT / (seconds_per_row * SECONDS_MARGIN)) if seconds_per_row > 0 else a.target
    rows = min(a.target, by_bytes, by_seconds)
    if rows == by_bytes and rows < a.target:
        reasons.append("the 16 MiB byte limit")
    if rows == by_seconds and rows < a.target:
        reasons.append("the 15 s planning limit")
    width = min(PRODUCT_CAPACITY, max(1, math.ceil(rows / rho)))
    if width == PRODUCT_CAPACITY:
        reasons.append("the product's 8192-id capacity")
    predicted_rows = min(int(rows), int(width * rho))
    if a.room is not None and width > a.room:
        print("STOP: the next span would pass the voucher mark (room %d, width %d)" % (a.room, width))
        return 1
    if predicted_rows < 1.5 * a.prev_rows:
        print("STOP: no larger step is allowed (planned rows %d, previous %d)" % (predicted_rows, a.prev_rows))
        return 1
    print("next width %d ids; planned rows %d; planned bytes %d (with margin); planned seconds %.1f (with margin); limited by: %s"
          % (width, predicted_rows, int(predicted_rows * per_row * BYTES_MARGIN), predicted_rows * seconds_per_row * SECONDS_MARGIN, ", ".join(reasons) or "the target"))
    return 0


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
    p = sub.add_parser("plan")
    p.add_argument("--prev-rows", type=int, required=True)
    p.add_argument("--prev-width", type=int, required=True)
    p.add_argument("--prev-bytes", type=int, required=True)
    p.add_argument("--prev-seconds", type=float, required=True)
    p.add_argument("--target", type=int, required=True)
    p.add_argument("--room", type=int)
    args = parser.parse_args()
    if args.check:
        check()
        return 0
    if args.mode == "plan":
        return plan(args)
    if args.mode in ("census", "marks"):
        name = company(open(args.company_file, encoding="utf-8-sig").read().rstrip("\r\n"))
        data = build_census(name, date(args.frm, (1, 2)), date(args.to, (31,)), args.lo, args.hi) if args.mode == "census" else build_marks(name)
        with open(args.out, "xb") as handle:
            handle.write(data)
        print("request written (%d bytes)" % len(data))
        return 0
    print(__doc__)
    return 2


if __name__ == "__main__":
    sys.exit(main())
