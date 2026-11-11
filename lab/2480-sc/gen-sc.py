#!/usr/bin/env python3
# gen-sc.py: writes row 2480-SC's nine new requests beside this file, UTF-16LE with a BOM, as sent, and checks every
# request of the folder. New for this row. It writes nothing until every check below has passed.
#   1. Every other file of this folder that row 2480-SK's sheet holds is SK's, byte for byte: its sha256 and size as
#      sk/SHA256.txt lists them (that file is SK's SHA256.txt, lab/2480-sk-sheet at c5f9d52f, pinned here).
#   2. scope-master.txt is the output of master's own LedgerScopeName::new (bridge-tally-protocol/src/ledger_scope.rs:65
#      at 7a3c9e49) on every line of names.tsv, by scope-test.rs (ROW.md, Files), pinned here. This script never
#      decides a name's scope itself: it reads master's verdicts.
#   3. The nine new files are master's CHILDOF form of the census (C-u-), the movement read (Vm-u-) and the window read
#      (Vw-u-) for ka, qa and ua: SK's control files C-u-pl, Vm-u-pl and Vw-u-pl with the control's name turned into the
#      id's, and nothing else. Each swap's count is asserted, and swapped back each gives its source byte for byte.
#   4. Every request of the folder, SK's and new, is parsed, and every CHILDOF element in it is read: its text is
#      either ##LedgerName or the name of an id of names.tsv that master's verdict ADMITTED, written in the request
#      exactly as the name (no escape changed it). No name master REFUSED is inside CHILDOF in any file. One line a
#      request is printed.
# Example: python3 -I gen-sc.py
import hashlib
import pathlib
import xml.etree.ElementTree as ET
here = pathlib.Path(__file__).resolve().parent
SK_LIST = "f39ed916e838659853703cd5719442933c68031d32c02c202b3c41c1f19a3971"  # sk/SHA256.txt
SCOPE = "3d0530d04419bf823bd3c867c7a13e68c7261352cfce88213e9a8af6bebcb5f4"  # scope-master.txt
MX_PLAIN = "BRIDGE Mx Plain"
NEW = ("ka", "qa", "ua")  # the names read in master's CHILDOF form, beside the control
FORMS = ("C-u", "Vm-u", "Vw-u")
# The verdicts this sheet is written for. They are checked against master's, never used in place of them.
WANT = {"pl": "ADMITTED", "ka": "ADMITTED", "kb": "REFUSED ledger_scope_name_character",
        "ra": "REFUSED ledger_scope_name_character", "rb": "ADMITTED", "qa": "ADMITTED",
        "qb": "REFUSED ledger_scope_name_character", "ua": "ADMITTED",
        "ub": "REFUSED ledger_scope_name_character", "ea": "ADMITTED", "eb": "ADMITTED"}

def sha(raw):
    return hashlib.sha256(raw).hexdigest()
def swap(text, old, new, count):
    assert text.count(old) == count, (old, text.count(old), count)
    assert text.count(new) == 0, (new, "already in the source")
    return text.replace(old, new)

# 1. SK's files.
listing = (here / "sk" / "SHA256.txt").read_bytes()
assert sha(listing) == SK_LIST, "sk/SHA256.txt is not SK's"
sk = {}
for line in listing.decode("utf-8").split("\n"):
    if line:
        digest, size, name = line.split("  ")
        sk[name] = (digest, int(size))
held = sorted(p.name for p in here.iterdir() if p.is_file())
new_names = {f"{form}-{ident}.xml" for form in FORMS for ident in NEW}
OWN = {"ROW.md", "gen-sc.py", "sc.py", "scope-test.rs", "scope-master.txt", "SHA256.txt"}
for name in held:
    if name in OWN or name in new_names:
        continue
    assert name in sk, f"{name} is neither SK's nor this row's own"
    raw = (here / name).read_bytes()
    assert (sha(raw), len(raw)) == sk[name], f"{name} is not SK's file of that name"
names = [line.split("\t") for line in (here / "names.tsv").read_text("utf-8").split("\n") if line]
name_of = {ident: name for ident, _, name in names}
assert [i for i, _, _ in names] == list(WANT), "names.tsv is not SK's"

# 2. Master's verdicts.
scope_raw = (here / "scope-master.txt").read_bytes()
assert sha(scope_raw) == SCOPE, "scope-master.txt is not the output ROW.md names"
scope_lines = [line.split("\t") for line in scope_raw.decode("utf-8").split("\n") if line]
assert [line[:3] for line in scope_lines] == names, "scope-master.txt's lines are not names.tsv's"
verdict = {line[0]: line[3] for line in scope_lines}
assert verdict == WANT, {i: verdict[i] for i in WANT if verdict[i] != WANT[i]}
admitted = {name_of[i] for i, v in verdict.items() if v == "ADMITTED"}
assert all(name_of[i] in admitted for i in NEW + ("pl",)), "a name this row writes in CHILDOF is one master refuses"

# 3. The nine new files.
out = {}
for form in FORMS:
    source = (here / f"{form}-pl.xml").read_bytes()
    assert source[:2] == b"\xff\xfe"
    text = source[2:].decode("utf-16-le")
    for ident in NEW:
        name = name_of[ident]
        made = swap(text, f"<CHILDOF>{MX_PLAIN}</CHILDOF>", f"<CHILDOF>{name}</CHILDOF>", 1)
        assert swap(made, f"<CHILDOF>{name}</CHILDOF>", f"<CHILDOF>{MX_PLAIN}</CHILDOF>", 1) == text
        assert MX_PLAIN not in made
        out[f"{form}-{ident}.xml"] = b"\xff\xfe" + made.encode("utf-16-le")

# 4. Every request of the folder, CHILDOF by CHILDOF.
requests = {name: (here / name).read_bytes() for name in held if name.endswith(".xml") and name not in out}
requests.update(out)
by_name = {n: i for i, n in name_of.items()}
lines = []
for file, raw in sorted(requests.items()):
    assert raw[:2] == b"\xff\xfe", file
    text = raw[2:].decode("utf-16-le")
    body = text.split("?>", 1)[1] if text.startswith("<?xml") else text
    found = [element.text or "" for element in ET.fromstring(body).iter("CHILDOF")]
    assert text.count("<CHILDOF>") == len(found) == text.count("</CHILDOF>"), file
    seen = []
    for value in found:
        if value == "##LedgerName":
            seen.append("##LedgerName")
            continue
        assert value in by_name, f"{file}: CHILDOF holds a name that is not in names.tsv"
        assert value in admitted, f"{file}: CHILDOF holds {by_name[value]}, which master's LedgerScopeName refuses"
        assert f"<CHILDOF>{value}</CHILDOF>" in text, f"{file}: the name is not written as itself"
        seen.append(f"{by_name[value]} ({verdict[by_name[value]]})")
    lines.append(f"CHILDOF {file}: {', '.join(seen) if seen else 'none'}")
for name, raw in sorted(out.items()):
    (here / name).write_bytes(raw)
    print(f"{name}: {len(raw)} bytes, sha256 {sha(raw)}")
for line in lines:
    print(line)
print(f"{len(out)} files written; {len(requests)} requests checked")
