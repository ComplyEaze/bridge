#!/usr/bin/env python3
"""Writes this sheet's report requests from the one committed request in templates/ (the Stock Summary report request,
fixture stock_summary_report_fy_request: TYPE Data, ID Stock Summary, company BRIDGE SHAPE LAB, 20250401 to 20260331).
Each output differs from it only in the report ID, in the value of SVTODATE, and in at most ONE added static variable:
EXPLODEFLAG with the value Yes (the variable section 12a.13 of the protocol reference measured on the Stock Summary), or
STOCKITEMNAME naming an item (the Stock Query shape this gateway answered on 28 Oct, lab/2018-stock-query-items-periods-answers
at a5b369bf, requests/r-07).

  Part A, the book as it stands, in the order they are sent:
  a-00-20260331-stock-summary            the template itself (the 28 Oct sitting's stock-summary request, byte for byte)
  a-01-20260331-godown-summary           ID `Godown Summary` (the 28 Oct sitting's godown-1 request, byte for byte)
  a-02-20260331-stock-summary-explode    a-00 with <EXPLODEFLAG>Yes</EXPLODEFLAG> (the shape section 12a.13 measured, on 7.1)
  a-03-20260331-godown-summary-explode   a-01 with <EXPLODEFLAG>Yes</EXPLODEFLAG> (never measured)
  a-04-20260331-control-unknown          ID `Bridge Probe Unknown` (the 28 Oct sitting's control request, byte for byte)
  a-05 .. a-08-20250831-*                a-00 .. a-03 at the period end 20250831 (the baseline before Part B's transfer date)
  Part B, after the keying, in the order they are sent, b-01 .. b-06 at 20250831 and b-07 .. b-12 at 20260331:
  stock-summary, godown-summary, stock-summary-explode, godown-summary-explode, stock-query-godown-split-item,
  stock-query-godown-single-item

The report IDs, the two variables and their values, the items and the period ends are fixed in this script; nothing from
outside chooses them. An added variable is the last element of STATICVARIABLES. Every name is plain (letters, digits,
spaces, hyphens), so no escaping is needed.

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
IDS = ("Stock Summary", "Godown Summary", "Stock Query", "Bridge Probe Unknown")
EXPLODE = ("EXPLODEFLAG", "Yes")
SPLIT, SINGLE = "Godown Split Item", "Godown Single Item"
ITEMS = (SPLIT, SINGLE)
TO_DATES = ("20250831", "20260331")


# What each kind of file holds; check() derives every file's ID, variable and period end from its name with these.
KINDS = {
    "stock-summary": ("Stock Summary", None),
    "stock-summary-explode": ("Stock Summary", EXPLODE),
    "godown-summary": ("Godown Summary", None),
    "godown-summary-explode": ("Godown Summary", EXPLODE),
    "control-unknown": ("Bridge Probe Unknown", None),
    "stock-query-godown-split-item": ("Stock Query", ("STOCKITEMNAME", SPLIT)),
    "stock-query-godown-single-item": ("Stock Query", ("STOCKITEMNAME", SINGLE)),
}
PART_B = ("stock-summary", "godown-summary", "stock-summary-explode", "godown-summary-explode",
          "stock-query-godown-split-item", "stock-query-godown-single-item")
NAMES = (
    "a-00-20260331-stock-summary", "a-01-20260331-godown-summary", "a-02-20260331-stock-summary-explode",
    "a-03-20260331-godown-summary-explode", "a-04-20260331-control-unknown",
    "a-05-20250831-stock-summary", "a-06-20250831-godown-summary", "a-07-20250831-stock-summary-explode",
    "a-08-20250831-godown-summary-explode",
    *("b-%02d-20250831-%s" % (n + 1, kind) for n, kind in enumerate(PART_B)),
    *("b-%02d-20260331-%s" % (n + 7, kind) for n, kind in enumerate(PART_B)),
)
NAME = re.compile(r"([ab])-([0-9]{2})-([0-9]{8})-([a-z-]+)")
JOBS = tuple((name, *KINDS[NAME.fullmatch(name).group(4)], NAME.fullmatch(name).group(3)) for name in NAMES)
# The template is the fixture stock_summary_report_fy_request; a-00, a-01 and a-04 are the 28 Oct sitting's requests
# (lab/2017-2018-stock-by-godown-batch-answers at 5b793065, reqs/stock-summary.xml, godown-1.xml and control.xml).
TEMPLATE_SHA256 = "17c4ff82b6ae49e6543f7be3745fbe023879ed0d2f47cc8c3bc3686e713afac8"
PINNED = {
    "a-00-20260331-stock-summary": TEMPLATE_SHA256,
    "a-01-20260331-godown-summary": "27ccda23a554946b16097fe004c58161d0d718ddb03d4aebf2008d38cd3a5e46",
    "a-04-20260331-control-unknown": "103f686bed40b37dabfca8d87f57570fd362a3ed479e3d47f3669d4396ded89c",
}

# Every request's sha256, as reviewed on 31 Oct 2026: a change to any byte of any request fails --check.
ALL_SHA256 = {
    "a-00-20260331-stock-summary": "17c4ff82b6ae49e6543f7be3745fbe023879ed0d2f47cc8c3bc3686e713afac8",
    "a-01-20260331-godown-summary": "27ccda23a554946b16097fe004c58161d0d718ddb03d4aebf2008d38cd3a5e46",
    "a-02-20260331-stock-summary-explode": "6566c8f06a11ff70ed96e75b0e807403ea5b87dafdfd91e87b6413d80fcfe52f",
    "a-03-20260331-godown-summary-explode": "1d2e2204628c33f8e226777e6bd3740e68cd2849b5db28c6163a837039914c9f",
    "a-04-20260331-control-unknown": "103f686bed40b37dabfca8d87f57570fd362a3ed479e3d47f3669d4396ded89c",
    "a-05-20250831-stock-summary": "902491688bb5c2178fc9c50e5fd202d6f6736a46de3c79d304af1a47bf839677",
    "a-06-20250831-godown-summary": "13ee81795756aca579dc4cecb8c2da87259f10eb21ecd26ff9be591219217233",
    "a-07-20250831-stock-summary-explode": "62c6cf033bc4a664e83f7c55a70daa049fe089fa1dee776e26d879bee85d1a54",
    "a-08-20250831-godown-summary-explode": "5756cbc7cf527f1aebd036d5c6f6e8159295dd6f548d6c3c622dc338d99bc592",
    "b-01-20250831-stock-summary": "902491688bb5c2178fc9c50e5fd202d6f6736a46de3c79d304af1a47bf839677",
    "b-02-20250831-godown-summary": "13ee81795756aca579dc4cecb8c2da87259f10eb21ecd26ff9be591219217233",
    "b-03-20250831-stock-summary-explode": "62c6cf033bc4a664e83f7c55a70daa049fe089fa1dee776e26d879bee85d1a54",
    "b-04-20250831-godown-summary-explode": "5756cbc7cf527f1aebd036d5c6f6e8159295dd6f548d6c3c622dc338d99bc592",
    "b-05-20250831-stock-query-godown-split-item": "5d0096e520ad8ed62ec8f19b5b13ff983500f746c8202223de0f930c20ade2a1",
    "b-06-20250831-stock-query-godown-single-item": "947a8668f36a8f8d19efdc362d6aca684b4f767021d5994d7c977e01c0634918",
    "b-07-20260331-stock-summary": "17c4ff82b6ae49e6543f7be3745fbe023879ed0d2f47cc8c3bc3686e713afac8",
    "b-08-20260331-godown-summary": "27ccda23a554946b16097fe004c58161d0d718ddb03d4aebf2008d38cd3a5e46",
    "b-09-20260331-stock-summary-explode": "6566c8f06a11ff70ed96e75b0e807403ea5b87dafdfd91e87b6413d80fcfe52f",
    "b-10-20260331-godown-summary-explode": "1d2e2204628c33f8e226777e6bd3740e68cd2849b5db28c6163a837039914c9f",
    "b-11-20260331-stock-query-godown-split-item": "ec5800b73343cdbf18b2d706bd5129bc7ea7334dcec6cbd748937f8ead285aa3",
    "b-12-20260331-stock-query-godown-single-item": "f58ac2df5c5a101dfafebe09b4b9b9bd22be9ace9d7edf2a28f1778ce0cc2975",
}

def build(report_id, variable, to_date):
    if len(report_id) > 60 or not PLAIN.fullmatch(report_id) or report_id not in IDS:
        raise SystemExit("report ID not in the closed set: %r" % report_id)
    if variable is not None:
        name, value = variable
        if not ((name, value) == EXPLODE and report_id in ("Stock Summary", "Godown Summary")
                or name == "STOCKITEMNAME" and value in ITEMS and report_id == "Stock Query"):
            raise SystemExit("variable not in the closed set for %r: %r" % (report_id, variable))
        if not PLAIN.fullmatch(value):
            raise SystemExit("variable value not plain: %r" % value)
    elif report_id == "Stock Query":
        raise SystemExit("a Stock Query names its item")
    if to_date not in TO_DATES:
        raise SystemExit("period end not in the closed set: %r" % to_date)
    data = open(TEMPLATE, "rb").read()
    if data[:2] != b"\xff\xfe":
        raise SystemExit("the template has no UTF-16LE byte-order mark")
    text = data[2:].decode("utf-16-le")
    for old in ("<ID>%s</ID>" % T_ID, '<SVFROMDATE TYPE="Date">%s<' % T_FROM, '<SVTODATE TYPE="Date">%s<' % T_TO,
                "</STATICVARIABLES>"):
        if text.count(old) != 1:
            raise SystemExit("the template is not the shape this script was written for")
    text = text.replace("<ID>%s</ID>" % T_ID, "<ID>%s</ID>" % report_id)
    text = text.replace('<SVTODATE TYPE="Date">%s<' % T_TO, '<SVTODATE TYPE="Date">%s<' % to_date)
    if variable is not None:
        text = text.replace("</STATICVARIABLES>", "<%s>%s</%s></STATICVARIABLES>" % (variable[0], variable[1], variable[0]))
    return b"\xff\xfe" + text.encode("utf-16-le")


def decoded(data):
    """The report ID, the period end and the added variables, read from a request by parsing it."""
    import xml.etree.ElementTree as ET
    root = ET.fromstring(data[2:].decode("utf-16-le"))
    variables = root.find("BODY/DESC/STATICVARIABLES")
    added = tuple((v.tag, v.text) for v in variables
                  if v.tag not in ("SVEXPORTFORMAT", "SVCURRENTCOMPANY", "SVFROMDATE", "SVTODATE"))
    return root.findtext("HEADER/ID"), variables.findtext("SVFROMDATE"), variables.findtext("SVTODATE"), added


def from_name(name):
    """The ID and variable a file's name says, read from its text alone (not from KINDS)."""
    kind = NAME.fullmatch(name).group(4)
    if kind.startswith("stock-query-"):
        item = {"stock-query-godown-split-item": SPLIT, "stock-query-godown-single-item": SINGLE}[kind]
        return "Stock Query", ("STOCKITEMNAME", item)
    report = {"stock-summary": "Stock Summary", "godown-summary": "Godown Summary",
              "control-unknown": "Bridge Probe Unknown"}[kind[:-len("-explode")] if kind.endswith("-explode") else kind]
    return report, (EXPLODE if kind.endswith("-explode") else None)


