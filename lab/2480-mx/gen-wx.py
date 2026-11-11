#!/usr/bin/env python3
# gen-wx.py: writes part B's requests of row 2480-MX beside this file, UTF-16LE with a BOM, as sent. New for this row,
# in the manner of row 2480-CV's gen-cv.py (lab/2480-cv-sheet at 9de0dee1). Run it after gen-mx.py: it takes each
# escaped name from this row's own L-lo-<id>-20260801.xml, the text rows NS, NC, NX and CV sent as the LedgerName
# static variable, and checks it against names.tsv.
#
# The source of every request is master's own: three requests the lab sent, which master keeps as captures (lines at
# master 812d39bb):
#   fx/movement-dn-party-month.request.utf16le.xml   render_agent_ledger_vouchers with LedgerVoucherFields::Movement,
#                 pinned byte for byte by agent_voucher_window_tests.rs:8842 (the movement read, CHILDOF one ledger);
#   fx/billtrail-ledger-p1-fy-entries.request.utf16le.xml   the ledger-scoped read with the window field list
#                 (LedgerVoucherFields::Window), pinned byte for byte by agent_voucher_window_tests.rs:9510, the
#                 outstandings party detail's read (#1356);
#   fx/pilot-lab-audit-vouchers.request.utf16le.xml   the book-wide window read (render_agent_vouchers, the field list
#                 AGENT_VOUCHER_FETCH) the lab sent to BRIDGE PILOT LAB itself for item 697-S0A.
# It carves each into a template with six holes (company, from, to, collection, scope, fetch) and asserts:
#   - each template, filled with its capture's own values, gives that capture byte for byte;
#   - the three templates are one: the three requests differ in those six values alone, as render_scoped_vouchers
#     (agent_read_profiles.rs:385) says, so the book-wide request below is master's;
#   - each capture's field list is master's constant for it (agent_read_profiles.rs:48, :80 and :96).
#
# Written here (company BRIDGE PILOT LAB, the one day 1-Aug-2026: from 20260801, day 1, to 20260801):
#   Vb-book.xml        the book-wide window read of that day: the ground truth part B compares with.
#   Vm-u-pl.xml        master's movement read of the control, <CHILDOF>BRIDGE Mx Plain</CHILDOF>.
#   Vm-uv-pl.xml       Vm-u-pl.xml with <LedgerName>BRIDGE Mx Plain</LedgerName> after SVCURRENTCOMPANY, and nothing else.
#   Vm-v-<id>.xml      Vm-uv-pl.xml with <CHILDOF>##LedgerName</CHILDOF> and the name of <id> in LedgerName.
#   Vw-u-pl.xml, Vw-uv-pl.xml, Vw-v-<id>.xml   the same three forms of the read with the window field list.
# No name ever enters a formula: in Vm-v and Vw-v the CHILDOF formula is the fixed text ##LedgerName.
# Example: python3 -I gen-wx.py
import hashlib
import pathlib
import re
import xml.etree.ElementTree as ET
here = pathlib.Path(__file__).resolve().parent
SOURCES = {
    "fx/movement-dn-party-month.request.utf16le.xml": "8060720ed68c5100600098959bfa68708fa72b07bf78589ec839411c3ce8af41",
    "fx/billtrail-ledger-p1-fy-entries.request.utf16le.xml": "6a5ee4a9ec54980cd8b7c0cc137d6e7d1f42c1ce1a700670cbf3fcbb996da97d",
    "fx/pilot-lab-audit-vouchers.request.utf16le.xml": "1680d37c06f2975b79ac66b1df18f7886dadc6658269d423ef7e2827e83d8a70",
}
# master's three field lists, agent_read_profiles.rs at 812d39bb
AGENT_VOUCHER_FETCH = ("DATE,VOUCHERNUMBER,VOUCHERTYPENAME,PARTYLEDGERNAME,NARRATION,GUID,ALTERID,MASTERID,ISCANCELLED,"
                       "ISOPTIONAL,ISPOSTDATED,ALLLEDGERENTRIES.*,REFERENCE,ISINVOICE,PARTYGSTIN,REFERENCEDATE")  # :48
AGENT_LEDGER_WINDOW_FETCH = ("DATE,VOUCHERNUMBER,VOUCHERTYPENAME,PARTYLEDGERNAME,NARRATION,GUID,ALTERID,MASTERID,"
                             "ISCANCELLED,ISOPTIONAL,ISPOSTDATED,ALLLEDGERENTRIES.*,REFERENCE,ISINVOICE,PARTYGSTIN")  # :80
AGENT_MOVEMENT_FETCH = ("DATE,VOUCHERNUMBER,VOUCHERTYPENAME,PARTYLEDGERNAME,NARRATION,GUID,ALTERID,MASTERID,ISCANCELLED,"
                        "ISOPTIONAL,ALLLEDGERENTRIES.LEDGERNAME,ALLLEDGERENTRIES.AMOUNT,"
                        "ALLLEDGERENTRIES.ISDEEMEDPOSITIVE")  # :96
