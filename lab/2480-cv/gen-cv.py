#!/usr/bin/env python3
# gen-cv.py: writes the census requests of row 2480-CV beside this file, UTF-16LE with a BOM, as sent. New for this row.
# Every other request of the row (LM-, I-, N-, M-marks, B-bills, S-snap) is row 2480-NX's file, byte for byte; this
# script checks each against NX's SHA256.txt (kept in nx/) before it writes anything.
#
# The source of every census request is master's own: two requests the lab sent, which master keeps as captures and
# which master's builders render byte for byte (lines at master 812d39bb):
#   fx/census-dn-party-year-span1.request.utf16le.xml   render_agent_ledger_census, pinned by
#                 agent_voucher_window_tests.rs:8829 (the ledger census, CHILDOF one ledger, span (0, 8192]);
#   fx/pilot-lab-audit-census-spanned.request.utf16le.xml   the book-wide census (render_agent_voucher_census) the lab
#                 sent to BRIDGE PILOT LAB for #2145, span (0, 68].
# It carves each into a template with five holes (company, from, to, span end, scope) and asserts:
#   - each template, filled with its capture's own values, gives that capture byte for byte;
#   - the ledger template with the book scope, filled with the pilot capture's values, gives the pilot capture byte
#     for byte. So the two scopes differ in the collection name and the TYPE/CHILDOF elements alone, as
#     VoucherScope::collection (agent_read_profiles.rs:279) says, and the book request below is master's.
#
# Written here (company BRIDGE PILOT LAB, 1-Apr-2026 to 1-Aug-2026, span (0, 8192]):
#   C-book.xml      the book-wide census.
#   C-u-pl.xml      master's ledger census of the control, <CHILDOF>BRIDGE Nx Plain</CHILDOF>: the one unquoted send.
#   C-uv-pl.xml     C-u-pl.xml with <LedgerName>BRIDGE Nx Plain</LedgerName> after SVCURRENTCOMPANY, and nothing else.
#   C-v-<id>.xml    C-uv with <CHILDOF>##LedgerName</CHILDOF> and the name of <id> in LedgerName, for each of the eleven.
#                   The name is the escaped text of NX's own L-lo-<id> request (nx/), byte for byte, so each name is
#                   escaped once as XML text exactly as rows NS, NC and NX sent it to Ledger Outstandings.
# No name ever enters a formula: in C-v the CHILDOF formula is the fixed text ##LedgerName.
# Example: python3 -I gen-cv.py
import hashlib
import pathlib
import re
import xml.etree.ElementTree as ET
here = pathlib.Path(__file__).resolve().parent
SOURCES = {
    "fx/census-dn-party-year-span1.request.utf16le.xml": "a9d3eaff4191aa04c62987f3a27bc0c4ee70922ac0b99d1bef5b867e30e0ae2f",
    "fx/pilot-lab-audit-census-spanned.request.utf16le.xml": "1125182a51310f09ea6d3c4a08dd4e0d411d13f1fc34f01b89be4ea86f729e4e",
    # Row 2480-NX's SHA256.txt (lab/2480-nx-sheet at ea84bcdf), never sent.
    "nx/SHA256.txt": "cb15e0eb4e53b02e8ae8235d3241a2845f88ff368f031a1e56beac6501bf6a94",
}
COMPANY = "BRIDGE PILOT LAB"
FROM, TO = "20260401", "20260801"
SPAN_END = 8192
LEDGER_SCOPE = ("Bridge Agent Ledger Census", "<TYPE>Vouchers : Ledger</TYPE><CHILDOF>{childof}</CHILDOF>")
BOOK_SCOPE = ("Bridge Agent Voucher Census", "<TYPE>Voucher</TYPE>")
VARIABLE = "LedgerName"  # the element rows NS, NC and NX sent to Ledger Outstandings
FIXED_FORMULA = "##" + VARIABLE

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

