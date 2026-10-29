#!/usr/bin/env python3
"""Writes this sheet's five Bills report requests, UTF-16LE with a byte-order mark, the way the transport sends them.

The request is `render_native_bills_request` (bridge-tally-protocol, native_outstandings/request.rs): a TYPE Data report whose
ID is `Bills Receivable` or `Bills Payable`, for BRIDGE CORPUS DENSE, from the books' first day (20250401) to the as-of day.
That is the request ComplyEaze Bridge's `outstandings` sends first and twice (the paired read) after its identity bracket, and
the same text with the ID `Bills Payable` after the groups snapshot. The only things that vary here are the two names and four as-of days below.

  r-1-receivable-20250630.utf16le.xml   control: a day the tool answered (7,432 open bills in its answer)
  r-2-receivable-20250930.utf16le.xml
  r-3-receivable-20251231.utf16le.xml
  r-4-receivable-20260331.utf16le.xml   the day the tool was refused: 770 bytes, the size of the request held 20,002 ms
  r-5-payable-20260331.utf16le.xml      the Payable report for the same day (764 bytes)

The set of report IDs, the company and the days are fixed in this script; nothing read from Tally or from outside chooses them.

Run:  python -I gen_requests.py --out requests
      python -I gen_requests.py --check
"""
import argparse
import os
import sys

COMPANY = "BRIDGE CORPUS DENSE"
FROM = "20250401"
KINDS = {"receivable": "Bills Receivable", "payable": "Bills Payable"}
DAYS = ("20250630", "20250930", "20251231", "20260331")
FORMAT = (
    "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Data</TYPE><ID>{id}</ID></HEADER>"
    "<BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>"
    "<SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY>"
    '<SVFROMDATE TYPE="Date">{start}</SVFROMDATE><SVTODATE TYPE="Date">{day}</SVTODATE>'
    "</STATICVARIABLES></DESC></BODY></ENVELOPE>"
)
JOBS = (
    ("r-1-receivable-20250630.utf16le.xml", "receivable", "20250630", 770),
    ("r-2-receivable-20250930.utf16le.xml", "receivable", "20250930", 770),
    ("r-3-receivable-20251231.utf16le.xml", "receivable", "20251231", 770),
    ("r-4-receivable-20260331.utf16le.xml", "receivable", "20260331", 770),
    ("r-5-payable-20260331.utf16le.xml", "payable", "20260331", 764),
)


def build(kind, day):
    if kind not in KINDS or day not in DAYS:
        raise SystemExit("outside the closed set: %r %r" % (kind, day))
    text = FORMAT.format(id=KINDS[kind], company=COMPANY, start=FROM, day=day)
    return b"\xff\xfe" + text.encode("utf-16-le")


def check():
    for name, kind, day, size in JOBS:
        data = build(kind, day)
        assert data[:2] == b"\xff\xfe", name
        assert len(data) == size, "%s is %d bytes, expected %d" % (name, len(data), size)
        text = data[2:].decode("utf-16-le")
        assert text.count("<ID>%s</ID>" % KINDS[kind]) == 1 and text.count("<SVTODATE TYPE=\"Date\">%s<" % day) == 1, name
    print("gen_requests: ok (5 requests; Receivable 770 bytes, Payable 764 bytes)")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--out")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    if args.check:
        check()
    if args.out:
        os.makedirs(args.out, exist_ok=True)
        for name, kind, day, _ in JOBS:
            with open(os.path.join(args.out, name), "wb") as handle:
                handle.write(build(kind, day))
    if not args.check and not args.out:
        print(__doc__)
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main())