LEDGER = "Bridge Agent Ledger Vouchers"
BOOK = "Bridge Agent Vouchers"
LEDGER_SCOPE = "<TYPE>Vouchers : Ledger</TYPE><CHILDOF>{childof}</CHILDOF>"
BOOK_SCOPE = "<TYPE>Voucher</TYPE>"
COMPANY = "BRIDGE PILOT LAB"
# The one day 1-Aug-2026, which holds every invoice of this row. The window starts on day 1 of the month and holds no
# day outside it. The whole month is not read: by master's own capture of this book (pilot-lab-audit-vouchers.utf16le
# .xml), its vouchers of August come to more than the 2,000,000-byte read bound before this row adds any.
FROM, TO = "20260801", "20260801"
VARIABLE = "LedgerName"  # the element rows NS, NC, NX and CV sent
FIXED_FORMULA = "##" + VARIABLE
NAME_PREFIX = "BRIDGE Mx "

def raw_of(name):
    raw = (here / name).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == SOURCES[name], f"{name} is not the file ROW.md names"
    return raw

def text_of(raw, name):
    assert raw[:2] == b"\xff\xfe", f"{name} has no UTF-16LE BOM"
    return raw[2:].decode("utf-16-le")

def encode(text):
    return b"\xff\xfe" + text.encode("utf-16-le")

def once(text, old, new):
    assert text.count(old) == 1, f"{old!r} occurs {text.count(old)} times"
    return text.replace(old, new)

def escape(value):
    # master's xml_text::escape_text, as gen-mx.py has it
    return (value.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace('"', "&quot;")
            .replace("'", "&apos;").replace("\r", "&#13;").replace("\n", "&#10;"))

HOLES = ("company", "start", "end", "collection", "scope", "fetch")

def carve(text, company, start, end, collection, scope, fetch):
    """The capture with its six values turned into holes; each value must occur exactly where master puts it."""
    assert "{" not in text and "}" not in text, "a brace in the capture"
    t = once(text, f"<SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY>", "<SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY>")
    t = once(t, f'<SVFROMDATE TYPE="Date">{start}</SVFROMDATE><SVTODATE TYPE="Date">{end}</SVTODATE>',
             '<SVFROMDATE TYPE="Date">{start}</SVFROMDATE><SVTODATE TYPE="Date">{end}</SVTODATE>')
    t = once(t, f'$Date &gt;= $$Date:"{start}" AND $Date &lt;= $$Date:"{end}"</SYSTEM>',
             '$Date &gt;= $$Date:"{start}" AND $Date &lt;= $$Date:"{end}"</SYSTEM>')
    t = once(t, f"<ID>{collection}</ID>", "<ID>{collection}</ID>")
    t = once(t, f'<COLLECTION NAME="{collection}" ISMODIFY="No">{scope}<FETCH>{fetch}</FETCH><FILTERS>',
             '<COLLECTION NAME="{collection}" ISMODIFY="No">{scope}<FETCH>{fetch}</FETCH><FILTERS>')
    return t

def fill(template, **values):
    assert set(values) == set(HOLES)
    return template.format(**values)

# 1. The three captures, and the one template they give.
captures = [
    ("fx/movement-dn-party-month.request.utf16le.xml",
     dict(company="BRIDGE CORPUS DENSE", start="20250501", end="20250531", collection=LEDGER,
          scope=LEDGER_SCOPE.format(childof="DN Party 001"), fetch=AGENT_MOVEMENT_FETCH)),
    ("fx/billtrail-ledger-p1-fy-entries.request.utf16le.xml",
     dict(company="BRIDGE BILLTRAIL LAB", start="20250401", end="20260331", collection=LEDGER,
          scope=LEDGER_SCOPE.format(childof="Lab Party P1"), fetch=AGENT_LEDGER_WINDOW_FETCH)),
    ("fx/pilot-lab-audit-vouchers.request.utf16le.xml",
     dict(company=COMPANY, start="20260401", end="20270331", collection=BOOK, scope=BOOK_SCOPE,
          fetch=AGENT_VOUCHER_FETCH)),
]
templates = []
for path, values in captures:
    text = text_of(raw_of(path), path)
    template = carve(text, **values)
    assert fill(template, **values) == text, f"{path}: the template does not give the capture back"
    templates.append(template)
assert templates[0] == templates[1] == templates[2], "the three captures differ outside the six values"
template = templates[0]
print("sources checked: three captures, one template, each capture given back byte for byte")

# 2. The escaped names, from this row's own L-lo files (gen-mx.py).
names = [line.split("\t") for line in (here / "names.tsv").read_text(encoding="utf-8").splitlines()]
ids = [i for i, _, _ in names]
assert len(names) == 11 and ids[0] == "pl" and len(set(ids)) == 11, "names.tsv is not this row's"
name_of = {i: n for i, _, n in names}
escaped = {}
for i, _, name in names:
    assert name.startswith(NAME_PREFIX) and name == name.strip() and "  " not in name, name
    path = f"L-lo-{i}-20260801.xml"
    found = re.findall(r"<LedgerName>(.*?)</LedgerName>", text_of((here / path).read_bytes(), path))
    assert len(found) == 1 and found[0] == escape(name), f"{path} does not carry the escaped name of {i}"
    assert ET.fromstring(f"<x>{found[0]}</x>").text == name, path
    escaped[i] = found[0]
