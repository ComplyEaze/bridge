#!/usr/bin/env python3
# st.py: row 2480-NS's import gate and name checks. New for this row. It reads each answer by an XML parser
# (LAB-COMMON C4), never a text search, prints never more than CAP lines per call (AGENTS.md P5), and STOPs (exit 1)
# on an answer it cannot read, never "nothing found".
# Modes:
#   st.py --import ANSWER   an import answer (an a-*-LM-* or a-*-I-* file). IMPORT: CREATED ONE (exit 0) only if the
#                           answer holds exactly one RESPONSE, each of COUNTERS exactly once as a whole number,
#                           CREATED 1, every one of ZERO 0, and no LINEERROR. Anything else is IMPORT: STOP (exit 1),
#                           with the counters as read: an ALTERED 1 (row 2480-NM's five), an ERRORS 1, an EXCEPTIONS
#                           1 or a counter missing alike. This is the row's write gate (ROW.md, step 4): the next
#                           request is sent only after exit 0.
#   st.py --before S        the snapshot read before any write (s2). For each name of names.tsv: STOP if the book
#                           already holds a ledger of that name, or if the name's key (below) equals the key of any
#                           ledger's NAME attribute or of any name in its LANGUAGENAME.LIST. Else FOLD: CLEAR (exit 0).
#   st.py --names S         every ledger whose LANGUAGENAME.LIST names are not exactly its NAME attribute, then each
#                           name of names.tsv with its row's names, and NAMES: EQUAL or NAMES: DIFFER. It records and
#                           never decides: it exits 0 either way.
# The key is a Python reading of master's letters_and_digits_key (bridge-tally-core/src/master_binding.rs): letters
# and digits only, upper-cased then lower-cased. str.isalnum keeps fewer characters than Rust's is_alphanumeric (a
# Devanagari vowel sign is dropped here), so this key finds every collision master's would, and possibly more.
# Example: python3 -I st.py --import a-s4hy-LM-hy.xml
import pathlib, re, sys
import xml.etree.ElementTree as ET
BOOK = "6b43e498-430c-4d5c-bfef-d32e2ab93c85"  # BRIDGE PILOT LAB, as nm.py and count20.py hold it
CAP = 200
COUNTERS = ("CREATED", "ALTERED", "DELETED", "LASTVCHID", "LASTMID", "COMBINED", "IGNORED", "ERRORS", "CANCELLED",
            "EXCEPTIONS")
ZERO = ("ALTERED", "DELETED", "COMBINED", "IGNORED", "ERRORS", "CANCELLED", "EXCEPTIONS")
# nm.py's: a character XML forbids, raw or as a reference, kept as U+FFFD, as master's sanitizer keeps it.
FORBIDDEN = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));", re.I)
lines = 0

def say(text):
    global lines
    lines += 1
    if lines == CAP:
        print(f"... more than {CAP} lines: the rest is not printed (STOP for this call)")
        raise SystemExit(1)
    print(text)

def stop(text):
    print(f"STOP: {text}")
    raise SystemExit(1)

def root_of(path):
    raw = pathlib.Path(path).read_bytes() if pathlib.Path(path).is_file() else b""
    if not raw:
        stop(f"{path} is missing or empty")
    if raw[:2] == b"\xff\xfe":
        raw = raw[2:]
    try:
        text = raw.decode("utf-16-le")
    except UnicodeDecodeError as error:
        stop(f"{path} does not decode as UTF-16LE: {error}")
    try:
        return ET.fromstring(FORBIDDEN.sub("�", text))
    except ET.ParseError as error:
        stop(f"{path} does not parse: {error}")

def key(value):
    return "".join(c for c in value if c.isalnum()).upper().lower()

def names_tsv():
    rows = {}
    for line in (pathlib.Path(__file__).resolve().parent / "names.tsv").read_text("utf-8").split("\n"):
        if line:
            ident, number, name = line.split("\t")
            rows[ident] = (number, name)
    if len(rows) != 11:
        stop("names.tsv does not hold eleven rows")
    return rows

