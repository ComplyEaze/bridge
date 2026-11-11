#!/usr/bin/env python3
# gen-sk.py: writes row 2480-SK's requests beside this file, UTF-16LE with a BOM, as sent. New for this row. Every
# request is made from a file another row sent, by swapping the control's name (and, for an import, its invoice number
# and REMOTEID tail) and nothing else; each swap's count is asserted, and swapped back it must give the source byte for
# byte. It checks each source's sha256 first and writes nothing until every check has passed.
#   Sources, never sent:
#     mx/   row 2480-MX's control files and row 2480-MW's voucher reads of it (lab/2480-mx-sheet at 080692b7 and
#           lab/2480-mw-sheet at a7bf1de3; the same bytes in both), and MX's names.tsv.
#     cv/   row 2480-CV's census requests (lab/2480-cv-sheet at 9de0dee1): the book-wide census and the control's two forms.
#     fx/   master's standard ledger catalogue request as the lab sent it to BRIDGE SHAPE LAB
#           (bridge-tally-protocol/tests/fixtures/agent/shape-lab-standard-ledger-catalogue.request.utf16le.xml,
#           which agent_voucher_groups_tests.rs:100 pins to standard_ledger_catalog_read).
#     nm/ ns/ nc/ nx/  the names of rows 2480-NM, NS, NC and NX.
#     keys-master.txt  master's letters_and_digits_key (bridge-tally-core/src/master_binding.rs:1982 at 7a3c9e49) run
#           on every name of those rows, of MX and of this row, by keys-test.rs (ROW.md, Files), a non-ASCII character
#           of a key written as its \u{..} escape (one to one). This script never computes a key itself: it reads
#           master's, and only compares keys for equality.
#   Written:
#     the control pl: MX's LM-pl, I-pl, N-0281 and L-lo-pl and MW's Vb-book, Vm-u-pl, Vm-v-pl, Vw-u-pl and Vw-v-pl,
#       byte for byte; C-u-pl and C-v-pl, CV's control census forms with CV's control name turned into MX's;
#     for each other id: LM-<id>, I-<id>, N-<number>, L-lo-<id>-20260801, C-v-<id>, Vm-v-<id> and Vw-v-<id>, each the
#       control's file with the control's name (number, tail) turned into the id's;
#     C-book (CV's, byte for byte) and G-cat (master's catalogue request with BRIDGE SHAPE LAB turned into BRIDGE PILOT
#       LAB, and nothing else).
# Example: python3 -I gen-sk.py
import hashlib
import pathlib
from decimal import Decimal
import xml.etree.ElementTree as ET
here = pathlib.Path(__file__).resolve().parent
SOURCES = {  # each sha256 as the SHA256.txt of the sheet it comes from lists it, or master's PROVENANCE (fx/)
    "mx/names.tsv": "9fb99798cb42a2f471ab8c9332c7abfc7659ec4f5db5b04c4e19e9fc49ddd6b3",
    "mx/LM-pl.xml": "9bb845b7639e37e0b501885b4dd1eb494b49dd62e0691cc0032950bfcaa92143",
    "mx/I-pl.xml": "ade6eb8e1c824e5aa03ba0b4c9dd553cdf6845a5131c654768bf6e9c8dbbeae9",
    "mx/N-0281.xml": "78bd051c49e09648dc2dca8f7f9a2c536ad98865cd0c6aa510f4bf486914e095",
    "mx/L-lo-pl-20260801.xml": "750c0eb988e5e638fb640b6187354f50ce25d9f8e2b3e77a9ddde9331c5d61a3",
    "mx/Vb-book.xml": "ce457756adfde498ce9548e6fca463893c5015de20db0dd862b14cf1ed4d40e3",
    "mx/Vm-u-pl.xml": "54eb2a7f1a6ca1a1380f75073bb3b4cefe13ba9349c415d9828c3e6b14d5239e",
    "mx/Vm-v-pl.xml": "79f7e52e1e5bbc311e07488f2908b7a0810bd0572a24b7a0f07ca3d1a4cde4a6",
    "mx/Vw-u-pl.xml": "56363d5694758403cbabb8899634e9a9d9af6f6968aeaec038684efd0867f3a3",
    "mx/Vw-v-pl.xml": "2ee6a6f2f4fac1d5ae109a0532b907acbe77ddcbf48388d8eca1f339bf724838",
    "cv/C-book.xml": "0994e5999e334dbcd96a81234b527b423d0b0388a3b78faa4da981af1b2c6496",
    "cv/C-u-pl.xml": "9bfcd0f79c7406be3a16c67b85563af8b9572a29c2bc965743bd97d008e3bc90",
    "cv/C-v-pl.xml": "8c5cfd76a70794aa9908b5717754f8e47eba6171fa29983fda099ce7b7e1ee37",
    "fx/shape-lab-standard-ledger-catalogue.request.utf16le.xml":
        "827f126cefc90cdaf4e15f41208bb5361458a87c3d8facfe3075bc6285c3dc28",
    "nm/names.tsv": "24d3c7f71d092a9bdf48c6fd3a703ba171c0096fef2cdbf75503c0d43720b4a5",
    "ns/names.tsv": "5c269be45db6485472ab50da1a8142fed7d6df08119ceddcc4e9a3a5b2ea3552",
    "nc/names.tsv": "de0c11a9cd201758b39ff5c6164cbcb5f013ed5a996f6db3ceb4b8ab9a086e21",
    "nx/names.tsv": "fd89bd419dbf1e4fc206ff6d9eec74b78d3e7a799a6ff3d6a6bc596e3d7b2856",
    "keys-master.txt": "fffb3660f4c3473133e6ddb73bac9f76a7912620b7bc967d0474a59bea0717c4",  # keys-test.rs's output
}
MX_PLAIN = "BRIDGE Mx Plain"  # MX's control, this row's control
CV_PLAIN = "BRIDGE Nx Plain"  # CV's control (NX's names)
AMOUNT = "-1051.00"  # the bill, as MX's invoices make it
PAIRS = (("ka", "kb"), ("ra", "rb"), ("qa", "qb"), ("ua", "ub"), ("ea", "eb"))
# The marks each name holds (a mark: neither an ASCII letter or digit nor a space), and how the pair differs.
MARKS = {"ka": "", "kb": ",", "ra": ",", "rb": "", "qa": "", "qb": "'", "ua": "", "ub": "_", "ea": "", "eb": ""}
NM_ALTERED = ".&/()"  # the five marks row 2480-NM saw answered as an alteration; no pair differs by one

