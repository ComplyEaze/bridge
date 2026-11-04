#!/usr/bin/env python3
# gen.py: writes beside this file, UTF-16LE with a BOM, as sent:
#   N-0051.xml, N-z051.xml  the number read as the build renders it today (render_invoice_number_request, with the
#                           e-invoice words), for BP/26-27/0051 and BP/26-27/051 over 1 Apr 2026 to 31 Mar 2027.
#   Y-all.xml               the same read with its number term removed: every voucher of that year, with its class.
# All three come from template-n-0001.utf16le.xml (lab/1342-capture-requests r10-number-control-k0.xml, byte-equal):
# its FETCH gains IRN, IRNACKNO and IRNACKDATE, as #2325 added them, and its number is changed or its number term
# removed. Its company and window are already this book's. Nothing else changes.
# M-marks.xml, run.py, I-0051.xml and I-z051.xml are not generated: they are lab/2162-s4's, byte-equal.
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
n = swap(n, "<FETCH>DATE, VOUCHERNUMBER, VOUCHERTYPENAME, ISCANCELLED</FETCH>",
         "<FETCH>DATE, VOUCHERNUMBER, VOUCHERTYPENAME, ISCANCELLED, IRN, IRNACKNO, IRNACKDATE</FETCH>", 1)
term = '$VoucherNumber = "BP/26-27/0001"'
save("N-0051.xml", swap(n, term, '$VoucherNumber = "BP/26-27/0051"', 1))
save("N-z051.xml", swap(n, term, '$VoucherNumber = "BP/26-27/051"', 1))
save("Y-all.xml", swap(n, " AND " + term, "", 1))
