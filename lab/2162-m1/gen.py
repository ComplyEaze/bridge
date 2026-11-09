#!/usr/bin/env python3
# gen.py: writes the month reads of row 2162-M1 beside this file from template-y-pilot.utf16le.xml, which is
# lab/2162-s7-sheet:lab/2162-s7/template-y-pilot.utf16le.xml byte-equal (S5's Y: today's number read less its number
# term, for BRIDGE PILOT LAB over 1 Apr 2026 to 31 Mar 2027). It is also S7's A-Y0-year.xml, byte-equal.
# Each change is asserted to occur exactly as often as expected, as S7's gen7.py did.
#   A-Y0-<yyyy-mm>.xml  Y for BRIDGE PILOT LAB over each calendar month of 1 Apr 2026 to 31 Mar 2027
#   C-Y0-<yyyy-mm>.xml  Y for BRIDGE EINVOICE LAB over each calendar month of 1 Apr 2025 to 31 Mar 2026
# Output is UTF-16LE with a byte-order mark, as the template. Run: python3 -I gen.py
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
def months(first_year):
    return [(first_year, m) for m in range(4, 13)] + [(first_year + 1, m) for m in range(1, 4)]
einvoice = swap(pilot, "<SVCURRENTCOMPANY>BRIDGE PILOT LAB</SVCURRENTCOMPANY>",
                "<SVCURRENTCOMPANY>BRIDGE EINVOICE LAB</SVCURRENTCOMPANY>", 1)
for prefix, book, first_year in [("A", pilot, 2026), ("C", einvoice, 2025)]:
    for year, month in months(first_year):
        last = calendar.monthrange(year, month)[1]
        write(f"{prefix}-Y0-{year}-{month:02d}.xml",
              window(book, f"{year}{month:02d}01", f"{year}{month:02d}{last:02d}"))
