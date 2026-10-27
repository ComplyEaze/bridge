#!/usr/bin/env python3 -I
"""Writes the three raw reads of the mailing-details capture sheet, from the one request ComplyEaze Bridge sent and
that was committed on 12 Oct 2026 (templates/request-4c.utf16le.xml, sha256 1bf17a26..., the party-master read that
names LEDMAILINGDETAILS.LIST, company "Bridge Lab Win 0.5.1", SVFROMDATE 20260401, SVTODATE 20260831).

Run:  python -I gen_requests.py --out requests [--company NAME]
      python -I gen_requests.py --check

Each output is the template with exactly two things changed, and nothing else:
  - the company text, when --company is given;
  - SVTODATE, the end of the period the request names: 20260831 (the template's own), 20260601, and any date --to gives.
and one with the FETCH suffix ComplyEaze Bridge appends when the caller asks for the dated GST details
(`render_party_ledger_master_request_with_gst_details`, pull request 1675 head d6d89545), added after
LEDMAILINGDETAILS.LIST. The date must be a real date whose day is 1, 2 or 31 (a day-30 boundary was refused by Tally),
and the company name plain letters, digits, space and . _ ( ) -.

The period of the request the built tool sends is the book's own (its books-from date to its last voucher date), not a
date the caller gives; a file here is the tool's request only when its SVTODATE is that last voucher date, and the
sheet compares sizes and the book's dates before it says so.

--check builds the files in memory for the template's own company and dates and fails unless the 20260831 file equals
the template byte for byte and each other file differs from it in exactly the stated places.
"""
import argparse
import datetime
import hashlib
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
TEMPLATE = os.path.join(HERE, "templates", "request-4c.utf16le.xml")
TEMPLATE_SHA256 = "1bf17a26c209253cb85a1725037cbf6000df4c6bc5ba77bfdd41ff65ab1ae8fa"
TEMPLATE_COMPANY = "Bridge Lab Win 0.5.1"
TEMPLATE_TO = "20260831"
FETCH_END = "LEDMAILINGDETAILS.LIST</FETCH>"
GST_SUFFIX = ", GSTDETAILS.LIST, RATEOFTAXCALCULATION, ROUNDINGMETHOD, ROUNDINGLIMIT"
PLAIN = re.compile(r"[A-Za-z0-9._()-]+(?: [A-Za-z0-9._()-]+)*")


def template_text():
    data = open(TEMPLATE, "rb").read()
    if hashlib.sha256(data).hexdigest() != TEMPLATE_SHA256 or data[:2] != b"\xff\xfe":
        raise SystemExit("the template is not the committed request (sha256 differs)")
    text = data[2:].decode("utf-16-le")
    if (text.count(TEMPLATE_COMPANY) != 1 or text.count('<SVTODATE TYPE="Date">%s</SVTODATE>' % TEMPLATE_TO) != 1
            or text.count(FETCH_END) != 1):
        raise SystemExit("the template is not the shape this generator was written for")
    return text


def check_to_date(value):
    try:
        day = datetime.datetime.strptime(value, "%Y%m%d")
    except ValueError:
        raise SystemExit("not a date: %r" % value)
    if day.day not in (1, 2, 31) or not re.fullmatch(r"[0-9]{8}", value):
        raise SystemExit("the as-of day must be 1, 2 or 31: %r" % value)


def build(company, to_date, gst):
    if len(company) > 100 or not PLAIN.fullmatch(company):
        raise SystemExit("company name not plain: %r" % company)
    check_to_date(to_date)
    text = template_text()
    text = text.replace(TEMPLATE_COMPANY, company)
    text = text.replace('<SVTODATE TYPE="Date">%s</SVTODATE>' % TEMPLATE_TO, '<SVTODATE TYPE="Date">%s</SVTODATE>' % to_date)
    if gst:
        text = text.replace(FETCH_END, "LEDMAILINGDETAILS.LIST%s</FETCH>" % GST_SUFFIX)
    return b"\xff\xfe" + text.encode("utf-16-le")


def files(company, to_date="20260831"):
    out = [("q-to-%s.utf16le.xml" % to_date, build(company, to_date, False)),
           ("q-gst-to-%s.utf16le.xml" % to_date, build(company, to_date, True))]
    if to_date != "20260601":
        out.insert(1, ("q-to-20260601.utf16le.xml", build(company, "20260601", False)))
    return out


def check():
    template = open(TEMPLATE, "rb").read()
    built = dict(files(TEMPLATE_COMPANY))
    assert built["q-to-20260831.utf16le.xml"] == template, "the 20260831 file is not the template"
    other = built["q-to-20260601.utf16le.xml"].decode("utf-16")
    assert other == template.decode("utf-16").replace("20260831</SVTODATE>", "20260601</SVTODATE>"), "20260601 differs elsewhere"
    gst = built["q-gst-to-20260831.utf16le.xml"].decode("utf-16")
    assert gst == template.decode("utf-16").replace("LEDMAILINGDETAILS.LIST</FETCH>",
                                                    "LEDMAILINGDETAILS.LIST%s</FETCH>" % GST_SUFFIX), "gst differs elsewhere"
    assert gst.count("GSTDETAILS.LIST") == 1 + 0 and gst.count("LEDGSTREGDETAILS.LIST") == 1
    for bad in ("20260830", "20260229", "2026083", "2026-08-31", "20260631"):
        try:
            check_to_date(bad)
        except SystemExit:
            continue
        raise AssertionError("an as-of date was accepted: " + bad)
    for bad in ('A"B', "A<B", "A&B", "", " A", "A  B"):
        try:
            build(bad, "20260831", False)
        except SystemExit:
            continue
        raise AssertionError("a company name was accepted: %r" % bad)
    print("gen_requests: ok (3 requests; the 20260831 file equals the committed request %s)" % TEMPLATE_SHA256[:8])
    return 0


def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument("--out")
    parser.add_argument("--company", default=TEMPLATE_COMPANY)
    parser.add_argument("--to", default="20260831")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args(argv[1:])
    if args.check:
        try:
            return check()
        except AssertionError as error:
            print("gen_requests FAILED: %s" % error)
            return 1
    if not args.out:
        print("usage: gen_requests.py --out DIR [--company NAME] | --check")
        return 2
    os.makedirs(args.out, exist_ok=True)
    for name, data in files(args.company, args.to):
        path = os.path.join(args.out, name)
        with open(path, "wb") as out:
            out.write(data)
        print("%s  %d  %s" % (hashlib.sha256(data).hexdigest(), len(data), name))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
