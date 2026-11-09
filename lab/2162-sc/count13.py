#!/usr/bin/env python3
# count13.py ANSWER...: row 2162-KC's count11.py (lab/2162-kc-sheet at 604bd4e0) for row 2162-SC, with two changes,
# each marked SC below: a kind U, the build's company registration read (render_company_registration_request, the
# TaxUnit collection): how many TAXUNIT rows, and for each its NAME, TAXTYPE and TAXREGISTRATION attributes, GUID,
# whether that GUID begins with BOOK's, GSTREGNUMBER, GSTOLDREGNUMBER and USEDFOR, then each dated
# GSTREGISTRATIONDETAILS.LIST that holds anything, with FROMDATE, STATE, PLACEOFSUPPLY, REGISTRATIONTYPE and
# ISINACTIVE; and --unit-diff A B, on two U answers: for each TAXUNIT by NAME, every leaf (by its path inside the
# row, attributes included) whose value differs, or that is in one only, at most UCAP lines in all.
# count11.py was row 2162-RS's count10.py (lab/2162-rs-sheet at 073144e5) for row 2162-KC, with two changes,
# each marked KC below: a kind W, the build's number read of one month (every VOUCHER row of the window, with its
# DATE, VOUCHERNUMBER, VOUCHERTYPENAME, BRIDGEVCHISSALES, ISCANCELLED, ISDELETED, whether IRN holds text, and GUID);
# and --guid GUID W..., which prints, for each W answer, how many rows carry that GUID and each such row.
# count10.py was row 2162-FY14R's count9.py (lab/2162-fy14r-sheet at 6ed4c28e) for row 2162-RS, with two
# changes, each marked RS below: V also prints each row's GUID, MASTERID and ALTERID; and --diff, on two answers of
# one VOUCHER row each (R or V), also compares every direct child that has children of its own (a list), by tag:
# how many of that tag each holds, and how many differ position by position, never their content.
# count9.py was row 2162-FY14's count7.py (lab/2162-fy14-sheet at e5d30d23) for row 2162-FY14R, with two
# changes, each marked FY14R below: the book is pinned by GUID (BOOK: the M kind prints whether each COMPANY row's GUID
# is the book's, and --before-write and --same-row STOP on any other), and T prints every non-empty leaf of each
# RESTARTFROMLIST.LIST by tag.
# count7.py was row 2162-KD's count6.py (lab/2162-keyed-dup-sheet) for row 2162-FY14, with three changes, each
# marked FY14 below: N and V print each row's DATE, and T prints three more series children (DUPLICATECONTROL,
# BEGINNINGNUMBER and how many children RESTARTFROMLIST.LIST holds).
# count6.py was row 2162-BC's count5.py (lab/2162-bc-sheet) with one kind added, T (below), for row 2162-KD.
# count5.py was S13a2's count.py (lab/2162-s13a2-sheet) with the kinds V and K added.
# It prints what ROW.md records of each answer, by an XML parser (LAB-COMMON C4), never a text search.
# The kind comes from the request's name inside the answer's name (a-<label>-<request>):
#   M- marks: the COMPANY rows, each GUID, ALTVCHID, ALTMSTID.
#   C- catalogue, Q- rates: the LEDGER rows, and for the ledger named by --ledger (default BRIDGE Customer RJ): how many
#      rows carry that name, then PARENT and ISBILLWISEON (C), or PARENT, PARTYGSTIN, STATENAME and every
#      LEDGSTREGDETAILS.LIST that holds anything, with each child (Q).
#   N- number read: every VOUCHER row, how many carry a VOUCHERNUMBER and how many do not, and each
#      VOUCHERNUMBER with BRIDGEVCHISSALES.
# Every read answer (all but I and L) prints its HEADER/STATUS and every LINEERROR first. An answer with no
# BODY/DATA/COLLECTION, or an import answer with no counters, is a failed request: count.py prints STOP and exits
# non-zero, and never reports it as 0 rows.
#   I- import, L- the setup ledger's import: every counter Tally returned and every LINEERROR. (S13's count.py had
#      no L kind; its seed answer had to be counted from a copy under an I name.)
#   V- every native field of the vouchers of one number: the VOUCHER rows; for each, the number, the type, the
#      party, the six elements of the buyer and consignee block (BLOCK) and COUNTRYOFRESIDENCE, PARTYGSTIN,
#      GSTREGISTRATIONTYPE and the three GST status flags; then how many direct children the row has.
#   K- every native field of one ledger by name: the LEDGER rows; for each, its name, MAILINGNAME, every
#      LEDMAILINGDETAILS.LIST with each child, and STATENAME, COUNTRYNAME and PARTYGSTIN.
#   T- the build's voucher-type read (render_voucher_types_request): how many VOUCHERTYPE rows, how many carry each
#      name given by --type (default BRIDGE Sales and BRIDGE Sales Keyed), and for each such row its PARENT, GUID,
#      NUMBERINGMETHOD and ISACTIVE, then every VOUCHERNUMBERSERIES.LIST with its NAME, NUMBERINGMETHOD and
#      PREVENTDUPLICATES. A series child absent prints <absent>.
#   W- (KC) the build's number read of one month: how many VOUCHER rows, then each, sorted by VOUCHERNUMBER and then
#      GUID. Above WCAP rows only the count is printed, then STOP, so the output stays bounded (AGENTS.md P5).
#   R- read-back: the VOUCHER rows; for each, the header fields and each ALLLEDGERENTRIES.LIST's LEDGERNAME,
#      ISDEEMEDPOSITIVE and AMOUNT.
# count.py --diff A B: two read-back answers, one VOUCHER row each: every header field and leg field whose value
#   differs between them, and every one present in one only. ROW.md says which differences each sub-row expects.
# count.py --before-write S1 M: M holds exactly one COMPANY row, with the GUID of S1's one row; else STOP. Each must
#   also be BOOK (FY14R).
# count.py --same-row S1 M: as --before-write, and ALTVCHID and ALTMSTID also equal S1's; else STOP.
# An element absent is printed <absent>, one present and empty is printed ''. Names are not trimmed: repr shows a
# trailing line break. Numbers, dates and flags are trimmed.
# Example: python3 -I count13.py a-s1-M-marks.xml
import pathlib, re, sys
import xml.etree.ElementTree as ET
BOOK = "6b43e498-430c-4d5c-bfef-d32e2ab93c85"  # FY14R: BRIDGE PILOT LAB's GUID, as rows 2162-KD and 2162-FY14 read it
FORBIDDEN = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));", re.I)
HEADER = ["DATE", "VOUCHERNUMBER", "VOUCHERTYPENAME", "REFERENCE", "REFERENCEDATE", "PARTYLEDGERNAME", "PARTYGSTIN",
          "STATENAME", "PLACEOFSUPPLY", "GSTREGISTRATIONTYPE", "ISINVOICE", "ISCANCELLED", "ISOPTIONAL", "GUID",
          "ALTERID", "VCHGSTSTATUSISINCLUDED", "VCHGSTSTATUSISUNCERTAIN", "VCHGSTSTATUSISEXCLUDED", "ISGSTOVERRIDDEN",
          "ISPOSTDATED"]
