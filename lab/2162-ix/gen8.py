#!/usr/bin/env python3
# gen8.py: writes the requests of the #2162 index rows (L1 to L5) beside this file.
#   python3 -I gen8.py          writes the fixed requests from template-y-pilot.utf16le.xml (S6's template, byte-equal,
#                               itself S5's Y-all.xml: today's number read less its number term, BRIDGE PILOT LAB,
#                               1 Apr 2026 to 31 Mar 2027) and M-dense.xml (S6's marks request, byte-equal)
#   python3 -I gen8.py k ANSWER writes L3b-pilot.xml for K = ALTVCHID of the marks answer ANSWER minus 10, and k.txt
# L3a adds one field, ALTERID, to the fetch list; L3b adds one filter term, $AlterID > K, to L3a (one variable a step).
# Every change is asserted to occur exactly as often as expected. K is digits only, so nothing a book holds can end
# the formula's literal. Output is UTF-16LE with a byte-order mark, as the templates.
import pathlib, re, sys
import xml.etree.ElementTree as ET
here = pathlib.Path(__file__).resolve().parent
def utf16(name):
    raw = (here / name).read_bytes()
    return (raw[2:] if raw[:2] == b"\xff\xfe" else raw).decode("utf-16-le")
def swap(text, old, new, times):
    assert text.count(old) == times, (old, text.count(old), times)
    return text.replace(old, new)
def window(text, start, end):
    return swap(swap(text, "20260401", start, 2), "20270331", end, 2)
def company(text, name):
    return swap(text, "<SVCURRENTCOMPANY>BRIDGE PILOT LAB</SVCURRENTCOMPANY>", f"<SVCURRENTCOMPANY>{name}</SVCURRENTCOMPANY>", 1)
def write(name, text):
    (here / name).write_bytes(b"\xff\xfe" + text.encode("utf-16-le"))
    print(f"{name}: {len(text.encode('utf-16-le')) + 2} bytes")
def alter_term(k):
    assert re.fullmatch(r"[0-9]{1,9}", k), k
    return f" AND $AlterID &gt; {k}</SYSTEM>"
pilot = utf16("template-y-pilot.utf16le.xml")
def fetch_alterid(text):
    return swap(text, "IRNACKDATE</FETCH>", "IRNACKDATE, ALTERID</FETCH>", 1)
if sys.argv[1:2] == ["k"]:
    # The marks answer: one COMPANY row under ./BODY/DATA/COLLECTION, read by path, never a text search.
    text = re.sub(r"&#([0-9]+);", lambda m: " " if int(m.group(1)) < 32 and m.group(1) not in ("9", "10", "13") else m.group(0), utf16(sys.argv[2]))
    root = ET.fromstring(text)
    rows = root.findall("./BODY/DATA/COLLECTION/COMPANY")
    assert len(rows) == 1, f"{len(rows)} COMPANY rows"
    mark = rows[0].findtext("ALTVCHID", "").strip()
    assert re.fullmatch(r"[0-9]{2,9}", mark), "ALTVCHID absent or not digits"
    k = str(int(mark) - 10)
    write("L3b-pilot.xml", swap(fetch_alterid(pilot), "</SYSTEM>", alter_term(k), 1))
    (here / "k.txt").write_text(f"ALTVCHID {mark}; K {k}\n")
    print(f"K {k}")
    sys.exit(0)
write("A-Y0-year.xml", pilot)
dense = company(pilot, "BRIDGE CORPUS DENSE")
write("B-Y0-month.xml", window(dense, "20250401", "20250430"))
write("L3a-pilot.xml", fetch_alterid(pilot))
write("L3b-dense.xml", swap(fetch_alterid(window(dense, "20250401", "20260331")), "</SYSTEM>", alter_term("29800"), 1))
bt = company(pilot, "BRIDGE BILLTRAIL LAB")
write("Y0-bt-20251001.xml", window(bt, "20251001", "20251001"))
write("Y0-bt-20251002.xml", window(bt, "20251002", "20251002"))
marks = utf16("M-dense.xml")
write("M-billtrail.xml", swap(marks, "<SVCURRENTCOMPANY>BRIDGE CORPUS DENSE</SVCURRENTCOMPANY>",
                              "<SVCURRENTCOMPANY>BRIDGE BILLTRAIL LAB</SVCURRENTCOMPANY>", 1))
