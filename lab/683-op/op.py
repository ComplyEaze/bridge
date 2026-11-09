#!/usr/bin/env python3
# op.py: row 683-OP's counter for two of master's ledger reads, by an XML parser (LAB-COMMON C4), never a text search,
# and never more than CAP lines per call (AGENTS.md P5). The kind comes from the answer's name a-<label>-<K>-<request>:
#   S  the ledger master read: master's List of Ledgers snapshot (render_native_ledger_snapshot_request), whose rows
#      carry NAME, PARENT, CURRENCYNAME, ISBILLWISEON, OPENINGBALANCE, CLOSINGBALANCE and BRIDGECOMPANYGUID;
#   X  the movement export: ledger_movement's opening read (render_native_ledger_export_request), whose rows carry
#      NAME, GUID, PARENT, ALTERID, MASTERID and OPENINGBALANCE (PARTYGSTIN where a ledger has one).
# Each amount is classed on its text exactly as sent, never trimmed, by tb.py's classes, whose code is copied here
# unchanged from tb.py (split_once_exact to is_balance_composite, and shape):
#   EMPTY      the element is present with no text;
#   PLAIN      -?[0-9]+(.[0-9]+)?, ASCII digits only, at most 256 bytes (master's ExactDecimal);
#   COMPOSITE  master's is_currency_composite (7.1) or is_unspaced_currency_composite (7.2) (currency_composite.rs);
#   OTHER      anything else, a space included.
# A numeric reference to a character XML forbids becomes U+FFFD #n; and a raw one U+FFFD, as in tb.py, so it stays
# in the value and never makes it plain.
# STOP, exit 1, never "0 rows", for an answer that is a failed request or that this script cannot read whole:
#   it does not decode as UTF-16LE or does not parse; it carries a DOCTYPE, or any LINEERROR, ERROR or DOCTYPE element;
#   its root is not ENVELOPE, or HEADER, BODY or BODY/DATA is missing or appears twice;
#   HEADER/STATUS is absent, appears twice, is empty, or is not 1;
#   BODY/DATA/COLLECTION is missing or appears twice; it has no LEDGER row;
#   a STATUS, GUID, PARENT, CURRENCYNAME, ISBILLWISEON, BRIDGECOMPANYGUID or amount element holds anything but text
#   (an element, a comment, CDATA) or a reference other than &amp; &lt; &gt; &quot; &apos;;
#   a LEDGER row has no NAME attribute, an empty one or two (any case), or two of one child element that holds text;
#   two rows share a name (any case);
#   a row lacks one of its kind's fields above (PARTYGSTIN aside), or an amount has no TYPE, two, or one other than
#   Amount;
#   S: a row's BRIDGECOMPANYGUID is not the book's (BOOK, any case); X: a row's GUID is not one of the book's (BOOK,
#   any case, "-", eight hex digits), or two rows share a GUID (any case).
# Modes:
#   op.py ANSWER...          for each LEDGER row its fields, and each amount's text and class.
#   --question ANSWER        those lines, then one line for QUESTION_LEDGER, the last line printed:
#                              QUESTION: NOT ANSWERED (no row)
#                              QUESTION: FOREIGN (<CURRENCYNAME>)   S only: its CURRENCYNAME is not BASE;
#                              QUESTION: COMPOSITE (<fields>)       an amount on its row is a composite;
#                              QUESTION: OTHER (<fields>)           an amount on its row neither PLAIN, EMPTY nor
#                                                                   a composite;
#                              QUESTION: EMPTY (OPENINGBALANCE)     its opening is present with no text;
#                              QUESTION: NO COMPOSITE (<opening>)   its opening PLAIN, and S's closing PLAIN or EMPTY.
#                            FOREIGN is printed before the amounts are looked at, as trial_balance sets such a row aside.
#   --same A B               two answers of one kind, row by row by NAME: every child element that holds text,
#                            as sent. Each row that differs, with each field that differs, and each row in one answer
#                            only; then one last line: ROWS SAME or ROWS DIFFER.
#   --others A B             --same with QUESTION_LEDGER left out of the comparison and its presence in each answer
#                            printed; then OTHERS SAME or OTHERS DIFFER.
# The classes are this script's reading of the shape. Master's own parsers are run on the answers when they become
# fixtures, and that run is the one that counts.
# Example: python3 -I op.py --question a-s1-S-ls-20250401-20250430.xml
import pathlib, re, sys
import xml.etree.ElementTree as ET
BOOK = "1bb48be5-baba-45b2-90d8-7593244915a9"  # BRIDGE FOREXGROUP LAB, as forex-register-book-extent reads it
BASE = "₹"  # the book's base currency master NAME, as row 683-LM's cur answer read it
QUESTION_LEDGER = "BRIDGE USD Opening"  # row 683-OP's new ledger
NEEDS = {"S": ("PARENT", "CURRENCYNAME", "ISBILLWISEON", "OPENINGBALANCE", "CLOSINGBALANCE", "BRIDGECOMPANYGUID"),
         "X": ("GUID", "PARENT", "ALTERID", "MASTERID", "OPENINGBALANCE")}
