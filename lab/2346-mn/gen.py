#!/usr/bin/env python3
# gen.py: writes the requests of row 2346-MN beside this file, UTF-16LE with a BOM, as sent. It starts from files
# beside it that are byte-equal to earlier rows' (ROW.md says which), and changes only what each line below names.
# Every change is asserted to occur exactly as often as expected, and each template's sha256 is checked first.
#   I-MC1.xml       row 2346-CD's I-CN.xml (its Credit Note, no number) with the type BRIDGE Credit Note Manual (in
#                   VCHTYPE and VOUCHERTYPENAME), the number CN/26-27/0001 in one VOUCHERNUMBER element placed after
#                   VOUCHERTYPENAME, where the build's Sales request places it, and the REMOTEID tail e11.
#   I-MC1-again.xml I-MC1.xml byte for byte: the same key and the same number, sent once at step 8 (ROW.md, C3).
#   I-MC1n.xml      I-MC1.xml with the tail e12 alone: a new key with the same number.
#   I-MD1.xml       row 2346-CD's I-DN.xml (its Debit Note, no number) with the type BRIDGE Debit Note Manual, the
#                   number DN/26-27/0001 placed as in I-MC1.xml, and the tail e13.
#   I-MC2.xml       I-MC1.xml with the type BRIDGE Credit Note Manual 2 and the tail e14.
#   I-MDX.xml       I-MD1.xml with the number CN/26-27/0001 and the tail e15.
#   R-MC1.xml       row 2346-CD's R-CN1.xml (the build's read-back for the number 1 on the type Credit Note) with
#                   the number CN/26-27/0001 and the type GUID ending 000000e8, predicted for BRIDGE Credit Note Manual.
#   R-MD1.xml       row 2346-CD's R-DN1.xml (the same on Debit Note) with DN/26-27/0001 and the GUID ending 000000e9.
#   R-MC2.xml       R-CN1.xml with CN/26-27/0001 and the GUID ending 000000ea (BRIDGE Credit Note Manual 2).
#   R-MDX.xml       R-DN1.xml with CN/26-27/0001 and the GUID ending 000000e9.
#   V-CN.xml        row 2346-CD's V-1.xml (every native field of the vouchers of one number) for CN/26-27/0001.
#   V-DN.xml        the same for DN/26-27/0001.
#   N-<n><c>.xml    the build's number read (template-n-0001.utf16le.xml) with the number n and the class c: s the
#                   COMPUTE unchanged (Sales), c the credit-note class, d the debit-note class, each written as
#                   agent_voucher_type_class.rs writes its class COMPUTE.
# Example: python3 -I gen.py
import hashlib
import pathlib
import xml.etree.ElementTree as ET
here = pathlib.Path(__file__).resolve().parent
TEMPLATES = {
    "I-CN.xml": "705a9c57db562825f415b6d170e370d954617573ab8678c0a5a8dbc9dd0dbea7",
    "I-DN.xml": "9f753e4ca962dd14499b1852a50c34645d65c4ef3d1ac15fd76954dd5de10a3a",
    "R-CN1.xml": "448064a42815248ce7db5a43b3bba8cdeaa4fe81f97d4b5661c9a159c65f0223",
    "R-DN1.xml": "aacdf01124f0530c8eeecbf322df6d34643fd61c31a508585c0587bd15ed3121",
    "V-1.xml": "ea8a66b7e23065d244bd0e9a5192daafcbb3e5bf6b4d066cc61e35e168826133",
    "template-n-0001.utf16le.xml": "90d188e7089852c411516ef36617dfc3a157df76cfc1787b1985804833ff31e6",
}
BOOK = "6b43e498-430c-4d5c-bfef-d32e2ab93c85"
CREDIT, DEBIT, CREDIT2 = "BRIDGE Credit Note Manual", "BRIDGE Debit Note Manual", "BRIDGE Credit Note Manual 2"
CN, DN = "CN/26-27/0001", "DN/26-27/0001"
def load(name):
    raw = (here / name).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == TEMPLATES[name], f"{name} is not the file ROW.md names"
    assert raw[:2] == b"\xff\xfe", name
    return raw[2:].decode("utf-16-le")
def encode(text):
    return b"\xff\xfe" + text.encode("utf-16-le")
def save(name, text):
    raw = encode(text)
    (here / name).write_bytes(raw)
    print(f"{name}: {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}")
def swap(text, old, new, count):
    assert text.count(old) == count, (old, text.count(old), count)
    return text.replace(old, new)
def tail(text, old, new):
    return swap(text, f"00000000-0000-4000-8000-000000000{old}", f"00000000-0000-4000-8000-000000000{new}", 1)
def retype(text, old, new):
    text = swap(text, f'VCHTYPE="{old}"', f'VCHTYPE="{new}"', 1)
    return swap(text, f"<VOUCHERTYPENAME>{old}</VOUCHERTYPENAME>", f"<VOUCHERTYPENAME>{new}</VOUCHERTYPENAME>", 1)
