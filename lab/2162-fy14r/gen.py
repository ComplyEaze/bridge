#!/usr/bin/env python3
# gen.py: writes the requests of row 2162-FY14 beside this file, UTF-16LE with a BOM, as sent. It starts from files
# beside it that are byte-equal to row 2162-KD's (lab/2162-keyed-dup-sheet), and changes only what each line below
# names. Every change is asserted to occur exactly as often as expected.
#   I-0071-y27.xml  template-import-0010.utf16le.xml (the build's request that created BP/26-27/0010) with the date
#                   20270405 (three times), the number BP/26-27/0071 (VOUCHERNUMBER and REFERENCE) and the REMOTEID
#                   tail 0794. Party, type and figures unchanged. The control: a number never used, in 2027-28.
#   I-0010-y27.xml  I-0071-y27.xml with the number BP/26-27/0010 (twice) and the REMOTEID tail 0795, nothing else:
#                   the number the book already holds in 2026-27, dated in 2027-28. The two files differ only there.
#   N-0010-y26.xml  the number read (template-n-0001.utf16le.xml) with the number BP/26-27/0010, over 2026-27.
#   N-0010-y27.xml  N-0010-y26.xml over 2027-28 (its four dates), the year the build would read for a 2027-28 invoice.
#   N-0071-y27.xml  N-0010-y27.xml with the number BP/26-27/0071.
#   V-0010.xml, V-0071.xml  every native field of the vouchers of that number from 20260401 to 20280331:
#                   template-full-1.utf16le.xml with the company, its two dates and its filter changed.
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
invoice = swap(invoice, "20260801", "20270405", 3)
control = swap(invoice, ">BP/26-27/0010<", ">BP/26-27/0071<", 2)
control = swap(control, "00000000-0000-4000-8000-000000000777", "00000000-0000-4000-8000-000000000794", 1)
save("I-0071-y27.xml", control)
held = swap(control, ">BP/26-27/0071<", ">BP/26-27/0010<", 2)
save("I-0010-y27.xml", swap(held, "00000000-0000-4000-8000-000000000794", "00000000-0000-4000-8000-000000000795", 1))

number = swap(load("template-n-0001.utf16le.xml"), '"BP/26-27/0001"', '"BP/26-27/0010"', 1)
save("N-0010-y26.xml", number)
next_year = swap(swap(number, "20260401", "20270401", 2), "20270331", "20280331", 2)
save("N-0010-y27.xml", next_year)
save("N-0071-y27.xml", swap(next_year, '"BP/26-27/0010"', '"BP/26-27/0071"', 1))

full = load("template-full-1.utf16le.xml")
full = swap(full, "<SVCURRENTCOMPANY>BRIDGE EINVOICE LAB</SVCURRENTCOMPANY>",
            "<SVCURRENTCOMPANY>BRIDGE PILOT LAB</SVCURRENTCOMPANY>", 1)
full = swap(full, ">20250401<", ">20260401<", 1)
full = swap(full, ">20260331<", ">20280331<", 1)
for tail in ("0010", "0071"):
    save(f"V-{tail}.xml", swap(full, "$MasterId = 1", f'$VoucherNumber = "BP/26-27/{tail}"', 1))
