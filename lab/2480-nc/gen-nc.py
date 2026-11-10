#!/usr/bin/env python3
# gen-nc.py: writes the requests of row 2480-NC beside this file, UTF-16LE with a BOM, as sent. It is row 2480-NS's
# gen-ns.py (lab/2480-ns-sheet at 6e44a3dc) with the lines marked NC changed: the names, numbers and REMOTEID tails of
# this row's names.tsv, its ids, which are its own, and a check against NS's names too. NS's gen-ns.py was NM's gen-nm.py
# (lab/2480-nm-sheet at 73cb01ae) with the lines marked NS changed. It starts from files byte-equal to rows 2346-PB's and 2480-NM's, checks each one's sha256 first, and
# asserts that the same functions give those rows' own files byte for byte before it writes anything.
#   LM-<id>.xml   the ledger master: PB's L-ledger.xml with its customer name (twice: the NAME attribute and the NAME
#                 element) turned into the name of <id> in names.tsv, escaped as master's escape_text escapes it.
#   I-<id>.xml    the invoice that makes the ledger's one open bill: PB's invoice function on template-import-0010,
#                 with the number of <id>, the REMOTEID tail 0e41 to 0e4b (NC) and the name of <id>, as PB made I-0099.xml.
#   N-<number>.xml  the number read (template-n-0001) for the number of <id>, as PB made N-0099.xml.
#   L-lo-<id>-20260801.xml  (NS) NM's L-lo-pl-20260801.xml (made by NM's render test from master's builder template)
#                 with its one escaped name turned into the escaped name of <id>, and nothing else. It first asserts
#                 that the same turn gives each of NM's eleven L-lo files from NM's L-lo-pl, byte for byte (nm/).
# Example: python3 -I gen-nc.py
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
    # NS: row 2480-NM's names and its eleven sent L-lo files (SHA256.txt of lab/2480-nm-sheet), kept in nm/, never sent.
    "nm/names.tsv": "24d3c7f71d092a9bdf48c6fd3a703ba171c0096fef2cdbf75503c0d43720b4a5",
    "nm/L-lo-pl-20260801.xml": "ffeda72f81376821d51645519373984944b7956ea8da652bed12cd80cf00a2f0",
    "nm/L-lo-hy-20260801.xml": "56730ccc612476a3a882498d2eb4698fb24ee4ec587e9bd87e22646a773bdf38",
    "nm/L-lo-dt-20260801.xml": "bf212fe7a88fa0636e21015e6042f0fc799a5e315b17eafbc4aac0e112dcc07e",
    "nm/L-lo-am-20260801.xml": "c851cf90e314f73d0d7256aa9af2fa403db13f91d29d481f2b719d422a1263c0",
    "nm/L-lo-sl-20260801.xml": "6eaedd028da0dcc2c3406fdcb9cf2f8f76a5c5f8749fd3e3ee4a94192d607a22",
    "nm/L-lo-lp-20260801.xml": "113c9e52ec43eeb145b76cfaacb1343a0985ade9761a3c65c9a47c9aa28fddd9",
    "nm/L-lo-rp-20260801.xml": "df4ba1cbee04bd4ebdf2709c75d98b7305a751ddef454bce27c6269a36d8e78d",
    "nm/L-lo-ap-20260801.xml": "946db783e4ecff75123047ebe9541e401071c44e8e25dda707e491a4187fcfb5",
    "nm/L-lo-op-20260801.xml": "89caf21e6bdd4289fc854443e4d9189d6407fd08329174c33ba1d9eb6d3b2261",
    "nm/L-lo-dd-20260801.xml": "c63d4b7a8694268888388607dcd013365813cec28495a3a3d54ce4706e5f7901",
    "nm/L-lo-sc-20260801.xml": "6c45cc1af9d6ee43373434cf63dc097ee9b5a837ce52015fac5e17c7d2867245",
    # NC: row 2480-NS's names (lab/2480-ns-sheet at 6e44a3dc), kept in ns/, never sent.
    "ns/names.tsv": "5c269be45db6485472ab50da1a8142fed7d6df08119ceddcc4e9a3a5b2ea3552",
}
PB_NAME = "BRIDGE Walk-in BC"  # PB's customer, in pb/L-ledger.xml and pb/I-0099.xml
AMOUNT = "-1051.00"  # the bill, as PB's invoice makes it
def load(name):
    raw = (here / name).read_bytes()
    assert name in TEMPLATES, name
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

