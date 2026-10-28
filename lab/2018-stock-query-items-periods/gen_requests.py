#!/usr/bin/env python3
"""Writes this sheet's sixteen report requests from the one committed request in templates/ (the Stock Summary report
request, fixture stock_summary_report_fy_request: TYPE Data, ID Stock Summary, company BRIDGE SHAPE LAB, 20250401 to
20260331). Each output differs from it only in the report ID, in ONE added static variable STOCKITEMNAME that names an
item, and (for three) in the value of SVTODATE.

  r-00-stock-summary                 the template itself (control: the request shape answers with data)
  r-01 .. r-11 stock-query-<item>    ID `Stock Query` and <STOCKITEMNAME><item></STOCKITEMNAME>, the book's eleven stock items, period
                                     20250401 to 20260331 (the year)
  r-12-stock-query-hcl-to-20250401   Hydrochloric Acid 33pc, period 20250401 to 20250401 (the opening day, before the 24 April purchase)
  r-13-stock-query-zero-stock-item-to-20250401   Zero Stock Item, the same opening-day period (no opening stock; closes at -50.000 Kgs)
  r-14-stock-query-cleaning-kit-a-to-20250401    Cleaning Kit A, the same period (a batch item that goes from 25 to 12 Nos in the year)
  r-15-stock-query-unknown-item      ID `Stock Query`, <STOCKITEMNAME>Bridge Probe Item</STOCKITEMNAME> (control: an item the book lacks)

The item names (eleven stock items of the lab book and one control name), the two extra period ends and the one variable
name are fixed in this script; nothing from outside chooses the report ID, the variable, an item or a date. The variable is
inserted as the last element of STATICVARIABLES. The names are plain (letters, digits, spaces, hyphens), so no escaping is needed.

Run:  python -I gen_requests.py --out requests
      python -I gen_requests.py --check
"""
import argparse
import hashlib
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
TEMPLATE = os.path.join(HERE, "templates", "stock_summary_request.utf16le.xml")
T_ID, T_FROM, T_TO = "Stock Summary", "20250401", "20260331"
PLAIN = re.compile(r"[A-Za-z0-9-]+(?: [A-Za-z0-9-]+)*")
VARIABLE = "STOCKITEMNAME"
ITEMS = (
    ("r-01-stock-query-carton-box-small", "Carton Box Small"),
    ("r-02-stock-query-caustic-soda-flakes", "Caustic Soda Flakes"),
    ("r-03-stock-query-cleaning-kit-a", "Cleaning Kit A"),
    ("r-04-stock-query-cleaning-kit-b", "Cleaning Kit B"),
    ("r-05-stock-query-descaler-concentrate", "Descaler Concentrate"),
    ("r-06-stock-query-hdpe-drum-50l", "HDPE Drum 50L"),
    ("r-07-stock-query-hydrochloric-acid-33pc", "Hydrochloric Acid 33pc"),
    ("r-08-stock-query-label-roll", "Label Roll"),
    ("r-09-stock-query-soda-ash-light", "Soda Ash Light"),
    ("r-10-stock-query-sulphuric-acid-98pc", "Sulphuric Acid 98pc"),
    ("r-11-stock-query-zero-stock-item", "Zero Stock Item"),
)
HCL = "Hydrochloric Acid 33pc"
PROBE = "Bridge Probe Item"
TO_DATES = ("20250401", "20260331")
JOBS = (
    ("r-00-stock-summary", "Stock Summary", None, T_TO),
    *((name, "Stock Query", item, T_TO) for name, item in ITEMS),
    ("r-12-stock-query-hcl-to-20250401", "Stock Query", HCL, "20250401"),
    ("r-13-stock-query-zero-stock-item-to-20250401", "Stock Query", "Zero Stock Item", "20250401"),
    ("r-14-stock-query-cleaning-kit-a-to-20250401", "Stock Query", "Cleaning Kit A", "20250401"),
    ("r-15-stock-query-unknown-item", "Stock Query", PROBE, T_TO),
)
ALLOWED_ITEMS = tuple(i for _, i in ITEMS) + (PROBE,)