BLOCK = ["BASICBUYERNAME", "PARTYMAILINGNAME", "CONSIGNEEMAILINGNAME", "CONSIGNEESTATENAME",
         "CONSIGNEECOUNTRYNAME", "VATDEALERTYPE"]
NATIVE = ["GUID", "MASTERID", "ALTERID",  # RS: the voucher's identity, which a person's save may change
          "DATE", "VOUCHERNUMBER", "VOUCHERTYPENAME",  # FY14: DATE added
          "PARTYLEDGERNAME"] + BLOCK + ["COUNTRYOFRESIDENCE", "PARTYGSTIN",
          "GSTREGISTRATIONTYPE", "VCHGSTSTATUSISINCLUDED", "VCHGSTSTATUSISUNCERTAIN", "VCHGSTSTATUSISEXCLUDED"]
NAMES = {"BASICBUYERNAME", "PARTYMAILINGNAME", "CONSIGNEEMAILINGNAME", "MAILINGNAME", "NAME", "LEDGERNAME", "PARTYLEDGERNAME", "PARENT"}
WCAP = 300  # KC: the most W rows printed one by one
UCAP = 200  # SC: the most --unit-diff lines printed
UFIELDS = ["GSTREGNUMBER", "GSTOLDREGNUMBER", "USEDFOR"]
UDATED = ["FROMDATE", "STATE", "PLACEOFSUPPLY", "REGISTRATIONTYPE", "ISINACTIVE"]
WFIELDS = ["DATE", "VOUCHERNUMBER", "VOUCHERTYPENAME", "BRIDGEVCHISSALES", "ISCANCELLED", "ISDELETED"]
COUNTERS = ["CREATED", "ALTERED", "DELETED", "LASTVCHID", "LASTMID", "COMBINED", "IGNORED", "ERRORS", "CANCELLED",
            "EXCEPTIONS"]

