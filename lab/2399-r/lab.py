#!/usr/bin/env python3
# lab.py marks ANSWER [--guid G] [--expect V M]   one COMPANY row, prints its GUID, ALTVCHID and ALTMSTID
# lab.py counters ANSWER                         an import answer's counters, LASTVCHID, LASTMID and any LINEERROR
# lab.py rows ANSWER                             the COLLECTION rows (C4): tag, then NAME, VOUCHERNUMBER, DATE, PARENT
# Decodes UTF-16LE (a BOM or none), drops only the characters XML forbids (0-8, 11, 12, 14-31; 9, 10 and 13 kept),
# parses as XML, never a text search (LAB-COMMON C4). Reads at most 2,000,000 bytes (C3).
# Exit 0: read and printed (with --expect, the marks equal). Exit 1 = STOP: empty, over the bound, does not decode,
# does not parse, no BODY/DATA/COLLECTION (marks, rows) or a missing counter (counters), other than one COMPANY row
# or a GUID that is not --guid (marks). Exit 2: `marks --expect` read the marks and they differ (a result).
import re, sys, pathlib, xml.etree.ElementTree as ET
BOUND = 2_000_000
FORBIDDEN = re.compile("[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:[0-8]|1[124]|1[4-9]|2[0-9]|3[01]);|&#x(?:[0-8]|b|c|e|f|1[0-9a-f]);", re.I)
def parse(path):
    raw = pathlib.Path(path).read_bytes() if pathlib.Path(path).is_file() else b""
    if not raw or len(raw) > BOUND:
        raise SystemExit(f"STOP: {path} is missing, empty or above {BOUND} bytes (the request failed)")
    body = raw[2:] if raw[:2] == b"\xff\xfe" else raw
    try:
        text = FORBIDDEN.sub("", body.decode("utf-16-le"))
        return ET.fromstring(text.encode("utf-8"))
    except (UnicodeDecodeError, ET.ParseError, ValueError) as error:
        raise SystemExit(f"STOP: {path} does not decode or parse ({error})")
def collection(root):
    node = root.find("BODY/DATA/COLLECTION")
    if node is None:
        raise SystemExit("STOP: no ENVELOPE/BODY/DATA/COLLECTION (the request failed, or Tally refused it)")
    return node
def text(node, tag):
    child = node.find(tag)
    return None if child is None else (child.text or "").strip()
mode, path, rest = sys.argv[1], sys.argv[2], sys.argv[3:]
root = parse(path)
if mode == "marks":
    rows = collection(root).findall("COMPANY")
    if len(rows) != 1:
        raise SystemExit(f"STOP: {len(rows)} COMPANY rows, expected exactly one")
    guid, v, m = text(rows[0], "GUID"), text(rows[0], "ALTVCHID"), text(rows[0], "ALTMSTID")
    print(f"COMPANY {guid} ALTVCHID {v} ALTMSTID {m}")
    if "--guid" in rest and guid != rest[rest.index("--guid") + 1]:
        raise SystemExit("STOP: the row's GUID is not the book's")
    if "--expect" in rest:
        want = rest[rest.index("--expect") + 1: rest.index("--expect") + 3]
        print("marks equal" if [v, m] == want else f"marks DIFFER from the expected {want}")
        raise SystemExit(0 if [v, m] == want else 2)
elif mode == "counters":
    names = ["CREATED", "ALTERED", "DELETED", "COMBINED", "IGNORED", "ERRORS", "CANCELLED", "EXCEPTIONS", "LASTVCHID", "LASTMID"]
    values = {n: text(root.find("BODY/DATA/IMPORTRESULT") if root.find("BODY/DATA/IMPORTRESULT") is not None else root, n) for n in names}
    missing = [n for n in names[:8] if values[n] is None]
    if missing:
        raise SystemExit(f"STOP: counters missing: {missing}")
    print(" ".join(f"{n} {values[n]}" for n in names))
    for node in root.iter("LINEERROR"):
        print("LINEERROR", repr(node.text))
else:
    rows = list(collection(root))
    print(f"rows {len(rows)}")
    for node in rows[:200]:
        fields = {t: text(node, t) for t in ("NAME", "VOUCHERNUMBER", "DATE", "PARENT") if node.find(t) is not None}
        print(node.tag, repr(fields))
