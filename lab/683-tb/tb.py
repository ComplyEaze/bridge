#!/usr/bin/env python3
# tb.py: row 683-TB's counter for a currency Trial Balance answer (a-<label>-T-<request>). It reads the answer by an
# XML parser (LAB-COMMON C4), never a text search, and prints never more than CAP lines per call (AGENTS.md P5).
# It mirrors master's Trial Balance with currency (native_trial_balance/wire.rs, parse_native_trial_balance_with_currency
# and parse_envelope) and master's composite check (currency_composite.rs, is_balance_composite). Each value is classed
# on its text exactly as sent, never trimmed, as master reads it.
# STOP, exit 1, never "0 rows", for an answer master's parse refuses as a whole before any row is classed:
#   it does not decode as UTF-16LE or does not parse; it carries a DOCTYPE, or any LINEERROR, ERROR or DOCTYPE element;
#   its root is not ENVELOPE, or HEADER, BODY or BODY/DATA is missing;
#   HEADER/STATUS is absent, appears twice, is empty, or is not 1;
#   BODY/DATA/COLLECTION is missing or appears twice; it has no LEDGER row;
#   a LEDGER row has no NAME attribute, or no GUID, or a GUID that is not the book's (BOOK, any case, "-", eight hex
#   digits), or two of one child element;
#   two rows share a GUID (any case) or a name (any case);
#   a row has no CURRENCYNAME, or an empty one; no row's CURRENCYNAME is BASE;
#   a row lacks any of TBALOPENING, DEBITTOTALS, CREDITTOTALS and TBALCLOSING.
# For every LEDGER row: NAME, CURRENCYNAME, then the four values exactly as sent, each with its class:
#   EMPTY      the element is present with no text;
#   PLAIN      an exact decimal as master's ExactDecimal admits it: -?[0-9]+(.[0-9]+)?, ASCII digits only;
#   COMPOSITE  master's is_currency_composite (the 7.1 layout) or is_unspaced_currency_composite (the 7.2 layout);
#   OTHER      anything else, a space included.
# and the row's verdict, as master sets rows aside:
#   FOREIGN    CURRENCYNAME is not BASE (its values are not read);
#   MIXED      not FOREIGN, and any of the four values is COMPOSITE;
#   PLAIN      every value PLAIN or EMPTY;
#   UNREAD     anything else: master's parse refuses the whole answer on it.
# A PLAIN row is also checked by master for its TYPE attributes, its movement's sign and its opening-plus-movement
# equation, which this script does not repeat. The verdict is this script's reading of the shape. Master's own parser
# is run on the answer when it becomes a fixture, and that run is the one that counts.
# Modes:
#   tb.py ANSWER...            the lines above for each answer, then one SET line per verdict.
#   --question ANSWER          the lines above and the SET lines, then for each ledger in WATCH its verdict and the
#                              classes of its four values, and one last line for QUESTION_LEDGER:
#                                QUESTION: COMPOSITE (<fields>)       a value on its row is a composite;
#                                QUESTION: NO COMPOSITE               its CURRENCYNAME is BASE and all four values are
#                                                                     PLAIN or EMPTY;
#                                QUESTION: FOREIGN (<CURRENCYNAME>)   its row is set aside by its currency, values unread;
#                                QUESTION: NOT ANSWERED (<why>)       no row has that name, or a value is OTHER.
#   --sets A B                 the SET lines of each answer, then each ledger whose verdict differs, and one last line:
#                              SETS SAME or SETS DIFFER.
# Example: python3 -I tb.py a-tb1-T-tb1-20250401-20250430.xml
import pathlib, re, sys
import xml.etree.ElementTree as ET
BOOK = "1bb48be5-baba-45b2-90d8-7593244915a9"  # BRIDGE FOREXGROUP LAB, as forex-register-book-extent reads it
BASE = "₹"  # the book's base currency master NAME, as row 683-LM's cur answer read it
QUESTION_LEDGER = "BRIDGE USD Customer"
WATCH = ("BRIDGE USD Customer", "BRIDGE USD Supplier", "BRIDGE Lab Bank")
FIELDS = ("TBALOPENING", "DEBITTOTALS", "CREDITTOTALS", "TBALCLOSING")
ONCE = ("GUID", "PARENT", "CURRENCYNAME") + FIELDS
CAP = 400
FORBIDDEN = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));", re.I)
PLAIN = re.compile(r"-?[0-9]+(?:\.[0-9]+)?")
HEX8 = re.compile(r"[0-9A-Fa-f]{8}")
lines = 0
NO_ROW = (None, None, None, "no row")

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

# Master's composite check, rule for rule (currency_composite.rs).
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

