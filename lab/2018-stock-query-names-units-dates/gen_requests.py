#!/usr/bin/env python3
"""Writes this sheet's report and collection requests from two committed requests in templates/:

  templates/stock_summary_request.utf16le.xml   the Stock Summary report request (fixture stock_summary_report_fy_request:
                                                TYPE Data, ID Stock Summary, company BRIDGE SHAPE LAB, 20250401 to 20260331)
  templates/stock_items_fy_request.utf16le.xml  the stock items collection request (fixture stock_items_fy_request: TYPE Collection,
                                                ID Bridge Audit Stock Items, FETCH NAME, GUID, PARENT, BASEUNITS, OPENINGBALANCE,
                                                OPENINGVALUE, CLOSINGBALANCE, CLOSINGVALUE, same company and period)

Part A (read only, existing items) and Part W (after the lab has created seven items by hand):

  a-00            the Stock Summary request itself (control)
  a-01 .. a-03    the stock items collection, SVTODATE 20250401, 20250501, 20260331 (the last is the fixture, byte for byte)
  a-04 .. a-13    ID `Stock Query` + <STOCKITEMNAME>, five existing items, SVTODATE 20250401 and 20250501
  a-14            ID `Stock Query`, item `Bridge Probe Item` (control: an item the book lacks), period 20250401 to 20260331
  w-01            the stock items collection, SVTODATE 20260331 (same bytes as a-03)
  w-02 .. w-10    ID `Stock Query` + <STOCKITEMNAME>, the seven created item names, SVTODATE 20260331; form F1 escapes only & < > ;
                  form F2 also escapes the apostrophe and the quotation mark (sent only where it differs from F1)

The report IDs, the one variable, the item names (five existing, one control, seven created), the escape forms and the period
ends are fixed in this script; nothing from outside chooses any of them. A name is escaped once, as XML text, here.

Run:  python -I gen_requests.py --out requests
      python -I gen_requests.py --check
"""
import argparse
import hashlib
import os
import sys
import xml.etree.ElementTree as ET

HERE = os.path.dirname(os.path.abspath(__file__))
SUMMARY = os.path.join(HERE, "templates", "stock_summary_request.utf16le.xml")
ITEMS = os.path.join(HERE, "templates", "stock_items_fy_request.utf16le.xml")
T_ID, T_TO = "Stock Summary", "20260331"
COLL_TO = "20260331"
VARIABLE = "STOCKITEMNAME"
EXISTING = ("Caustic Soda Flakes", "Soda Ash Light", "Cleaning Kit A", "Zero Stock Item", "Hydrochloric Acid 33pc")
PROBE = "Bridge Probe Item"
CREATED = (
    "Kit & Co Item",
    "Gauge (Test) 50L",
    "Owner's Item",
    'Quote "Q" Item',
    "Alt Unit Item",
    "Mix 5% / Set, Large",
    "Dash-Item 7",
)
ALLOWED_ITEMS = EXISTING + (PROBE,) + CREATED
TO_DATES = ("20250401", "20250501", "20260331")


def escape(name, form):
    out = name.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")
    if form == "F2":
        out = out.replace("'", "&apos;").replace('"', "&quot;")
    elif form != "F1":
        raise SystemExit("form not in the closed set: %r" % form)
    return out


def read(path):
    data = open(path, "rb").read()
    if data[:2] != b"\xff\xfe":
        raise SystemExit("%s has no UTF-16LE byte-order mark" % os.path.basename(path))
    return data[2:].decode("utf-16-le")


def encode(text):
    return b"\xff\xfe" + text.encode("utf-16-le")


def summary_like(report_id, item, form, to_date):
    if report_id not in (T_ID, "Stock Query"):
        raise SystemExit("report ID not in the closed set: %r" % report_id)
    if item is not None and item not in ALLOWED_ITEMS:
        raise SystemExit("item not in the closed set: %r" % item)
    if to_date not in TO_DATES:
        raise SystemExit("period end not in the closed set: %r" % to_date)
    text = read(SUMMARY)
    for old in ("<ID>%s</ID>" % T_ID, '<SVTODATE TYPE="Date">%s<' % T_TO, "</STATICVARIABLES>"):
        if text.count(old) != 1:
            raise SystemExit("the summary template is not the shape this script was written for")
    text = text.replace("<ID>%s</ID>" % T_ID, "<ID>%s</ID>" % report_id)
    text = text.replace('<SVTODATE TYPE="Date">%s<' % T_TO, '<SVTODATE TYPE="Date">%s<' % to_date)
    if item is not None:
        text = text.replace("</STATICVARIABLES>", "<%s>%s</%s></STATICVARIABLES>" % (VARIABLE, escape(item, form), VARIABLE))
    return encode(text)