def raw_of(name):
    raw = (here / name).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == SOURCES[name], f"{name} is not the file ROW.md names"
    return raw
def text_of(name):
    raw = raw_of(name)
    assert raw[:2] == b"\xff\xfe", name
    return raw[2:].decode("utf-16-le")
def encode(text):
    return b"\xff\xfe" + text.encode("utf-16-le")
def swap(text, old, new, count):
    assert text.count(old) == count, (old, text.count(old), count)
    assert new == old or text.count(new) == 0, (new, "already in the source")
    return text.replace(old, new)
def escape(value):
    # master's xml_text::escape_text (bridge-tally-protocol/src/xml_text.rs), as gen-mx.py writes it.
    return (value.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace('"', "&quot;")
            .replace("'", "&apos;").replace("\r", "&#13;").replace("\n", "&#10;"))
def table(raw):
    return [line.split("\t") for line in raw.decode("utf-8").split("\n") if line]

# The names: this row's, MX's (the control is MX's control), and master's keys of every row's names.
rows = table((here / "names.tsv").read_bytes())
ids = [r[0] for r in rows]
assert ids == ["pl"] + [i for pair in PAIRS for i in pair], ids
assert rows[0] == ["pl", "0281", MX_PLAIN], rows[0]
mx_rows = table(raw_of("mx/names.tsv"))
assert mx_rows[0] == ["pl", "0281", MX_PLAIN] and len(mx_rows) == 11
earlier = {tag: table(raw_of(f"{tag}/names.tsv")) for tag in ("nm", "ns", "nc", "nx")}
earlier["mx"] = mx_rows
assert all(len(v) == 11 for v in earlier.values())
keys = {}
seen = {tag: [] for tag in ("nm", "ns", "nc", "nx", "mx", "sk")}
for line in raw_of("keys-master.txt").decode("utf-8").split("\n"):
    if not line:
        continue
    tag, ident, number, name, key = line.split("\t")
    seen[tag].append([ident, number, name])
    keys[(tag, ident)] = key
# keys-master.txt holds exactly every row's names, in their order, each with master's key.
for tag, want in list(earlier.items()) + [("sk", rows)]:
    assert seen[tag] == want, f"keys-master.txt's {tag} lines are not that row's names.tsv"
key = {ident: keys[("sk", ident)] for ident in ids}
# Each pair shares one key by master's function; no two pairs share one; the control's key is its own.
for a, b in PAIRS:
    assert key[a] == key[b], (a, b, "the pair does not share master's key")
pair_keys = [key[a] for a, _ in PAIRS]
assert len(set(pair_keys)) == len(PAIRS), "two pairs share a key"
assert key["pl"] not in pair_keys
# No pair's key is the key of any name of NM, NS, NC, NX or MX; the control's is the key of MX's control alone.
others = {k for (tag, ident), k in keys.items() if tag != "sk"}
assert not set(pair_keys) & others, "a pair's key is an earlier row's"
assert [i for (tag, i), k in keys.items() if tag != "sk" and k == key["pl"]] == ["pl"], "the control's key"
# The names themselves: no double or edge space; each holds exactly its marks; within a pair the letters and digits
# are the same, in the same case and order (so no pair differs by case), and no pair differs by one of NM's five.
numbers = {r[0]: r[1] for r in rows}
names = {r[0]: r[2] for r in rows}
for ident in ids[1:]:
    name = names[ident]
    assert name.startswith("BRIDGE Sk ") and name == name.strip() and "  " not in name, name
    held = "".join(c for c in name if not ((c.isascii() and c.isalnum()) or c == " "))
    assert held == MARKS[ident], (ident, held)
    assert all(c.isascii() for c in name), name
