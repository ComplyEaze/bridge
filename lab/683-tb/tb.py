#!/usr/bin/env python3
# tb.py: row 683-TB's counter for a currency Trial Balance answer (a-<label>-T-<request>). It reads the answer by an
# XML parser (LAB-COMMON C4), never a text search, and prints never more than CAP lines per call (AGENTS.md P5).
# An answer that does not decode, does not parse, or has no BODY/DATA/COLLECTION is a failed request: STOP, exit 1,
# never "0 rows". HEADER/STATUS and every LINEERROR print first.
# For every LEDGER row (a child of BODY/DATA/COLLECTION; CMPINFO's bare counters are not rows): NAME, whether its GUID
# begins with BOOK, CURRENCYNAME, then TBALOPENING, DEBITTOTALS, CREDITTOTALS and TBALCLOSING exactly as sent, each
# with its class:
#   EMPTY      the element is present and empty;
#   PLAIN      -?digits[.digits];
#   COMPOSITE  <foreign amount> @ <base symbol> [rate]/<foreign symbol> = <base amount>, in the 7.1 layout (a space
#              after each symbol, two before "=") or the 7.2 layout (no space after the foreign symbol, one before "=");
#              the rate's two symbols must be the base amount's and the foreign amount's, and differ;
#   ABSENT     no such element;
#   OTHER      anything else.
# and the row's verdict, as master's Trial Balance with currency sets rows aside (native_trial_balance/wire.rs:41):
#   FOREIGN    CURRENCYNAME is not BASE;
#   MIXED      not FOREIGN, and any of the four values is COMPOSITE;
#   PLAIN      every value PLAIN or EMPTY;
#   UNREAD     anything else (master's parse would refuse the whole answer on it).
# The verdict is this script's reading of the shape. Master's own parser is run on the answer when it becomes a
# fixture, and that run is the one that counts.
# Modes:
#   tb.py ANSWER...            the lines above for each answer, then one SET line per verdict.
#   --question ANSWER          the lines above and the SET lines, then for each ledger in WATCH the classes of its four
#                              values, and one last line for QUESTION_LEDGER: QUESTION: COMPOSITE (<fields>),
#                              QUESTION: NO COMPOSITE, or QUESTION: NOT ANSWERED (no row) when no row has that name.
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
CAP = 400
FORBIDDEN = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));", re.I)
PLAIN = re.compile(r"-?\d+(?:\.\d+)?")
SYMBOL = r"[^\s\d.@=/-]+"
SPACED = re.compile(  # 7.1: -$ 100.00 @ I₹ 86/$  = -I₹ 8600.00
    r"-?(?P<fs>" + SYMBOL + r") \d+\.\d{2} @ (?P<rb>" + SYMBOL + r") (?:\d+(?:\.\d+)?)?/(?P<rf>" + SYMBOL + r")"
    r"  = -?(?P<bs>" + SYMBOL + r") \d+\.\d{2}")
UNSPACED = re.compile(  # 7.2: -$100.00 @ ₹ 86/$ = -₹ 8600.00
    r"-?(?P<fs>" + SYMBOL + r")\d+\.\d{2} @ (?P<rb>" + SYMBOL + r") (?:\d+(?:\.\d+)?)?/(?P<rf>" + SYMBOL + r")"
    r" = -?(?P<bs>" + SYMBOL + r") \d+\.\d{2}")
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
    try:
        root = ET.fromstring(FORBIDDEN.sub("", text))
    except ET.ParseError as error:
        stop(f"{path} does not parse: {error}")
    status = root.find("HEADER/STATUS")
    print(f"{path}: HEADER/STATUS {status.text.strip() if status is not None and status.text else '<absent>'}")
    for error in root.iter("LINEERROR"):
        print(f"  LINEERROR {error.text!r}")
    collection = root.find("BODY/DATA/COLLECTION")
    if collection is None:
        stop(f"{path} has no BODY/DATA/COLLECTION: the request failed")
    return collection

def value(row, tag):
    found = row.find(tag)
    return None if found is None else (found.text or "").strip()

def shape(text):
    if text is None:
        return "ABSENT"
    if text == "":
        return "EMPTY"
    if PLAIN.fullmatch(text):
        return "PLAIN"
    for layout in (SPACED, UNSPACED):
        match = layout.fullmatch(text)
        if match and match["rb"] == match["bs"] and match["rf"] == match["fs"] and match["fs"] != match["bs"]:
            return "COMPOSITE"
    return "OTHER"

def rows(path, show=True):
    out = {}
    found = load(path).findall("LEDGER")
    not_book = 0
    for row in found:
        name = row.get("NAME") if row.get("NAME") is not None else value(row, "NAME")
        if name in out:
            stop(f"{path}: ledger {name!r} on two LEDGER rows")
        not_book += not (value(row, "GUID") or "").startswith(BOOK + "-")
        currency = value(row, "CURRENCYNAME")
        values = {field: value(row, field) for field in FIELDS}
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
    if show:
        say(f"  LEDGER rows: {len(found)}; GUID not beginning with the book's: {not_book}")
        for name, (currency, values, classes, verdict) in sorted(out.items()):
            say(f"  {name!r} CURRENCYNAME {currency!r}: {verdict}")
            for field in FIELDS:
                say(f"    {field} {values[field]!r} {classes[field]}")
    return out

def sets(table):
    for verdict in ("FOREIGN", "MIXED", "PLAIN", "UNREAD"):
        names = sorted(name for name, row in table.items() if row[3] == verdict)
        say(f"SET {verdict} ({len(names)}): {names}")

args = sys.argv[1:]
if not args:
    stop("no answer named")
if args[0] == "--question":
    if len(args) != 2:
        stop("--question takes one answer")
    table = rows(args[1])
    sets(table)
    for name in WATCH:
        classes = table[name][2] if name in table else None
        say(f"WATCH {name!r}: " + (", ".join(f"{f} {classes[f]}" for f in FIELDS) if classes else "no row"))
    if QUESTION_LEDGER not in table:
        print("QUESTION: NOT ANSWERED (no row)")
    else:
        hits = [f for f in FIELDS if table[QUESTION_LEDGER][2][f] == "COMPOSITE"]
        print(f"QUESTION: COMPOSITE ({', '.join(hits)})" if hits else "QUESTION: NO COMPOSITE")
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