def carve(text, company, start, end, span_end, collection, scope):
    """The capture with its five values turned into holes; each value must occur exactly where master puts it."""
    t = once(text, f"<SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY>", "<SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY>")
    t = once(t, f'<SVFROMDATE TYPE="Date">{start}</SVFROMDATE><SVTODATE TYPE="Date">{end}</SVTODATE>',
             '<SVFROMDATE TYPE="Date">{start}</SVFROMDATE><SVTODATE TYPE="Date">{end}</SVTODATE>')
    t = once(t, f'$Date &gt;= $$Date:"{start}" AND $Date &lt;= $$Date:"{end}" AND $AlterID &gt; 0 AND $AlterID &lt;= {span_end}<',
             '$Date &gt;= $$Date:"{start}" AND $Date &lt;= $$Date:"{end}" AND $AlterID &gt; 0 AND $AlterID &lt;= {span_end}<')
    t = once(t, f"<ID>{collection}</ID>", "<ID>{collection}</ID>")
    t = once(t, f'<COLLECTION NAME="{collection}" ISMODIFY="No">{scope}<FETCH>',
             '<COLLECTION NAME="{collection}" ISMODIFY="No">{scope}<FETCH>')
    assert "{" not in t.replace("{company}", "").replace("{start}", "").replace("{end}", "").replace(
        "{span_end}", "").replace("{collection}", "").replace("{scope}", ""), "a brace in the capture"
    return t

def fill(template, company, start, end, span_end, collection, scope):
    return template.format(company=company, start=start, end=end, span_end=span_end, collection=collection, scope=scope)

def checked_nx_files():
    """NX's SHA256.txt as {path: (sha256, size)}; every LM-, I-, N- and sent read file here must match it."""
    listed = {}
    for line in raw_of("nx/SHA256.txt").decode("ascii").splitlines():
        sha, size, path = line.split("  ")
        listed[path] = (sha, int(size))
    return listed

# 1. The two captures, and the template each gives.
ledger_text = text_of(raw_of("fx/census-dn-party-year-span1.request.utf16le.xml"), "ledger capture")
book_text = text_of(raw_of("fx/pilot-lab-audit-census-spanned.request.utf16le.xml"), "book capture")
ledger_scope = LEDGER_SCOPE[1].format(childof="DN Party 001")
ledger_t = carve(ledger_text, "BRIDGE CORPUS DENSE", "20250401", "20260331", 8192, LEDGER_SCOPE[0], ledger_scope)
book_t = carve(book_text, "BRIDGE PILOT LAB", "20260401", "20270331", 68, BOOK_SCOPE[0], BOOK_SCOPE[1])
assert fill(ledger_t, "BRIDGE CORPUS DENSE", "20250401", "20260331", 8192, LEDGER_SCOPE[0], ledger_scope) == ledger_text
assert fill(book_t, "BRIDGE PILOT LAB", "20260401", "20270331", 68, *BOOK_SCOPE) == book_text
assert ledger_t == book_t, "the two captures differ outside the scope: master's two scopes are not one template"
assert fill(ledger_t, "BRIDGE PILOT LAB", "20260401", "20270331", 68, *BOOK_SCOPE) == book_text
template = ledger_t

# 2. NX's files, byte for byte.
nx = checked_nx_files()
names = [line.split("\t") for line in (here / "names.tsv").read_text(encoding="utf-8").splitlines()]
assert [n[0] for n in names] == ["pl", "cm", "us", "dq", "ps", "at", "hs", "cl", "pc", "st", "ex"], "names.tsv is not NX's"
same = [".gitattributes", "LAB-COMMON.md", "run.py", "count20.py", "nm.py", "st.py", "rf.py", "M-marks.xml",
        "B-bills.xml", "S-snap-20260401-20260801.xml", "names.tsv"]
