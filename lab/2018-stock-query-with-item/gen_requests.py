#!/usr/bin/env python3
"""Writes this sheet's seven report requests from the one committed request in templates/ (the Stock Summary report request,
fixture stock_summary_report_fy_request: TYPE Data, ID Stock Summary, company BRIDGE SHAPE LAB, 20250401 to 20260331).
Each output differs from it only in the report ID and, for three of them, in ONE added static variable that names the item.

  r-1-stock-summary.utf16le.xml          the template itself (control: the request shape answers with data)
  r-2-unknown-id.utf16le.xml             ID `Bridge Probe Unknown` (control: what a refusal looks like)
  r-3-stock-query.utf16le.xml            ID `Stock Query`, no variable (the 2018 routes row's request; its answer was an empty envelope)
  r-4-stock-query-stockitemname.utf16le.xml   ID `Stock Query` and <STOCKITEMNAME>Hydrochloric Acid 33pc</STOCKITEMNAME>
  r-5-stock-query-svstockitem.utf16le.xml     ID `Stock Query` and <SVSTOCKITEM>Hydrochloric Acid 33pc</SVSTOCKITEM>
  r-6-stock-query-probe-variable.utf16le.xml  ID `Stock Query` and <BRIDGEPROBEVAR>Hydrochloric Acid 33pc</BRIDGEPROBEVAR>
                                              (control: a variable name Tally does not have)
  r-7-stock-query-probe-item.utf16le.xml      ID `Stock Query` and <STOCKITEMNAME>Bridge Probe Item</STOCKITEMNAME>
                                              (control: an item name the book does not have; a refusal that names the
                                              variable would show Tally knows it even if the report stays empty)

The two item variable names are candidates (unverified guesses at the names Tally's reports take for a chosen item: the
field name an inventory entry carries, and the SV prefix of Tally's system variables); the third is a control, and the
seventh request is a second control with an item name the book does not hold. The set of names and the two item names are fixed in this script; nothing from outside chooses
the report ID, a variable name or the item. The variable is inserted as the last element of STATICVARIABLES.

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
T_ID = "Stock Summary"
ITEM = "Hydrochloric Acid 33pc"
PROBE_ITEM = "Bridge Probe Item"
ITEMS = (ITEM, PROBE_ITEM)
PLAIN = re.compile(r"[A-Za-z0-9-]+(?: [A-Za-z0-9-]+)*")
VARIABLES = ("STOCKITEMNAME", "SVSTOCKITEM", "BRIDGEPROBEVAR")
JOBS = (
    ("r-1-stock-summary.utf16le.xml", "Stock Summary", None, ITEM),
    ("r-2-unknown-id.utf16le.xml", "Bridge Probe Unknown", None, ITEM),
    ("r-3-stock-query.utf16le.xml", "Stock Query", None, ITEM),
    ("r-4-stock-query-stockitemname.utf16le.xml", "Stock Query", "STOCKITEMNAME", ITEM),
    ("r-5-stock-query-svstockitem.utf16le.xml", "Stock Query", "SVSTOCKITEM", ITEM),
    ("r-6-stock-query-probe-variable.utf16le.xml", "Stock Query", "BRIDGEPROBEVAR", ITEM),
    ("r-7-stock-query-probe-item.utf16le.xml", "Stock Query", "STOCKITEMNAME", PROBE_ITEM),
)


def build(report_id, variable, item=ITEM):
    if len(report_id) > 60 or not PLAIN.fullmatch(report_id):
        raise SystemExit("report ID not plain: %r" % report_id)
    if variable is not None and variable not in VARIABLES:
        raise SystemExit("variable not in the closed set: %r" % variable)
    if item not in ITEMS or not PLAIN.fullmatch(item):
        raise SystemExit("item not in the closed set: %r" % item)
    data = open(TEMPLATE, "rb").read()
    if data[:2] != b"\xff\xfe":
        raise SystemExit("the template has no UTF-16LE byte-order mark")
    text = data[2:].decode("utf-16-le")
    if text.count("<ID>%s</ID>" % T_ID) != 1 or text.count("</STATICVARIABLES>") != 1:
        raise SystemExit("the template is not the shape this script was written for")
    text = text.replace("<ID>%s</ID>" % T_ID, "<ID>%s</ID>" % report_id)
    if variable is not None:
        text = text.replace("</STATICVARIABLES>", "<%s>%s</%s></STATICVARIABLES>" % (variable, item, variable))
    return b"\xff\xfe" + text.encode("utf-16-le")


def check():
    tpl = open(TEMPLATE, "rb").read()
    assert build(T_ID, None) == tpl, "the first output is not the template"
    base = tpl[2:].decode("utf-16-le")
    for name, report_id, variable, item in JOBS:
        out = build(report_id, variable, item)
        assert out[:2] == b"\xff\xfe"
        back = out[2:].decode("utf-16-le").replace("<ID>%s</ID>" % report_id, "<ID>%s</ID>" % T_ID)
        if variable is not None:
            added = "<%s>%s</%s>" % (variable, item, variable)
            assert back.count(added) == 1 and added + "</STATICVARIABLES>" in back, "variable not last: " + name
            back = back.replace(added, "")
        assert back == base, "differs elsewhere: " + name
    for bad_item in ("Bridge Probe Item2", "", "A&B"):
        try:
            build("Stock Query", "STOCKITEMNAME", bad_item)
        except SystemExit:
            continue
        raise AssertionError("an item was accepted: %r" % bad_item)
    for bad_id in ('A"B', "A<B", "A&B", "", " A", "A  B", "A/B"):
        try:
            build(bad_id, None)
        except SystemExit:
            continue
        raise AssertionError("a report ID was accepted: %r" % bad_id)
    for bad_var in ("STOCKITEM", "X", "", "STOCKITEMNAME>"):
        try:
            build("Stock Query", bad_var)
        except SystemExit:
            continue
        raise AssertionError("a variable was accepted: %r" % bad_var)
    committed = os.path.join(HERE, "requests")
    names = sorted(os.listdir(committed)) if os.path.isdir(committed) else []
    assert names == sorted(j[0] for j in JOBS), "requests/ holds %r" % names
    for name, report_id, variable, item in JOBS:
        assert open(os.path.join(committed, name), "rb").read() == build(report_id, variable, item), "committed request differs from a fresh one: " + name
    print("gen_requests: ok (7 requests, each the Stock Summary request with only the ID and at most one named-item variable changed)")
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
    for name, report_id, variable, item in JOBS:
        data = build(report_id, variable, item)
        with open(os.path.join(args.out, name), "wb") as out:
            out.write(data)
        print("%s  %d  %s" % (hashlib.sha256(data).hexdigest(), len(data), name))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
