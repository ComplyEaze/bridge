#!/usr/bin/env python3
# gen.py: writes the requests of row 2162-IRN9 beside this file, UTF-16LE with a BOM, as sent. It starts from two files
# beside it, byte-equal to their sources, and changes only what each line below names. Every change is asserted to
# occur exactly as often as expected.
#   I-4.xml   template-import-0010.utf16le.xml (lab/2162-bc-sheet, the build's request that created BP/26-27/0010 on
#             BRIDGE PILOT LAB) made into an invoice for BRIDGE EINVOICE LAB: the company, the voucher type Sales, the
#             date 20250403, the number 4, the REMOTEID tail 0804, the party BRIDGE IRN Customer, the state
#             Karnataka, the registration type Regular, a PARTYGSTIN element where the build writes one, and the
#             legs: the party 118.00, BRIDGE IRN Sales 100.00, BRIDGE IRN IGST 18.00. PARTYGSTIN holds 15 X here:
#             fill.py puts the customer's GSTIN in its place on the lab PC, from that sitting's own ledger read.
#   I-3.xml   I-4 with the number 3, the REMOTEID tail 0803, and IRN, IRNACKNO and IRNACKDATE added after ISINVOICE.
#             Nothing else differs between the two.
#   F-3.xml, F-4.xml  template-full-1.utf16le.xml (lab/2162-irn-read-sheet: every native method, no fetch list) with its
#             filter changed from the first voucher to the number 3 or 4.
#   K-customer.xml    the same read with TYPE Ledger, filtered to the customer by name (as row 2162-BC's K reads).
# The IRN, the acknowledgement number and its date are invented. They are not invoice A's.
# Example: python3 -I gen.py
import pathlib, re
here = pathlib.Path(__file__).resolve().parent
def load(name):
    raw = (here / name).read_bytes()
    assert raw[:2] == b"\xff\xfe", name
    return raw[2:].decode("utf-16-le")
def save(name, text):
    (here / name).write_bytes(b"\xff\xfe" + text.encode("utf-16-le"))
    print(f"{name}: {len(text.encode('utf-16-le')) + 2} bytes")
def swap(text, old, new, count):
    assert text.count(old) == count, (old, text.count(old), count)
    return text.replace(old, new)

PARTY = "BRIDGE IRN Customer"
PLACEHOLDER = "X" * 15
IRN = "c3d4e5f6a7b8c9d0" * 4
IRN_ELEMENTS = f"<IRN>{IRN}</IRN><IRNACKNO>445566</IRNACKNO><IRNACKDATE>20250403</IRNACKDATE>"
LEGS = ("<LEDGERENTRIES.LIST><LEDGERNAME>BRIDGE IRN Customer</LEDGERNAME><ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE>"
        "<AMOUNT>-118.00</AMOUNT></LEDGERENTRIES.LIST>"
        "<LEDGERENTRIES.LIST><LEDGERNAME>BRIDGE IRN Sales</LEDGERNAME><ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE>"
        "<AMOUNT>100.00</AMOUNT></LEDGERENTRIES.LIST>"
        "<LEDGERENTRIES.LIST><LEDGERNAME>BRIDGE IRN IGST</LEDGERNAME><ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE>"
        "<AMOUNT>18.00</AMOUNT></LEDGERENTRIES.LIST>")

invoice = load("template-import-0010.utf16le.xml")
invoice = swap(invoice, "<SVCURRENTCOMPANY>BRIDGE PILOT LAB</SVCURRENTCOMPANY>",
               "<SVCURRENTCOMPANY>BRIDGE EINVOICE LAB</SVCURRENTCOMPANY>", 1)
invoice = swap(invoice, 'VCHTYPE="BRIDGE Sales"', 'VCHTYPE="Sales"', 1)
invoice = swap(invoice, "<VOUCHERTYPENAME>BRIDGE Sales</VOUCHERTYPENAME>", "<VOUCHERTYPENAME>Sales</VOUCHERTYPENAME>", 1)
invoice = swap(invoice, ">20260801<", ">20250403<", 3)
invoice = swap(invoice, ">BP/26-27/0010<", ">4<", 2)
invoice = swap(invoice, "00000000-0000-4000-8000-000000000777", "00000000-0000-4000-8000-000000000804", 1)
legs = re.findall(r"<LEDGERENTRIES\.LIST>.*?</LEDGERENTRIES\.LIST>", invoice)
assert len(legs) == 6, len(legs)
invoice = swap(invoice, "".join(legs), LEGS, 1)
invoice = swap(invoice, ">BRIDGE Walk-in<", f">{PARTY}<", 6)
invoice = swap(invoice, f"<CONSIGNEEMAILINGNAME>{PARTY}</CONSIGNEEMAILINGNAME>",
               f"<CONSIGNEEMAILINGNAME>{PARTY}</CONSIGNEEMAILINGNAME><PARTYGSTIN>{PLACEHOLDER}</PARTYGSTIN>", 1)
invoice = swap(invoice, ">Rajasthan<", ">Karnataka<", 3)
invoice = swap(invoice, "<GSTREGISTRATIONTYPE>Unregistered/Consumer</GSTREGISTRATIONTYPE>",
               "<GSTREGISTRATIONTYPE>Regular</GSTREGISTRATIONTYPE>", 1)
assert "BRIDGE PILOT LAB" not in invoice and "Walk-in" not in invoice and "BP/26-27" not in invoice
assert invoice.count(PLACEHOLDER) == 1 and "<IRN" not in invoice
save("I-4.xml", invoice)
with_irn = swap(invoice, ">4<", ">3<", 2)
with_irn = swap(with_irn, "000000000804", "000000000803", 1)
with_irn = swap(with_irn, "<ISINVOICE>Yes</ISINVOICE>", f"<ISINVOICE>Yes</ISINVOICE>{IRN_ELEMENTS}", 1)
save("I-3.xml", with_irn)

full = load("template-full-1.utf16le.xml")
assert "<SVCURRENTCOMPANY>BRIDGE EINVOICE LAB</SVCURRENTCOMPANY>" in full
for n in ("3", "4"):
    save(f"F-{n}.xml", swap(full, ">$MasterId = 1<", f'>$VoucherNumber = "{n}"<', 1))
ledger = swap(full, "<TYPE>Voucher</TYPE>", "<TYPE>Ledger</TYPE>", 1)
save("K-customer.xml", swap(ledger, ">$MasterId = 1<", f'>$Name = "{PARTY}"<', 1))
