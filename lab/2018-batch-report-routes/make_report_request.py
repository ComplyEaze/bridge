#!/usr/bin/env python3 -I
"""Writes the Stock Summary report request of ComplyEaze Bridge (templates/stock_summary_request.utf16le.xml, the
committed fixture stock_summary_report_fy_request: TYPE Data, ID Stock Summary, company BRIDGE SHAPE LAB, 20250401 to
20260331) with ONLY these changed: the report ID, the company, and the two dates. It exists so that a Tally report
other than the Stock Summary can be requested in the same shape, with the ID taken from the title Tally's own screen
shows for that report (never guessed): the sheet names no ID, it names the screen's title.

Run:  python -I make_report_request.py --id "<title read from Tally's screen>" --out FILE
          [--company "BRIDGE SHAPE LAB"] [--from 20250401] [--to 20260331]
      python -I make_report_request.py --check

The ID must be plain: letters, digits, spaces, hyphens, one or more words, no other character. The dates must be real
dates with the day 1, 2 or 31 (Tally refused a day-30 boundary). --check proves that the Stock Summary ID with the
template's own company and dates reproduces the committed fixture byte for byte and that a changed ID, company and
dates change those and nothing else.
"""
import argparse
import datetime
import hashlib
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
TEMPLATE = os.path.join(HERE, "templates", "stock_summary_request.utf16le.xml")
TEMPLATE_SHA256 = hashlib.sha256(open(TEMPLATE, "rb").read()).hexdigest()
PLAIN_ID = re.compile(r"[A-Za-z0-9-]+(?: [A-Za-z0-9-]+)*")
PLAIN_COMPANY = re.compile(r"[A-Za-z0-9._()-]+(?: [A-Za-z0-9._()-]+)*")
T_ID, T_COMPANY, T_FROM, T_TO = "Stock Summary", "BRIDGE SHAPE LAB", "20250401", "20260331"


def check_date(value):
    try:
        day = datetime.datetime.strptime(value, "%Y%m%d")
    except ValueError:
        raise SystemExit("not a date: %r" % value)
    if day.day not in (1, 2, 31) or not re.fullmatch(r"[0-9]{8}", value):
        raise SystemExit("the day must be 1, 2 or 31: %r" % value)


def build(report_id, company, date_from, date_to):
    if len(report_id) > 60 or not PLAIN_ID.fullmatch(report_id):
        raise SystemExit("report ID not plain: %r" % report_id)
    if len(company) > 100 or not PLAIN_COMPANY.fullmatch(company):
        raise SystemExit("company name not plain: %r" % company)
    check_date(date_from)
    check_date(date_to)
    if date_from > date_to:
        raise SystemExit("dates reversed")
    data = open(TEMPLATE, "rb").read()
    if data[:2] != b"\xff\xfe":
        raise SystemExit("the template has no UTF-16LE byte-order mark")
    text = data[2:].decode("utf-16-le")
    for old in ("<ID>%s</ID>" % T_ID, "<SVCURRENTCOMPANY>%s</SVCURRENTCOMPANY>" % T_COMPANY,
                '<SVFROMDATE TYPE="Date">%s</SVFROMDATE>' % T_FROM, '<SVTODATE TYPE="Date">%s</SVTODATE>' % T_TO):
        if text.count(old) != 1:
            raise SystemExit("the template is not the shape this script was written for")
    text = text.replace("<ID>%s</ID>" % T_ID, "<ID>%s</ID>" % report_id)
    text = text.replace("<SVCURRENTCOMPANY>%s</SVCURRENTCOMPANY>" % T_COMPANY, "<SVCURRENTCOMPANY>%s</SVCURRENTCOMPANY>" % company)
    text = text.replace('<SVFROMDATE TYPE="Date">%s</SVFROMDATE>' % T_FROM, '<SVFROMDATE TYPE="Date">%s</SVFROMDATE>' % date_from)
    text = text.replace('<SVTODATE TYPE="Date">%s</SVTODATE>' % T_TO, '<SVTODATE TYPE="Date">%s</SVTODATE>' % date_to)
    return b"\xff\xfe" + text.encode("utf-16-le")


def check():
    template = open(TEMPLATE, "rb").read()
    assert build(T_ID, T_COMPANY, T_FROM, T_TO) == template, "the Stock Summary request is not reproduced"
    other = build("Sample Report", "BRIDGE X LAB", "20250501", "20260801")
    assert other != template and other[:2] == b"\xff\xfe"
    back = other[2:].decode("utf-16-le").replace("Sample Report", T_ID).replace("BRIDGE X LAB", T_COMPANY).replace("20260801", T_TO).replace("20250501", T_FROM)
    assert back == template[2:].decode("utf-16-le"), "differs elsewhere"
    for bad_id in ('A"B', "A<B", "A&B", "", " A", "A  B", "A/B", "A.B"):
        try:
            build(bad_id, T_COMPANY, T_FROM, T_TO)
        except SystemExit:
            continue
        raise AssertionError("a report ID was accepted: %r" % bad_id)
    for bad_date in ("20260830", "20260229", "2026-08-31"):
        try:
            build(T_ID, T_COMPANY, T_FROM, bad_date)
        except SystemExit:
            continue
        raise AssertionError("a date was accepted: " + bad_date)
    print("make_report_request: ok (the Stock Summary request is reproduced byte for byte; only ID, company and dates change)")
    return 0


def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument("--id")
    parser.add_argument("--out")
    parser.add_argument("--company", default=T_COMPANY)
    parser.add_argument("--from", dest="date_from", default=T_FROM)
    parser.add_argument("--to", dest="date_to", default=T_TO)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args(argv[1:])
    if args.check:
        try:
            return check()
        except AssertionError as error:
            print("make_report_request FAILED: %s" % error)
            return 1
    if not args.id or not args.out:
        print("usage: make_report_request.py --id TITLE --out FILE [--company N] [--from D] [--to D] | --check")
        return 2
    if os.path.exists(args.out):
        print("refused: %s exists" % args.out)
        return 2
    data = build(args.id, args.company, args.date_from, args.date_to)
    with open(args.out, "wb") as out:
        out.write(data)
    print("%s  %d  %s" % (hashlib.sha256(data).hexdigest(), len(data), os.path.basename(args.out)))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