same += [f"{kind}-{i}.xml" for i, _, _ in names for kind in ("LM", "I")] + [f"N-{num}.xml" for _, num, _ in names]
for path in same:
    raw = (here / path).read_bytes()
    assert (hashlib.sha256(raw).hexdigest(), len(raw)) == nx[path], f"{path} is not NX's file"
escaped = {}
for i, _, name in names:
    path = f"L-lo-{i}-20260801.xml"
    raw = (here / "nx" / path).read_bytes()
    assert (hashlib.sha256(raw).hexdigest(), len(raw)) == nx[path], f"nx/{path} is not NX's file"
    found = re.findall(r"<LedgerName>(.*?)</LedgerName>", text_of(raw, path))
    assert len(found) == 1, path
    assert ET.fromstring(f"<x>{found[0]}</x>").text == name, f"{path}'s escaped name is not {name!r}"
    escaped[i] = found[0]
print(f"sources checked: two captures, {len(same)} of NX's files and its eleven L-lo files, byte for byte")

# 3. The requests.
def ledger_census(childof, variable_text=None):
    text = fill(template, COMPANY, FROM, TO, SPAN_END, LEDGER_SCOPE[0], LEDGER_SCOPE[1].format(childof=childof))
    if variable_text is not None:
        text = once(text, "</SVCURRENTCOMPANY>", f"</SVCURRENTCOMPANY><{VARIABLE}>{variable_text}</{VARIABLE}>")
    return text

out = {
    "C-book.xml": fill(template, COMPANY, FROM, TO, SPAN_END, *BOOK_SCOPE),
    "C-u-pl.xml": ledger_census(escaped["pl"]),
    "C-uv-pl.xml": ledger_census(escaped["pl"], escaped["pl"]),
}
for i, _, _ in names:
    out[f"C-v-{i}.xml"] = ledger_census(FIXED_FORMULA, escaped[i])

# 4. Read every request back by an XML parser before writing any.
plain = dict((i, n) for i, _, n in names)["pl"]
assert escaped["pl"] == plain and re.fullmatch(r"[A-Za-z0-9]+( [A-Za-z0-9]+)*", plain), "the control is not plain"
for path, text in out.items():
    root = ET.fromstring(text)
    childof = [e.text for e in root.iter("CHILDOF")]
    variable = [e.text for e in root.iter(VARIABLE)]
    types = [e.text for e in root.iter("TYPE")]
    assert [e.text for e in root.iter("SVCURRENTCOMPANY")] == [COMPANY], path
    if path == "C-book.xml":
        assert childof == [] and variable == [] and types == ["Collection", "Voucher"], path
    elif path == "C-u-pl.xml":
        assert childof == [plain] and variable == [], path
    elif path == "C-uv-pl.xml":
        assert childof == [plain] and variable == [plain], path
    else:
        i = path[len("C-v-"):-len(".xml")]
        assert childof == [FIXED_FORMULA] and variable == [dict((a, n) for a, _, n in names)[i]], path
    if path != "C-book.xml":
        assert types == ["Collection", "Vouchers : Ledger"], path
# C-uv differs from C-u only by the variable, and C-v-pl from C-uv only by the CHILDOF text.
assert out["C-uv-pl.xml"].replace(f"<{VARIABLE}>{plain}</{VARIABLE}>", "", 1) == out["C-u-pl.xml"]
assert out["C-v-pl.xml"] == once(out["C-uv-pl.xml"], f"<CHILDOF>{plain}</CHILDOF>", f"<CHILDOF>{FIXED_FORMULA}</CHILDOF>")
# Each C-v request differs from C-v-pl only by the escaped name in the variable.
for i in escaped:
    assert out[f"C-v-{i}.xml"] == once(out["C-v-pl.xml"], f"<{VARIABLE}>{plain}<", f"<{VARIABLE}>{escaped[i]}<"), i
for path, text in out.items():
    raw = encode(text)
    (here / path).write_bytes(raw)
    print(f"{path}: {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}")