def renumber(text, old, new):
    return swap(text, f"<VOUCHERNUMBER>{old}</VOUCHERNUMBER>", f"<VOUCHERNUMBER>{new}</VOUCHERNUMBER>", 1)
def check_import(text, kind, name, number):
    # Read by a parser, not a text search: the type, the one number right after the type's name, the six legs.
    voucher = ET.fromstring(text.split("?>", 1)[1]).find("BODY/IMPORTDATA/REQUESTDATA/TALLYMESSAGE/VOUCHER")
    tags = [c.tag for c in voucher]
    assert voucher.get("VCHTYPE") == name and voucher.findtext("VOUCHERTYPENAME") == name, name
    assert tags.count("VOUCHERNUMBER") == 1 and voucher.findtext("VOUCHERNUMBER") == number, number
    assert tags.index("VOUCHERNUMBER") == tags.index("VOUCHERTYPENAME") + 1
    assert tags.count("LEDGERENTRIES.LIST") == 6
    party = voucher.find("LEDGERENTRIES.LIST")
    assert party.findtext("LEDGERNAME") == "BRIDGE Walk-in"
    assert party.findtext("ISDEEMEDPOSITIVE") == ("No" if kind == "credit" else "Yes")
    for word in ("BILLALLOCATIONS", "GSTREGISTRATION>", "CMPGSTIN", "GSTNATUREOFRETURN"):
        assert word not in text, word
def note(template, old_type, old_tail, name, number, new_tail):
    text = load(template)
    assert "<VOUCHERNUMBER>" not in text
    text = retype(text, old_type, name)
    text = swap(text, f"<VOUCHERTYPENAME>{name}</VOUCHERTYPENAME>",
                f"<VOUCHERTYPENAME>{name}</VOUCHERTYPENAME><VOUCHERNUMBER>{number}</VOUCHERNUMBER>", 1)
    return tail(text, old_tail, new_tail)

mc1 = note("I-CN.xml", "Credit Note", "d01", CREDIT, CN, "e11")
check_import(mc1, "credit", CREDIT, CN)
save("I-MC1.xml", mc1)
save("I-MC1-again.xml", mc1)
assert (here / "I-MC1.xml").read_bytes() == (here / "I-MC1-again.xml").read_bytes()
mc1n = tail(mc1, "e11", "e12")
check_import(mc1n, "credit", CREDIT, CN)
save("I-MC1n.xml", mc1n)
md1 = note("I-DN.xml", "Debit Note", "d02", DEBIT, DN, "e13")
check_import(md1, "debit", DEBIT, DN)
save("I-MD1.xml", md1)
mc2 = tail(retype(mc1, CREDIT, CREDIT2), "e11", "e14")
check_import(mc2, "credit", CREDIT2, CN)
save("I-MC2.xml", mc2)
mdx = tail(renumber(md1, DN, CN), "e13", "e15")
check_import(mdx, "debit", DEBIT, CN)
save("I-MDX.xml", mdx)

def readback(template, old_suffix, number, suffix):
    text = swap(load(template), '$VoucherNumber = "1"', f'$VoucherNumber = "{number}"', 1)
    return swap(text, f'"{BOOK}-{old_suffix}"', f'"{BOOK}-{suffix}"', 1)
save("R-MC1.xml", readback("R-CN1.xml", "00000028", CN, "000000e8"))
save("R-MD1.xml", readback("R-DN1.xml", "0000002a", DN, "000000e9"))
save("R-MC2.xml", readback("R-CN1.xml", "00000028", CN, "000000ea"))
save("R-MDX.xml", readback("R-DN1.xml", "0000002a", CN, "000000e9"))

full = load("V-1.xml")
save("V-CN.xml", swap(full, '$VoucherNumber = "1"', f'$VoucherNumber = "{CN}"', 1))
save("V-DN.xml", swap(full, '$VoucherNumber = "1"', f'$VoucherNumber = "{DN}"', 1))

SALES = "<COMPUTE>BRIDGEVCHISSALES:$$IsSales:$VoucherTypeName</COMPUTE>"
CLASS = {"s": SALES,
         "c": "<COMPUTE>BRIDGEVCHISCREDITNOTE:$$IsCreditNote:$VoucherTypeName</COMPUTE>",
         "d": "<COMPUTE>BRIDGEVCHISDEBITNOTE:$$IsDebitNote:$VoucherTypeName</COMPUTE>"}
number_read = load("template-n-0001.utf16le.xml")
for short, number, classes in (("0010", "BP/26-27/0010", "scd"), ("CN", CN, "cd"), ("DN", DN, "cd")):
    for c in classes:
        text = swap(number_read, '"BP/26-27/0001"', f'"{number}"', 1)
        save(f"N-{short}{c}.xml", swap(text, SALES, CLASS[c], 1))
