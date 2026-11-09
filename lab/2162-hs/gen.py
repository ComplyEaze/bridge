#!/usr/bin/env python3
# gen.py: writes the requests of row 2162-HS beside this file, UTF-16LE with a BOM, as sent. It starts from files
# beside it that are byte-equal to row 2162-KD's or row 2162-RU's (ROW.md says which), and changes only what each
# line below names. Every change is asserted to occur exactly as often as expected.
#   I-0072.xml   template-import-0010.utf16le.xml (the build's request that created BP/26-27/0010) with the number
#                BP/26-27/0072 (VOUCHERNUMBER and REFERENCE) and the REMOTEID tail 0872. Party, figures, ledgers, type
#                and date unchanged: two sales lines, 600.40 on BRIDGE Svc 998313 5% and 400.40 on BRIDGE Svc 998314 5%.
#   N-0072.xml   the number read (template-n-0001.utf16le.xml) with the number BP/26-27/0072.
#   R-0072.xml   the build's read-back (R-0010.xml) with the number BP/26-27/0072.
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
invoice = swap(invoice, ">BP/26-27/0010<", ">BP/26-27/0072<", 2)
invoice = swap(invoice, "00000000-0000-4000-8000-000000000777", "00000000-0000-4000-8000-000000000872", 1)
assert invoice.count("BRIDGE Svc 998313 5%") == 1 and invoice.count("BRIDGE Svc 998314 5%") == 1
save("I-0072.xml", invoice)

number = load("template-n-0001.utf16le.xml")
save("N-0072.xml", swap(number, '"BP/26-27/0001"', '"BP/26-27/0072"', 1))

readback = load("R-0010.xml")
save("R-0072.xml", swap(readback, '"BP/26-27/0010"', '"BP/26-27/0072"', 1))
