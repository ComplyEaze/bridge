#!/usr/bin/env python3
# gen.py: writes the requests of row 2346-CD beside this file, UTF-16LE with a BOM, as sent. It starts from files
# beside it that are byte-equal to earlier rows' (ROW.md says which), and changes only what each line below names.
# Every change is asserted to occur exactly as often as expected, and each template's sha256 is checked first.
#   I-CN.xml     template-import-0010.utf16le.xml (the build's request that created BP/26-27/0010) as a sales-side
#                Credit Note against that invoice: the type Credit Note (VCHTYPE and VOUCHERTYPENAME), no
#                VOUCHERNUMBER (the Credit Note type numbers automatically), the REMOTEID tail 0d01, and every leg's
#                side turned over (ISDEEMEDPOSITIVE and the AMOUNT's sign). REFERENCE and REFERENCEDATE stay the
#                invoice's, BP/26-27/0010 and 20260801, where a note's original invoice is kept (§8.2k).
#   I-DN.xml     the same template as a sales-side Debit Note: the type Debit Note, no VOUCHERNUMBER, the REMOTEID
#                tail 0d02, and every leg as the invoice has it (the party Dr, the rest Cr).
#   V-1.xml      template-full-0057.utf16le.xml (row 2162-RS's V-0057.xml: every native field of the vouchers
#                holding one number) with the number 1, the first number of an automatic series.
#   R-CN1.xml    the build's read-back (R-0010.xml) for the number 1 on the type Credit Note (its GUID in place of
#   R-DN1.xml    BRIDGE Sales's), and the same on the type Debit Note.
# Example: python3 -I gen.py
import hashlib
import pathlib
here = pathlib.Path(__file__).resolve().parent
TEMPLATES = {
    "template-import-0010.utf16le.xml": "b333f0dc819146ae24b2f434a28817e1d142eba0e0386f65bbc5484171b4b21e",
    "R-0010.xml": "10f9711cc936c9c4d0f70bccb42b54cc10fbd5e7de11287737329cfe7ae56c21",
    "template-full-0057.utf16le.xml": "cd8e91b441040b742f575955f10d57e187f93e618139e2a6325064c935b09020",  # row 2162-RS's V-0057.xml
}
BOOK = "6b43e498-430c-4d5c-bfef-d32e2ab93c85"
SALES, CREDIT, DEBIT = BOOK + "-000000ce", BOOK + "-00000028", BOOK + "-0000002a"  # row 2162-RU's a-s2-T-types.xml
LEGS = [("BRIDGE Walk-in", "Yes", "-1051.00"), ("BRIDGE Svc 998313 5%", "No", "600.40"),
        ("BRIDGE Svc 998314 5%", "No", "400.40"), ("BRIDGE CGST 2.5%", "No", "25.02"),
        ("BRIDGE SGST 2.5%", "No", "25.02"), ("BRIDGE Round Off", "No", "0.16")]
def load(name):
    raw = (here / name).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == TEMPLATES[name], f"{name} is not the template ROW.md names"
    assert raw[:2] == b"\xff\xfe", name
    return raw[2:].decode("utf-16-le")
def save(name, text):
    raw = b"\xff\xfe" + text.encode("utf-16-le")
    (here / name).write_bytes(raw)
    print(f"{name}: {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}")
def swap(text, old, new, count):
    assert text.count(old) == count, (old, text.count(old), count)
    return text.replace(old, new)
def leg(name, side, amount):
    return (f"<LEDGERENTRIES.LIST><LEDGERNAME>{name}</LEDGERNAME><ISDEEMEDPOSITIVE>{side}</ISDEEMEDPOSITIVE>"
            f"<AMOUNT>{amount}</AMOUNT></LEDGERENTRIES.LIST>")
def note(text, kind, tail):
    text = swap(text, 'VCHTYPE="BRIDGE Sales"', f'VCHTYPE="{kind}"', 1)
    text = swap(text, "<VOUCHERTYPENAME>BRIDGE Sales</VOUCHERTYPENAME>", f"<VOUCHERTYPENAME>{kind}</VOUCHERTYPENAME>", 1)
    text = swap(text, "<VOUCHERNUMBER>BP/26-27/0010</VOUCHERNUMBER>", "", 1)
    text = swap(text, "00000000-0000-4000-8000-000000000777", f"00000000-0000-4000-8000-00000000{tail}", 1)
    assert text.count("<LEDGERENTRIES.LIST>") == len(LEGS)
    return text

invoice = load("template-import-0010.utf16le.xml")
credit = note(invoice, "Credit Note", "0d01")
for name, side, amount in LEGS:
    turned = ("No" if side == "Yes" else "Yes", amount[1:] if amount.startswith("-") else "-" + amount)
    credit = swap(credit, leg(name, side, amount), leg(name, *turned), 1)
for word in ("GSTREGISTRATION>", "CMPGSTIN", "GSTNATUREOFRETURN", "BILLALLOCATIONS", "VOUCHERNUMBER"):
    assert word not in credit, f"the Credit Note names {word}"
save("I-CN.xml", credit)

debit = note(invoice, "Debit Note", "0d02")
for name, side, amount in LEGS:
    assert debit.count(leg(name, side, amount)) == 1, name
for word in ("GSTREGISTRATION>", "CMPGSTIN", "GSTNATUREOFRETURN", "BILLALLOCATIONS", "VOUCHERNUMBER"):
    assert word not in debit, f"the Debit Note names {word}"
save("I-DN.xml", debit)

full = load("template-full-0057.utf16le.xml")
save("V-1.xml", swap(full, '"BP/26-27/0057"', '"1"', 1))

readback = load("R-0010.xml")
readback = swap(readback, '"BP/26-27/0010"', '"1"', 1)
save("R-CN1.xml", swap(readback, f'"{SALES}"', f'"{CREDIT}"', 1))
save("R-DN1.xml", swap(readback, f'"{SALES}"', f'"{DEBIT}"', 1))
