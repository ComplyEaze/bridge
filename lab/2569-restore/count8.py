#!/usr/bin/env python3
# count8.py ANSWER...: row 2162-IRN10's count.py (lab/2162-irn-pending-sheet at 836dd394) for row 2440-CP, with two changes,
# each marked CP below: the K kind's tax family also takes ROUNDING (a duty ledger's rounding method and limit), and a
# kind O is added for the company object. IRN10's count.py was row 2162-IRN9's (lab/2162-irn-import-sheet) with its K
# kind widened to any ledger by name.
# It prints what ROW.md records of each answer, by an XML parser (LAB-COMMON C4), never a text search.
# Its parsing, its STOP rules and its --before-write and --same-row checks are row 2162-BC's count5.py (lab/2162-bc-sheet);
# the kinds are this row's. The kind is the first letter of the request's name inside the answer's name (a-<label>-<request>):
#   M- marks: the COMPANY rows, each GUID (whether it is the book's), ALTVCHID, ALTMSTID.
#   L- list: every VOUCHER row's number, date, type and MASTERID.
#   V- the vouchers window read with the three e-invoice elements named: each row's number and type, then DATE, IRN
#      (its length), IRNACKNO and IRNACKDATE, each with its count and TYPE.
#   F- every native field of the vouchers of one number: for each VOUCHER row, its number, type and size, the GST
#      header fields named in GST below, and every direct child of the e-invoice family (EINVOICE below).
#   K- every native field of one ledger by name: each LEDGER row's name, PARENT, ISBILLWISEON, TAXTYPE and GSTDUTYHEAD,
#      how many GSTIN values it holds (the GSTIN itself is never printed), then every non-empty leaf of the GST and tax
#      family (TAXFAMILY below) by path; for BRIDGE IRN IGST, whether it reads as a GST ledger with duty head IGST.
#   I- import: every counter Tally returned and every LINEERROR.
#   O- the company object (CP): whether its GUID is the book's and ISGSTCLASSIFON, then every non-empty leaf of each
#      CMPGSTDETAILS.LIST and HSNDETAILS.LIST directly under the company, by path. An answer with no
#      BODY/DATA/TALLYMESSAGE/COMPANY is a failed request: STOP.
# Every read answer (all but I) prints its HEADER/STATUS and every LINEERROR first. An answer with no BODY/DATA/COLLECTION,
# or an import answer with no counters, is a failed request: count.py prints STOP and exits non-zero, and never reports it
# as 0 rows. Every answer prints its bytes and its count of GSTIN shapes.
# count.py --diff A B: two F answers, one VOUCHER row each: every direct child, without children of its own, whose value
#   differs between them or that is present in one only.
# count.py --before-write S1 M: M holds exactly one COMPANY row, with the GUID of S1's one row; else STOP.
# count.py --same-row S1 M: as --before-write, and ALTVCHID and ALTMSTID also equal S1's; else STOP.
# An element absent is printed <absent>, one present and empty ''. A value of GSTIN shape prints <GSTIN shape>, a run of
# more than 8 digits <n digits>, and any other value longer than 24 characters <n chars>.
# Example: python3 -I count.py a-s1-M-marks.xml
import pathlib, re, sys
import xml.etree.ElementTree as ET
FORBIDDEN = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));", re.I)
GSTIN = re.compile(r"\d{2}[A-Z]{5}\d{4}[A-Z][A-Z\d]Z[A-Z\d]")
BOOK = "eb2f5190-292e-4ed8-bfd3-eace839fe699"
IRN_TAGS = ["IRN", "IRNACKNO", "IRNACKDATE"]
GST = ["PARTYGSTIN", "CONSIGNEEGSTIN", "STATENAME", "PLACEOFSUPPLY", "GSTREGISTRATIONTYPE", "ISINVOICE", "PERSISTEDVIEW",
       "VCHENTRYMODE", "NUMBERINGSTYLE", "VCHGSTCLASS", "VCHGSTSTATUSISAPPLICABLE", "VCHGSTSTATUSISINCLUDED",
       "VCHGSTSTATUSISUNCERTAIN", "VCHGSTSTATUSISEXCLUDED", "ISEWAYBILLAPPLICABLE"]
EINVOICE = re.compile(r"^IRN|EINV|IRNQRCODE|QRCODE")
TAXFAMILY = re.compile(r"GST|TAX|HSN|SAC|RATE|DUTY|CESS|APPLICAB|TYPEOFSUPPLY|ROUNDING")  # CP: ROUNDING added
COUNTERS = ["CREATED", "ALTERED", "DELETED", "LASTVCHID", "LASTMID", "COMBINED", "IGNORED", "ERRORS", "CANCELLED",
            "EXCEPTIONS"]