def parse(path):
    raw = path.read_bytes()
    body = raw[2:] if raw[:2] == b"\xff\xfe" else raw
    text = FORBIDDEN.sub("", body.decode("utf-16-le"))
    text = re.sub(r"^\s*<\?xml[^>]*\?>", "", text)
    return ET.fromstring(text)

def status(root):
    # Every read answer's HEADER/STATUS and every LINEERROR, printed before any count.
    found = root.find("HEADER/STATUS")
    print("  STATUS", "<absent>" if found is None else repr((found.text or "").strip()))
    for error in root.iter("LINEERROR"):
        print("  LINEERROR", repr("".join(error.itertext())))

def rows(root, tag):
    # An answer with no COLLECTION is a failed read, never "0 rows" (AGENTS.md P5): stop.
    collection = root.find("BODY/DATA/COLLECTION")
    if collection is None:
        raise SystemExit("STOP: no BODY/DATA/COLLECTION in the answer; the request failed, it did not find nothing")
    return [child for child in collection if child.tag == tag]

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
    if kind not in ("I", "L"):
        status(root)
    if kind == "M":
        for company in rows(root, "COMPANY"):
            print("  COMPANY", value(company, "GUID"), "ALTVCHID", value(company, "ALTVCHID"), "ALTMSTID",
                  value(company, "ALTMSTID"))
            # FY14R: the pin, compared on the raw text.
            print("  GUID is the book's:", "yes" if (company.findtext("GUID") or "").strip() == BOOK else "no")
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
        every = rows(root, "VOUCHER")
        found = [row for row in every if any(c.tag == "VOUCHERNUMBER" for c in row)]
        print("  VOUCHER rows:", len(every), "of which with a VOUCHERNUMBER:", len(found),
              "without one:", len(every) - len(found))
        for row in found:
            print("  ", value(row, "VOUCHERNUMBER"), "DATE", value(row, "DATE"),  # FY14: DATE added
                  "BRIDGEVCHISSALES", value(row, "BRIDGEVCHISSALES"))
    elif kind in ("I", "L"):
        # Tally 7.2 answered S4's imports with a bare RESPONSE holding the counters; an IMPORTRESULT is read too.
        result = root if root.tag == "RESPONSE" else root.find(".//IMPORTRESULT")
        print("  root", root.tag)
        if result is None:
            for error in root.iter("LINEERROR"):
                print("  LINEERROR", repr("".join(error.itertext())))
            raise SystemExit("STOP: no import counters in the answer; the request failed")
        for tag in COUNTERS:
            print(f"  {tag} {value(result, tag) if result is not None else '<no counters>'}")
        for error in root.iter("LINEERROR"):
            print("  LINEERROR", repr("".join(error.itertext())))
    elif kind == "V":
        found = rows(root, "VOUCHER")
        print("  VOUCHER rows:", len(found))
        for row in found:
            for tag in NATIVE:
                print(f"   {tag} {value(row, tag)}")
            print("   direct children:", len(row))
    elif kind == "K":
        found = rows(root, "LEDGER")
        print("  LEDGER rows:", len(found))
        for row in found:
            print("   NAME", repr(name_of(row)), "MAILINGNAME", value(row, "MAILINGNAME"))
            for detail in (d for d in row if d.tag == "LEDMAILINGDETAILS.LIST"):
                print("   LEDMAILINGDETAILS.LIST:", "; ".join(f"{c.tag} {''.join(c.itertext())!r}" for c in detail) or "empty")
            for tag in ("STATENAME", "COUNTRYNAME", "PARTYGSTIN"):
                print(f"   {tag} {value(row, tag)}")
    elif kind == "W":
        # KC: the build judges these rows itself (number_holding, control_row_found, control_miss).
        found = rows(root, "VOUCHER")
        print("  VOUCHER rows:", len(found))
        if len(found) > WCAP:
            raise SystemExit(f"STOP: {len(found)} rows, above the {WCAP} this script prints; the answer is kept")
        for row in sorted(found, key=lambda r: (value(r, "VOUCHERNUMBER"), value(r, "GUID"))):
            print("  ", "; ".join(f"{tag} {value(row, tag)}" for tag in WFIELDS),
                  "; IRN", "text" if (row.findtext("IRN") or "").strip() else "none", "; GUID", value(row, "GUID"))
    elif kind == "R":
        found = rows(root, "VOUCHER")
        print("  VOUCHER rows:", len(found))
        for row in found:
            for tag in HEADER:
                print(f"   {tag} {value(row, tag)}")
            for leg in legs(row):
                print("   leg", " | ".join(leg))
    elif kind == "T":
        found = rows(root, "VOUCHERTYPE")
        print("  VOUCHERTYPE rows:", len(found))
        for name in types:
            mine = [row for row in found if row.get("NAME") == name]
            print(f"  named {name!r}: {len(mine)}")
            for row in mine:
                print("  ", "; ".join(f"{tag} {value(row, tag)}" for tag in ("PARENT", "GUID", "NUMBERINGMETHOD", "ISACTIVE")))
                series = [s for s in row if s.tag == "VOUCHERNUMBERSERIES.LIST"]
                print("   VOUCHERNUMBERSERIES.LIST elements:", len(series))
                for s in series:
                    print("   series:", "; ".join(f"{tag} {value(s, tag)}" for tag in ("NAME", "NUMBERINGMETHOD", "PREVENTDUPLICATES",
                                                                             "DUPLICATECONTROL", "BEGINNINGNUMBER")))
                    # FY14: a series that restarts its numbering lists its restart dates here; none is expected.
                    restart = [c for c in s if c.tag == "RESTARTFROMLIST.LIST"]
                    print("   RESTARTFROMLIST.LIST elements:", len(restart),
                          "children:", [len(r) for r in restart])
                    # FY14R: what a restart entry holds, leaf by leaf.
                    for number, entry in enumerate(restart, 1):
                        for leaf in entry.iter():
                            text = (leaf.text or "").strip()
                            if leaf is not entry and not len(leaf) and text:
                                print(f"   restart [{number}] {leaf.tag} {text!r}")
    elif kind == "U":
        # SC: the company's tax units, as the build reads them.
        found = rows(root, "TAXUNIT")
        print("  TAXUNIT rows:", len(found))
        for row in found:
            guid = (row.findtext("GUID") or "").strip()
            print("  ", "; ".join(f"{attr} {row.get(attr)!r}" for attr in ("NAME", "TAXTYPE", "TAXREGISTRATION")),
                  "; GUID", repr(guid), "; the book's:", "yes" if guid.startswith(BOOK + "-") else "no")
            print("   ", "; ".join(f"{tag} {value(row, tag)}" for tag in UFIELDS))
            for dated in (d for d in row if d.tag == "GSTREGISTRATIONDETAILS.LIST" and len(d)):
                print("    dated:", "; ".join(f"{tag} {value(dated, tag)}" for tag in UDATED))
    else:
        raise SystemExit(f"{path.name}: no kind for this name")

