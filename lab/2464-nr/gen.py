#!/usr/bin/env python3
# gen.py: writes the requests of row 2464-NR beside this file, UTF-16LE with a BOM, as sent. It starts from files
# beside it that are byte-equal to row 2464-GN's (ROW.md says which), and changes only what each line below names.
# Every change is asserted to occur exactly as often as expected, and each template's sha256 is checked first.
#   I-0085.xml   (import A') template-import-0010.utf16le.xml (the build's request that created BP/26-27/0010) with
#                the number BP/26-27/0085 (VOUCHERNUMBER and REFERENCE), the REMOTEID tail 0985, and one element
#                added: <GSTREGISTRATION>Rajasthan Registration</GSTREGISTRATION>, right after PARTYNAME.
#                It is row 2464-GN's I-0095.xml with the unit's name, the number and the tail changed: the same
#                function, given GN's number, tail and unit, writes GN's I-0095.xml byte for byte (asserted).
#                With the element taken out again it is the template renumbered (asserted).
#   I-0087.xml   (import B') the same with the number BP/26-27/0087, the tail 0987, and two elements added right
#                after PARTYNAME: that GSTREGISTRATION, then <CMPGSTIN>08ZZZZZ0000Z1ZQ</CMPGSTIN>.
#                The place is the order Tally's own native read gives these fields (ROW.md, Files).
#   N-0085.xml   the number read (template-n-0001.utf16le.xml) with the number BP/26-27/0085; N-0087.xml with 0087.
#   R-0085.xml   the build's read-back (R-0010.xml) with the number BP/26-27/0085; R-0087.xml with 0087.
#   RC-0010.xml  the candidate read-back: R-0010.xml with GSTREGISTRATION and CMPGSTIN added at the end of its FETCH
#                list, and nothing else changed (byte-equal to GN's, asserted); RC-0085.xml and RC-0087.xml are it
#                with the number 0085 and 0087.
#   V-0010.xml   template-full-0057.utf16le.xml (row 2162-RS's V-0057.xml: every native field of the one voucher
#   V-0085.xml   holding the number) with the number BP/26-27/0010, 0085 and 0087. V-0010.xml is byte-equal to
#   V-0087.xml   row 2464-GN's (asserted).
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
# Row 2464-GN's files (lab/2464-gn-sheet at 23b2e57b, SHA256.txt), the full sha256 of each.
GN = {
    "I-0095.xml": "d955b1636a1b06624b2a5ab4cc19e35b222b05c9e84153c458ed710a11b9b73d",
    "RC-0010.xml": "bd9717c7830df5217a21ef3d8f4fb1ffb1bc0bc150a7d66dc5cefe12ebe852b5",
    "V-0010.xml": "310bc3e1f333b5560d5be5acfa24f03b0c8d155a4af6b0cde6689af7e0ac46c3",
}
PARTY = "<PARTYNAME>BRIDGE Walk-in</PARTYNAME>"
UNIT = "<GSTREGISTRATION>Rajasthan Registration</GSTREGISTRATION>"
GSTIN = "<CMPGSTIN>08ZZZZZ0000Z1ZQ</CMPGSTIN>"
GN_UNIT = "<GSTREGISTRATION>Maharashtra Registration</GSTREGISTRATION>"
def load(name):
    raw = (here / name).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == TEMPLATES[name], f"{name} is not the template ROW.md names"
    assert raw[:2] == b"\xff\xfe", name
    return raw[2:].decode("utf-16-le")
def encode(text):
    return b"\xff\xfe" + text.encode("utf-16-le")
def save(name, text):
    raw = encode(text)
    (here / name).write_bytes(raw)
    digest = hashlib.sha256(raw).hexdigest()
    print(f"{name}: {len(raw)} bytes, sha256 {digest}")
    return digest
def swap(text, old, new, count):
    assert text.count(old) == count, (old, text.count(old), count)
    return text.replace(old, new)

def renumbered(number, tail):
    text = load("template-import-0010.utf16le.xml")
    for word in ("GSTREGISTRATION>", "CMPGSTIN", "CMPGSTSTATE"):
        assert word not in text, f"the template already names {word}"
    text = swap(text, ">BP/26-27/0010<", f">BP/26-27/{number}<", 2)
    text = swap(text, "00000000-0000-4000-8000-000000000777", f"00000000-0000-4000-8000-00000000{tail}", 1)
    assert len(text.split("REMOTEID=\"", 1)[1].split("\"", 1)[0]) == 36, "a REMOTEID of 36 characters"
    return text

def invoice(number, tail, added):
    text = swap(renumbered(number, tail), PARTY, PARTY + "".join(added), 1)
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
    # Taken out again, the added elements leave the template renumbered, byte for byte.
    assert swap(text, "".join(added), "", 1) == renumbered(number, tail)
    return text

# The same function writes GN's refused import A, byte for byte: so A' differs from it only in the unit, number, tail.
assert hashlib.sha256(encode(invoice("0095", "0995", [GN_UNIT]))).hexdigest() == GN["I-0095.xml"]

save("I-0085.xml", invoice("0085", "0985", [UNIT]))
save("I-0087.xml", invoice("0087", "0987", [UNIT, GSTIN]))

number = load("template-n-0001.utf16le.xml")
readback = load("R-0010.xml")
candidate = swap(readback, ", ISPOSTDATED</FETCH>", ", ISPOSTDATED, GSTREGISTRATION, CMPGSTIN</FETCH>", 1)
assert save("RC-0010.xml", candidate) == GN["RC-0010.xml"], "RC-0010.xml is not GN's"
for n in ("0085", "0087"):
    save(f"N-{n}.xml", swap(number, '"BP/26-27/0001"', f'"BP/26-27/{n}"', 1))
    save(f"R-{n}.xml", swap(readback, '"BP/26-27/0010"', f'"BP/26-27/{n}"', 1))
    save(f"RC-{n}.xml", swap(candidate, '"BP/26-27/0010"', f'"BP/26-27/{n}"', 1))

full = load("template-full-0057.utf16le.xml")
assert save("V-0010.xml", swap(full, '"BP/26-27/0057"', '"BP/26-27/0010"', 1))== GN["V-0010.xml"]
for n in ("0085", "0087"):
    save(f"V-{n}.xml", swap(full, '"BP/26-27/0057"', f'"BP/26-27/{n}"', 1))