def show(text):
    text = (text or "").strip()
    if GSTIN.fullmatch(text):
        return "<GSTIN shape>"
    if re.fullmatch(r"\d{9,}", text):
        return f"<{len(text)} digits>"
    return repr(text) if len(text) <= 24 else f"<{len(text)} chars>"

def masked(text):
    return GSTIN.sub("<GSTIN shape>", text)

def parse(path):
    raw = path.read_bytes()
    body = raw[2:] if raw[:2] == b"\xff\xfe" else raw
    decoded = body.decode("utf-16-le")
    text = FORBIDDEN.sub("", decoded)
    text = re.sub(r"^\s*<\?xml[^>]*\?>", "", text)
    return ET.fromstring(text), len(raw), len(GSTIN.findall(decoded))

def status(root):
    found = root.find("HEADER/STATUS")
    print("  STATUS", "<absent>" if found is None else repr((found.text or "").strip()))
    for error in root.iter("LINEERROR"):
        print("  LINEERROR", repr(masked("".join(error.itertext()))))

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
    return show("".join(found[0].itertext()))

def name_of(node):
    found = [child for child in node if child.tag == "NAME"]
    return node.get("NAME") if not found else "".join(found[0].itertext())

def report(path):
    root, size, gstins = parse(path)
    kind = path.name.split("-", 2)[-1][:1]
    print(f"== {path.name}: {size} bytes, GSTIN shapes {gstins}")
    if kind != "I":
        status(root)
    if kind == "M":
        found = rows(root, "COMPANY")
        for company in found:
            guid = (company.findtext("GUID") or "").strip()
            print("  COMPANY: GUID is the book's:", "yes" if guid == BOOK else "no", "ALTVCHID", value(company, "ALTVCHID"),
                  "ALTMSTID", value(company, "ALTMSTID"))
        print("  COMPANY rows:", len(found))
    elif kind == "L":
        found = rows(root, "VOUCHER")
        print("  VOUCHER rows:", len(found))
        for row in found:
            print("  ", value(row, "VOUCHERNUMBER"), value(row, "DATE"), value(row, "VOUCHERTYPENAME"), "MASTERID",
                  value(row, "MASTERID"))
    elif kind == "V":
        found = rows(root, "VOUCHER")
        print("  VOUCHER rows:", len(found))
        for row in found:
            print("   row", value(row, "VOUCHERNUMBER"), value(row, "VOUCHERTYPENAME"), "DATE", value(row, "DATE"))
            for tag in IRN_TAGS:
                elements = [child for child in row if child.tag == tag]
                if not elements:
                    print(f"    {tag}: 0")
                for element in elements:
                    text = (element.text or "").strip()
                    shown = f"length {len(text)}" if tag == "IRN" else show(text)
                    print(f"    {tag}: {len(elements)}, TYPE {element.get('TYPE', '')!r}, {shown}")
    elif kind == "F":
        found = rows(root, "VOUCHER")
        print("  VOUCHER rows:", len(found))
        for row in found:
            print("   voucher", value(row, "VOUCHERNUMBER"), value(row, "VOUCHERTYPENAME"), "DATE", value(row, "DATE"),
                  f"direct children {len(row)}, elements {sum(1 for _ in row.iter())}")
            for tag in GST:
                print(f"    {tag} {value(row, tag)}")
            family = [child for child in row if EINVOICE.search(child.tag) and not len(child)]
            print("    e-invoice family, direct children:", len(family))
            for child in family:
                text = (child.text or "").strip()
                shown = f"length {len(text)}" if child.tag in ("IRN", "IRNQRCODE") else show(text)
                print(f"     {child.tag} TYPE {child.get('TYPE', '')!r} {shown}")
    elif kind == "K":
        found = rows(root, "LEDGER")
        print(f"  LEDGER rows: {len(found)}")
        for row in found:
            held = [e for e in row if e.tag == "PARTYGSTIN" and (e.text or "").strip()]
            for detail in (d for d in row if d.tag == "LEDGSTREGDETAILS.LIST"):
                held += [e for e in detail if e.tag == "GSTIN" and (e.text or "").strip()]
            print("   NAME", repr(name_of(row)), "PARENT", value(row, "PARENT"), "ISBILLWISEON", value(row, "ISBILLWISEON"),
                  "TAXTYPE", value(row, "TAXTYPE"), "GSTDUTYHEAD", value(row, "GSTDUTYHEAD"),
                  "GSTIN values held:", len(held), "distinct:", len({(e.text or "").strip() for e in held}))
            # Every non-empty leaf of the GST and tax family, by path, so the ledger's tax set-up can be compared.
            def walk(node, path):
                for child in node:
                    here_path = f"{path}/{child.tag}" if path else child.tag
                    text = (child.text or "").strip()
                    if not len(child) and text and TAXFAMILY.search(here_path) and child.tag not in ("PARTYGSTIN", "GSTIN"):
                        print(f"    {here_path} {show(text)}")
                    walk(child, here_path)
            walk(row, "")
            if name_of(row) == "BRIDGE IRN IGST":
                # TallyPrime 7.2 stores the Tax type chosen on screen as IGST in GSTDUTYHEAD (row 2162-IRN10's s4r,
                # lab/2162-irn-pending-answers): the screen's list offers IGST, CGST, SGST/UTGST and Cess.
                ready = value(row, "TAXTYPE") == "'GST'" and value(row, "GSTDUTYHEAD") == "'IGST'"
                print("   IGST ledger reads as a GST ledger with duty head IGST:", "yes" if ready else "no")
    elif kind == "O":  # CP: the company object
        company = root.find("BODY/DATA/TALLYMESSAGE/COMPANY")
        if company is None:
            raise SystemExit("STOP: no BODY/DATA/TALLYMESSAGE/COMPANY in the answer; the request failed, it did not find nothing")
        guid = (company.findtext("GUID") or "").strip()
        print("  COMPANY", repr(company.get("NAME")), "GUID is the book's:", "yes" if guid == BOOK else "no",
              "ISGSTCLASSIFON", value(company, "ISGSTCLASSIFON"))
        for list_tag in ("CMPGSTDETAILS.LIST", "HSNDETAILS.LIST"):
            lists = [child for child in company if child.tag == list_tag]
            print(f"  {list_tag}: {len(lists)}")
            for number, entry in enumerate(lists, 1):
                for leaf in entry.iter():
                    text = (leaf.text or "").strip()
                    if leaf is not entry and not len(leaf) and text:
                        print(f"    [{number}] {leaf.tag} {show(text)}")
    elif kind == "I":
        result = root if root.tag == "RESPONSE" else root.find(".//IMPORTRESULT")
        print("  root", root.tag)
        if result is None:
            for error in root.iter("LINEERROR"):
                print("  LINEERROR", repr(masked("".join(error.itertext()))))
            raise SystemExit("STOP: no import counters in the answer; the request failed")
        for tag in COUNTERS:
            print(f"  {tag} {value(result, tag)}")
        for error in root.iter("LINEERROR"):
            print("  LINEERROR", repr(masked("".join(error.itertext()))))
    else:
        raise SystemExit(f"{path.name}: no kind for this name")