def ledgers(path):
    """(NAME attribute, [names of LANGUAGENAME.LIST]) for each LEDGER row of a snapshot answer, each the book's."""
    root = root_of(path)
    if list(root.iter("LINEERROR")):
        stop(f"{path} carries a LINEERROR: the request failed")
    found = root.find("BODY/DATA/COLLECTION")
    if found is None:
        stop(f"{path} has no BODY/DATA/COLLECTION: the request failed")
    out = []
    for row in found.findall("LEDGER"):
        guid = row.findtext("BRIDGECOMPANYGUID")
        if guid is None or guid.strip().lower() != BOOK:
            stop(f"{path}: a LEDGER row's BRIDGECOMPANYGUID {guid!r} is not the book's")
        name = row.get("NAME")
        if name is None:
            stop(f"{path}: a LEDGER row without a NAME attribute")
        listed = [n.text or "" for n in row.findall("LANGUAGENAME.LIST/NAME.LIST/NAME")]
        if not listed:
            stop(f"{path}: {name!r} has no LANGUAGENAME.LIST name")
        out.append((name, listed))
    if not out:
        stop(f"{path}: no LEDGER row, so nothing can be compared")
    return out

args = sys.argv[1:]
if len(args) != 2 or args[0] not in ("--import", "--before", "--names"):
    stop("st.py takes --import ANSWER, --before S or --names S")
mode, path = args
if mode == "--import":
    root = root_of(path)
    responses = [root] if root.tag == "RESPONSE" else list(root.iter("RESPONSE"))
    if len(responses) != 1:
        stop(f"{path}: {len(responses)} RESPONSE elements, not one")
    read = {}
    for tag in COUNTERS:
        found = responses[0].findall(tag)
        text = (found[0].text or "").strip() if len(found) == 1 else None
        read[tag] = int(text) if text is not None and text.isdigit() and text.isascii() else None
    errors = len(list(root.iter("LINEERROR")))
    say(f"{path}: " + ", ".join(f"{tag} {read[tag]}" for tag in COUNTERS) + f", LINEERROR {errors}")
    wrong = [tag for tag in COUNTERS if read[tag] is None]
    wrong += [] if read["CREATED"] == 1 else ["CREATED is not 1"]
    wrong += [f"{tag} is not 0" for tag in ZERO if read[tag] not in (None, 0)]
    wrong += ["a LINEERROR"] if errors else []
    if wrong:
        say("IMPORT: STOP (" + "; ".join(f"{w} missing or not a whole number" if w in COUNTERS else w for w in wrong) + ")")
        raise SystemExit(1)
    say("IMPORT: CREATED ONE")
elif mode == "--before":
    rows, book = names_tsv(), ledgers(path)
    say(f"{path}: LEDGER rows {len(book)}")
    held = {}
    for name, listed in book:
        for spelling in [name] + listed:
            held.setdefault(key(spelling), set()).add(name)
    clashes = []
    for ident, (_, name) in rows.items():
        if any(name == n or name in listed for n, listed in book):
            clashes.append(f"{ident} {name!r}: the book already holds a ledger of that name")
        elif key(name) in held:
            clashes.append(f"{ident} {name!r}: its key {key(name)!r} is the key of {sorted(held[key(name)])!r}")
    for line in clashes:
        say(f"  {line}")
    if clashes:
        stop(f"{len(clashes)} name(s) of names.tsv meet a ledger of the book")
    say("FOLD: CLEAR")
else:
    rows, book = names_tsv(), ledgers(path)
    say(f"{path}: LEDGER rows {len(book)}")
    differ = [(name, listed) for name, listed in book if listed != [name]]
    for name, listed in differ:
        say(f"  name list not the NAME attribute: NAME {name!r}, names {listed!r}")
    for ident, (_, name) in rows.items():
        mine = [listed for n, listed in book if n == name]
        say(f"  {ident} {name!r}: " + ("no row" if not mine else f"names {mine[0]!r}" if len(mine) == 1 else "two rows"))
    say("NAMES: " + ("DIFFER" if differ else "EQUAL"))
