#!/usr/bin/env python3
# count7.py ANSWER...: row 2162-IRN10's count.py (lab/2162-irn-pending-sheet) cut to the three kinds this row sends,
# with its K kind widened to every leaf and a C kind and a --leafdiff check added. It prints what ROW.md records of each
# answer, by an XML parser (LAB-COMMON C4), never a text search. Its parsing, its STOP rules and its --before-write and
# --same-row checks are that file's, with the book's GUID changed to BRIDGE PILOT LAB's. The kind is the first letter of
# the request's name inside the answer's name (a-<label>-<request>):
#   M- marks: the COMPANY rows, each GUID (whether it is the book's), ALTVCHID, ALTMSTID.
#   K- every native field of BRIDGE Customer RJ: each LEDGER row's name, PARENT, ISBILLWISEON and STATENAME, how many
#      GSTIN values it holds (a GSTIN is never printed), how many LEDGSTREGDETAILS.LIST it holds, then every leaf with
#      a value, by path, at most LEAF_CAP of them.
#   C- the build's compliance read of every ledger: how many LEDGER rows, then for the row named TARGET its PARENT,
#      STATENAME and every leaf of each LEDGSTREGDETAILS.LIST, empty ones included.
# Every answer prints its HEADER/STATUS and every LINEERROR first, then its bytes and its count of GSTIN shapes. An
# answer with no BODY/DATA/COLLECTION is a failed request: count7.py prints STOP and exits non-zero, and never reports
# it as 0 rows.
# count7.py --leafdiff A B: two answers of one kind (K or C), each holding exactly one LEDGER row named TARGET (else
#   STOP): every leaf of that row, by indexed path, whose value or TYPE differs between them or that is in one only,
#   empty leaves included; then how many differ. "0 leaves differ" is printed only after both rows were found.
# count7.py --before-write S1 M: M holds exactly one COMPANY row, with the GUID of S1's one row; else STOP.
# count7.py --same-row S1 M: as --before-write, and ALTVCHID and ALTMSTID also equal S1's; else STOP.
# An element absent is printed <absent>, one present and empty ''. A value of GSTIN shape prints <GSTIN shape>, a run
# of more than 8 digits <n digits>, and any other value longer than 24 characters <n chars>.
# Example: python3 -I count7.py a-s1-M-marks.xml
import pathlib, re, sys
import xml.etree.ElementTree as ET
FORBIDDEN = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));", re.I)
GSTIN = re.compile(r"\d{2}[A-Z]{5}\d{4}[A-Z][A-Z\d]Z[A-Z\d]")
BOOK = "6b43e498-430c-4d5c-bfef-d32e2ab93c85"
TARGET = "BRIDGE Customer RJ"
LEAF_CAP = 400

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

def leaves(node, path=""):
    # Every leaf below node, by a path whose steps carry the element's index among siblings of its tag, so two
    # LEDGSTREGDETAILS.LIST entries never share a path. Yields (path, TYPE attribute, trimmed text).
    seen = {}
    for child in node:
        seen[child.tag] = seen.get(child.tag, 0) + 1
        step = f"{child.tag}[{seen[child.tag]}]"
        here_path = f"{path}/{step}" if path else step
        if len(child):
            yield from leaves(child, here_path)
        else:
            yield here_path, child.get("TYPE", ""), (child.text or "").strip()

def gstins_held(row):
    held = [e for e in row if e.tag == "PARTYGSTIN" and (e.text or "").strip()]
    for detail in (d for d in row if d.tag == "LEDGSTREGDETAILS.LIST"):
        held += [e for e in detail if e.tag == "GSTIN" and (e.text or "").strip()]
    return held

def report(path):
    root, size, gstins = parse(path)
    kind = path.name.split("-", 2)[-1][:1]
    print(f"== {path.name}: {size} bytes, GSTIN shapes {gstins}")
    status(root)
    if kind == "M":
        found = rows(root, "COMPANY")
        for company in found:
            guid = (company.findtext("GUID") or "").strip()
            print("  COMPANY: GUID is the book's:", "yes" if guid == BOOK else "no", "ALTVCHID", value(company, "ALTVCHID"),
                  "ALTMSTID", value(company, "ALTMSTID"))
        print("  COMPANY rows:", len(found))
    elif kind == "K":
        found = rows(root, "LEDGER")
        print(f"  LEDGER rows: {len(found)}")
        for row in found:
            held = gstins_held(row)
            print("   NAME", repr(name_of(row)), "PARENT", value(row, "PARENT"), "ISBILLWISEON", value(row, "ISBILLWISEON"),
                  "STATENAME", value(row, "STATENAME"), "GSTIN values held:", len(held),
                  "distinct:", len({(e.text or "").strip() for e in held}),
                  "LEDGSTREGDETAILS.LIST:", sum(1 for d in row if d.tag == "LEDGSTREGDETAILS.LIST"))
            shown = [(p, t, x) for p, t, x in leaves(row) if x]
            for leaf_path, kind_attr, text in shown[:LEAF_CAP]:
                print(f"    {leaf_path} {kind_attr!r} {show(text)}")
            if len(shown) > LEAF_CAP:
                print(f"    ... {len(shown) - LEAF_CAP} more leaves with a value, not printed")
    elif kind == "C":
        found = rows(root, "LEDGER")
        print(f"  LEDGER rows: {len(found)}")
        named = [row for row in found if name_of(row) == TARGET]
        print(f"  rows named {TARGET!r}: {len(named)}")
        for row in named:
            details = [d for d in row if d.tag == "LEDGSTREGDETAILS.LIST"]
            print("   PARENT", value(row, "PARENT"), "STATENAME", value(row, "STATENAME"),
                  "GSTIN values held:", len(gstins_held(row)), "LEDGSTREGDETAILS.LIST:", len(details))
            for index, detail in enumerate(details, 1):
                for leaf_path, kind_attr, text in leaves(detail):
                    print(f"    [{index}] {leaf_path} {kind_attr!r} {show(text)}")
    else:
        raise SystemExit(f"{path.name}: no kind for this name")

def target_row(path):
    found = [row for row in rows(parse(path)[0], "LEDGER") if name_of(row) == TARGET]
    if len(found) != 1:
        raise SystemExit(f"STOP: {path.name} holds {len(found)} LEDGER rows named {TARGET!r}, not one")
    return found[0]

def leafdiff(a, b):
    kinds = {p.name.split("-", 2)[-1][:1] for p in (a, b)}
    if len(kinds) != 1 or kinds - {"K", "C"}:
        raise SystemExit(f"STOP: --leafdiff needs two K or two C answers, not {sorted(kinds)}")
    left = {p: (t, x) for p, t, x in leaves(target_row(a))}
    right = {p: (t, x) for p, t, x in leaves(target_row(b))}
    print(f"== --leafdiff {a.name} {b.name}: {len(left)} and {len(right)} leaves")
    differ = 0
    for leaf_path in sorted(set(left) | set(right)):
        if left.get(leaf_path) != right.get(leaf_path):
            differ += 1
            def shown(entry):
                return "<absent>" if entry is None else f"{entry[0]!r} {show(entry[1])}"
            print(f"   {leaf_path}: {shown(left.get(leaf_path))} -> {shown(right.get(leaf_path))}")
    print(f"  {differ} leaves differ")

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
elif args[:1] == ["--leafdiff"]:
    leafdiff(pathlib.Path(args[1]), pathlib.Path(args[2]))
else:
    for name in args:
        report(pathlib.Path(name))
