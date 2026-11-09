#!/usr/bin/env python3
# gen.py: writes the requests of row 2162-RP beside this file, UTF-16LE with a BOM, as sent, and checks the one it
# does not write. It starts from files beside it that are byte-equal to row 2162-HS's (ROW.md says which), and
# changes only what each line below names. Every change is asserted to occur exactly as often as expected.
#   I-0040.xml   NOT written here: the build's own request, written by the lab-only render test
#                (src-tauri/src/agent_lab_2162_rp_render_tests.rs on this branch). Checked here to equal
#                template-import-0010.utf16le.xml with the number BP/26-27/0040 (VOUCHERNUMBER and REFERENCE) and the
#                REMOTEID tail 0940, and nothing else changed.
#   I-0010.xml   template-import-0010.utf16le.xml copied byte for byte: the build's request that created
#                BP/26-27/0010, with its own number and its own REMOTEID (tail 0777). Part B's replay.
#   N-0040.xml   the number read (template-n-0001.utf16le.xml) with the number BP/26-27/0040.
#   N-0010.xml   the same number read with the number BP/26-27/0010.
#   R-0040.xml   the build's read-back (R-0010.xml) with the number BP/26-27/0040.
# Example: python3 -I gen.py
import hashlib
import pathlib
here = pathlib.Path(__file__).resolve().parent
REPLAYED = "b333f0dc819146ae24b2f434a28817e1d142eba0e0386f65bbc5484171b4b21e"  # PILOT_REQUEST_SHA256 on master
def load(name):
    raw = (here / name).read_bytes()
    assert raw[:2] == b"\xff\xfe", name
    return raw[2:].decode("utf-16-le")
def save(name, raw):
    (here / name).write_bytes(raw)
    print(f"{name}: {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}")
def encode(text):
    return b"\xff\xfe" + text.encode("utf-16-le")
def swap(text, old, new, count):
    assert text.count(old) == count, (old, text.count(old), count)
    return text.replace(old, new)

template = (here / "template-import-0010.utf16le.xml").read_bytes()
assert hashlib.sha256(template).hexdigest() == REPLAYED
save("I-0010.xml", template)

invoice = load("template-import-0010.utf16le.xml")
invoice = swap(invoice, ">BP/26-27/0010<", ">BP/26-27/0040<", 2)
invoice = swap(invoice, "00000000-0000-4000-8000-000000000777", "00000000-0000-4000-8000-000000000940", 1)
built = (here / "I-0040.xml").read_bytes()
assert built == encode(invoice), "I-0040.xml is not the template with only the number and the REMOTEID tail changed"
print(f"I-0040.xml: {len(built)} bytes, sha256 {hashlib.sha256(built).hexdigest()}, equal to the template's two changes")

number = load("template-n-0001.utf16le.xml")
save("N-0040.xml", encode(swap(number, '"BP/26-27/0001"', '"BP/26-27/0040"', 1)))
save("N-0010.xml", encode(swap(number, '"BP/26-27/0001"', '"BP/26-27/0010"', 1)))

readback = load("R-0010.xml")
save("R-0040.xml", encode(swap(readback, '"BP/26-27/0010"', '"BP/26-27/0040"', 1)))
