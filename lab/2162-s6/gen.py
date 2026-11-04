#!/usr/bin/env python3
# gen.py: writes the seven reads of row 2162-S6 beside this file from template-y-pilot.utf16le.xml, which is
# lab/2162-s5-sheet:lab/2162-s5/Y-all.xml byte-equal (S5's Y: today's number read less its number term, for
# BRIDGE PILOT LAB over 1 Apr 2026 to 31 Mar 2027). Each change is asserted to occur exactly as often as expected.
#   Y0-<w>.xml  S5's Y for BRIDGE CORPUS DENSE over window w: the company and the four dates changed, nothing else
#   Y1-<w>.xml  Y0-<w> with " AND $$IsSales:$VoucherTypeName" added at the end of its filter formula (the one variable)
#   N1-year.xml Y0-year with ' AND $VoucherNumber = "1"' added at the end of its filter formula: the number read as
#               the build renders it today for number 1 over the year
# Windows: day 20250401..20250401, month 20250401..20250430, year 20250401..20260331.
# Output is UTF-16LE with a byte-order mark, as the template. Run: python3 -I gen.py
import pathlib
here = pathlib.Path(__file__).resolve().parent
raw = (here / "template-y-pilot.utf16le.xml").read_bytes()
assert raw[:2] == b"\xff\xfe", "the template starts with a UTF-16LE byte-order mark"
base = raw[2:].decode("utf-16-le")
def swap(text, old, new, times):
    assert text.count(old) == times, (old, text.count(old), times)
    return text.replace(old, new)
base = swap(base, "<SVCURRENTCOMPANY>BRIDGE PILOT LAB</SVCURRENTCOMPANY>",
            "<SVCURRENTCOMPANY>BRIDGE CORPUS DENSE</SVCURRENTCOMPANY>", 1)
def window(text, start, end):
    text = swap(text, "20260401", start, 2)
    return swap(text, "20270331", end, 2)
def write(name, text):
    (here / name).write_bytes(b"\xff\xfe" + text.encode("utf-16-le"))
    print(f"{name}: {len(text.encode('utf-16-le')) + 2} bytes")
WINDOWS = {"day": ("20250401", "20250401"), "month": ("20250401", "20250430"), "year": ("20250401", "20260331")}
for w, (start, end) in WINDOWS.items():
    y0 = window(base, start, end)
    write(f"Y0-{w}.xml", y0)
    write(f"Y1-{w}.xml", swap(y0, "</SYSTEM>", " AND $$IsSales:$VoucherTypeName</SYSTEM>", 1))
    if w == "year":
        write("N1-year.xml", swap(y0, "</SYSTEM>", ' AND $VoucherNumber = "1"</SYSTEM>', 1))
