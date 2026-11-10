#!/usr/bin/env python3
# gen.py: writes the requests of row 2464-GR beside this file, UTF-16LE with a BOM, as sent. It starts from files
# beside it that are byte-equal to earlier rows' (ROW.md says which), and changes only what each line below names.
# Every change is asserted to occur exactly as often as expected, and each template's sha256 is checked first.
#   I-0091.xml   template-import-0010.utf16le.xml (the build's request that created BP/26-27/0010) with the number
#                BP/26-27/0091 (VOUCHERNUMBER and REFERENCE) and the REMOTEID tail 0991. An edit of a captured
#                request, as row 2162-HS's I-0072.xml was; it names no registration, as the build's never does.
#   N-0091.xml   the number read (template-n-0001.utf16le.xml) with the number BP/26-27/0091.
#   R-0091.xml   the build's read-back (R-0010.xml) with the number BP/26-27/0091.
#   V-0010.xml   template-full-0057.utf16le.xml (row 2162-RS's V-0057.xml: every native field of the one voucher
#   V-0091.xml   holding the number) with the number BP/26-27/0010, and with BP/26-27/0091.
# Example: python3 -I gen.py
import hashlib
import pathlib
here = pathlib.Path(__file__).resolve().parent
TEMPLATES = {
    "template-import-0010.utf16le.xml": "b333f0dc819146ae24b2f434a28817e1d142eba0e0386f65bbc5484171b4b21e",
    "template-n-0001.utf16le.xml": "90d188e7089852c411516ef36617dfc3a157df76cfc1787b1985804833ff31e6",
    "R-0010.xml": "10f9711cc936c9c4d0f70bccb42b54cc10fbd5e7de11287737329cfe7ae56c21",
    "template-full-0057.utf16le.xml": "cd8e91b441040b742f575955f10d57e187f93e618139e2a6325064c935b09020",  # row 2162-RS's V-0057.xml
}
def load(name):
    raw = (here / name).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == TEMPLATES[name], f"{name} is not the template ROW.md names"
    assert raw[:2] == b"\xff\xfe", name
    return raw[2:].decode("utf-16-le")
def save(name, text):
    raw = b"\xff\xfe" + text.encode("utf-16-le")
    (here / name).write_bytes(raw)
    print(f"{name}: {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}")
def swap(text, old, new, count):
    assert text.count(old) == count, (old, text.count(old), count)
    return text.replace(old, new)

invoice = load("template-import-0010.utf16le.xml")
invoice = swap(invoice, ">BP/26-27/0010<", ">BP/26-27/0091<", 2)
invoice = swap(invoice, "00000000-0000-4000-8000-000000000777", "00000000-0000-4000-8000-000000000991", 1)
for word in ("GSTREGISTRATION>", "CMPGSTIN", "CMPGSTSTATE"):
    assert word not in invoice, f"the import names {word}"
save("I-0091.xml", invoice)

number = load("template-n-0001.utf16le.xml")
save("N-0091.xml", swap(number, '"BP/26-27/0001"', '"BP/26-27/0091"', 1))

readback = load("R-0010.xml")
save("R-0091.xml", swap(readback, '"BP/26-27/0010"', '"BP/26-27/0091"', 1))

full = load("template-full-0057.utf16le.xml")
save("V-0010.xml", swap(full, '"BP/26-27/0057"', '"BP/26-27/0010"', 1))
save("V-0091.xml", swap(full, '"BP/26-27/0057"', '"BP/26-27/0091"', 1))
