#!/usr/bin/env python3
# gen10.py: writes, on the lab PC only, the two requests of row 2162-PB that carry the buyer's GSTIN, UTF-16LE with a
# BOM, as sent. The buyer's PAN part is not in any public file: the run coordinator sends it privately, and it is put,
# alone on one line, in buyer-pan.local beside this file. buyer-pan.local, the two files written here and
# filled-sha256.local.txt are named in .gitignore and are never committed.
#   L-seedB.xml   row 2162-S13a2's L-seed.xml (lab/2162-s13a2-sheet at 6e157faf, byte-equal beside this file) with its
#                 one GSTIN replaced by the buyer's.
#   I-0053B.xml   that row's I-0053.xml with its one PARTYGSTIN replaced by the buyer's. Nothing else changes.
# The buyer's GSTIN is S13a2's with only the PAN part changed: state code 08 (Rajasthan, the company's state and the
# invoice's place of supply), the PAN part from buyer-pan.local, entity code 3 and Z as in S13a2, and the check
# character (mod 36) computed. Every change is asserted to occur exactly once, and the result is asserted to be
# S13a2's file again once the GSTIN is set back. The GSTIN is never printed; each file's sha256 and size are, and are
# written to filled-sha256.local.txt for RESULTS.md, and so is the GSTIN's check character.
# Example: python3 -I gen10.py
import hashlib, pathlib, re
here = pathlib.Path(__file__).resolve().parent
def load(name):
    raw = (here / name).read_bytes()
    assert raw[:2] == b"\xff\xfe", name
    return raw[2:].decode("utf-16-le")
def swap(text, old, new, count):
    assert text.count(old) == count, ("swap", text.count(old), count)
    return text.replace(old, new)
def check_character(first14):
    chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    total = 0
    for index, char in enumerate(first14):
        value = chars.index(char) * (2 if index % 2 else 1)
        total += value // 36 + value % 36
    return chars[(36 - total % 36) % 36]

S13A2_GSTIN = "08ZZZZZ0000Z3ZO"
COMPANY_GSTIN = "08ZZZZZ0000Z1ZQ"
BOOK_GSTINS = {"08ZZZZZ0000Z2ZP", S13A2_GSTIN}
for known in (S13A2_GSTIN, COMPANY_GSTIN):
    assert known[-1] == check_character(known[:14]), "the check character rule"
pan = (here / "buyer-pan.local").read_text(encoding="ascii").strip()
assert re.fullmatch(r"[A-Z]{5}[0-9]{4}[A-Z]", pan), "buyer-pan.local does not hold one PAN part"
assert pan != COMPANY_GSTIN[2:12], "the buyer's PAN part is the company's: that is row S13a2's case"
first14 = "08" + pan + "3Z"
gstin = first14 + check_character(first14)
assert gstin != COMPANY_GSTIN and gstin not in BOOK_GSTINS

lines = []
for source, target in (("L-seed.xml", "L-seedB.xml"), ("I-0053.xml", "I-0053B.xml")):
    text = swap(load(source), S13A2_GSTIN, gstin, 1)
    assert text.replace(gstin, S13A2_GSTIN) == load(source)
    raw = b"\xff\xfe" + text.encode("utf-16-le")
    (here / target).write_bytes(raw)
    lines.append(f"{hashlib.sha256(raw).hexdigest()}  {len(raw)}  {target}")
(here / "filled-sha256.local.txt").write_text("\n".join(lines) + "\n", encoding="ascii")
print("\n".join(lines))
# The check character alone is not the PAN; ROW.md's EXPECT lines name the GSTIN as 08<PAN-B>3Z and this character.
print("check character:", gstin[-1])
