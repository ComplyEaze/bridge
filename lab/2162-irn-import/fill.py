#!/usr/bin/env python3
# fill.py K-ANSWER: on the lab PC, writes I-3-sent.xml and I-4-sent.xml beside this file, from I-3.xml and I-4.xml, with
# the 15 X of their one PARTYGSTIN element replaced by the customer's GSTIN as that sitting's own ledger read holds it.
# The book's GSTINs are invented and stay on the lab PC (as in lab/1378-irn-capture-answers), so the committed requests
# carry the placeholder, and the two sent files never leave the lab PC.
# Before writing, it checks I-3.xml and I-4.xml against SHA256.txt. It takes the GSTIN from the K answer's one LEDGER row
# named BRIDGE IRN Customer: every non-empty PARTYGSTIN and every GSTIN inside a LEDGSTREGDETAILS.LIST. Exactly one
# distinct value, of GSTIN shape and with state code 29 (Karnataka, the customer's state), else STOP. It prints each sent
# file's bytes and sha256 for kept-local.txt, and never the GSTIN itself.
# Example: python3 -I fill.py a-s1-K-customer.xml
import hashlib, pathlib, re, sys
import xml.etree.ElementTree as ET
here = pathlib.Path(__file__).resolve().parent
FORBIDDEN = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));", re.I)
GSTIN = re.compile(r"\d{2}[A-Z]{5}\d{4}[A-Z][A-Z\d]Z[A-Z\d]")
PLACEHOLDER = "<PARTYGSTIN>" + "X" * 15 + "</PARTYGSTIN>"
PARTY = "BRIDGE IRN Customer"

listed = {}
for line in (here / "SHA256.txt").read_text().splitlines():
    digest, _, name = line.split()
    listed[name] = digest
def checked(name):
    raw = (here / name).read_bytes()
    if hashlib.sha256(raw).hexdigest() != listed.get(name):
        raise SystemExit(f"STOP: {name} does not match SHA256.txt")
    assert raw[:2] == b"\xff\xfe", name
    return raw[2:].decode("utf-16-le")

raw = pathlib.Path(sys.argv[1]).read_bytes()
text = FORBIDDEN.sub("", (raw[2:] if raw[:2] == b"\xff\xfe" else raw).decode("utf-16-le"))
root = ET.fromstring(re.sub(r"^\s*<\?xml[^>]*\?>", "", text))
collection = root.find("BODY/DATA/COLLECTION")
if collection is None:
    raise SystemExit("STOP: no BODY/DATA/COLLECTION in the K answer; the read failed")
def name_of(node):
    found = [child for child in node if child.tag == "NAME"]
    return node.get("NAME") if not found else "".join(found[0].itertext())
mine = [row for row in collection if row.tag == "LEDGER" and name_of(row) == PARTY]
if len(mine) != 1:
    raise SystemExit(f"STOP: the K answer holds {len(mine)} LEDGER rows named {PARTY!r}, not one")
values = {(e.text or "").strip() for e in mine[0] if e.tag == "PARTYGSTIN"}
for detail in (d for d in mine[0] if d.tag == "LEDGSTREGDETAILS.LIST"):
    values |= {(e.text or "").strip() for e in detail if e.tag == "GSTIN"}
values.discard("")
if len(values) != 1:
    raise SystemExit(f"STOP: {len(values)} distinct GSTIN values on the customer's ledger, not one")
gstin = values.pop()
if not GSTIN.fullmatch(gstin):
    raise SystemExit("STOP: the customer's GSTIN is not of GSTIN shape")
if gstin[:2] != "29":
    raise SystemExit("STOP: the customer's GSTIN does not carry state code 29")
for name in ("I-3.xml", "I-4.xml"):
    request = checked(name)
    assert request.count(PLACEHOLDER) == 1 and request.count("X" * 15) == 1, name
    sent = here / name.replace(".xml", "-sent.xml")
    data = b"\xff\xfe" + request.replace(PLACEHOLDER, f"<PARTYGSTIN>{gstin}</PARTYGSTIN>").encode("utf-16-le")
    assert len(data) == (here / name).stat().st_size, name
    sent.write_bytes(data)
    print(f"{hashlib.sha256(data).hexdigest()}  {len(data)}  {sent.name}")
