#!/usr/bin/env python3
# gen.py: writes the raw requests of row 2440-CP beside this file, UTF-16LE with a BOM, as sent. It starts from three
# files beside it and changes only what each line below names. Every change is asserted to occur exactly as often as
# expected.
#   O-company.xml  template-company-object.utf16le.xml (byte-equal to the committed pilot-lab-company-object request,
#                  which reads one company by name with every field) with BRIDGE PILOT LAB changed to BRIDGE EINVOICE LAB.
#   I-seed.xml     template-seed.xml (row 2162-S9a's L-seed.xml, itself row 2162-BC's) with the company, the ledger's
#                  name (twice), the entry's date, its place of supply and its GSTIN changed to this row's customer.
#   K-customer.xml, K-cgst.xml, K-sgst.xml, K-sales.xml  template-full-1.utf16le.xml (row 2162-IRN10's: every native
#                  method, no fetch list) with TYPE Ledger, filtered to one ledger by name, as row 2162-IRN10's K reads.
#   F-0101.xml, F-0102.xml  the same template filtered to one voucher number, as row 2162-IRN10's F-5.xml.
# Example: python3 -I gen.py
import pathlib
here = pathlib.Path(__file__).resolve().parent
BOOK = "BRIDGE EINVOICE LAB"
CUSTOMER = "BRIDGE CP Customer MH"
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

company = load("template-company-object.utf16le.xml")
save("O-company.xml", swap(company, "BRIDGE PILOT LAB", BOOK, 2))

seed = load("template-seed.xml")
seed = swap(seed, "<SVCURRENTCOMPANY>BRIDGE PILOT LAB</SVCURRENTCOMPANY>", f"<SVCURRENTCOMPANY>{BOOK}</SVCURRENTCOMPANY>", 1)
seed = swap(seed, "BRIDGE Customer RJ", CUSTOMER, 2)
seed = swap(seed, "<APPLICABLEFROM>20260401</APPLICABLEFROM>", "<APPLICABLEFROM>20250401</APPLICABLEFROM>", 1)
seed = swap(seed, "<PLACEOFSUPPLY>Rajasthan</PLACEOFSUPPLY>", "<PLACEOFSUPPLY>Maharashtra</PLACEOFSUPPLY>", 1)
seed = swap(seed, "<GSTIN>08ZZZZZ0000Z3ZO</GSTIN>", "<GSTIN>27ZZZZZ0000Z3ZO</GSTIN>", 1)
assert "PILOT" not in seed and "Rajasthan" not in seed and "2026" not in seed
save("I-seed.xml", seed)

full = load("template-full-1.utf16le.xml")
assert f"<SVCURRENTCOMPANY>{BOOK}</SVCURRENTCOMPANY>" in full
ledger = swap(full, "<TYPE>Voucher</TYPE>", "<TYPE>Ledger</TYPE>", 1)
for name, master in [("K-customer.xml", CUSTOMER), ("K-cgst.xml", "BRIDGE CP CGST"), ("K-sgst.xml", "BRIDGE CP SGST"),
                     ("K-sales.xml", "BRIDGE IRN Sales")]:
    save(name, swap(ledger, ">$MasterId = 1<", f'>$Name = "{master}"<', 1))
for number in ["0101", "0102"]:
    save(f"F-{number}.xml", swap(full, ">$MasterId = 1<", f'>$VoucherNumber = "CP/25-26/{number}"<', 1))