AMOUNTS = {"S": ("OPENINGBALANCE", "CLOSINGBALANCE"), "X": ("OPENINGBALANCE",)}
CAP = 400
FORBIDDEN = re.compile(r"&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));", re.I)
PLAIN = re.compile(r"-?[0-9]+(?:\.[0-9]+)?")
MAX_DECIMAL_BYTES = 256  # bridge-tally-primitives MAX_EXACT_DECIMAL_BYTES
RAW_C0 = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]")
SCALARS = ("STATUS", "GUID", "PARENT", "CURRENCYNAME", "ISBILLWISEON", "BRIDGECOMPANYGUID", "OPENINGBALANCE",
           "CLOSINGBALANCE")
SCALAR = re.compile(r"<(" + "|".join(SCALARS) + r")(\s[^>]*)?>(.*?)</\1\s*>", re.S)
NAMED_REF = re.compile(r"&(?:amp|lt|gt|quot|apos);")
HEX8 = re.compile(r"[0-9A-Fa-f]{8}")
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

# Copied unchanged from tb.py: master's composite check, rule for rule (currency_composite.rs).
def split_once_exact(text, separator):
    left, found, right = text.partition(separator)
    return (left, right) if found and separator not in right else None

def is_symbol(text):
    return text != "" and all(not c.isspace() and c not in "0123456789-@=/" for c in text)

def is_ascii_decimal(text):
    whole, dot, fraction = text.partition(".")
    return (whole != "" and all(c in "0123456789" for c in whole) and all(c in "0123456789" for c in fraction)
            and (fraction != "" or not text.endswith(".")))

def symbol_amount(text):  # (symbol, digits): an optional '-', the symbol, one space, the amount
    text = text[1:] if text.startswith("-") else text
    symbol, space, amount = text.partition(" ")
    return (symbol, amount) if space and is_symbol(symbol) and is_ascii_decimal(amount) else None

def unspaced_amount(text):  # (symbol, digits): an optional '-', the symbol up to the first ASCII digit, the amount
    text = text[1:] if text.startswith("-") else text
    first = next((i for i, c in enumerate(text) if c in "0123456789"), None)
    if first is None:
        return None
    symbol, digits = text[:first], text[first:]
    return (symbol, digits) if is_symbol(symbol) and is_ascii_decimal(digits) else None

def rate_symbols(text):  # (base, per): the base symbol, one space, an optional amount, '/', the foreign symbol
    base, space, rest = text.partition(" ")
    number, slash, per = rest.partition("/")
    if not space or not slash:
        return None
    return (base, per) if is_symbol(base) and (number == "" or is_ascii_decimal(number)) and is_symbol(per) else None

def has_two_decimals(digits):
    whole, dot, fraction = digits.partition(".")
    return dot != "" and len(fraction) == 2

def is_currency_composite(text):  # 7.1
    parts = split_once_exact(text, " @ ")
    if parts is None:
        return False
    foreign, rest = parts
    parts = split_once_exact(rest, " = ")
    if parts is None:
        return False
    rate, base = parts
    foreign, rate, base = symbol_amount(foreign), rate_symbols(rate.rstrip(" ")), symbol_amount(base)
    if foreign is None or rate is None or base is None:
        return False
    return rate[0] == base[0] and rate[1] == foreign[0] and foreign[0] != base[0]

def is_unspaced_currency_composite(text):  # 7.2
    parts = split_once_exact(text, " @ ")
    if parts is None:
        return False
    foreign, rest = parts
    parts = split_once_exact(rest, " = ")
    if parts is None:
        return False
    rate, base = parts
    if rate.endswith(" "):
        return False
    foreign, rate, base = unspaced_amount(foreign), rate_symbols(rate), symbol_amount(base)
    if foreign is None or rate is None or base is None:
        return False
    return (has_two_decimals(foreign[1]) and has_two_decimals(base[1])
            and rate[0] == base[0] and rate[1] == foreign[0] and foreign[0] != base[0])

