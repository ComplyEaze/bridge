#!/usr/bin/env python3
# gen.py: writes the requests of row 2464-GN beside this file, UTF-16LE with a BOM, as sent. It starts from files
# beside it that are byte-equal to row 2464-GR's (ROW.md says which), and changes only what each line below names.
# Every change is asserted to occur exactly as often as expected, and each template's sha256 is checked first.
#   I-0095.xml   (import A) template-import-0010.utf16le.xml (the build's request that created BP/26-27/0010) with
#                the number BP/26-27/0095 (VOUCHERNUMBER and REFERENCE), the REMOTEID tail 0995, and one element
#                added: <GSTREGISTRATION>Maharashtra Registration</GSTREGISTRATION>, right after PARTYNAME.
#   I-0097.xml   (import B) the same with the number BP/26-27/0097, the tail 0997, and two elements added right
#                after PARTYNAME: that GSTREGISTRATION, then <CMPGSTIN>27ZZZZZ0000Z1ZQ</CMPGSTIN>.
#                The place is the order Tally's own native read gives these fields (ROW.md, Files).
#   N-0095.xml   the number read (template-n-0001.utf16le.xml) with the number BP/26-27/0095; N-0097.xml with 0097.
#   R-0095.xml   the build's read-back (R-0010.xml) with the number BP/26-27/0095; R-0097.xml with 0097.
#   RC-0010.xml  the candidate read-back: R-0010.xml with GSTREGISTRATION and CMPGSTIN added at the end of its FETCH
#                list, and nothing else changed; RC-0095.xml and RC-0097.xml are it with the number 0095 and 0097.
#   V-0010.xml   template-full-0057.utf16le.xml (row 2162-RS's V-0057.xml: every native field of the one voucher
#   V-0095.xml   holding the number) with the number BP/26-27/0010, 0095 and 0097. V-0010.xml is byte-equal to
#   V-0097.xml   row 2464-GR's.
# Example: python3 -I gen.py
import hashlib
import pathlib
import xml.etree.ElementTree as ET
here = pathlib.Path(__file__).resolve().parent
TEMPLATES = {
    "template-import-0010.utf16le.xml": "b333f0dc819146ae24b2f434a28817e1d142eba0e0386f65bbc5484171b4b21e",
    "template-n-0001.utf16le.xml": "90d188e7089852c411516ef36617dfc3a157df76cfc1787b1985804833ff31e6",
    "R-0010.xml": "10f9711cc936c9c4d0f70bccb42b54cc10fbd5e7de11287737329cfe7ae56c21",
    "template-full-0057.utf16le.xml": "cd8e91b441040b742f575955f10d57e187f93e618139e2a6325064c935b09020",  # row 2162-RS's V-0057.xml
}
GR_V_0010 = "310bc3e1"  # the first 8 hex characters of row 2464-GR's V-0010.xml (lab/2464-gr-sheet at 3508daa0)
PARTY = "<PARTYNAME>BRIDGE Walk-in</PARTYNAME>"
UNIT = "<GSTREGISTRATION>Maharashtra Registration</GSTREGISTRATION>"
GSTIN = "<CMPGSTIN>27ZZZZZ0000Z1ZQ</CMPGSTIN>"
def load(name):
    raw = (here / name).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == TEMPLATES[name], f"{name} is not the template ROW.md names"
    assert raw[:2] == b"\xff\xfe", name
    return raw[2:].decode("utf-16-le")
def save(name, text):
    raw = b"\xff\xfe" + text.encode("utf-16-le")
    (here / name).write_bytes(raw)
    digest = hashlib.sha256(raw).hexdigest()
    print(f"{name}: {len(raw)} bytes, sha256 {digest}")
    return digest
def swap(text, old, new, count):
    assert text.count(old) == count, (old, text.count(old), count)
    return text.replace(old, new)

def invoice(number, tail, added):
    text = load("template-import-0010.utf16le.xml")
    for word in ("GSTREGISTRATION>", "CMPGSTIN", "CMPGSTSTATE"):
        assert word not in text, f"the template already names {word}"
    text = swap(text, ">BP/26-27/0010<", f">BP/26-27/{number}<", 2)
    text = swap(text, "00000000-0000-4000-8000-000000000777", f"00000000-0000-4000-8000-00000000{tail}", 1)
    assert len(text.split("REMOTEID=\"", 1)[1].split("\"", 1)[0]) == 36, "a REMOTEID of 36 characters"
    text = swap(text, PARTY, PARTY + "".join(added), 1)
    # The order of the voucher's children around the change, read by a parser, not by a text search.
    voucher = ET.fromstring(text.split("?>", 1)[1]).find("BODY/IMPORTDATA/REQUESTDATA/TALLYMESSAGE/VOUCHER")
    tags = [child.tag for child in voucher]
    at = tags.index("PARTYNAME")
    want = ["PARTYNAME"] + [ET.fromstring(e).tag for e in added] + ["BASICBASEPARTYNAME"]
    assert tags[at:at + len(want)] == want, tags[at:at + len(want)]
    for e in added:
        found = [c for c in voucher if c.tag == ET.fromstring(e).tag]
        assert len(found) == 1 and not found[0].attrib and found[0].text == ET.fromstring(e).text, e
    assert tags.count("CMPGSTSTATE") == 0 and tags.count("CMPGSTREGISTRATIONTYPE") == 0
    return text

save("I-0095.xml", invoice("0095", "0995", [UNIT]))
save("I-0097.xml", invoice("0097", "0997", [UNIT, GSTIN]))

number = load("template-n-0001.utf16le.xml")
readback = load("R-0010.xml")
candidate = swap(readback, ", ISPOSTDATED</FETCH>", ", ISPOSTDATED, GSTREGISTRATION, CMPGSTIN</FETCH>", 1)
save("RC-0010.xml", candidate)
for n in ("0095", "0097"):
    save(f"N-{n}.xml", swap(number, '"BP/26-27/0001"', f'"BP/26-27/{n}"', 1))
    save(f"R-{n}.xml", swap(readback, '"BP/26-27/0010"', f'"BP/26-27/{n}"', 1))
    save(f"RC-{n}.xml", swap(candidate, '"BP/26-27/0010"', f'"BP/26-27/{n}"', 1))

full = load("template-full-0057.utf16le.xml")
assert save("V-0010.xml", swap(full, '"BP/26-27/0057"', '"BP/26-27/0010"', 1)).startswith(GR_V_0010)
for n in ("0095", "0097"):
    save(f"V-{n}.xml", swap(full, '"BP/26-27/0057"', f'"BP/26-27/{n}"', 1))