def build(report_id, item, to_date):
    if len(report_id) > 60 or not PLAIN.fullmatch(report_id) or report_id not in (T_ID, "Stock Query"):
        raise SystemExit("report ID not in the closed set: %r" % report_id)
    if item is not None and (item not in ALLOWED_ITEMS or not PLAIN.fullmatch(item)):
        raise SystemExit("item not in the closed set: %r" % item)
    if to_date not in TO_DATES:
        raise SystemExit("period end not in the closed set: %r" % to_date)
    data = open(TEMPLATE, "rb").read()
    if data[:2] != b"\xff\xfe":
        raise SystemExit("the template has no UTF-16LE byte-order mark")
    text = data[2:].decode("utf-16-le")
    for old in ("<ID>%s</ID>" % T_ID, '<SVTODATE TYPE="Date">%s<' % T_TO, "</STATICVARIABLES>"):
        if text.count(old) != 1:
            raise SystemExit("the template is not the shape this script was written for")
    text = text.replace("<ID>%s</ID>" % T_ID, "<ID>%s</ID>" % report_id)
    text = text.replace('<SVTODATE TYPE="Date">%s<' % T_TO, '<SVTODATE TYPE="Date">%s<' % to_date)
    if item is not None:
        text = text.replace("</STATICVARIABLES>", "<%s>%s</%s></STATICVARIABLES>" % (VARIABLE, item, VARIABLE))
    return b"\xff\xfe" + text.encode("utf-16-le")


def check():
    tpl = open(TEMPLATE, "rb").read()
    assert build(T_ID, None, T_TO) == tpl, "the first output is not the template"
    base = tpl[2:].decode("utf-16-le")
    assert len({name for name, *_ in JOBS}) == 16 and len(ALLOWED_ITEMS) == 12
    for name, report_id, item, to_date in JOBS:
        out = build(report_id, item, to_date)
        assert out[:2] == b"\xff\xfe"
        back = out[2:].decode("utf-16-le").replace("<ID>%s</ID>" % report_id, "<ID>%s</ID>" % T_ID)
        back = back.replace('<SVTODATE TYPE="Date">%s<' % to_date, '<SVTODATE TYPE="Date">%s<' % T_TO)
        if item is not None:
            added = "<%s>%s</%s>" % (VARIABLE, item, VARIABLE)
            assert back.count(added) == 1 and added + "</STATICVARIABLES>" in back, "variable not last: " + name
            back = back.replace(added, "")
        assert back == base, "differs elsewhere: " + name
    for bad in (("Stock Query", "Carton Box", T_TO), ("Stock Query", "A&B", T_TO), ("Godown Summary", None, T_TO),
                ("Stock Query", HCL, "20250501"), ("Stock Query", "", T_TO)):
        try:
            build(*bad)
        except SystemExit:
            continue
        raise AssertionError("a request was accepted: %r" % (bad,))
    committed = os.path.join(HERE, "requests")
    names = sorted(os.listdir(committed)) if os.path.isdir(committed) else []
    assert names == sorted(n + ".utf16le.xml" for n, *_ in JOBS), "requests/ holds %r" % names
    for name, report_id, item, to_date in JOBS:
        assert open(os.path.join(committed, name + ".utf16le.xml"), "rb").read() == build(report_id, item, to_date), "committed request differs from a fresh one: " + name
    print("gen_requests: ok (16 requests, each the Stock Summary request with only the ID, at most one item and the period end changed)")
    return 0


def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument("--out")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args(argv[1:])
    if args.check:
        try:
            return check()
        except AssertionError as error:
            print("gen_requests FAILED: %s" % error)
            return 1
    if not args.out:
        print("usage: gen_requests.py --out DIR | --check")
        return 2
    os.makedirs(args.out, exist_ok=True)
    for name, report_id, item, to_date in JOBS:
        data = build(report_id, item, to_date)
        with open(os.path.join(args.out, name + ".utf16le.xml"), "wb") as out:
            out.write(data)
        print("%s  %d  %s.utf16le.xml" % (hashlib.sha256(data).hexdigest(), len(data), name))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
