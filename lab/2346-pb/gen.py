#!/usr/bin/env python3
# gen.py: writes the new requests of row 2346-PB beside this file, UTF-16LE with a BOM, as sent. It starts from files
# beside it that are byte-equal to rows 2346-AR's and 2377-NB's (ROW.md says which), and changes only what each line
# below names. Every change is asserted to occur exactly as often as expected, and each template's sha256 is checked
# first. Row 2346-AR's set-up and notes are sent byte-equal to AR's files, so they are not written here; this script
# asserts that AR's own functions, with AR's values, give each of them byte for byte.
#   I-0101.xml        (set-up) a second invoice: AR's I-0099.xml with the number BP/26-27/0101 (twice, and as its
#                     New Ref's name) and the REMOTEID tail 0d14. The bill the part note settles part of.
#   I-CN-P.xml        (p, the part note) AR's I-CN-1.xml with the tail 0d15, REFERENCE BP/26-27/0101, the Agst Ref
#                     named BP/26-27/0101, and every AMOUNT halved: 1051.00 to 525.50 on the party leg and its
#                     allocation, and each other leg's half. The halves are exact and still balance (asserted).
#   N-0101.xml        the number read (template-n-0001.utf16le.xml) with the number BP/26-27/0101.
#   R-0101.xml        the build's read-back (R-0010.xml) with the number BP/26-27/0101.
#   R-CN4.xml         row 2346-CD's R-CN1.xml (the read-back on the type Credit Note for the number 1) with the number 4.
#   L-lo-bc-20260801.xml           row 2377-NB's L-lo-rj-20260804.xml (render_ledger_outstandings_request) with the
#                                  party BRIDGE Walk-in BC and the to-date 20260801.
#   S-snap-20260401-20260801.xml   row 2377-NB's S-snap-20260401-20260804.xml (render_native_ledger_snapshot_request)
#                                  with the to-date 20260801.
#   B-payable.xml     AR's B-bills.xml (render_native_bills_request, Bills Receivable) with the report Bills Payable:
#                     the same function's other kind, the same company and dates.
# Example: python3 -I gen.py
import hashlib
import pathlib
from decimal import Decimal
import xml.etree.ElementTree as ET
here = pathlib.Path(__file__).resolve().parent
TEMPLATES = {
    "template-import-0010.utf16le.xml": "b333f0dc819146ae24b2f434a28817e1d142eba0e0386f65bbc5484171b4b21e",
    "template-n-0001.utf16le.xml": "90d188e7089852c411516ef36617dfc3a157df76cfc1787b1985804833ff31e6",
    "R-0010.xml": "10f9711cc936c9c4d0f70bccb42b54cc10fbd5e7de11287737329cfe7ae56c21",
    "I-CN.xml": "705a9c57db562825f415b6d170e370d954617573ab8678c0a5a8dbc9dd0dbea7",
    "R-CN1.xml": "448064a42815248ce7db5a43b3bba8cdeaa4fe81f97d4b5661c9a159c65f0223",
    "B-bills.xml": "bd9e47b8eb354abe6baa2006425588505b493c24372873924a1caaa5c658f00b",
    "template-nb-L-lo-rj-20260804.xml": "89d0f65d723013b20fdd504e197a8d27f3924347b4decb4767c06bbfe72dcc12",
    "template-nb-S-snap-20260401-20260804.xml": "56296e7c90042202ee7f60e75a9b8a274cebf51f9ff8eda1193dbf6f0e2d92e8",
    # Row 2346-AR's sent files, each sent here unchanged (lab/2346-ar-sheet at fd0aef60, its SHA256.txt).
    "I-0099.xml": "b0b6db9d0ac62670246a3122447e645fe43deb49bc54e00caef8b4323266c062",
    "I-CN-1.xml": "4e6367469525fa96c5d801f7ed409b32bb60c0abd5b01ddc592616751a64bc00",
    "I-CN-2.xml": "d32161f35fd0569e8e76e7694e23e4c12e110a57d5889a81396b85cb93fbe09f",
    "I-CN-3.xml": "ab4846974e37e540f57a8da246820cb0c97d57eda464b8b20d56d6e0ba4788ef",
    "N-0099.xml": "1e91dd60721707fabc42f4881a2e482470ff8a2f8b158430d950769a11958b79",
    "R-0099.xml": "979c56fea1de1b84888c904a21a66335bf04fb80333866108bd66e2d6439a4d0",
    "R-CN2.xml": "d94d082643530015a2c3e2bd5735a8223dadec1a2d81c204ba8080fab8e50c00",
    "R-CN3.xml": "d707f2cb0d55e4c1ceb5c6c90e06c4562e100067731e6540acb34342fbc80e91",
}
def load(name):
    raw = (here / name).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == TEMPLATES[name], f"{name} is not the file ROW.md names"
    assert raw[:2] == b"\xff\xfe", name
    return raw[2:].decode("utf-16-le")
