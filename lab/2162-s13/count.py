#!/usr/bin/env python3
# count.py ANSWER...: prints what ROW.md records of each answer, by an XML parser (LAB-COMMON C4), never a text search.
# The kind comes from the request's name inside the answer's name (a-<label>-<request>):
#   M- marks: the COMPANY rows, each GUID, ALTVCHID, ALTMSTID.
#   C- catalogue, Q- rates: the LEDGER rows, and for the ledger named by --ledger (default BRIDGE Customer RJ): how many
#      rows carry that name, then PARENT and ISBILLWISEON (C), or PARENT, PARTYGSTIN, STATENAME and every
#      LEDGSTREGDETAILS.LIST that holds anything, with each child (Q).
#   N- number read: the VOUCHER rows, each VOUCHERNUMBER and BRIDGEVCHISSALES.
#   I- import: every counter Tally returned and every LINEERROR.
#   R- read-back: the VOUCHER rows; for each, the header fields and each ALLLEDGERENTRIES.LIST's LEDGERNAME,
#      ISDEEMEDPOSITIVE and AMOUNT.
# count.py --diff A B: two read-back answers, one VOUCHER row each: every header field and leg field whose value
#   differs between them, and every one present in one only. ROW.md says which differences each sub-row expects.
# An element absent is printed <absent>, one present and empty is printed ''. Names are not trimmed: repr shows a
# trailing line break. Numbers, dates and flags are trimmed.
# Example: python3 -I count.py a-s1-M-marks.xml
import pathlib, re, sys
import xml.etree.ElementTree as ET
FORBIDDEN = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));", re.I)
HEADER = ["DATE", "VOUCHERNUMBER", "VOUCHERTYPENAME", "REFERENCE", "REFERENCEDATE", "PARTYLEDGERNAME", "PARTYGSTIN",
          "STATENAME", "PLACEOFSUPPLY", "GSTREGISTRATIONTYPE", "ISINVOICE", "ISCANCELLED", "ISOPTIONAL", "GUID",
          "ALTERID", "VCHGSTSTATUSISINCLUDED", "VCHGSTSTATUSISUNCERTAIN", "VCHGSTSTATUSISEXCLUDED", "ISGSTOVERRIDDEN",
          "ISPOSTDATED"]
NAMES = {"NAME", "LEDGERNAME", "PARTYLEDGERNAME", "PARENT"}
COUNTERS = ["CREATED", "ALTERED", "DELETED", "LASTVCHID", "LASTMID", "COMBINED", "IGNORED", "ERRORS", "CANCELLED",
            "EXCEPTIONS"]

def parse(path):
    raw = path.read_bytes()
    body = raw[2:] if raw[:2] == b"\xff\xfe" else raw
    text = FORBIDDEN.sub("", body.decode("utf-16-le"))
    text = re.sub(r"^\s*<\?xml[^>]*\?>", "", text)
    return ET.fromstring(text)

def rows(root, tag):
    collection = root.find("BODY/DATA/COLLECTION")
    return [] if collection is None else [child for child in collection if child.tag == tag]

def value(node, tag):
    found = [child for child in node if child.tag == tag]
    if not found:
        return "<absent>"
    if len(found) > 1:
        return f"<{len(found)} times>"
    text = "".join(found[0].itertext())
    return repr(text) if tag in NAMES else repr(text.strip())

def name_of(node):
    found = [child for child in node if child.tag == "NAME"]
    return node.get("NAME") if not found else "".join(found[0].itertext())

def legs(voucher):
    return [[value(leg, "LEDGERNAME"), value(leg, "ISDEEMEDPOSITIVE"), value(leg, "AMOUNT")]
            for leg in voucher if leg.tag == "ALLLEDGERENTRIES.LIST"]

