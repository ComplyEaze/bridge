#!/usr/bin/env python3
# gen.py: writes the requests of row 2346-AR beside this file, UTF-16LE with a BOM, as sent. It starts from files
# beside it that are byte-equal to earlier rows' (ROW.md says which), and changes only what each line below names.
# Every change is asserted to occur exactly as often as expected, and each template's sha256 is checked first.
#   L-ledger.xml  (set-up) row 2162-S3a's L-ledger.xml with the ledger BRIDGE Walk-in BC: under Sundry Debtors,
#                 ISBILLWISEON Yes.
#   I-0099.xml    (set-up) the build's request that created BP/26-27/0010, as row 2162-S3a changed it for a
#                 bill-wise customer: the number BP/26-27/0099, the REMOTEID tail 0d10, the party BRIDGE Walk-in BC
#                 in its seven places, and on the party leg one New Ref named by the number for -1051.00. The same
#                 function with S3a's number, tail and party gives S3a's I-0047.xml byte for byte (asserted).
#   I-CN-1.xml    (c1) row 2346-CD's I-CN.xml (the Credit Note: that request as a Credit Note, every side turned
#                 over, no number) with the party BRIDGE Walk-in BC in its seven places, the tail 0d11, REFERENCE
#                 BP/26-27/0099, and on the party leg one Agst Ref named BP/26-27/0099 for 1051.00.
#   I-CN-2.xml    (c2) I-CN-1.xml with the tail 0d12 alone: sent after c1, against the bill c1 settled.
#   I-CN-3.xml    (c3) I-CN-1.xml with the Agst Ref named BP/26-27/0098, a bill the customer does not hold, and the
#                 tail 0d13.
#   N-0099.xml    the number read (template-n-0001.utf16le.xml) with the number BP/26-27/0099.
#   R-0099.xml    the build's read-back (R-0010.xml) with the number BP/26-27/0099.
#   R-CN2.xml     row 2346-CD's R-CN1.xml (the build's read-back on the type Credit Note for the number 1) with the
#   R-CN3.xml     number 2, and 3.
# Example: python3 -I gen.py
import hashlib
import pathlib
import xml.etree.ElementTree as ET
here = pathlib.Path(__file__).resolve().parent
TEMPLATES = {
    "template-import-0010.utf16le.xml": "b333f0dc819146ae24b2f434a28817e1d142eba0e0386f65bbc5484171b4b21e",
    "template-s3a-I-0047.xml": None,
    "template-s3a-L-ledger.xml": None,
    "template-n-0001.utf16le.xml": "90d188e7089852c411516ef36617dfc3a157df76cfc1787b1985804833ff31e6",
    "R-0010.xml": "10f9711cc936c9c4d0f70bccb42b54cc10fbd5e7de11287737329cfe7ae56c21",
    "I-CN.xml": None,
    "R-CN1.xml": None,
}
PREFIX = {  # the first 8 hex characters, for files whose full value ROW.md gives by its source's SHA256.txt
    "template-s3a-I-0047.xml": "e99c279b", "template-s3a-L-ledger.xml": "0671ddc3",
    "I-CN.xml": "705a9c57", "R-CN1.xml": "448064a4",
}
def load(name):
    raw = (here / name).read_bytes()
    digest = hashlib.sha256(raw).hexdigest()
    assert digest == TEMPLATES[name] if TEMPLATES[name] else digest.startswith(PREFIX[name]), f"{name} is not the file ROW.md names"
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
def tail(text, old, new):
    return swap(text, f"00000000-0000-4000-8000-000000000{old}", f"00000000-0000-4000-8000-000000000{new}", 1)
def party(text, name):
    return swap(text, ">BRIDGE Walk-in<", f">{name}<", 7)