def is_balance_composite(text):
    return is_currency_composite(text) or is_unspaced_currency_composite(text)

def shape(text):
    if text == "":
        return "EMPTY"
    if PLAIN.fullmatch(text) and len(text.encode()) <= MAX_DECIMAL_BYTES:
        return "PLAIN"
    if is_balance_composite(text):
        return "COMPOSITE"
    return "OTHER"
# End of the copy.

def reference_number(reference):  # &#n; or &#xh; as its decimal number
    body = reference[2:-1]
    return int(body[1:], 16) if body[:1] in "xX" else int(body)

def attribute(element, key):  # the key in any case, once, not empty
    found = [value for name, value in element.attrib.items() if name.lower() == key.lower()]
    return found[0] if len(found) == 1 and found[0] != "" else None

def kind(path):
    match = re.match(r"a-[^-]+-([SX])-", pathlib.Path(path).name)
    if not match:
        stop(f"{path}: not an answer name a-<label>-S-<request> or a-<label>-X-<request>")
    return match.group(1)

def load(path):
    raw = pathlib.Path(path).read_bytes()
    if raw[:2] == b"\xff\xfe":
        raw = raw[2:]
    try:
        text = raw.decode("utf-16-le")
    except UnicodeDecodeError as error:
        stop(f"{path} does not decode as UTF-16LE: {error}")
    if "<!DOCTYPE" in text.upper():
        stop(f"{path} carries a DOCTYPE")
    text = RAW_C0.sub("�", FORBIDDEN.sub(lambda m: f"�#{reference_number(m.group(0))};", text))
    for match in SCALAR.finditer(text):
        inner = match.group(3)
        if "<" in inner or NAMED_REF.sub("", inner).count("&"):
            stop(f"{path}: a {match.group(1)} element holds more than text: {inner[:60]!r}")
    try:
        root = ET.fromstring(text)
    except ET.ParseError as error:
        stop(f"{path} does not parse: {error}")
    statuses = root.findall("HEADER/STATUS")
    shown = statuses[0].text if statuses and statuses[0].text is not None else "<empty>" if statuses else "<absent>"
    print(f"{path}: HEADER/STATUS {shown!r}")
    errors = [e for e in root.iter() if e.tag in ("LINEERROR", "ERROR", "DOCTYPE")]
    for error in errors[:5]:
        print(f"  {error.tag} {error.text!r}")
    if errors:
        stop(f"{path} carries {len(errors)} LINEERROR, ERROR or DOCTYPE element(s): the request failed")
    if (root.tag != "ENVELOPE" or len(root.findall("HEADER")) != 1 or len(root.findall("BODY")) != 1
            or len(root.findall("BODY/DATA")) != 1):
        stop(f"{path}: root {root.tag!r}, or not one HEADER, BODY and BODY/DATA: not a complete envelope")
    if len(statuses) != 1:
        stop(f"{path}: {len(statuses)} HEADER/STATUS elements, not one")
    status = (statuses[0].text or "").strip()
    if status == "":
        stop(f"{path}: HEADER/STATUS is empty: no answer")
    if status != "1":
        stop(f"{path}: HEADER/STATUS {status!r}: Tally reported a failure")
    collections = root.findall("BODY/DATA/COLLECTION")
    if len(collections) != 1:
        stop(f"{path}: {len(collections)} BODY/DATA/COLLECTION elements, not one: the request failed")
    return collections[0]