def encode(text):
    return b"\xff\xfe" + text.encode("utf-16-le")
def save(name, text):
    raw = encode(text)
    (here / name).write_bytes(raw)
    print(f"{name}: {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}")
def swap(text, old, new, count):
    assert text.count(old) == count, (old, text.count(old), count)
    return text.replace(old, new)

# Row 2346-AR's functions, unchanged (its gen.py), so its files can be checked and the new ones made the same way.
def tail(text, old, new):
    return swap(text, f"00000000-0000-4000-8000-000000000{old}", f"00000000-0000-4000-8000-000000000{new}", 1)
def party(text, name):
    return swap(text, ">BRIDGE Walk-in<", f">{name}<", 7)
def allocation(text, name, side, amount, bill, kind):
    leg = f"<LEDGERNAME>{name}</LEDGERNAME><ISDEEMEDPOSITIVE>{side}</ISDEEMEDPOSITIVE><AMOUNT>{amount}</AMOUNT>"
    bills = f"<BILLALLOCATIONS.LIST><NAME>{bill}</NAME><BILLTYPE>{kind}</BILLTYPE><AMOUNT>{amount}</AMOUNT></BILLALLOCATIONS.LIST>"
    return swap(text, leg, leg + bills, 1)
def voucher_of(text):
    return ET.fromstring(text.split("?>", 1)[1]).find("BODY/IMPORTDATA/REQUESTDATA/TALLYMESSAGE/VOUCHER")
def check_import(text, bill, kind, amount):
    voucher = voucher_of(text)
    legs = [c for c in voucher if c.tag == "LEDGERENTRIES.LIST"]
    found = [(leg.findtext("LEDGERNAME"), a.findtext("NAME"), a.findtext("BILLTYPE"), a.findtext("AMOUNT"))
             for leg in legs for a in leg if a.tag == "BILLALLOCATIONS.LIST"]
    assert found == [("BRIDGE Walk-in BC", bill, kind, amount)], found
    assert len(legs) == 6, len(legs)
    assert (voucher.find("VOUCHERNUMBER") is None) == (kind == "Agst Ref"), "a note sends no number; the invoice sends one"
    for word in ("GSTREGISTRATION>", "CMPGSTIN", "GSTNATUREOFRETURN"):
        assert word not in text, word
    # The legs balance: their signed amounts sum to zero.
    assert sum(Decimal(leg.findtext("AMOUNT")) for leg in legs) == 0, "the legs do not balance"
def invoice(number, remote, name):
    text = load("template-import-0010.utf16le.xml")
    text = swap(text, ">BP/26-27/0010<", f">BP/26-27/{number}<", 2)
    text = tail(text, "777", remote)
    text = party(text, name)
    return allocation(text, name, "Yes", "-1051.00", f"BP/26-27/{number}", "New Ref")
def note(remote, bill, reference="BP/26-27/0099"):
    text = load("I-CN.xml")
    assert "<VOUCHERNUMBER>" not in text and "BILLALLOCATIONS" not in text
    text = tail(text, "d01", remote)
    text = party(text, "BRIDGE Walk-in BC")
    text = swap(text, "<REFERENCE>BP/26-27/0010</REFERENCE>", f"<REFERENCE>{reference}</REFERENCE>", 1)
    text = allocation(text, "BRIDGE Walk-in BC", "No", "1051.00", bill, "Agst Ref")
    check_import(text, bill, "Agst Ref", "1051.00")
    return text

# AR's sent files are what AR's functions give with AR's values.
assert encode(invoice("0099", "d10", "BRIDGE Walk-in BC")) == (here / "I-0099.xml").read_bytes() and load("I-0099.xml")
for name, remote, bill in (("I-CN-1.xml", "d11", "BP/26-27/0099"), ("I-CN-2.xml", "d12", "BP/26-27/0099"),
                           ("I-CN-3.xml", "d13", "BP/26-27/0098")):
    assert encode(note(remote, bill)) == (here / name).read_bytes() and load(name), name
