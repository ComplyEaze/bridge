#!/usr/bin/env python3
# gen.py NUMBER: writes C-NUMBER.xml and R-NUMBER.xml beside this file from the two captured
# read-back requests of lab/1578-postdated, changing only the voucher number. UTF-16LE with a BOM,
# as the templates are. Example: python3 -I gen.py BP/26-27/0045
import pathlib, sys
number = sys.argv[1]
here = pathlib.Path(__file__).resolve().parent
tail = number.rsplit("/", 1)[-1]
for letter, name in (("C", "template-c0-0010.utf16le.xml"), ("R", "template-r1-0010.utf16le.xml")):
    raw = (here / name).read_bytes()
    assert raw[:2] == b"\xff\xfe"
    text = raw[2:].decode("utf-16-le")
    assert text.count('"BP/26-27/0010"') == 1
    out = text.replace('"BP/26-27/0010"', f'"{number}"')
    (here / f"{letter}-{tail}.xml").write_bytes(b"\xff\xfe" + out.encode("utf-16-le"))
