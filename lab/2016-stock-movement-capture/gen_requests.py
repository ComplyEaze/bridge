#!/usr/bin/env python3 -I
"""Writes this sheet's reads from three committed requests, each with only the company text and the dates changed.

  templates/a07-vouchers-shape-20250424.xml   the shipped `Bridge Agent Vouchers` request (lab/missing-reads-answers a07)
  templates/a08-inventory-shape-20250424.xml  the same request whose FETCH asks `ALLINVENTORYENTRIES.*` (a08, the
                                              lab inventory fetch #2016 names; one Purchase read through it is the
                                              only capture of this shape)
  templates/stock_summary_request.utf16le.xml the Stock Summary report request (fixture stock_summary_report_fy_request)

Run:  python -I gen_requests.py --out requests --days 20260511 20260512 20260513 20260514
          [--company "BRIDGE SHAPE LAB"] [--summary-from 20260401 --summary-to 20270331]
      python -I gen_requests.py --check

For each day D it writes r-a07-D.utf16le.xml and r-a08-D.utf16le.xml (the day replaces the template's 20250424 in the
four places it stands: the two date variables and the two bounds of the window formula), and it writes
r-stock-summary.utf16le.xml for the period given (default the template's own). Days must be real dates; the Stock
Summary period's day must be 1, 2 or 31 (a day-30 boundary was refused by Tally); the voucher window uses literal dates in
its formula, so a day-of-month boundary does not widen it. --check proves each output differs from its template only in
those places.
"""
import argparse
import datetime
import hashlib
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
T = {
    "a07": os.path.join(HERE, "templates", "a07-vouchers-shape-20250424.xml"),
    "a08": os.path.join(HERE, "templates", "a08-inventory-shape-20250424.xml"),
    "sum": os.path.join(HERE, "templates", "stock_summary_request.utf16le.xml"),
}
OLD_COMPANY, OLD_DAY = "BRIDGE SHAPE LAB", "20250424"
SUM_FROM, SUM_TO = "20250401", "20260331"
PLAIN = re.compile(r"[A-Za-z0-9._()-]+(?: [A-Za-z0-9._()-]+)*")


def real_date(value):
    try:
        datetime.datetime.strptime(value, "%Y%m%d")
    except ValueError:
        raise SystemExit("not a date: %r" % value)
    if not re.fullmatch(r"[0-9]{8}", value):
        raise SystemExit("not a date: %r" % value)


def text_of(path):
    data = open(path, "rb").read()
    if data[:2] != b"\xff\xfe":
        raise SystemExit("%s has no UTF-16LE byte-order mark" % os.path.basename(path))
    return data[2:].decode("utf-16-le")


def encode(text):
    return b"\xff\xfe" + text.encode("utf-16-le")


def company_checked(company):
    if len(company) > 100 or not PLAIN.fullmatch(company):
        raise SystemExit("company name not plain: %r" % company)


def voucher_request(kind, company, day):
    company_checked(company)
    real_date(day)
    text = text_of(T[kind])
    if text.count(OLD_DAY) != 4 or text.count(OLD_COMPANY) != 1:
        raise SystemExit("the %s template is not the shape this script was written for" % kind)
    return encode(text.replace(OLD_COMPANY, company).replace(OLD_DAY, day))


def summary_request(company, date_from, date_to):
    company_checked(company)
    for value in (date_from, date_to):
        real_date(value)
        if int(value[6:]) not in (1, 2, 31):
            raise SystemExit("the Stock Summary period's day must be 1, 2 or 31: %r" % value)
    text = text_of(T["sum"])
    for old in (OLD_COMPANY, '<SVFROMDATE TYPE="Date">%s<' % SUM_FROM, '<SVTODATE TYPE="Date">%s<' % SUM_TO):
        if text.count(old) != 1:
            raise SystemExit("the summary template is not the shape this script was written for")
    text = text.replace(OLD_COMPANY, company)
    text = text.replace('<SVFROMDATE TYPE="Date">%s<' % SUM_FROM, '<SVFROMDATE TYPE="Date">%s<' % date_from)
    text = text.replace('<SVTODATE TYPE="Date">%s<' % SUM_TO, '<SVTODATE TYPE="Date">%s<' % date_to)
    return encode(text)


def check():
    for kind in ("a07", "a08"):
        tpl = open(T[kind], "rb").read()
        assert voucher_request(kind, OLD_COMPANY, OLD_DAY) == tpl, kind + " not reproduced"
        out = voucher_request(kind, "BRIDGE X LAB", "20260512")
        assert out[2:].decode("utf-16-le").replace("BRIDGE X LAB", OLD_COMPANY).replace("20260512", OLD_DAY) == tpl[2:].decode("utf-16-le")
    tpl = open(T["sum"], "rb").read()
    assert summary_request(OLD_COMPANY, SUM_FROM, SUM_TO) == tpl, "summary not reproduced"
    for bad in ("20260230", "2026-05-11", "202605"):
        try:
            voucher_request("a07", OLD_COMPANY, bad)
        except SystemExit:
            continue
        raise AssertionError("a day was accepted: " + bad)
    try:
        summary_request(OLD_COMPANY, "20260401", "20270330")
    except SystemExit:
        pass
    else:
        raise AssertionError("a day-30 boundary was accepted")
    print("gen_requests: ok (the three templates are reproduced byte for byte; only company and dates change)")
    return 0


def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument("--out")
    parser.add_argument("--days", nargs="*", default=[])
    parser.add_argument("--company", default=OLD_COMPANY)
    parser.add_argument("--summary-from", default=SUM_FROM)
    parser.add_argument("--summary-to", default=SUM_TO)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args(argv[1:])
    if args.check:
        try:
            return check()
        except AssertionError as error:
            print("gen_requests FAILED: %s" % error)
            return 1
    if not args.out:
        print("usage: gen_requests.py --out DIR --days D... [--company N] [--summary-from D --summary-to D] | --check")
        return 2
    os.makedirs(args.out, exist_ok=True)
    jobs = []
    for day in args.days:
        jobs.append(("r-a07-%s.utf16le.xml" % day, voucher_request("a07", args.company, day)))
        jobs.append(("r-a08-%s.utf16le.xml" % day, voucher_request("a08", args.company, day)))
    jobs.append(("r-stock-summary.utf16le.xml", summary_request(args.company, args.summary_from, args.summary_to)))
    for name, data in jobs:
        with open(os.path.join(args.out, name), "wb") as out:
            out.write(data)
        print("%s  %d  %s" % (hashlib.sha256(data).hexdigest(), len(data), name))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
