#!/usr/bin/env python3
# gen.py: writes the requests of row 2162-KD beside this file, UTF-16LE with a BOM, as sent. It starts from files
# beside it that are byte-equal to row 2162-BC's (lab/2162-bc-sheet), and changes only what each line below names.
# Every change is asserted to occur exactly as often as expected.
#   I-0061.xml   template-import-0010.utf16le.xml (the build's request that created BP/26-27/0010) with the number
#                BP/26-27/0061 (VOUCHERNUMBER and REFERENCE), the voucher type BRIDGE Sales Keyed (VCHTYPE and
#                VOUCHERTYPENAME) and the REMOTEID tail 0792. Party, figures and date unchanged.
#   I-0061b.xml  I-0061.xml with the REMOTEID tail 0793 and nothing else changed: the same number, sent again.
#   N-0061.xml   the number read (template-n-0001.utf16le.xml) with the number BP/26-27/0061.
#   V-0061.xml   every native field of the vouchers numbered BP/26-27/0061 in 2026-27: template-full-1.utf16le.xml
#                with the company, the two dates and its filter changed, as row 2162-BC's V files.
# T-types.xml and M-marks.xml are not generated: ROW.md says where each comes from.
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

invoice = load("template-import-0010.utf16le.xml")
invoice = swap(invoice, ">BP/26-27/0010<", ">BP/26-27/0061<", 2)
invoice = swap(invoice, 'VCHTYPE="BRIDGE Sales"', 'VCHTYPE="BRIDGE Sales Keyed"', 1)
invoice = swap(invoice, "<VOUCHERTYPENAME>BRIDGE Sales</VOUCHERTYPENAME>",
               "<VOUCHERTYPENAME>BRIDGE Sales Keyed</VOUCHERTYPENAME>", 1)
assert invoice.count("BRIDGE Sales") == 2
first = swap(invoice, "00000000-0000-4000-8000-000000000777", "00000000-0000-4000-8000-000000000792", 1)
save("I-0061.xml", first)
save("I-0061b.xml", swap(first, "00000000-0000-4000-8000-000000000792", "00000000-0000-4000-8000-000000000793", 1))

number = load("template-n-0001.utf16le.xml")
save("N-0061.xml", swap(number, '"BP/26-27/0001"', '"BP/26-27/0061"', 1))

full = load("template-full-1.utf16le.xml")
full = swap(full, "<SVCURRENTCOMPANY>BRIDGE EINVOICE LAB</SVCURRENTCOMPANY>",
            "<SVCURRENTCOMPANY>BRIDGE PILOT LAB</SVCURRENTCOMPANY>", 1)
full = swap(full, ">20250401<", ">20260401<", 1)
full = swap(full, ">20260331<", ">20270331<", 1)
save("V-0061.xml", swap(full, "$MasterId = 1", '$VoucherNumber = "BP/26-27/0061"', 1))
