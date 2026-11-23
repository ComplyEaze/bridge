#!/usr/bin/env python3
# gen.py: writes the raw reads of row 2162-SALES-Q beside this file, UTF-16LE with a BOM, as sent. It starts from
# files beside it that are byte-equal to rows 2464-GN's and 2162-HS's (RUN-SHEET.md, Files, says which), and changes
# only the voucher number in each. Every change is asserted to occur exactly once, each template's sha256 is checked
# first, and the two reads row 2162-HS sent for BP/26-27/0072 are written again and asserted byte-equal to its files.
#   N-0072.xml N-0073.xml N-0074.xml   the build's number read (template-n-0001.utf16le.xml), one number each.
#   R-0072.xml R-0073.xml              the build's read-back with ISPOSTDATED (R-0010.xml), one number each.
#   V-0072.xml V-0073.xml              the native read of one voucher by number (template-full-0057.utf16le.xml).
# No import is written: this row sends no raw write.
# Example: python3 -I gen.py
import hashlib
import pathlib
here = pathlib.Path(__file__).resolve().parent
TEMPLATES = {
    "template-n-0001.utf16le.xml": "90d188e7089852c411516ef36617dfc3a157df76cfc1787b1985804833ff31e6",
    "R-0010.xml": "10f9711cc936c9c4d0f70bccb42b54cc10fbd5e7de11287737329cfe7ae56c21",
    "template-full-0057.utf16le.xml": "cd8e91b441040b742f575955f10d57e187f93e618139e2a6325064c935b09020",
}
HS = {  # row 2162-HS's files (lab/2162-hs-sheet at b6767b87), which this script must reproduce byte for byte
    "N-0072.xml": "5568b9658ce3ea8b22c7633f37ca272c719f567a9ec49c399c594aee8312c619",
    "R-0072.xml": "cb287d00c0a273e30f69c74db671498cb581dc5414529839a603659bdf0fb335",
}
def load(name):
    raw = (here / name).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == TEMPLATES[name], f"{name} is not the template RUN-SHEET.md names"
    assert raw[:2] == b"\xff\xfe", name
    return raw[2:].decode("utf-16-le")
def save(name, text):
    raw = b"\xff\xfe" + text.encode("utf-16-le")
    digest = hashlib.sha256(raw).hexdigest()
    if name in HS:
        assert digest == HS[name], f"{name} differs from row 2162-HS's"
    (here / name).write_bytes(raw)
    print(f"{name}: {len(raw)} bytes, sha256 {digest}")
def swap(text, old, new):
    assert text.count(old) == 1, (old, text.count(old))
    return text.replace(old, new)

number = load("template-n-0001.utf16le.xml")
readback = load("R-0010.xml")
full = load("template-full-0057.utf16le.xml")
for n in ("0072", "0073", "0074"):
    save(f"N-{n}.xml", swap(number, '"BP/26-27/0001"', f'"BP/26-27/{n}"'))
for n in ("0072", "0073"):
    save(f"R-{n}.xml", swap(readback, '"BP/26-27/0010"', f'"BP/26-27/{n}"'))
    save(f"V-{n}.xml", swap(full, '"BP/26-27/0057"', f'"BP/26-27/{n}"'))