def diff(a, b):
    va, vb = (rows(parse(p)[0], "VOUCHER") for p in (a, b))
    if len(va) != 1 or len(vb) != 1:
        raise SystemExit(f"--diff needs one VOUCHER row in each: {len(va)}, {len(vb)}")
    va, vb = va[0], vb[0]
    print(f"== --diff {a.name} {b.name}")
    tags = sorted({c.tag for c in va if not len(c)} | {c.tag for c in vb if not len(c)})
    def raw(node, tag):
        # Compared by the trimmed text itself; show() is only how a difference is printed.
        return [("".join(c.itertext())).strip() for c in node if c.tag == tag]
    for tag in tags:
        if raw(va, tag) != raw(vb, tag):
            print(f"   {tag}: {value(va, tag)} -> {value(vb, tag)}")

def company_row(path):
    found = rows(parse(path)[0], "COMPANY")
    if len(found) != 1:
        raise SystemExit(f"STOP: {path.name} holds {len(found)} COMPANY rows, not one")
    # The raw values, trimmed: show() would print a GUID as its length.
    return tuple((found[0].findtext(tag) or "").strip() for tag in ("GUID", "ALTVCHID", "ALTMSTID"))

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
if args[:1] in (["--before-write"], ["--same-row"]):
    same_company(pathlib.Path(args[1]), pathlib.Path(args[2]), args[0] == "--same-row")
elif args[:1] == ["--diff"]:
    diff(pathlib.Path(args[1]), pathlib.Path(args[2]))
else:
    for name in args:
        report(pathlib.Path(name))