def load(path):
    if not re.match(r"a-[^-]+-T-", pathlib.Path(path).name):
        stop(f"{path}: not a Trial Balance answer name a-<label>-T-<request>")
    raw = pathlib.Path(path).read_bytes()
    if raw[:2] == b"\xff\xfe":
        raw = raw[2:]
    try:
        text = raw.decode("utf-16-le")
    except UnicodeDecodeError as error:
        stop(f"{path} does not decode as UTF-16LE: {error}")
    if "<!DOCTYPE" in text.upper():
        stop(f"{path} carries a DOCTYPE")
    try:
        root = ET.fromstring(FORBIDDEN.sub("", text))
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
    if root.tag != "ENVELOPE" or root.find("HEADER") is None or root.find("BODY/DATA") is None:
        stop(f"{path}: root {root.tag!r}, or no HEADER or BODY/DATA: not a complete envelope")
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
    out = {}
    guids = set()
    names = set()
    found = load(path).findall("LEDGER")
    if not found:
        stop(f"{path}: no LEDGER row")
    for row in found:
        name = row.get("NAME")
        if name is None:
            stop(f"{path}: a LEDGER row has no NAME attribute")
        for tag in ONCE:
            if len(row.findall(tag)) > 1:
                stop(f"{path}: ledger {name!r} has {tag} twice")
        children = {tag: row.find(tag) for tag in ONCE}
        guid = children["GUID"].text if children["GUID"] is not None else None
        if (guid is None or guid[:len(BOOK)].lower() != BOOK or guid[len(BOOK):len(BOOK) + 1] != "-"
                or not HEX8.fullmatch(guid[len(BOOK) + 1:])):
            stop(f"{path}: ledger {name!r} GUID {guid!r} is not one of the book's")
        if guid.lower() in guids:
            stop(f"{path}: GUID {guid!r} on two LEDGER rows")
        guids.add(guid.lower())
        if name.lower() in names:
            stop(f"{path}: ledger name {name!r} on two LEDGER rows (any case)")
        names.add(name.lower())
        if children["CURRENCYNAME"] is None:
            stop(f"{path}: ledger {name!r} has no CURRENCYNAME")
        currency = children["CURRENCYNAME"].text or ""
        if currency.strip() == "":
            stop(f"{path}: ledger {name!r} has an empty CURRENCYNAME")
        for field in FIELDS:
            if children[field] is None:
                stop(f"{path}: ledger {name!r} has no {field}")
        values = {field: children[field].text or "" for field in FIELDS}
        classes = {field: shape(text) for field, text in values.items()}
        if currency != BASE:
            verdict = "FOREIGN"
        elif "COMPOSITE" in classes.values():
            verdict = "MIXED"
        elif all(c in ("PLAIN", "EMPTY") for c in classes.values()):
            verdict = "PLAIN"
        else:
            verdict = "UNREAD"
        out[name] = (currency, values, classes, verdict)
    if not any(row[0] == BASE for row in out.values()):
        stop(f"{path}: no row's CURRENCYNAME is {BASE!r}")
    if show:
        say(f"  LEDGER rows: {len(found)}")
        for name, (currency, values, classes, verdict) in sorted(out.items()):
            say(f"  {name!r} CURRENCYNAME {currency!r}: {verdict}")
            for field in FIELDS:
                say(f"    {field} {values[field]!r} {classes[field]}")
    return out

def shape(text):
    if text == "":
        return "EMPTY"
    if PLAIN.fullmatch(text):
        return "PLAIN"
    if is_balance_composite(text):
        return "COMPOSITE"
    return "OTHER"

def sets(table):
    for verdict in ("FOREIGN", "MIXED", "PLAIN", "UNREAD"):
        names = sorted(name for name, row in table.items() if row[3] == verdict)
        say(f"SET {verdict} ({len(names)}): {names}")
    if any(row[3] == "UNREAD" for row in table.values()):
        say("UNREAD rows present: master's parse refuses this answer as a whole")

def question(table):
    if QUESTION_LEDGER not in table:
        return "NOT ANSWERED (no row)"
    currency, values, classes, verdict = table[QUESTION_LEDGER]
    if verdict == "FOREIGN":
        return f"FOREIGN ({currency!r})"
    hits = [f for f in FIELDS if classes[f] == "COMPOSITE"]
    if hits:
        return f"COMPOSITE ({', '.join(hits)})"
    if verdict == "PLAIN":
        return "NO COMPOSITE"
    return f"NOT ANSWERED (OTHER in {', '.join(f for f in FIELDS if classes[f] == 'OTHER')})"

args = sys.argv[1:]
if not args:
    stop("no answer named")
if args[0] == "--question":
    if len(args) != 2:
        stop("--question takes one answer")
    table = rows(args[1])
    sets(table)
    for name in WATCH:
        row = table.get(name)
        say(f"WATCH {name!r}: " + (f"{row[3]}; " + ", ".join(f"{f} {row[2][f]}" for f in FIELDS) if row else "no row"))
    print(f"QUESTION: {question(table)}")
elif args[0] == "--sets":
    if len(args) != 3:
        stop("--sets takes two answers")
    a, b = rows(args[1], False), rows(args[2], False)
    say(f"{args[1]}:")
    sets(a)
    say(f"{args[2]}:")
    sets(b)
    differ = sorted(n for n in set(a) | set(b) if a.get(n, NO_ROW)[3] != b.get(n, NO_ROW)[3])
    for name in differ:
        say(f"  {name!r}: {a.get(name, NO_ROW)[3]} -> {b.get(name, NO_ROW)[3]}")
    print("SETS SAME" if not differ else "SETS DIFFER")
else:
    for path in args:
        sets(rows(path))
