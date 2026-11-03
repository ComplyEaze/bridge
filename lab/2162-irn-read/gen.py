#!/usr/bin/env python3
# gen.py: writes beside this file, UTF-16LE with a BOM, as sent:
#   N-<number>.xml   the number read as the build renders it (render_invoice_number_request), from
#                    template-n-0001.utf16le.xml (lab/1342-capture-requests r10-number-control-k0.xml, byte-equal),
#                    with the company, the window and the number changed to this book's.
#   NI-<number>.xml  N-<number>.xml with IRN, IRNACKNO and IRNACKDATE added to its FETCH. Nothing else changes.
#   F-note.xml       template-full-1.utf16le.xml (lab/1378-irn-capture-answers reqs/full1.xml, byte-equal) with
#                    its filter changed from the first voucher to the voucher of type Credit Note.
# M-marks.xml, L-list.xml and V-named.xml are not generated: they are 1378's reqs/marks.xml, vlist.xml and
# named.xml, byte-equal.
# Example: python3 -I gen.py
import pathlib
here = pathlib.Path(__file__).resolve().parent
def load(name):
    raw = (here / name).read_bytes()
    assert raw[:2] == b"\xff\xfe"
    return raw[2:].decode("utf-16-le")
def save(name, text):
    (here / name).write_bytes(b"\xff\xfe" + text.encode("utf-16-le"))
def swap(text, old, new, count):
    assert text.count(old) == count, (old, text.count(old))
    return text.replace(old, new)
n = load("template-n-0001.utf16le.xml")
n = swap(n, ">BRIDGE PILOT LAB<", ">BRIDGE EINVOICE LAB<", 1)
n = swap(n, '"20260401"', '"20250401"', 1)
n = swap(n, ">20260401<", ">20250401<", 1)
n = swap(n, '"20270331"', '"20260331"', 1)
n = swap(n, ">20270331<", ">20260331<", 1)
plain_fetch = "<FETCH>DATE, VOUCHERNUMBER, VOUCHERTYPENAME, ISCANCELLED</FETCH>"
irn_fetch = "<FETCH>DATE, VOUCHERNUMBER, VOUCHERTYPENAME, ISCANCELLED, IRN, IRNACKNO, IRNACKDATE</FETCH>"
for number in ("1", "2"):
    plain = swap(n, '$VoucherNumber = "BP/26-27/0001"', f'$VoucherNumber = "{number}"', 1)
    save(f"N-{number}.xml", plain)
    save(f"NI-{number}.xml", swap(plain, plain_fetch, irn_fetch, 1))
full = load("template-full-1.utf16le.xml")
full = swap(full, ">$MasterId = 1<", '>$VoucherTypeName = "Credit Note"<', 1)
save("F-note.xml", full)
