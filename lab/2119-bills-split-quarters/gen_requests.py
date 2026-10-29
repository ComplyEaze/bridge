#!/usr/bin/env python3
"""Writes this sheet's four Bills Receivable requests, UTF-16LE with a byte-order mark, the way the transport sends them.

The request is `render_native_bills_request` (bridge-tally-protocol, native_outstandings/request.rs): a TYPE Data report whose
ID is `Bills Receivable`, for BRIDGE CORPUS DENSE, as of 31 March 2026. ComplyEaze Bridge always sends it from the books'
first day (20250401); this sheet varies only that first day, SVFROMDATE, over the four quarter starts of the books' first
year. Every other byte is the product's.

  q-1-from-20250401.utf16le.xml   the product's own request (the same bytes as r-4 of lab/2119-bills-read-timing)
  q-2-from-20250701.utf16le.xml
  q-3-from-20251001.utf16le.xml
  q-4-from-20260101.utf16le.xml

The report ID, the company, the as-of day and the four first days are fixed in this script; nothing read from Tally or from
outside chooses them. Each request is 770 bytes.

Run:  python -I gen_requests.py --out requests
      python -I gen_requests.py --check
"""
import argparse
import hashlib
import os
import sys

COMPANY = "BRIDGE CORPUS DENSE"
AS_OF = "20260331"
FIRST_DAYS = ("20250401", "20250701", "20251001", "20260101")
FORMAT = (
    "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Data</TYPE><ID>Bills Receivable</ID></HEADER>"
    "<BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>"
    "<SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY>"
    '<SVFROMDATE TYPE="Date">{start}</SVFROMDATE><SVTODATE TYPE="Date">{day}</SVTODATE>'
    "</STATICVARIABLES></DESC></BODY></ENVELOPE>"
)
JOBS = tuple(("q-%d-from-%s.utf16le.xml" % (index, start), start) for index, start in enumerate(FIRST_DAYS, 1))
# The product's request as the earlier sheet committed it (lab/2119-bills-read-timing, r-4-receivable-20260331).
PRODUCT_SHA256 = "6520d94ec6d013e3e44d9af62ee9c9830a65085b4cb5b54710340df0d03c750f"


def build(start):
    if start not in FIRST_DAYS:
        raise SystemExit("outside the closed set: %r" % (start,))
    text = FORMAT.format(company=COMPANY, start=start, day=AS_OF)
    return b"\xff\xfe" + text.encode("utf-16-le")


def check():
    for name, start in JOBS:
        data = build(start)
        assert len(data) == 770, "%s is %d bytes, expected 770" % (name, len(data))
        text = data[2:].decode("utf-16-le")
        assert text.count('<SVFROMDATE TYPE="Date">%s<' % start) == 1, name
        assert text.count('<SVTODATE TYPE="Date">%s<' % AS_OF) == 1, name
    assert hashlib.sha256(build(FIRST_DAYS[0])).hexdigest() == PRODUCT_SHA256, "q-1 is not the product's request"
    print("gen_requests: ok (4 requests of 770 bytes; q-1 is the product's request)")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--out")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    if args.check:
        check()
    if args.out:
        os.makedirs(args.out, exist_ok=True)
        for name, start in JOBS:
            with open(os.path.join(args.out, name), "wb") as handle:
                handle.write(build(start))
    if not args.check and not args.out:
        print(__doc__)
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main())
