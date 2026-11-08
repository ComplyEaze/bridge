#!/usr/bin/env python3
# gen7.py: writes the reads of row 2162-S7 beside this file from template-y-pilot.utf16le.xml, which is
# lab/2162-s6-sheet:lab/2162-s6/template-y-pilot.utf16le.xml byte-equal, itself lab/2162-s5-sheet:lab/2162-s5/Y-all.xml
# byte-equal (S5's Y: today's number read less its number term, for BRIDGE PILOT LAB over 1 Apr 2026 to 31 Mar 2027).
# Each change is asserted to occur exactly as often as expected. S6's gen.py made the same changes.
#   A-Y0-year.xml       the template, unchanged
#   A-Y1-year.xml       A-Y0-year with " AND $$IsSales:$VoucherTypeName" at the end of its filter formula
#   B-Y0-month.xml      Y for BRIDGE CORPUS DENSE over April 2025: S6's Y0-month.xml
#   B-Y1-month.xml      B-Y0-month with the same one term: S6's Y1-month.xml
#   C-Y0-<yyyy-mm>.xml  Y for BRIDGE CORPUS DENSE over each calendar month of 1 Apr 2025 to 31 Mar 2026
#   C-N1-2025-04.xml    C-Y0-2025-04 with ' AND $VoucherNumber = "1"' at the end of its filter formula: the number read
#                       as the build renders it today for number 1, over April 2025 (S6's N1-year with the month's dates)
# Output is UTF-16LE with a byte-order mark, as the template. Run: python3 -I gen7.py
import calendar, pathlib
here = pathlib.Path(__file__).resolve().parent
raw = (here / "template-y-pilot.utf16le.xml").read_bytes()
assert raw[:2] == b"\xff\xfe", "the template starts with a UTF-16LE byte-order mark"
pilot = raw[2:].decode("utf-16-le")
def swap(text, old, new, times):
    assert text.count(old) == times, (old, text.count(old), times)
    return text.replace(old, new)
def window(text, start, end):
    text = swap(text, "20260401", start, 2)
    return swap(text, "20270331", end, 2)
def write(name, text):
    (here / name).write_bytes(b"\xff\xfe" + text.encode("utf-16-le"))
    print(f"{name}: {len(text.encode('utf-16-le')) + 2} bytes")
SALES = " AND $$IsSales:$VoucherTypeName</SYSTEM>"
NUMBER_1 = ' AND $VoucherNumber = "1"</SYSTEM>'
write("A-Y0-year.xml", pilot)
write("A-Y1-year.xml", swap(pilot, "</SYSTEM>", SALES, 1))
dense = swap(pilot, "<SVCURRENTCOMPANY>BRIDGE PILOT LAB</SVCURRENTCOMPANY>",
             "<SVCURRENTCOMPANY>BRIDGE CORPUS DENSE</SVCURRENTCOMPANY>", 1)
april = window(dense, "20250401", "20250430")
write("B-Y0-month.xml", april)
write("B-Y1-month.xml", swap(april, "</SYSTEM>", SALES, 1))
for year, month in [(2025, m) for m in range(4, 13)] + [(2026, m) for m in range(1, 4)]:
    last = calendar.monthrange(year, month)[1]
    write(f"C-Y0-{year}-{month:02d}.xml", window(dense, f"{year}{month:02d}01", f"{year}{month:02d}{last:02d}"))
write("C-N1-2025-04.xml", swap(april, "</SYSTEM>", NUMBER_1, 1))
