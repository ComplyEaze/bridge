#!/usr/bin/env python3
# gen8.py: writes the requests of row 2162-RS beside this file, UTF-16LE with a BOM, as sent. It starts from row
# 2162-BC's files (lab/2162-bc-sheet at e2b1ca82, folder lab/2162-bc/), which are beside it byte-equal, and changes
# only what each line below names. Every change is asserted to occur exactly as often as expected.
#   I-0057.xml   the unregistered customer BRIDGE Walk-in: template-import-0010 (the build's request that created
#                BP/26-27/0010) with the number BP/26-27/0057 and the REMOTEID tail 0795, as BC's gen5.py made 0055.
#   R-0057.xml   the build's read-back (R-0010, BC's) for that number.
#   N-0057.xml   the number read with the e-invoice words (N-0053, BC's) for that number.
#   V-0057.xml   every native field of the vouchers numbered BP/26-27/0057 in 2026-27: row 2162-IRN's
#                template-full-1 with the company, the two dates and its filter changed, as BC's gen5.py made V-0055.
# Example: python3 -I gen8.py
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

invoice = load("template-import-0010.utf16le.xml")
invoice = swap(invoice, "BP/26-27/0010", "BP/26-27/0057", 2)
save("I-0057.xml", swap(invoice, "000000000777", "000000000795", 1))
save("R-0057.xml", swap(load("R-0010.xml"), '"BP/26-27/0010"', '"BP/26-27/0057"', 1))
save("N-0057.xml", swap(load("N-0053.xml"), '"BP/26-27/0053"', '"BP/26-27/0057"', 1))
full = load("template-full-1.utf16le.xml")
full = swap(full, "<SVCURRENTCOMPANY>BRIDGE EINVOICE LAB</SVCURRENTCOMPANY>",
            "<SVCURRENTCOMPANY>BRIDGE PILOT LAB</SVCURRENTCOMPANY>", 1)
full = swap(full, ">20250401<", ">20260401<", 1)
full = swap(full, ">20260331<", ">20270331<", 1)
save("V-0057.xml", swap(full, "$MasterId = 1", '$VoucherNumber = "BP/26-27/0057"', 1))