def collection(to_date):
    if to_date not in TO_DATES:
        raise SystemExit("period end not in the closed set: %r" % to_date)
    text = read(ITEMS)
    old = '<SVTODATE TYPE="Date">%s<' % COLL_TO
    if text.count(old) != 1:
        raise SystemExit("the collection template is not the shape this script was written for")
    return encode(text.replace(old, '<SVTODATE TYPE="Date">%s<' % to_date))


def jobs():
    out = [("a-00-stock-summary", lambda: summary_like(T_ID, None, "F1", T_TO), None, None)]
    for to in TO_DATES:
        out.append(("a-%02d-items-to-%s" % (len(out), to), lambda to=to: collection(to), None, None))
    for item in EXISTING:
        for to in ("20250401", "20250501"):
            out.append(("a-%02d-stock-query-%s-to-%s" % (len(out), item.lower().replace(" ", "-"), to), lambda i=item, t=to: summary_like("Stock Query", i, "F1", t), item, "F1"))
    out.append(("a-%02d-stock-query-unknown-item" % len(out), lambda: summary_like("Stock Query", PROBE, "F1", T_TO), PROBE, "F1"))
    out.append(("w-01-items-to-20260331", lambda: collection(COLL_TO), None, None))
    n = 1
    for item in CREATED:
        forms = ["F1"] + (["F2"] if escape(item, "F1") != escape(item, "F2") else [])
        for form in forms:
            n += 1
            slug = "".join(c if c.isalnum() else "-" for c in item.lower()).strip("-")
            slug = "-".join(p for p in slug.split("-") if p)
            out.append(("w-%02d-stock-query-%s-%s" % (n, slug, form.lower()), lambda i=item, f=form: summary_like("Stock Query", i, f, T_TO), item, form))
    return out


def check():
    s_tpl = open(SUMMARY, "rb").read()
    i_tpl = open(ITEMS, "rb").read()
    assert summary_like(T_ID, None, "F1", T_TO) == s_tpl, "a-00 is not the template"
    assert collection(COLL_TO) == i_tpl, "the collection at the fixture's period is not the template"
    names = [j[0] for j in jobs()]
    assert len(names) == len(set(names)), "duplicate file names"
    for name, make, item, form in jobs():
        data = make()
        assert data[:2] == b"\xff\xfe"
        root = ET.fromstring(data[2:].decode("utf-16-le"))
        found = [el.text for el in root.iter(VARIABLE)]
        assert found == ([item] if item is not None else []), "name does not round-trip: " + name
    for name, item, form in (("a", "Carton Box", "F1"), ("b", "A&B", "F1"), ("c", None, "F3")):
        try:
            if form == "F3":
                escape("x", form)
            else:
                summary_like("Stock Query", item, form, T_TO)
        except SystemExit:
            continue
        raise AssertionError("an input outside the closed sets was accepted: " + name)
    try:
        collection("20250430")
    except SystemExit:
        pass
    else:
        raise AssertionError("a period end outside the closed set was accepted")
    committed = os.path.join(HERE, "requests")
    have = sorted(os.listdir(committed)) if os.path.isdir(committed) else []
    assert have == sorted(n + ".utf16le.xml" for n in names), "requests/ holds %r" % have
    for name, make, _item, _form in jobs():
        assert open(os.path.join(committed, name + ".utf16le.xml"), "rb").read() == make(), "committed request differs from a fresh one: " + name
    print("gen_requests: ok (%d requests, each a committed template with only the report ID, one escaped item name and the period end changed)" % len(names))
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
    for name, make, _item, _form in jobs():
        data = make()
        with open(os.path.join(args.out, name + ".utf16le.xml"), "wb") as out:
            out.write(data)
        print("%s  %d  %s.utf16le.xml" % (hashlib.sha256(data).hexdigest(), len(data), name))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
