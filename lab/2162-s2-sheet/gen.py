#!/usr/bin/env python3
# gen.py NUMBER: writes I-<tail>.xml (the import) and R-<tail>.xml (the read-back) beside this file.
# I is the request that created BP/26-27/0010 (lab/1342-capture-requests, rendered by the build's
# native_post_request) with the number in VOUCHERNUMBER and REFERENCE and the REMOTEID changed.
# R is the read-back as the code at 094c3edc renders it (S1's R). UTF-16LE with a BOM, as sent.
# Example: python3 -I gen.py BP/26-27/0046
import pathlib, sys
number = sys.argv[1]
here = pathlib.Path(__file__).resolve().parent
tail = number.rsplit("/", 1)[-1]
def load(name):
    raw = (here / name).read_bytes()
    assert raw[:2] == b"\xff\xfe"
    return raw[2:].decode("utf-16-le")
def save(name, text):
    (here / name).write_bytes(b"\xff\xfe" + text.encode("utf-16-le"))
i = load("template-import-0010.utf16le.xml")
assert i.count(">BP/26-27/0010<") == 2
assert i.count('REMOTEID="00000000-0000-4000-8000-000000000777"') == 1
i = i.replace(">BP/26-27/0010<", f">{number}<")
i = i.replace('REMOTEID="00000000-0000-4000-8000-000000000777"', 'REMOTEID="00000000-0000-4000-8000-000000000778"')
save(f"I-{tail}.xml", i)
r = load("template-r-0010.utf16le.xml")
assert r.count('"BP/26-27/0010"') == 1
save(f"R-{tail}.xml", r.replace('"BP/26-27/0010"', f'"{number}"'))
