#!/usr/bin/env python3
# gen.py: writes the requests of row 2360-L beside this file, UTF-16LE with a BOM, as sent. It starts from two files
# beside it that are byte-equal to row 2162-FY14R's (lab/2162-fy14r-sheet), and changes only what each line below
# names. Every change is asserted to occur exactly as often as expected.
#   N-day-20260801.xml  the build's number read (template-n-0001.utf16le.xml) over the one day 20260801, with the
#                   number clause taken out of its filter: every voucher of that day, by the same FETCH and COMPUTE.
#   I-A1.xml ... I-A3.xml, I-A9.xml  run A, one file per sitting and one spare: the build's request that created
#                   BP/26-27/0010 (template-import-0010.utf16le.xml), dated 20260801 as it is, with its party
#                   BRIDGE Walk-in renamed BRIDGE Absent 2360 in all seven places (a ledger the book does not hold,
#                   on the debit line and in the party fields), a fresh number BP/26-27/0081 ... 0083 or 0089
#                   (VOUCHERNUMBER and REFERENCE) and a fresh REMOTEID.
#   I-B1.xml ... I-B3.xml, I-B9.xml  run B: the same request with only its REMOTEID changed, so it reuses the number
#                   BP/26-27/0010 that the book holds, on the date it holds it.
# Within a sitting, A and B differ in the cause only; across sittings, each file differs from the last in its number
# (A) and its REMOTEID (A and B) only, since a write file is sent once in the row's life (LAB-COMMON C3). No REMOTEID
# here is the held voucher's (…0777): a REMOTEID equal to it would alter that voucher, not be refused (§9.14).
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

number = load("template-n-0001.utf16le.xml")
number = swap(number, ' AND $VoucherNumber = "BP/26-27/0001"', "", 1)
number = swap(number, "20260401", "20260801", 2)
save("N-day-20260801.xml", swap(number, "20270331", "20260801", 2))

HELD_ID = "00000000-0000-4000-8000-000000000777"
invoice = load("template-import-0010.utf16le.xml")
assert invoice.count("20260801") == 3 and invoice.count(">BP/26-27/0010<") == 2 and invoice.count(HELD_ID) == 1
absent = swap(invoice, ">BRIDGE Walk-in<", ">BRIDGE Absent 2360<", 7)
for sitting, tail in (("1", "1"), ("2", "2"), ("3", "3"), ("9", "9")):
    a = swap(absent, ">BP/26-27/0010<", f">BP/26-27/008{tail}<", 2)
    save(f"I-A{sitting}.xml", swap(a, HELD_ID, f"00000000-0000-4000-8000-00000000236{tail}", 1))
    b = swap(invoice, HELD_ID, f"00000000-0000-4000-8000-00000000237{tail}", 1)
    save(f"I-B{sitting}.xml", b)
