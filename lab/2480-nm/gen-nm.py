#!/usr/bin/env python3
# gen-nm.py: writes the set-up requests of row 2480-NM beside this file, UTF-16LE with a BOM, as sent. It starts from
# files byte-equal to row 2346-PB's (lab/2346-pb-sheet at 53540665), kept in pb/ and in this folder, and changes only
# what each line below names. Every change is asserted to occur exactly as often as expected, each template's sha256
# is checked first, and the same functions are asserted to give row 2346-PB's own sent files byte for byte.
#   LM-<id>.xml   the ledger master: PB's L-ledger.xml with its customer name (twice: the NAME attribute and the NAME
#                 element) turned into the name of <id> in names.tsv, escaped as master's escape_text escapes it.
#   I-<id>.xml    the invoice that makes the ledger's one open bill: PB's invoice function (its gen.py) on
#                 template-import-0010 (the request the build sent for BP/26-27/0010), with the number of <id>, the
#                 REMOTEID tail 0e01 to 0e0b and the name of <id>, as PB made I-0099.xml.
#   N-<number>.xml  the number read (template-n-0001) for the number of <id>, as PB made N-0099.xml.
# The Ledger Outstandings requests are not made here: the render test (src-tauri/crates/bridge-tally-protocol/tests/
# lab_2480_nm_render.rs) makes them, through the same template as master's builder.
# Example: python3 -I gen-nm.py
import hashlib
import pathlib
from decimal import Decimal
import xml.etree.ElementTree as ET
here = pathlib.Path(__file__).resolve().parent
TEMPLATES = {
    "template-import-0010.utf16le.xml": "b333f0dc819146ae24b2f434a28817e1d142eba0e0386f65bbc5484171b4b21e",
    "template-n-0001.utf16le.xml": "90d188e7089852c411516ef36617dfc3a157df76cfc1787b1985804833ff31e6",
    # Row 2346-PB's sent files (SHA256.txt of lab/2346-pb-sheet), kept in pb/, never sent here.
    "pb/I-0099.xml": "b0b6db9d0ac62670246a3122447e645fe43deb49bc54e00caef8b4323266c062",
    "pb/L-ledger.xml": "00ae11c328069b7de58d8464e40a35883ee5686e49c71364cbb4a7a27bd36b42",
    "pb/N-0099.xml": "1e91dd60721707fabc42f4881a2e482470ff8a2f8b158430d950769a11958b79",
}
PB_NAME = "BRIDGE Walk-in BC"  # PB's customer, in pb/L-ledger.xml and pb/I-0099.xml
AMOUNT = "-1051.00"  # the bill, as PB's invoice makes it
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
def escape(value):
    # master's xml_text::escape_text (bridge-tally-protocol/src/xml_text.rs), the one escape every request renderer uses.
    return (value.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace('"', "&quot;")
            .replace("'", "&apos;").replace("\r", "&#13;").replace("\n", "&#10;"))

def ledger(name):
    return swap(load("pb/L-ledger.xml"), PB_NAME, escape(name), 2)
def invoice(number, tail, name):
    text = load("template-import-0010.utf16le.xml")
    text = swap(text, ">BP/26-27/0010<", f">BP/26-27/{number}<", 2)
    text = swap(text, "00000000-0000-4000-8000-000000000777", f"00000000-0000-4000-8000-000000000{tail}", 1)
    text = swap(text, ">BRIDGE Walk-in<", f">{escape(name)}<", 7)
    leg = f"<LEDGERNAME>{escape(name)}</LEDGERNAME><ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE><AMOUNT>{AMOUNT}</AMOUNT>"
    bill = (f"<BILLALLOCATIONS.LIST><NAME>BP/26-27/{number}</NAME><BILLTYPE>New Ref</BILLTYPE>"
            f"<AMOUNT>{AMOUNT}</AMOUNT></BILLALLOCATIONS.LIST>")
    return swap(text, leg, leg + bill, 1)
def voucher_of(text):
    return ET.fromstring(text.split("?>", 1)[1]).find("BODY/IMPORTDATA/REQUESTDATA/TALLYMESSAGE/VOUCHER")
def check_invoice(text, number, tail, name):
    voucher = voucher_of(text)
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
    for word in ("GSTREGISTRATION>", "CMPGSTIN", "GSTNATUREOFRETURN"):
        assert word not in text, word
def check_ledger(text, name):
    ledger_tag = ET.fromstring(text.split("?>", 1)[1]).find("BODY/IMPORTDATA/REQUESTDATA/TALLYMESSAGE/LEDGER")
    assert ledger_tag.get("NAME") == name and ledger_tag.findtext("NAME") == name
    assert ledger_tag.findtext("PARENT") == "Sundry Debtors" and ledger_tag.findtext("ISBILLWISEON") == "Yes"
    assert ledger_tag.get("ACTION") == "Create"

# These functions give row 2346-PB's own sent files, byte for byte, with PB's values.
assert encode(ledger(PB_NAME)) == (here / "pb/L-ledger.xml").read_bytes() and load("pb/L-ledger.xml")
assert encode(invoice("0099", "d10", PB_NAME)) == (here / "pb/I-0099.xml").read_bytes() and load("pb/I-0099.xml")
number_read = load("template-n-0001.utf16le.xml")
assert encode(swap(number_read, '"BP/26-27/0001"', '"BP/26-27/0099"', 1)) == (here / "pb/N-0099.xml").read_bytes()

rows = [line.split("\t") for line in (here / "names.tsv").read_text("utf-8").split("\n") if line]
assert len(rows) == 11 and len({r[0] for r in rows}) == 11 and len({r[1] for r in rows}) == 11
assert len({r[2] for r in rows}) == 11 and all(r[2].startswith("BRIDGE Nm ") for r in rows)
for index, (ident, number, name) in enumerate(rows, 1):
    tail = f"e{index:02x}"  # 0e01 to 0e0b: no file of rows 2346-PB, 2346-TD or 2162-RS uses them (ROW.md)
    assert name == name.strip() and "  " not in name, name
    text = ledger(name)
    check_ledger(text, name)
    save(f"LM-{ident}.xml", text)
    text = invoice(number, tail, name)
    check_invoice(text, number, tail, name)
    # Against PB's I-0099: the name (7 times), the number (3 times) and the tail, and nothing else.
    back = swap(text, escape(name), PB_NAME, 7)
    back = swap(back, f"BP/26-27/{number}", "BP/26-27/0099", 3)
    back = swap(back, f"000000000{tail}", "000000000d10", 1)
    assert back == load("pb/I-0099.xml"), f"{ident}: the invoice differs from PB's beyond its name, number and tail"
    save(f"I-{ident}.xml", text)
    save(f"N-{number}.xml", swap(number_read, '"BP/26-27/0001"', f'"BP/26-27/{number}"', 1))
