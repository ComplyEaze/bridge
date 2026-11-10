#!/usr/bin/env python3
# ld.py: row 683-LD's counter for a movement window answer (a-<label>-V-<request>), by an XML parser (LAB-COMMON C4),
# never a text search, and never more than CAP lines per call (AGENTS.md P5).
# Copied unchanged: from lm.py, say, stop, load and leaf (load prints HEADER/STATUS and every LINEERROR, and STOPs on
# an answer that does not decode or parse or has no BODY/DATA/COLLECTION: a failed request, never "0 vouchers");
# from tb.py, its classes and master's composite check (split_once_exact to is_balance_composite, and shape):
#   EMPTY      the element is present with no text;
#   PLAIN      -?[0-9]+(.[0-9]+)?, ASCII digits only, at most 256 bytes (master's ExactDecimal);
#   COMPOSITE  master's is_currency_composite (7.1) or is_unspaced_currency_composite (7.2) (currency_composite.rs);
#   OTHER      anything else, a space included.
# Each AMOUNT is classed on its text exactly as sent, never trimmed. An absent AMOUNT is ABSENT.
# STOP, exit 1, also when: HEADER/STATUS is not one element holding 1; the answer carries a LINEERROR or ERROR
# element; it has no VOUCHER row; two VOUCHER rows share a GUID; a VOUCHER row has no GUID or
# no DATE, or a GUID that is not one of the book's (BOOK, "-", eight hex digits, any case).
# Modes:
#   --day MV DAY     every voucher of MV dated DAY (YYYYMMDD): its GUID, type and number, each direct
#                    ALLLEDGERENTRIES.LIST child's LEDGERNAME, ISDEEMEDPOSITIVE, AMOUNT text and class (with the
#                    voucher's ISCANCELLED and ISOPTIONAL, <absent> where absent), then per voucher
#                    one line: ALL COMPOSITE (n), ALL PLAIN (n), or NOT ONE CLASS (the classes). The last line:
#                    DAY <DAY>: <k> vouchers, <m> on other days.
#   --sum MV NAME    every entry on ledger NAME (exact text) in MV, and one last line:
#                      SUM <NAME>: <total> (<n> entries on <v> vouchers)      every entry PLAIN;
#                      SUM <NAME>: NOT A FIGURE (<n> vouchers not ...)        a voucher with an entry on NAME is
#                                                                             not ISCANCELLED No and ISOPTIONAL No;
#                      SUM <NAME>: NOT A FIGURE (<n> entries not PLAIN)       any entry another class;
#                      SUM <NAME>: NO ENTRY                                   no entry on NAME.
# Example: python3 -I ld.py --day a-mvy-V-mvy-20250401-20260331.xml 20250430
import decimal, pathlib, re, sys
import xml.etree.ElementTree as ET
BOOK = "1bb48be5-baba-45b2-90d8-7593244915a9"  # BRIDGE FOREXGROUP LAB, as in lm.py and tb.py
CAP = 400
FORBIDDEN = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));", re.I)
PLAIN = re.compile(r"-?[0-9]+(?:\.[0-9]+)?")
MAX_DECIMAL_BYTES = 256  # bridge-tally-primitives MAX_EXACT_DECIMAL_BYTES
HEX8 = re.compile(r"[0-9A-Fa-f]{8}")
decimal.getcontext().prec = 1000
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

def load(path):
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

def leaf(row, tag, trim=True):
    found = row.find(tag)
    if found is None:
        return "<absent>"
    text = found.text or ""
    return text.strip() if trim else text

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