def report(path, ledger):
    root, kind = parse(path), path.name.split("-", 2)[-1][:1]
    print(f"== {path.name}: {path.stat().st_size} bytes")
    if kind == "M":
        for company in rows(root, "COMPANY"):
            print("  COMPANY", value(company, "GUID"), "ALTVCHID", value(company, "ALTVCHID"), "ALTMSTID",
                  value(company, "ALTMSTID"))
        print("  COMPANY rows:", len(rows(root, "COMPANY")))
    elif kind in "CQ":
        found = rows(root, "LEDGER")
        mine = [row for row in found if name_of(row) == ledger]
        print(f"  LEDGER rows: {len(found)}; named {ledger!r}: {len(mine)}")
        for row in mine:
            fields = ["PARENT", "ISBILLWISEON"] if kind == "C" else ["PARENT", "PARTYGSTIN", "STATENAME"]
            print("  ", "; ".join(f"{tag} {value(row, tag)}" for tag in fields))
            for detail in (d for d in row if d.tag == "LEDGSTREGDETAILS.LIST"):
                if len(detail):
                    print("   LEDGSTREGDETAILS.LIST:", "; ".join(f"{c.tag} {(c.text or '').strip()!r}" for c in detail))
            if kind == "Q":
                print("   LEDGSTREGDETAILS.LIST elements:", sum(d.tag == "LEDGSTREGDETAILS.LIST" for d in row),
                      "of which non-empty:", sum(d.tag == "LEDGSTREGDETAILS.LIST" and len(d) > 0 for d in row))
    elif kind == "N":
        found = [row for row in rows(root, "VOUCHER") if any(c.tag == "VOUCHERNUMBER" for c in row)]
        print("  VOUCHER rows:", len(found))
        for row in found:
            print("  ", value(row, "VOUCHERNUMBER"), "BRIDGEVCHISSALES", value(row, "BRIDGEVCHISSALES"))
    elif kind == "I":
        # Tally 7.2 answered S4's imports with a bare RESPONSE holding the counters; an IMPORTRESULT is read too.
        result = root if root.tag == "RESPONSE" else root.find(".//IMPORTRESULT")
        print("  root", root.tag)
        for tag in COUNTERS:
            print(f"  {tag} {value(result, tag) if result is not None else '<no counters>'}")
        for error in root.iter("LINEERROR"):
            print("  LINEERROR", repr("".join(error.itertext())))
    elif kind == "R":
        found = rows(root, "VOUCHER")
        print("  VOUCHER rows:", len(found))
        for row in found:
            for tag in HEADER:
                print(f"   {tag} {value(row, tag)}")
            for leg in legs(row):
                print("   leg", " | ".join(leg))
    else:
        raise SystemExit(f"{path.name}: no kind for this name")

def diff(a, b):
    va, vb = (rows(parse(p), "VOUCHER") for p in (a, b))
    if len(va) != 1 or len(vb) != 1:
        raise SystemExit(f"--diff needs one VOUCHER row in each: {len(va)}, {len(vb)}")
    va, vb = va[0], vb[0]
    print(f"== --diff {a.name} {b.name}")
    tags = sorted({c.tag for c in va if not len(c)} | {c.tag for c in vb if not len(c)})
    for tag in tags:
        if value(va, tag) != value(vb, tag):
            print(f"   {tag}: {value(va, tag)} -> {value(vb, tag)}")
    la, lb = legs(va), legs(vb)
    print(f"   legs: {len(la)} -> {len(lb)}")
    for index in range(max(len(la), len(lb))):
        x = la[index] if index < len(la) else None
        y = lb[index] if index < len(lb) else None
        if x != y:
            print(f"   leg {index + 1}: {x} -> {y}")

args = sys.argv[1:]
ledger = "BRIDGE Customer RJ"
if args[:1] == ["--ledger"]:
    ledger, args = args[1], args[2:]
if args[:1] == ["--diff"]:
    diff(pathlib.Path(args[1]), pathlib.Path(args[2]))
else:
    for name in args:
        report(pathlib.Path(name), ledger)
