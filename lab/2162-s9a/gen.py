#!/usr/bin/env python3
# gen.py: writes the one generated request of row 2162-S9a beside this file, UTF-16LE with a BOM, as sent. It starts
# from template-full-1.utf16le.xml beside it, byte-equal to row 2162-KD's (lab/2162-keyed-dup-sheet), and changes only
# what the lines below name. Every change is asserted to occur exactly as often as expected.
#   K-rj.xml  every native field of one ledger by name: the template with the company BRIDGE PILOT LAB, the year
#             2026-27, TYPE Ledger, and its filter $Name = "BRIDGE Customer RJ" (as row 2162-IRN10's K files).
# C-compliance.xml and M-marks.xml are not generated: ROW.md says where each comes from.
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

full = load("template-full-1.utf16le.xml")
full = swap(full, "<SVCURRENTCOMPANY>BRIDGE EINVOICE LAB</SVCURRENTCOMPANY>",
            "<SVCURRENTCOMPANY>BRIDGE PILOT LAB</SVCURRENTCOMPANY>", 1)
full = swap(full, ">20250401<", ">20260401<", 1)
full = swap(full, ">20260331<", ">20270331<", 1)
full = swap(full, "<TYPE>Voucher</TYPE>", "<TYPE>Ledger</TYPE>", 1)
save("K-rj.xml", swap(full, ">$MasterId = 1<", '>$Name = "BRIDGE Customer RJ"<', 1))