plain = name_of["pl"]
assert escaped["pl"] == plain and re.fullmatch(r"[A-Za-z0-9]+( [A-Za-z0-9]+)*", plain), "the control is not plain"

# 3. The requests.
def request(collection, scope, fetch):
    return fill(template, company=COMPANY, start=FROM, end=TO, collection=collection, scope=scope, fetch=fetch)

def ledger_read(fetch, childof, variable_text=None):
    text = request(LEDGER, LEDGER_SCOPE.format(childof=childof), fetch)
    if variable_text is not None:
        text = once(text, "</SVCURRENTCOMPANY>", f"</SVCURRENTCOMPANY><{VARIABLE}>{variable_text}</{VARIABLE}>")
    return text

FORMS = {"Vm": AGENT_MOVEMENT_FETCH, "Vw": AGENT_LEDGER_WINDOW_FETCH}
out = {"Vb-book.xml": request(BOOK, BOOK_SCOPE, AGENT_VOUCHER_FETCH)}
for form, fetch in FORMS.items():
    out[f"{form}-u-pl.xml"] = ledger_read(fetch, plain)
    out[f"{form}-uv-pl.xml"] = ledger_read(fetch, plain, plain)
    for i in ids:
        out[f"{form}-v-{i}.xml"] = ledger_read(fetch, FIXED_FORMULA, escaped[i])

# 4. Read every request back by an XML parser before writing any.
assert FROM[:6] == TO[:6] == "202608" and FROM[6:] == "01" and FROM <= "20260801" <= TO, "the window"
for path, text in out.items():
    root = ET.fromstring(text)
    one = lambda tag: [e.text for e in root.iter(tag)]
    assert one("SVCURRENTCOMPANY") == [COMPANY] and one("SVFROMDATE") == [FROM] and one("SVTODATE") == [TO], path
    assert one("SYSTEM") == [f'$Date >= $$Date:"{FROM}" AND $Date <= $$Date:"{TO}"'], path
    form = path[:2]
    if path == "Vb-book.xml":
        assert one("TYPE") == ["Collection", "Voucher"] and one("CHILDOF") == [] and one(VARIABLE) == [], path
        assert one("ID") == [BOOK] and one("FETCH") == [AGENT_VOUCHER_FETCH], path
        continue
    assert one("TYPE") == ["Collection", "Vouchers : Ledger"] and one("ID") == [LEDGER], path
    assert one("FETCH") == [FORMS[form]], path
    kind = path[3:].split("-", 1)[0]
    if kind == "u":
        assert one("CHILDOF") == [plain] and one(VARIABLE) == [], path
    elif kind == "uv":
        assert one("CHILDOF") == [plain] and one(VARIABLE) == [plain], path
    else:
        i = path[len("Vm-v-"):-len(".xml")]
        assert kind == "v" and one("CHILDOF") == [FIXED_FORMULA] and one(VARIABLE) == [name_of[i]], path
# The movement form is master's movement read, and the window form master's party detail read, with this book, this
# day and the control's name; each step from one form to the next changes one thing.
for form, fetch in FORMS.items():
    assert out[f"{form}-uv-pl.xml"].replace(f"<{VARIABLE}>{plain}</{VARIABLE}>", "", 1) == out[f"{form}-u-pl.xml"]
    assert out[f"{form}-v-pl.xml"] == once(out[f"{form}-uv-pl.xml"], f"<CHILDOF>{plain}</CHILDOF>",
                                            f"<CHILDOF>{FIXED_FORMULA}</CHILDOF>")
    for i in ids:
        assert out[f"{form}-v-{i}.xml"] == once(out[f"{form}-v-pl.xml"], f"<{VARIABLE}>{plain}<",
                                                f"<{VARIABLE}>{escaped[i]}<"), (form, i)
# The two forms differ in the FETCH element alone (master's test at agent_voucher_window_tests.rs:8859).
for key in [f"u-pl.xml", "uv-pl.xml"] + [f"v-{i}.xml" for i in ids]:
    assert out[f"Vw-{key}"] == once(out[f"Vm-{key}"], f"<FETCH>{AGENT_MOVEMENT_FETCH}</FETCH>",
                                    f"<FETCH>{AGENT_LEDGER_WINDOW_FETCH}</FETCH>"), key
# The book-wide read is master's own form for this book, its window alone changed.
pilot = text_of(raw_of("fx/pilot-lab-audit-vouchers.request.utf16le.xml"), "pilot")
assert out["Vb-book.xml"] == pilot.replace('"20260401"', f'"{FROM}"').replace(">20260401<", f">{FROM}<").replace(
    '"20270331"', f'"{TO}"').replace(">20270331<", f">{TO}<")
assert len(out) == 1 + 2 * (2 + len(ids))
for path, text in out.items():
    raw = encode(text)
    (here / path).write_bytes(raw)
    print(f"{path}: {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}")
