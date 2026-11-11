#!/usr/bin/env python3
# gen-sv.py: writes row 2480-SV's twelve new requests beside this file, UTF-16LE with a BOM, as sent, and checks
# them. New for this row. It writes nothing until every check below has passed.
#   1. Every other file of this folder that row 2480-SC's sheet holds is SC's, byte for byte: its sha256 and size as
#      sc/SHA256.txt lists them (that file is SC's SHA256.txt, lab/2480-sc-sheet at 39162ad5, pinned here). The
#      files this row adds are named in OWN and NEW; any other file stops it.
#   2. The six vouchers J-<id>.xml, for qa, qb, ua, ub, ka and kb (the control gets none: ROW.md, Sizes): each is the control's invoice I-pl.xml (MX's,
#      SC's and SK's, byte for byte) with the control's name turned into the id's (escaped as master's
#      xml_text::escape_text writes it, as gen-sk.py does), its number BP/26-27/0281 into the id's new number, and its
#      REMOTEID tail e81 into the id's new tail, and nothing else. Each swap's count is asserted, and swapped back
#      each gives I-pl.xml byte for byte. Where SK's invoice I-<id>.xml is in this folder (all but kb), J-<id> with
#      SK's number and tail swapped in again is that file byte for byte: the name is written as SK's imports wrote it.
#   3. The six reads N-<new number>.xml: SC's N-0281.xml with the number turned into the new one, once, and nothing
#      else, swapped back to N-0281.xml byte for byte.
#   4. The new numbers and tails are none that names.tsv, MX (e81 to e8b) or SK (ec2 to ecb) used, and no two equal.
# Example: python3 -I gen-sv.py
import hashlib
import pathlib
from decimal import Decimal
import xml.etree.ElementTree as ET
here = pathlib.Path(__file__).resolve().parent
SC_LIST = "4a71b9960fa129ea19db317537af611b738b283c34a9d65b0418209683c3e55e"  # sc/SHA256.txt
MX_PLAIN = "BRIDGE Mx Plain"
AMOUNT = "-1051.00"
# The voucher this row sends for each id, in the order it sends them: (id, new number, new REMOTEID tail).
VOUCHERS = (("qa", "0303", "ed3"), ("qb", "0304", "ed4"), ("ua", "0305", "ed5"),
            ("ub", "0306", "ed6"), ("ka", "0307", "ed7"), ("kb", "0308", "ed8"))
OWN = {"ROW.md", "gen-sv.py", "sv.py", "tried.txt", "SHA256.txt"}
NEW = {f"J-{i}.xml" for i, _, _ in VOUCHERS} | {f"N-{n}.xml" for _, n, _ in VOUCHERS}

def sha(raw):
    return hashlib.sha256(raw).hexdigest()
def swap(text, old, new, count):
    assert text.count(old) == count, (old, text.count(old), count)
    assert new == old or text.count(new) == 0, (new, "already in the source")
    return text.replace(old, new)
def escape(value):
    # master's xml_text::escape_text (bridge-tally-protocol/src/xml_text.rs), as gen-sk.py writes it.
    return (value.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace('"', "&quot;")
            .replace("'", "&apos;").replace("\r", "&#13;").replace("\n", "&#10;"))
def text_of(name):
    raw = (here / name).read_bytes()
    assert raw[:2] == b"\xff\xfe", name
    return raw[2:].decode("utf-16-le")
def encode(text):
    return b"\xff\xfe" + text.encode("utf-16-le")

# 1. SC's files.
listing = (here / "sc" / "SHA256.txt").read_bytes()
assert sha(listing) == SC_LIST, "sc/SHA256.txt is not SC's"
sc = {}
for line in listing.decode("utf-8").split("\n"):
    if line:
        digest, size, name = line.split("  ")
        sc[name] = (digest, int(size))
held = sorted(str(p.relative_to(here)) for p in here.rglob("*") if p.is_file())
for name in held:
    if name in OWN or name in NEW or name == "sc/SHA256.txt":
        continue
    assert name in sc, f"{name} is neither SC's nor this row's own"
    raw = (here / name).read_bytes()
    assert (sha(raw), len(raw)) == sc[name], f"{name} is not SC's file of that name"
assert not [n for n in sc if n not in held and n != "ROW.md" and n != "SHA256.txt"], "an SC file is missing"

