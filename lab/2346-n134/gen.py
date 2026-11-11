#!/usr/bin/env python3
# gen.py: writes the requests of row 2346-N134 beside this file, UTF-16LE with a BOM, as sent. It starts from files
# beside it that are byte-equal to earlier rows' (ROW.md says which), and changes only what each line below names.
# Every change is asserted to occur exactly as often as expected, and each template's sha256 is checked first.
#   I-NC1.xml  row 2346-MN's I-MC1.xml (template-I-MC1.utf16le.xml here: CN/26-27/0001 into BRIDGE Credit Note
#              Manual, REFERENCE BP/26-27/0010) with the REMOTEID tail e31 in place of e11, and nothing else. It is
#              parsed, and its type, number, REFERENCE, REFERENCEDATE, six legs and the party's side are asserted.
#   R-NCa.xml  row 2346-MN's R-MC1.xml (the build's read-back for CN/26-27/0001 on the type whose GUID ends
#              000000e8) with the type GUID ending 000000ea: the read-back to send if the keyed type takes that
#              GUID instead (row 2346-MN's second key did).
#   W-08c.xml  W-08s.xml (row 2162-M1's A-Y0-2026-08.xml: the build's number read of August 2026 on this book,
#              pinned by a test on master) with the COMPUTE as BRIDGEVCHISCREDITNOTE:$$IsCreditNote:$VoucherTypeName.
#   W-08d.xml  the same with BRIDGEVCHISDEBITNOTE:$$IsDebitNote:$VoucherTypeName.
#   Both COMPUTE lines are the element and form src-tauri/src/agent_voucher_type_class.rs writes for its class read,
#   and row 2346-MN sent them in its one-number reads.
# Example: python3 -I gen.py
import hashlib
import pathlib
import xml.etree.ElementTree as ET
here = pathlib.Path(__file__).resolve().parent
TEMPLATES = {
    "template-I-MC1.utf16le.xml": "bcec0c4608d5f9823fed6b629252876650199444d526b2bffa1fbb243c549bf7",
    "R-MC1.xml": "eccb528fdcd6066a9db2f437854409af9da7d3427bbeeeef68f2f6a426dd3621",
    "W-08s.xml": "b226ad52af00b919a62d9c29e2d2e6db0fe1ad4071fbe2a97da3a82852c23d26",
}
BOOK = "6b43e498-430c-4d5c-bfef-d32e2ab93c85"
CREDIT, CN, ORIGINAL = "BRIDGE Credit Note Manual", "CN/26-27/0001", "BP/26-27/0010"
SALES_COMPUTE = "<COMPUTE>BRIDGEVCHISSALES:$$IsSales:$VoucherTypeName</COMPUTE>"
def load(name):
    raw = (here / name).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == TEMPLATES[name], f"{name} is not the file ROW.md names"
    assert raw[:2] == b"\xff\xfe", name
    return raw[2:].decode("utf-16-le")
def save(name, text):
    raw = b"\xff\xfe" + text.encode("utf-16-le")
    (here / name).write_bytes(raw)
    print(f"{name}: {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}")
def swap(text, old, new, count):
    assert text.count(old) == count, (old, text.count(old), count)
    return text.replace(old, new)
def key(tail):
    # The REMOTEID of the lab's synthetic family with this tail, as row 2346-MN's gen.py writes it.
    return "00000000-0000-4000-8000-000000000" + tail

# I-NC1.xml: a new key, nothing else.
note = swap(load("template-I-MC1.utf16le.xml"), key("e11"), key("e31"), 1)
voucher = ET.fromstring(note[note.index("<ENVELOPE>"):]).find("BODY/IMPORTDATA/REQUESTDATA/TALLYMESSAGE/VOUCHER")
assert voucher.get("REMOTEID") == key("e31")
assert voucher.get("VCHTYPE") == CREDIT and voucher.findtext("VOUCHERTYPENAME") == CREDIT
assert [c.text for c in voucher if c.tag == "VOUCHERNUMBER"] == [CN]
assert voucher.findtext("REFERENCE") == ORIGINAL and voucher.findtext("REFERENCEDATE") == "20260801"
assert voucher.findtext("DATE") == "20260801"
legs = voucher.findall("LEDGERENTRIES.LIST")
assert len(legs) == 6, len(legs)
party = legs[0]
assert party.findtext("LEDGERNAME") == "BRIDGE Walk-in" and party.findtext("ISDEEMEDPOSITIVE") == "No"
assert not party.findall("BILLALLOCATIONS.LIST"), "the party is not bill-wise: no bill allocation"
assert all(leg.findtext("ISDEEMEDPOSITIVE") == "Yes" for leg in legs[1:])
assert sum(round(float(leg.findtext("AMOUNT")) * 100) for leg in legs) == 0, "the legs balance"
save("I-NC1.xml", note)

# R-NCa.xml: the read-back on the type GUID ending 000000ea.
save("R-NCa.xml", swap(load("R-MC1.xml"), f'"{BOOK}-000000e8"', f'"{BOOK}-000000ea"', 1))

# W-08c.xml and W-08d.xml: the month read with a note class's compute.
month = load("W-08s.xml")
assert month.count("<COMPUTE>") == 1
for name, compute in (("W-08c.xml", "<COMPUTE>BRIDGEVCHISCREDITNOTE:$$IsCreditNote:$VoucherTypeName</COMPUTE>"),
                      ("W-08d.xml", "<COMPUTE>BRIDGEVCHISDEBITNOTE:$$IsDebitNote:$VoucherTypeName</COMPUTE>")):
    save(name, swap(month, SALES_COMPUTE, compute, 1))