def check():
    tpl = open(TEMPLATE, "rb").read()
    assert hashlib.sha256(tpl).hexdigest() == TEMPLATE_SHA256, "the template is not the fixture stock_summary_report_fy_request"
    assert build(T_ID, None, T_TO) == tpl, "a-00 is not the template"
    base = tpl[2:].decode("utf-16-le")
    assert len(JOBS) == 21 and len({name for name, *_ in JOBS}) == 21
    for name, report_id, variable, to_date in JOBS:
        said = from_name(name)
        assert (report_id, variable) == said, "KINDS differs from the name: " + name
        assert decoded(build(report_id, variable, to_date)) == (said[0], T_FROM, NAME.fullmatch(name).group(3), (said[1],) if said[1] else ()), "contents differ from the name: " + name
        assert hashlib.sha256(build(report_id, variable, to_date)).hexdigest() == ALL_SHA256[name], "not the reviewed bytes: " + name
    for name, digest in PINNED.items():
        job = next(j for j in JOBS if j[0] == name)
        assert hashlib.sha256(build(*job[1:])).hexdigest() == digest, "not the 28 Oct request: " + name
    for name, report_id, variable, to_date in JOBS:
        out = build(report_id, variable, to_date)
        assert out[:2] == b"\xff\xfe"
        back = out[2:].decode("utf-16-le").replace("<ID>%s</ID>" % report_id, "<ID>%s</ID>" % T_ID)
        back = back.replace('<SVTODATE TYPE="Date">%s<' % to_date, '<SVTODATE TYPE="Date">%s<' % T_TO)
        if variable is not None:
            added = "<%s>%s</%s>" % (variable[0], variable[1], variable[0])
            assert back.count(added) == 1 and added + "</STATICVARIABLES>" in back, "variable not last: " + name
            back = back.replace(added, "")
        assert back == base, "differs elsewhere: " + name
    for bad in (("Godown Vouchers", None, T_TO), ("Stock Query", None, T_TO), ("Stock Query", ("STOCKITEMNAME", "A&B"), T_TO),
                ("Stock Query", ("STOCKITEMNAME", "Soda Ash Light"), T_TO), ("Stock Query", EXPLODE, T_TO),
                ("Godown Summary", ("EXPLODEFLAG", "No"), T_TO), ("Godown Summary", ("GODOWNNAME", "Main Location"), T_TO),
                ("Godown Summary", ("STOCKITEMNAME", SPLIT), T_TO), ("Godown Summary", None, "20250930"),
                ("Godown Summary", None, "20250401")):
        try:
            build(*bad)
        except SystemExit:
            continue
        raise AssertionError("a request was accepted: %r" % (bad,))
    committed = os.path.join(HERE, "requests")
    names = sorted(os.listdir(committed)) if os.path.isdir(committed) else []
    assert names == sorted(n + ".utf16le.xml" for n, *_ in JOBS), "requests/ holds %r" % names
    for name, report_id, variable, to_date in JOBS:
        with open(os.path.join(committed, name + ".utf16le.xml"), "rb") as f:
            assert f.read() == build(report_id, variable, to_date), "committed request differs from a fresh one: " + name
    print("gen_requests: ok (21 requests, each the Stock Summary request with only the ID, the period end and at most one variable changed, as its name says)")
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
    for name, report_id, variable, to_date in JOBS:
        data = build(report_id, variable, to_date)
        with open(os.path.join(args.out, name + ".utf16le.xml"), "wb") as out:
            out.write(data)
        print("%s  %d  %s.utf16le.xml" % (hashlib.sha256(data).hexdigest(), len(data), name))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