def read(path):
    if not re.match(r"a-[^-]+-V-", pathlib.Path(path).name):
        stop(f"{path}: not a movement answer name a-<label>-V-<request>")
    collection = load(path)
    raw = pathlib.Path(path).read_bytes()
    root = ET.fromstring(FORBIDDEN.sub("", (raw[2:] if raw[:2] == b"\xff\xfe" else raw).decode("utf-16-le")))
    status = root.findall("HEADER/STATUS")
    if len(status) != 1 or (status[0].text or "").strip() != "1":
        stop(f"{path}: HEADER/STATUS is not one element holding 1")
    if root.find(".//LINEERROR") is not None or root.find(".//ERROR") is not None:
        stop(f"{path}: the answer carries a LINEERROR or ERROR")
    rows = collection.findall("VOUCHER")
    if not rows:
        stop(f"{path}: no VOUCHER row")
    out = {}
    for row in rows:
        guid, date = leaf(row, "GUID"), leaf(row, "DATE")
        if (guid[:len(BOOK)].lower() != BOOK or guid[len(BOOK):len(BOOK) + 1] != "-"
                or not HEX8.fullmatch(guid[len(BOOK) + 1:])):
            stop(f"{path}: VOUCHER GUID {guid!r} is not one of the book's")
        if guid.lower() in out:
            stop(f"{path}: GUID {guid!r} on two VOUCHER rows")
        if date in ("<absent>", ""):
            stop(f"{path}: voucher {guid} has no DATE")
        legs = []
        for entry in row.findall("ALLLEDGERENTRIES.LIST"):
            amount = leaf(entry, "AMOUNT", False)
            legs.append((leaf(entry, "LEDGERNAME", False), leaf(entry, "ISDEEMEDPOSITIVE"), amount,
                         "ABSENT" if amount == "<absent>" else shape(amount)))
        flags = f"ISCANCELLED {leaf(row, 'ISCANCELLED')} ISOPTIONAL {leaf(row, 'ISOPTIONAL')}"
        out[guid.lower()] = (guid, date, leaf(row, "VOUCHERTYPENAME"), leaf(row, "VOUCHERNUMBER"), legs, flags)
    return out

args = sys.argv[1:]
if len(args) != 3 or args[0] not in ("--day", "--sum"):
    stop("usage: ld.py --day MV YYYYMMDD | ld.py --sum MV NAME")
table = read(args[1])
if args[0] == "--day":
    day = args[2]
    if not re.fullmatch(r"[0-9]{8}", day):
        stop(f"{day!r} is not YYYYMMDD")
    found = [v for v in table.values() if v[1] == day]
    for guid, date, vtype, number, legs, flags in sorted(found):
        say(f"  {guid} {date} {vtype} {number!r} {flags}: {len(legs)} entries")
        for name, deemed, amount, kind in legs:
            say(f"    {name!r} {deemed} {amount!r} {kind}")
        kinds = sorted({kind for *_, kind in legs})
        if kinds == ["COMPOSITE"]:
            say(f"  {vtype} {number!r}: ALL COMPOSITE ({len(legs)})")
        elif kinds == ["PLAIN"]:
            say(f"  {vtype} {number!r}: ALL PLAIN ({len(legs)})")
        else:
            say(f"  {vtype} {number!r}: NOT ONE CLASS ({kinds})")
    print(f"DAY {day}: {len(found)} vouchers, {len(table) - len(found)} on other days")
else:
    name = args[2]
    hits = [(v[0], amount, kind) for v in table.values() for leg, _, amount, kind in v[4] if leg == name]
    flagged = sorted(v[0] for v in table.values() if any(leg == name for leg, *_ in v[4])
                     and v[5] != "ISCANCELLED No ISOPTIONAL No")
    for guid, amount, kind in sorted(hits):
        say(f"  {guid} {amount!r} {kind}")
    if flagged:
        print(f"SUM {name}: NOT A FIGURE ({len(flagged)} vouchers not ISCANCELLED No and ISOPTIONAL No: {flagged})")
    elif not hits:
        print(f"SUM {name}: NO ENTRY")
    elif all(kind == "PLAIN" for _, _, kind in hits):
        total = sum(decimal.Decimal(amount) for _, amount, _ in hits)
        print(f"SUM {name}: {total} ({len(hits)} entries on {len({g for g, _, _ in hits})} vouchers)")
    else:
        print(f"SUM {name}: NOT A FIGURE ({sum(kind != 'PLAIN' for _, _, kind in hits)} entries not PLAIN)")