def allocation(text, name, side, amount, bill, kind):
    leg = f"<LEDGERNAME>{name}</LEDGERNAME><ISDEEMEDPOSITIVE>{side}</ISDEEMEDPOSITIVE><AMOUNT>{amount}</AMOUNT>"
    bills = f"<BILLALLOCATIONS.LIST><NAME>{bill}</NAME><BILLTYPE>{kind}</BILLTYPE><AMOUNT>{amount}</AMOUNT></BILLALLOCATIONS.LIST>"
    return swap(text, leg, leg + bills, 1)
def check_import(text, bill, kind, amount):
    # The one allocation, on the party leg only, read by a parser, not by a text search.
    voucher = ET.fromstring(text.split("?>", 1)[1]).find("BODY/IMPORTDATA/REQUESTDATA/TALLYMESSAGE/VOUCHER")
    legs = [c for c in voucher if c.tag == "LEDGERENTRIES.LIST"]
    found = [(leg.findtext("LEDGERNAME"), a.findtext("NAME"), a.findtext("BILLTYPE"), a.findtext("AMOUNT"))
             for leg in legs for a in leg if a.tag == "BILLALLOCATIONS.LIST"]
    assert found == [("BRIDGE Walk-in BC", bill, kind, amount)], found
    assert len(legs) == 6, len(legs)
    assert (voucher.find("VOUCHERNUMBER") is None) == (kind == "Agst Ref"), "a note sends no number; the invoice sends one"
    for word in ("GSTREGISTRATION>", "CMPGSTIN", "GSTNATUREOFRETURN"):
        assert word not in text, word

def invoice(number, remote, name):
    text = load("template-import-0010.utf16le.xml")
    text = swap(text, ">BP/26-27/0010<", f">BP/26-27/{number}<", 2)
    text = tail(text, "777", remote)
    text = party(text, name)
    return allocation(text, name, "Yes", "-1051.00", f"BP/26-27/{number}", "New Ref")

# The set-up invoice is S3a's shape: the same function with S3a's values gives its file byte for byte.
assert encode(invoice("0047", "779", "BRIDGE Walk-in BW")) == (here / "template-s3a-I-0047.xml").read_bytes() \
    and load("template-s3a-I-0047.xml")
ledger = swap(load("template-s3a-L-ledger.xml"), "BRIDGE Walk-in BW", "BRIDGE Walk-in BC", 2)
assert "<ISBILLWISEON>Yes</ISBILLWISEON>" in ledger and "<PARENT>Sundry Debtors</PARENT>" in ledger
save("L-ledger.xml", ledger)
setup = invoice("0099", "d10", "BRIDGE Walk-in BC")
check_import(setup, "BP/26-27/0099", "New Ref", "-1051.00")
save("I-0099.xml", setup)

def note(remote, bill):
    text = load("I-CN.xml")
    assert "<VOUCHERNUMBER>" not in text and "BILLALLOCATIONS" not in text
    text = tail(text, "d01", remote)
    text = party(text, "BRIDGE Walk-in BC")
    text = swap(text, "<REFERENCE>BP/26-27/0010</REFERENCE>", "<REFERENCE>BP/26-27/0099</REFERENCE>", 1)
    text = allocation(text, "BRIDGE Walk-in BC", "No", "1051.00", bill, "Agst Ref")
    check_import(text, bill, "Agst Ref", "1051.00")
    return text
save("I-CN-1.xml", note("d11", "BP/26-27/0099"))
save("I-CN-2.xml", note("d12", "BP/26-27/0099"))
save("I-CN-3.xml", note("d13", "BP/26-27/0098"))

save("N-0099.xml", swap(load("template-n-0001.utf16le.xml"), '"BP/26-27/0001"', '"BP/26-27/0099"', 1))
save("R-0099.xml", swap(load("R-0010.xml"), '"BP/26-27/0010"', '"BP/26-27/0099"', 1))
readback = load("R-CN1.xml")
for n in ("2", "3"):
    save(f"R-CN{n}.xml", swap(readback, '$VoucherNumber = "1"', f'$VoucherNumber = "{n}"', 1))
