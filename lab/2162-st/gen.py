#!/usr/bin/env python3
# gen.py: writes the requests of row 2162-ST beside this file, UTF-16LE with a BOM, as sent. It starts from files
# beside it that ROW.md names, and changes only what each line below says. Every change is asserted to occur exactly
# as often as expected, and each request turned back is its source byte for byte.
#   I-0010k.xml  template-import-0010.utf16le.xml (the build's request that created BP/26-27/0010 on BRIDGE Sales)
#                with the voucher type BRIDGE Sales Keyed (VCHTYPE and VOUCHERTYPENAME) and the REMOTEID tail 0796.
#                The number BP/26-27/0010, party, figures and date are unchanged.
#   V-0010.xml   every native field of the vouchers numbered BP/26-27/0010 in 2026-27: template-full-1.utf16le.xml
#                with the company, the two dates and its filter changed, as row 2162-KD's V-0061.xml.
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

source = load("template-import-0010.utf16le.xml")
assert source.count(">BP/26-27/0010<") == 2 and source.count("BRIDGE Sales") == 2
invoice = swap(source, 'VCHTYPE="BRIDGE Sales"', 'VCHTYPE="BRIDGE Sales Keyed"', 1)
invoice = swap(invoice, "<VOUCHERTYPENAME>BRIDGE Sales</VOUCHERTYPENAME>",
               "<VOUCHERTYPENAME>BRIDGE Sales Keyed</VOUCHERTYPENAME>", 1)
invoice = swap(invoice, "00000000-0000-4000-8000-000000000777", "00000000-0000-4000-8000-000000000796", 1)
assert invoice.count("BRIDGE Sales Keyed") == 2 and invoice.count(">BP/26-27/0010<") == 2
back = invoice.replace("BRIDGE Sales Keyed", "BRIDGE Sales").replace("000000000796", "000000000777")
assert back == source
save("I-0010k.xml", invoice)

full = load("template-full-1.utf16le.xml")
full = swap(full, "<SVCURRENTCOMPANY>BRIDGE EINVOICE LAB</SVCURRENTCOMPANY>",
            "<SVCURRENTCOMPANY>BRIDGE PILOT LAB</SVCURRENTCOMPANY>", 1)
full = swap(full, ">20250401<", ">20260401<", 1)
full = swap(full, ">20260331<", ">20270331<", 1)
save("V-0010.xml", swap(full, "$MasterId = 1", '$VoucherNumber = "BP/26-27/0010"', 1))
