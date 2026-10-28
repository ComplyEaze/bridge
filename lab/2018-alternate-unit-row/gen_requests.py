#!/usr/bin/env python3
"""Writes this sheet's six requests from two committed requests in templates/ (the Stock Summary report request and the stock
items collection request, fixtures stock_summary_report_fy_request and stock_items_fy_request: company BRIDGE SHAPE LAB,
20250401 to 20260331).

  a-00-stock-summary                   the Stock Summary request itself
  a-01-items-fy                        the stock items collection request itself (the fixture, byte for byte)
  a-02-stock-query-carton-box-small    ID `Stock Query` + <STOCKITEMNAME>Carton Box Small</STOCKITEMNAME> (an existing item whose unit is Box)
  a-03-stock-query-unknown-item        the same with `Bridge Probe Item` (control: an item the book lacks)
  a-04-stock-query-alt-unit-probe-a    the same with `Alt Unit Probe A` (created at step 3)
  a-05-stock-query-alt-unit-probe-b    the same with `Alt Unit Probe B` (created at step 3)

The report IDs, the one variable and the four item names are fixed in this script (all letters, digits and spaces, so no
escaping is needed); nothing from outside chooses any of them.

Run:  python -I gen_requests.py --out requests
      python -I gen_requests.py --check
"""
import argparse
import hashlib
import os
import re
import sys
import xml.etree.ElementTree as ET

HERE = os.path.dirname(os.path.abspath(__file__))
SUMMARY = os.path.join(HERE, "templates", "stock_summary_request.utf16le.xml")
ITEMS = os.path.join(HERE, "templates", "stock_items_fy_request.utf16le.xml")
T_ID = "Stock Summary"
VARIABLE = "STOCKITEMNAME"
PLAIN = re.compile(r"[A-Za-z0-9]+(?: [A-Za-z0-9]+)*")
ALLOWED_ITEMS = ("Carton Box Small", "Bridge Probe Item", "Alt Unit Probe A", "Alt Unit Probe B")
JOBS = (
    ("a-00-stock-summary", T_ID, None),
    ("a-01-items-fy", None, None),
    ("a-02-stock-query-carton-box-small", "Stock Query", "Carton Box Small"),
    ("a-03-stock-query-unknown-item", "Stock Query", "Bridge Probe Item"),
    ("a-04-stock-query-alt-unit-probe-a", "Stock Query", "Alt Unit Probe A"),
    ("a-05-stock-query-alt-unit-probe-b", "Stock Query", "Alt Unit Probe B"),
)


def read(path):
    data = open(path, "rb").read()
    if data[:2] != b"\xff\xfe":
        raise SystemExit("%s has no UTF-16LE byte-order mark" % os.path.basename(path))
    return data[2:].decode("utf-16-le")


def build(report_id, item):
    if report_id is None:
        if item is not None:
            raise SystemExit("the collection request takes no item")
        return open(ITEMS, "rb").read()
    if report_id not in (T_ID, "Stock Query"):
        raise SystemExit("report ID not in the closed set: %r" % report_id)
    if item is not None and (item not in ALLOWED_ITEMS or not PLAIN.fullmatch(item)):
        raise SystemExit("item not in the closed set: %r" % item)
    text = read(SUMMARY)
    if text.count("<ID>%s</ID>" % T_ID) != 1 or text.count("</STATICVARIABLES>") != 1:
        raise SystemExit("the template is not the shape this script was written for")
    text = text.replace("<ID>%s</ID>" % T_ID, "<ID>%s</ID>" % report_id)
    if item is not None:
        text = text.replace("</STATICVARIABLES>", "<%s>%s</%s></STATICVARIABLES>" % (VARIABLE, item, VARIABLE))
    return b"\xff\xfe" + text.encode("utf-16-le")


def check():
    assert build(T_ID, None) == open(SUMMARY, "rb").read(), "a-00 is not the template"
    assert build(None, None) == open(ITEMS, "rb").read(), "a-01 is not the template"
    base = read(SUMMARY)
    for name, report_id, item in JOBS:
        data = build(report_id, item)
        assert data[:2] == b"\xff\xfe"
        if report_id in (T_ID, "Stock Query"):
            root = ET.fromstring(data[2:].decode("utf-16-le"))
            found = [el.text for el in root.iter(VARIABLE)]
            assert found == ([item] if item else []), "item does not round-trip: " + name
            back = data[2:].decode("utf-16-le").replace("<ID>%s</ID>" % report_id, "<ID>%s</ID>" % T_ID)
            if item:
                back = back.replace("<%s>%s</%s>" % (VARIABLE, item, VARIABLE), "")
            assert back == base, "differs elsewhere: " + name
    for bad in (("Stock Query", "Carton Box"), ("Stock Query", "A&B"), ("Godown Summary", None), (None, "Alt Unit Probe A")):
        try:
            build(*bad)
        except SystemExit:
            continue
        raise AssertionError("an input outside the closed sets was accepted: %r" % (bad,))
    committed = os.path.join(HERE, "requests")
    have = sorted(os.listdir(committed)) if os.path.isdir(committed) else []
    assert have == sorted(j[0] + ".utf16le.xml" for j in JOBS), "requests/ holds %r" % have
    for name, report_id, item in JOBS:
        assert open(os.path.join(committed, name + ".utf16le.xml"), "rb").read() == build(report_id, item), "committed request differs from a fresh one: " + name
    print("gen_requests: ok (6 requests, each a committed template with only the report ID and one item name changed)")
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
    for name, report_id, item in JOBS:
        data = build(report_id, item)
        with open(os.path.join(args.out, name + ".utf16le.xml"), "wb") as out:
            out.write(data)
        print("%s  %d  %s.utf16le.xml" % (hashlib.sha256(data).hexdigest(), len(data), name))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