def leaves(row):
    # SC: every attribute and every text leaf of a TAXUNIT row, keyed by its path inside the row; repeated tags are
    # numbered in document order.
    out = {f"@{k}": v for k, v in row.attrib.items()}
    def walk(node, path):
        seen = {}
        for child in node:
            seen[child.tag] = seen.get(child.tag, 0) + 1
            here = f"{path}/{child.tag}[{seen[child.tag]}]"
            for k, v in child.attrib.items():
                out[f"{here}@{k}"] = v
            if len(child):
                walk(child, here)
            else:
                out[here] = (child.text or "").strip()
    walk(row, "")
    return out

def unit_diff(a, b):
    # SC: what the alteration changed in each tax unit, leaf by leaf, bounded (AGENTS.md P5).
    ua, ub = ({row.get("NAME"): row for row in rows(parse(p), "TAXUNIT")} for p in (a, b))
    print(f"== --unit-diff {a.name} {b.name}: TAXUNIT names {sorted(ua)} -> {sorted(ub)}")
    printed = 0
    for name in sorted(set(ua) | set(ub), key=str):
        if name not in ua or name not in ub:
            print(f"   unit {name!r}: in {'the second' if name in ub else 'the first'} only")
            continue
        la, lb = leaves(ua[name]), leaves(ub[name])
        changed = [k for k in sorted(set(la) | set(lb)) if la.get(k) != lb.get(k)]
        print(f"   unit {name!r}: {len(changed)} leaves differ")
        for key in changed:
            if printed == UCAP:
                raise SystemExit(f"STOP: more than {UCAP} differing leaves; the answers are kept")
            print(f"     {key}: {la.get(key, '<absent>')!r} -> {lb.get(key, '<absent>')!r}")
            printed += 1

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
    # RS: every list child, by tag: counts, and positions whose whole subtree differs. Content is never printed, so
    # the output stays one line per tag (LAB-COMMON C11, AGENTS.md P5).
    lists = sorted({c.tag for c in va if len(c)} | {c.tag for c in vb if len(c)})
    for tag in lists:
        xa = [ET.tostring(c) for c in va if c.tag == tag]
        xb = [ET.tostring(c) for c in vb if c.tag == tag]
        differ = sum(1 for x, y in zip(xa, xb) if x != y) + abs(len(xa) - len(xb))
        if differ:
            print(f"   list {tag}: {len(xa)} -> {len(xb)}, {differ} differ")
    print(f"   lists compared: {len(lists)}")
    la, lb = legs(va), legs(vb)
    print(f"   legs: {len(la)} -> {len(lb)}")
    for index in range(max(len(la), len(lb))):
        x = la[index] if index < len(la) else None
        y = lb[index] if index < len(lb) else None
        if x != y:
            print(f"   leg {index + 1}: {x} -> {y}")