def table(raw):
    return [line.split("\t") for line in raw.split("\n") if line]
def key(value):
    # NS: a Python reading of master's letters_and_digits_key (bridge-tally-core/src/master_binding.rs): letters and
    # digits only, upper-cased then lower-cased. str.isalnum keeps fewer marks than Rust's is_alphanumeric (a
    # Devanagari vowel sign is dropped here), so this key can only find more collisions than master's, never fewer.
    return "".join(c for c in value if c.isalnum()).upper().lower()

# NS: the turn that makes this row's L-lo files gives each of NM's own eleven from NM's L-lo-pl, byte for byte.
nm_raw = (here / "nm/names.tsv").read_bytes()
assert hashlib.sha256(nm_raw).hexdigest() == TEMPLATES["nm/names.tsv"], "nm/names.tsv is not NM's"
nm_rows = table(nm_raw.decode("utf-8"))
NM_PLAIN = "BRIDGE Nm Plain"
assert nm_rows[0][0] == "pl" and nm_rows[0][2] == NM_PLAIN and len(nm_rows) == 11
lo_plain = load("nm/L-lo-pl-20260801.xml")
def report(name):
    return swap(lo_plain, f">{escape(NM_PLAIN)}<", f">{escape(name)}<", 1)
for ident, _, name in nm_rows:
    assert encode(report(name)) == (here / f"nm/L-lo-{ident}-20260801.xml").read_bytes(), f"NM's L-lo-{ident} differs"

rows = table((here / "names.tsv").read_text("utf-8"))
assert len(rows) == 11 and len({r[0] for r in rows}) == 11 and len({r[1] for r in rows}) == 11
assert rows[0][0] == "pl" and rows[0][2] == "BRIDGE Nc Plain", "the control comes first"  # NC: the ids are this row's
assert len({r[2] for r in rows}) == 11 and all(r[2].startswith("BRIDGE Nc ") for r in rows)
# NC: NS's names, checked as NM's are.
ns_raw = (here / "ns/names.tsv").read_bytes()
assert hashlib.sha256(ns_raw).hexdigest() == TEMPLATES["ns/names.tsv"], "ns/names.tsv is not NS's"
ns_rows = table(ns_raw.decode("utf-8"))
assert len(ns_rows) == 11 and all(r[2].startswith("BRIDGE Ns ") for r in ns_rows)
# NC: each name but the control holds two or more of the marks NS measured one at a time, and no other mark.
MARKS = "-.&/()'"
for ident, _, name in rows[1:]:
    tail_of = name[len("BRIDGE Nc "):]
    held = {c for c in tail_of if c in MARKS} | ({"and"} if " and " in f" {tail_of} " else set())
    held |= {"digits-dash"} if any(tail_of[i].isdigit() and tail_of[i + 1] == "-" for i in range(len(tail_of) - 1)) else set()
    held |= {"devanagari"} if any("\u0900" <= c <= "\u097f" for c in tail_of) else set()
    assert len(held) >= 2, (ident, held)
    other = {c for c in tail_of if not (c.isalnum() or c == " " or c in MARKS or "\u0900" <= c <= "\u097f")}
    assert not other, (ident, other)
# NC: no two names of this row, and no name of this row and one of NM's or NS's, share a key.
keys = [key(r[2]) for r in rows]
assert len(set(keys)) == 11, keys
assert not set(keys) & {key(r[2]) for r in nm_rows + ns_rows}, "a name folds onto one of NM's or NS's"
assert not {r[1] for r in rows} & {r[1] for r in nm_rows + ns_rows}, "an invoice number is NM's or NS's"
for index, (ident, number, name) in enumerate(rows, 1):
    tail = f"e{0x40 + index:02x}"  # NC: 0e41 to 0e4b; NS sent 0e21 to 0e2b, NM 0e01 to 0e0b, and PB, TD and RS use 777, 795, d..
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
    text = report(name)  # NS, kept by NC
    assert text.count(f">{escape(name)}<") == 1 and swap(text, f">{escape(name)}<", f">{escape(NM_PLAIN)}<", 1) == lo_plain
    save(f"L-lo-{ident}-20260801.xml", text)
