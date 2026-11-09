#!/usr/bin/env python3
# gen.py: writes the requests of row 2162-IRN10 beside this file, UTF-16LE with a BOM, as sent. It starts from two files
# beside it, byte-equal to row 2162-IRN9's (lab/2162-irn-import-sheet), and changes only what each line below names.
# Every change is asserted to occur exactly as often as expected.
#   I-5.xml   row 2162-IRN9's I-4.xml (the invoice without e-invoice details, which Tally created on that row) with
#             the number 5 (twice) and the REMOTEID tail 0805. Nothing else changes: its PARTYGSTIN still holds 15 X,
#             which fill.py fills on the lab PC.
#   F-5.xml   template-full-1.utf16le.xml (every native method, no fetch list) filtered to the number 5.
#   K-igst.xml, K-sales.xml  the same read with TYPE Ledger, filtered to BRIDGE IRN IGST or BRIDGE IRN Sales by name
#             (as row 2162-IRN9's K-customer.xml).
# Example: python3 -I gen.py
import pathlib
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

invoice = load("I-4.xml")
assert "<IRN" not in invoice and invoice.count("X" * 15) == 1
invoice = swap(invoice, ">4<", ">5<", 2)
invoice = swap(invoice, "00000000-0000-4000-8000-000000000804", "00000000-0000-4000-8000-000000000805", 1)
save("I-5.xml", invoice)

full = load("template-full-1.utf16le.xml")
assert "<SVCURRENTCOMPANY>BRIDGE EINVOICE LAB</SVCURRENTCOMPANY>" in full
save("F-5.xml", swap(full, ">$MasterId = 1<", '>$VoucherNumber = "5"<', 1))
ledger = swap(full, "<TYPE>Voucher</TYPE>", "<TYPE>Ledger</TYPE>", 1)
for name, master in [("K-igst.xml", "BRIDGE IRN IGST"), ("K-sales.xml", "BRIDGE IRN Sales")]:
    save(name, swap(ledger, ">$MasterId = 1<", f'>$Name = "{master}"<', 1))