names = [line.split("\t") for line in (here / "names.tsv").read_text("utf-8").split("\n") if line]
number_of = {i: n for i, n, _ in names}
name_of = {i: name for i, _, name in names}

# 4. The numbers and tails.
new_numbers = [n for _, n, _ in VOUCHERS]
new_tails = [t for _, _, t in VOUCHERS]
assert len(set(new_numbers)) == 6 and len(set(new_tails)) == 6
assert not set(new_numbers) & set(number_of.values()), "a new number is one names.tsv holds"
used_tails = {f"e{0x80 + k:02x}" for k in range(1, 12)} | {f"e{0xc0 + k:02x}" for k in range(2, 12)}
assert not set(new_tails) & used_tails, "a new tail is one MX or SK sent"

# 2. The six vouchers.
def check_invoice(text, number, tail, name):  # gen-sk.py's checks, unchanged
    voucher = ET.fromstring(text.split("?>", 1)[1]).find("BODY/IMPORTDATA/REQUESTDATA/TALLYMESSAGE/VOUCHER")
    assert voucher.get("REMOTEID") == f"00000000-0000-4000-8000-000000000{tail}"
    for tag in ("PARTYLEDGERNAME", "PARTYNAME", "BASICBASEPARTYNAME", "PARTYMAILINGNAME", "BASICBUYERNAME",
                "CONSIGNEEMAILINGNAME"):
        assert voucher.findtext(tag) == name, (tag, voucher.findtext(tag))
    for tag in ("VOUCHERNUMBER", "REFERENCE"):
        assert voucher.findtext(tag) == f"BP/26-27/{number}", tag
    legs = [c for c in voucher if c.tag == "LEDGERENTRIES.LIST"]
    assert len(legs) == 6 and legs[0].findtext("LEDGERNAME") == name, len(legs)
    found = [(leg.findtext("LEDGERNAME"), a.findtext("NAME"), a.findtext("BILLTYPE"), a.findtext("AMOUNT"))
             for leg in legs for a in leg if a.tag == "BILLALLOCATIONS.LIST"]
    assert found == [(name, f"BP/26-27/{number}", "New Ref", AMOUNT)], found
    assert sum(Decimal(leg.findtext("AMOUNT")) for leg in legs) == 0, "the legs do not balance"
    assert voucher.get("ACTION") == "Create"
    for word in ("GSTREGISTRATION>", "CMPGSTIN", "GSTNATUREOFRETURN"):
        assert word not in text, word

inv = text_of("I-pl.xml")
check_invoice(inv, "0281", "e81", MX_PLAIN)
num = text_of("N-0281.xml")
out = {}
for ident, number, tail in VOUCHERS:
    name = name_of[ident]
    e = escape(name)
    text = swap(swap(swap(inv, MX_PLAIN, e, 7), "BP/26-27/0281", f"BP/26-27/{number}", 3),
                "000000000e81", f"000000000{tail}", 1)
    check_invoice(text, number, tail, name)
    assert swap(swap(swap(text, e, MX_PLAIN, 7), f"BP/26-27/{number}", "BP/26-27/0281", 3),
                f"000000000{tail}", "000000000e81", 1) == inv
    if (here / f"I-{ident}.xml").is_file():
        sk_tail = None
        sk_text = text_of(f"I-{ident}.xml")
        sk_number = number_of[ident]
        for k in range(1, 12):
            candidate = f"e{0xc0 + k:02x}" if ident != "pl" else "e81"
            if f"000000000{candidate}" in sk_text:
                sk_tail = candidate
        assert sk_tail is not None, ident
        again = swap(swap(text, f"BP/26-27/{number}", f"BP/26-27/{sk_number}", 3),
                     f"000000000{tail}", f"000000000{sk_tail}", 1)
        assert again == sk_text, f"J-{ident}.xml does not write the name as I-{ident}.xml does"
    else:
        assert ident == "kb", ident
    out[f"J-{ident}.xml"] = encode(text)
    read = swap(num, '"BP/26-27/0281"', f'"BP/26-27/{number}"', 1)
    assert swap(read, f'"BP/26-27/{number}"', '"BP/26-27/0281"', 1) == num
    out[f"N-{number}.xml"] = encode(read)
assert set(out) == NEW
for name, raw in sorted(out.items()):
    (here / name).write_bytes(raw)
    print(f"{name}: {len(raw)} bytes, sha256 {sha(raw)}")
print(f"{len(out)} files written; {len(held)} files of the folder checked")