def company_row(path):
    found = rows(parse(path), "COMPANY")
    if len(found) != 1:
        raise SystemExit(f"STOP: {path.name} holds {len(found)} COMPANY rows, not one")
    row = tuple(value(found[0], tag) for tag in ("GUID", "ALTVCHID", "ALTMSTID"))
    # FY14R: never the right company by name alone.
    if (found[0].findtext("GUID") or "").strip() != BOOK:
        raise SystemExit(f"STOP: {path.name}'s COMPANY row is not BRIDGE PILOT LAB's GUID")
    return row

def same_company(first, later, marks):
    # --before-write: the marks read just before a write holds one COMPANY row, with s1's GUID (LAB-COMMON C3).
    # --same-row: after the restore and reload, the COMPANY row equals s1's: GUID, ALTVCHID and ALTMSTID. The
    # answer's bytes are not compared, since its CMPINFO counters are per load.
    a, b = company_row(first), company_row(later)
    width = 3 if marks else 1
    print(f"== {later.name} against {first.name}: GUID, ALTVCHID, ALTMSTID {b} against {a}")
    if a[:width] != b[:width]:
        raise SystemExit("STOP: the COMPANY row differs")
    print("  equal" if marks else "  one COMPANY row, the same GUID: the write may be sent")

args = sys.argv[1:]
ledger = "BRIDGE Customer RJ"
types = ["BRIDGE Sales", "BRIDGE Sales Keyed"]
if args[:1] == ["--type"]:
    types, args = [args[1]], args[2:]
if args[:1] in (["--before-write"], ["--same-row"]):
    same_company(pathlib.Path(args[1]), pathlib.Path(args[2]), args[0] == "--same-row")
    raise SystemExit(0)
if args[:1] == ["--ledger"]:
    ledger, args = args[1], args[2:]
if args[:1] == ["--guid"]:
    # KC: whether a voucher's GUID is in each W answer, and under which number, date and type.
    for name in args[2:]:
        found = [row for row in rows(parse(pathlib.Path(name)), "VOUCHER") if (row.findtext("GUID") or "").strip() == args[1]]
        print(f"== --guid {args[1]} {name}: {len(found)} rows")
        for row in found:
            print("  ", "; ".join(f"{tag} {value(row, tag)}" for tag in WFIELDS))
    raise SystemExit(0)
if args[:1] == ["--unit-diff"]:
    unit_diff(pathlib.Path(args[1]), pathlib.Path(args[2]))
    raise SystemExit(0)
if args[:1] == ["--diff"]:
    diff(pathlib.Path(args[1]), pathlib.Path(args[2]))
else:
    for name in args:
        report(pathlib.Path(name), ledger)