def rows(path, show=True):
    k = kind(path)
    found = load(path).findall("LEDGER")
    if not found:
        stop(f"{path}: no LEDGER row")
    out, names, guids = {}, set(), set()
    for row in found:
        name = attribute(row, "NAME")
        if name is None:
            stop(f"{path}: a LEDGER row has no NAME attribute, an empty one, or two")
        if name.lower() in names:
            stop(f"{path}: ledger name {name!r} on two LEDGER rows (any case)")
        names.add(name.lower())
        leaves = {}
        for child in row:
            if len(child):
                continue  # a list such as LANGUAGENAME.LIST: not read
            if child.tag in leaves:
                stop(f"{path}: ledger {name!r} has {child.tag} twice")
            leaves[child.tag] = child.text or ""
        for tag in NEEDS[k]:
            if tag not in leaves:
                stop(f"{path}: ledger {name!r} has no {tag}")
        for tag in AMOUNTS[k]:
            if attribute(row.find(tag), "TYPE") != "Amount":
                stop(f"{path}: ledger {name!r} {tag} has no TYPE, two, or one other than Amount")
        if k == "S" and leaves["BRIDGECOMPANYGUID"].lower() != BOOK:
            stop(f"{path}: ledger {name!r} BRIDGECOMPANYGUID {leaves['BRIDGECOMPANYGUID']!r} is not the book's")
        if k == "X":
            guid = leaves["GUID"]
            if (guid[:len(BOOK)].lower() != BOOK or guid[len(BOOK):len(BOOK) + 1] != "-"
                    or not HEX8.fullmatch(guid[len(BOOK) + 1:])):
                stop(f"{path}: ledger {name!r} GUID {guid!r} is not one of the book's")
            if guid.lower() in guids:
                stop(f"{path}: GUID {guid!r} on two LEDGER rows")
            guids.add(guid.lower())
        out[name] = leaves
    if show:
        say(f"  LEDGER rows: {len(found)}")
        for name, leaves in sorted(out.items()):
            others = [t for t in NEEDS[k] if t not in AMOUNTS[k] and t not in ("BRIDGECOMPANYGUID", "ALTERID", "MASTERID")]
            say(f"  {name!r} " + " ".join(f"{t} {leaves[t]!r}" for t in others))
            for tag in AMOUNTS[k]:
                say(f"    {tag} {leaves[tag]!r} {shape(leaves[tag])}")
    return k, out

def question(k, table):
    if QUESTION_LEDGER not in table:
        return "NOT ANSWERED (no row)"
    leaves = table[QUESTION_LEDGER]
    if k == "S" and leaves["CURRENCYNAME"] != BASE:
        return f"FOREIGN ({leaves['CURRENCYNAME']!r})"
    classes = {tag: shape(leaves[tag]) for tag in AMOUNTS[k]}
    hits = [tag for tag, c in classes.items() if c == "COMPOSITE"]
    if hits:
        return f"COMPOSITE ({', '.join(hits)})"
    odd = [tag for tag, c in classes.items() if c == "OTHER"]
    if odd:
        return f"OTHER ({', '.join(odd)})"
    if classes["OPENINGBALANCE"] == "EMPTY":
        return "EMPTY (OPENINGBALANCE)"
    return f"NO COMPOSITE (OPENINGBALANCE {leaves['OPENINGBALANCE']!r})"

def compare(a_path, b_path, leave_out):
    (ka, a), (kb, b) = rows(a_path, False), rows(b_path, False)
    if ka != kb:
        stop(f"{a_path} and {b_path} are not of one kind")
    differ = False
    for name in sorted((set(a) | set(b)) - leave_out):
        if name not in a or name not in b:
            differ = True
            say(f"  {name!r}: in {a_path if name in a else b_path} only")
            continue
        fields = [t for t in sorted(set(a[name]) | set(b[name])) if a[name].get(t) != b[name].get(t)]
        if fields:
            differ = True
            say(f"  {name!r}: " + "; ".join(f"{t} {a[name].get(t, '<absent>')!r} -> {b[name].get(t, '<absent>')!r}"
                                            for t in fields))
    say(f"  rows: {len(a)} and {len(b)}")
    return differ, a, b

args = sys.argv[1:]
if not args:
    stop("no answer named")
if args[0] == "--question":
    if len(args) != 2:
        stop("--question takes one answer")
    k, table = rows(args[1])
    print(f"QUESTION: {question(k, table)}")
elif args[0] in ("--same", "--others"):
    if len(args) != 3:
        stop(f"{args[0]} takes two answers")
    leave_out = {QUESTION_LEDGER} if args[0] == "--others" else set()
    differ, a, b = compare(args[1], args[2], leave_out)
    if args[0] == "--others":
        say(f"  {QUESTION_LEDGER!r}: in {args[1]} {'yes' if QUESTION_LEDGER in a else 'no'}, "
            f"in {args[2]} {'yes' if QUESTION_LEDGER in b else 'no'} (not compared)")
        print("OTHERS DIFFER" if differ else "OTHERS SAME")
    else:
        print("ROWS DIFFER" if differ else "ROWS SAME")
else:
    for path in args:
        rows(path)