number = load("template-n-0001.utf16le.xml")
readback = load("R-0010.xml")
assert encode(swap(number, '"BP/26-27/0001"', '"BP/26-27/0099"', 1)) == (here / "N-0099.xml").read_bytes() and load("N-0099.xml")
assert encode(swap(readback, '"BP/26-27/0010"', '"BP/26-27/0099"', 1)) == (here / "R-0099.xml").read_bytes() and load("R-0099.xml")
note_readback = load("R-CN1.xml")
for n in ("2", "3"):
    assert encode(swap(note_readback, '$VoucherNumber = "1"', f'$VoucherNumber = "{n}"', 1)) \
        == (here / f"R-CN{n}.xml").read_bytes() and load(f"R-CN{n}.xml")

# The second invoice: the first with its number and tail alone changed.
second = invoice("0101", "d14", "BRIDGE Walk-in BC")
check_import(second, "BP/26-27/0101", "New Ref", "-1051.00")
assert swap(swap(second, "BP/26-27/0101", "BP/26-27/0099", 3), "000000000d14", "000000000d10", 1) == load("I-0099.xml")
save("I-0101.xml", second)

# The part note: c1's note against the second invoice, every amount halved.
HALVES = {"1051.00": "525.50", "-600.40": "-300.20", "-400.40": "-200.20", "-25.02": "-12.51", "-0.16": "-0.08"}
part = note("d15", "BP/26-27/0101", reference="BP/26-27/0101")
amounts = [a.text for a in voucher_of(part).iter("AMOUNT")]
assert sorted(amounts) == sorted(["1051.00", "1051.00", "-600.40", "-400.40", "-25.02", "-25.02", "-0.16"]), amounts
for whole, half in HALVES.items():
    assert Decimal(half) * 2 == Decimal(whole), (whole, half)
    part = swap(part, f"<AMOUNT>{whole}</AMOUNT>", f"<AMOUNT>{half}</AMOUNT>", amounts.count(whole))
check_import(part, "BP/26-27/0101", "Agst Ref", "525.50")
# Against c1's request: the tail, the reference, the bill's name and the amounts, and nothing else.
back = part
for whole, half in HALVES.items():
    back = back.replace(f"<AMOUNT>{half}</AMOUNT>", f"<AMOUNT>{whole}</AMOUNT>")
back = swap(swap(back, "BP/26-27/0101", "BP/26-27/0099", 2), "000000000d15", "000000000d11", 1)
assert back == load("I-CN-1.xml"), "the part note differs from c1 beyond its tail, reference, bill and amounts"
save("I-CN-P.xml", part)

save("N-0101.xml", swap(number, '"BP/26-27/0001"', '"BP/26-27/0101"', 1))
save("R-0101.xml", swap(readback, '"BP/26-27/0010"', '"BP/26-27/0101"', 1))
save("R-CN4.xml", swap(note_readback, '$VoucherNumber = "1"', '$VoucherNumber = "4"', 1))

lo = load("template-nb-L-lo-rj-20260804.xml")
lo = swap(lo, "<LedgerName>BRIDGE Supplier RJ</LedgerName>", "<LedgerName>BRIDGE Walk-in BC</LedgerName>", 1)
lo = swap(lo, '<SVTODATE TYPE="Date">20260804</SVTODATE>', '<SVTODATE TYPE="Date">20260801</SVTODATE>', 1)
assert "SVFROMDATE" not in lo and "FETCH" not in lo, "the report takes a to-date only and no fetch list"
save("L-lo-bc-20260801.xml", lo)
snap = swap(load("template-nb-S-snap-20260401-20260804.xml"), '<SVTODATE TYPE="Date">20260804</SVTODATE>',
            '<SVTODATE TYPE="Date">20260801</SVTODATE>', 1)
assert '<SVFROMDATE TYPE="Date">20260401</SVFROMDATE>' in snap, "the snapshot reads from the books' first day"
save("S-snap-20260401-20260801.xml", snap)
bills = load("B-bills.xml")
assert '<SVFROMDATE TYPE="Date">20260401</SVFROMDATE><SVTODATE TYPE="Date">20260801</SVTODATE>' in bills
save("B-payable.xml", swap(bills, "<ID>Bills Receivable</ID>", "<ID>Bills Payable</ID>", 1))