for a, b in PAIRS:
    alnum = lambda s: [c for c in s if c.isalnum()]
    assert alnum(names[a]) == alnum(names[b]) and names[a] != names[b], (a, b)
    assert not set(MARKS[a] + MARKS[b]) & set(NM_ALTERED), (a, b)
assert len(set(names.values())) == 11 and len(set(numbers.values())) == 11
assert not set(names.values()) & {r[2] for v in earlier.values() for r in v} - {MX_PLAIN}
assert not set(numbers.values()) - {"0281"} & {r[1] for v in earlier.values() for r in v}
assert [numbers[i] for i in ids[1:]] == [f"{n:04d}" for n in range(292, 302)]

# The control's files, MX's and MW's, byte for byte.
out = {}
for name in ("LM-pl.xml", "I-pl.xml", "N-0281.xml", "L-lo-pl-20260801.xml", "Vb-book.xml", "Vm-u-pl.xml",
             "Vm-v-pl.xml", "Vw-u-pl.xml", "Vw-v-pl.xml"):
    out[name] = raw_of(f"mx/{name}")
out["C-book.xml"] = raw_of("cv/C-book.xml")
lm, inv, num = text_of("mx/LM-pl.xml"), text_of("mx/I-pl.xml"), text_of("mx/N-0281.xml")
lo, vm, vw = text_of("mx/L-lo-pl-20260801.xml"), text_of("mx/Vm-v-pl.xml"), text_of("mx/Vw-v-pl.xml")
# The census of the control: CV's two forms with CV's control name turned into MX's, and nothing else.
for form, count in (("C-u-pl.xml", 1), ("C-v-pl.xml", 1)):
    text = swap(text_of(f"cv/{form}"), CV_PLAIN, MX_PLAIN, count)
    assert swap(text, MX_PLAIN, CV_PLAIN, count) == text_of(f"cv/{form}")
    out[form] = encode(text)
cv_text = out["C-v-pl.xml"][2:].decode("utf-16-le")
assert "<CHILDOF>##LedgerName</CHILDOF>" in cv_text and f"<LedgerName>{MX_PLAIN}</LedgerName>" in cv_text
assert f"<CHILDOF>{MX_PLAIN}</CHILDOF>" in out["C-u-pl.xml"][2:].decode("utf-16-le")
# The catalogue: master's request with this book's name, and nothing else.
cat = text_of("fx/shape-lab-standard-ledger-catalogue.request.utf16le.xml")
cat_here = swap(cat, "<SVCURRENTCOMPANY>BRIDGE SHAPE LAB</SVCURRENTCOMPANY>",
                "<SVCURRENTCOMPANY>BRIDGE PILOT LAB</SVCURRENTCOMPANY>", 1)
assert cat_here.count("SHAPE LAB") == 0 and cat_here.count("PILOT LAB") == 1
out["G-cat.xml"] = encode(cat_here)

def voucher_of(text):
    return ET.fromstring(text.split("?>", 1)[1]).find("BODY/IMPORTDATA/REQUESTDATA/TALLYMESSAGE/VOUCHER")
def check_invoice(text, number, tail, name):  # gen-mx.py's checks, unchanged
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
check_ledger(lm, MX_PLAIN)
check_invoice(inv, "0281", "e81", MX_PLAIN)

for index, ident in enumerate(ids[1:], 2):
    name, number, tail = names[ident], numbers[ident], f"e{0xc0 + index:02x}"  # ec2 to ecb; MX sent e81 to e8b
    e = escape(name)
    text = swap(lm, MX_PLAIN, e, 2)
    check_ledger(text, name)
    assert swap(text, e, MX_PLAIN, 2) == lm
    out[f"LM-{ident}.xml"] = encode(text)
    text = swap(swap(swap(inv, MX_PLAIN, e, 7), "BP/26-27/0281", f"BP/26-27/{number}", 3),
                "000000000e81", f"000000000{tail}", 1)
    check_invoice(text, number, tail, name)
    assert swap(swap(swap(text, e, MX_PLAIN, 7), f"BP/26-27/{number}", "BP/26-27/0281", 3),
                f"000000000{tail}", "000000000e81", 1) == inv
    out[f"I-{ident}.xml"] = encode(text)
    out[f"N-{number}.xml"] = encode(swap(num, '"BP/26-27/0281"', f'"BP/26-27/{number}"', 1))
    for file, source, old in ((f"L-lo-{ident}-20260801.xml", lo, f">{MX_PLAIN}<"),
                              (f"C-v-{ident}.xml", cv_text, f"<LedgerName>{MX_PLAIN}</LedgerName>"),
                              (f"Vm-v-{ident}.xml", vm, f"<LedgerName>{MX_PLAIN}</LedgerName>"),
                              (f"Vw-v-{ident}.xml", vw, f"<LedgerName>{MX_PLAIN}</LedgerName>")):
        new = old.replace(MX_PLAIN, e)
        text = swap(source, old, new, 1)
        assert text.count(e) == 1 and swap(text, new, old, 1) == source
        out[file] = encode(text)

for name, raw in sorted(out.items()):
    (here / name).write_bytes(raw)
    print(f"{name}: {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}")
print(f"{len(out)} files")
