#!/usr/bin/env python3
# lm.py: row 683-LM's counter. It prints what ROW.md records of each answer, by an XML parser (LAB-COMMON C4),
# never a text search, and never more than CAP lines per call (AGENTS.md P5).
# The kind comes from the request's name inside the answer's name (a-<label>-<request>):
#   M- marks: each COMPANY row's GUID, whether it is BOOK's, ALTVCHID and ALTMSTID.
#   C- currencies: how many CURRENCY rows, and each one's NAME, MAILINGNAME and DECIMALPLACES.
#   V- a movement window: how many VOUCHER rows, then each by GUID with DATE, VOUCHERTYPENAME, VOUCHERNUMBER, and
#      each direct ALLLEDGERENTRIES.LIST child's LEDGERNAME, ISDEEMEDPOSITIVE and AMOUNT text.
#   X- a ledger export: how many LEDGER rows, then each by NAME with its OPENINGBALANCE text and how it reads (below).
# Every answer prints HEADER/STATUS and every LINEERROR first. An answer that does not parse, or has no
# BODY/DATA/COLLECTION, is a failed request: lm.py prints STOP and exits 1, never "0 rows".
# Modes:
#   --same-row M0 M1: each holds exactly one COMPANY row, BOOK's, with equal ALTVCHID and ALTMSTID; else MARKS DIFFER
#      (exit 2). One row that is not BOOK's, or any other row count, is STOP (exit 1).
#   --capture MV CAPTURE: MV (a V answer) against a captured `vouchers` answer of the same day, voucher by voucher by
#      GUID: for each GUID in both, SAME if the entry lists (LEDGERNAME, AMOUNT text, in order) are equal, else DIFFERS
#      with both lists; then every GUID found in one only. Ends with one line: CAPTURE SAME or CAPTURE DIFFERS.
#   --ledger MVL MV NAME: MVL's vouchers against MV's vouchers that hold an entry on NAME: the GUID sets must be equal,
#      and for each GUID, the entries on NAME (AMOUNT text, in order) equal. Ends with LEDGER SAME or LEDGER DIFFERS.
#   --tie LX1 MV LX2: for every ledger NAME in LX1 or LX2: LX1's opening base part + the sum of MV's entry base parts
#      on that ledger, against LX2's opening base part. Each ledger prints TIES, DIFFERS (with LX2 minus the sum), or
#      NOT A FIGURE (naming which text did not read). A ledger in one export only is NOT A FIGURE.
# The base part: a plain figure (-?digits[.digits]) is itself; a composite (text holding " = ") is the figure after
# its last "=", which must read -?<BASE> digits[.digits] with optional spaces, where BASE is the book's base symbol;
# anything else, empty included, is NOT A FIGURE. Absent elements print <absent>; empty ones print ''.
# Example: python3 -I lm.py a-m0-M-marks.xml
import pathlib, re, sys
from decimal import Decimal
import xml.etree.ElementTree as ET
BOOK = "1bb48be5-baba-45b2-90d8-7593244915a9"  # BRIDGE FOREXGROUP LAB, as forex-register-book-extent reads it
BASE = "₹"  # the book's base currency master NAME, as currency_masters_forex_72 reads it
CAP = 400
FORBIDDEN = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));", re.I)
PLAIN = re.compile(r"-?\d+(?:\.\d+)?")
BASED = re.compile(r"(-?)\s*" + BASE + r"\s*(\d+(?:\.\d+)?)")
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

def kind(path):
    name = pathlib.Path(path).name
    match = re.match(r"a-[^-]+-([A-Z])-", name)
    if not match:
        stop(f"{name}: not an answer name a-<label>-<request>")
    return match.group(1)

def marks(path):
    rows = load(path).findall("COMPANY")
    out = [(leaf(r, "GUID"), leaf(r, "ALTVCHID"), leaf(r, "ALTMSTID")) for r in rows]
    say(f"  COMPANY rows: {len(rows)}")
    for guid, v, m in out:
        say(f"  GUID {guid} (the book's: {'yes' if guid == BOOK else 'NO'}) ALTVCHID {v} ALTMSTID {m}")
    return out

def entries(row):
    return [(leaf(e, "LEDGERNAME", False), leaf(e, "ISDEEMEDPOSITIVE"), leaf(e, "AMOUNT"))
            for e in row.findall("ALLLEDGERENTRIES.LIST")]

def vouchers(path, show=True):
    rows = load(path).findall("VOUCHER")
    out = {}
    for r in rows:
        guid = leaf(r, "GUID")
        if guid in out:
            stop(f"{path}: GUID {guid} on two VOUCHER rows")
        out[guid] = (leaf(r, "DATE"), leaf(r, "VOUCHERTYPENAME"), leaf(r, "VOUCHERNUMBER"), entries(r))
    if show:
        say(f"  VOUCHER rows: {len(rows)}")
        for guid, (date, vtype, number, legs) in sorted(out.items()):
            say(f"  {guid} {date} {vtype} {number!r}: {len(legs)} entries")
            for name, deemed, amount in legs:
                say(f"    {name!r} {deemed} {amount!r}")
    return out

def base(text):
    """The base part of an AMOUNT or OPENINGBALANCE text, or None when it is not a figure."""
    if text in ("<absent>", ""):
        return None
    if PLAIN.fullmatch(text):
        return Decimal(text)
    if " = " in text:
        match = BASED.fullmatch(text.rsplit("=", 1)[1].strip())
        if match:
            return Decimal(match.group(1) + match.group(2))
    return None

def ledgers(path, show=True):
    rows = load(path).findall("LEDGER")
    out = {}
    for r in rows:
        name = r.get("NAME") if r.get("NAME") is not None else leaf(r, "NAME", False)
        if name in out:
            stop(f"{path}: ledger {name!r} on two LEDGER rows")
        out[name] = leaf(r, "OPENINGBALANCE")
    if show:
        say(f"  LEDGER rows: {len(rows)}")
        for name, opening in sorted(out.items()):
            figure = base(opening)
            say(f"  {name!r} OPENINGBALANCE {opening!r} reads {'NOT A FIGURE' if figure is None else figure}")
    return out

def legs_only(legs):
    return [(name, amount) for name, _, amount in legs]

args = sys.argv[1:]
if not args:
    stop("no answer named")
if args[0] == "--same-row":
    a, b = marks(args[1]), marks(args[2])
    if len(a) != 1 or len(b) != 1 or a[0][0] != BOOK or b[0][0] != BOOK:
        stop("each marks answer must hold exactly one COMPANY row, the book's")
    if a != b:
        print(f"MARKS DIFFER: {a[0]} -> {b[0]}")
        raise SystemExit(2)
    print("SAME MARKS")
elif args[0] == "--capture":
    mv, cap = vouchers(args[1], False), vouchers(args[2], False)
    same = True
    for guid in sorted(set(mv) & set(cap)):
        a, b = legs_only(mv[guid][3]), legs_only(cap[guid][3])
        if a == b:
            say(f"  {guid}: SAME ({len(a)} entries)")
        else:
            same = False
            say(f"  {guid}: DIFFERS\n    movement {a}\n    capture  {b}")
    for guid in sorted(set(mv) ^ set(cap)):
        same = False
        say(f"  {guid}: in {'the movement window' if guid in mv else 'the capture'} only")
    print(f"vouchers: movement {len(mv)}, capture {len(cap)}")
    print("CAPTURE SAME" if same else "CAPTURE DIFFERS")
elif args[0] == "--ledger":
    mvl, mv, name = vouchers(args[1], False), vouchers(args[2], False), args[3]
    want = {g: v for g, v in mv.items() if any(leg[0] == name for leg in v[3])}
    same = set(mvl) == set(want)
    say(f"  ledger {name!r}: ledger-scoped {len(mvl)} vouchers, book-scoped with an entry on it {len(want)}")
    for guid in sorted(set(mvl) | set(want)):
        a = [amount for leg, _, amount in mvl[guid][3] if leg == name] if guid in mvl else None
        b = [amount for leg, _, amount in want[guid][3] if leg == name] if guid in want else None
        if a != b:
            same = False
        say(f"  {guid}: ledger-scoped {a} book-scoped {b}")
    print("LEDGER SAME" if same else "LEDGER DIFFERS")
elif args[0] == "--tie":
    lx1, mv, lx2 = ledgers(args[1], False), vouchers(args[2], False), ledgers(args[3], False)
    moves, unread = {}, {}
    for guid, (_, _, _, legs) in mv.items():
        for name, _, amount in legs:
            figure = base(amount)
            if figure is None:
                unread.setdefault(name, []).append(f"{guid} {amount!r}")
            moves[name] = moves.get(name, Decimal(0)) + (figure or Decimal(0))
    for name in sorted(set(lx1) | set(lx2) | set(moves)):
        if name not in lx1 or name not in lx2:
            say(f"  {name!r}: NOT A FIGURE (in {'lx2' if name in lx2 else 'lx1' if name in lx1 else 'the window'} only)")
            continue
        o1, o2 = base(lx1[name]), base(lx2[name])
        if o1 is None or o2 is None or name in unread:
            what = [f"lx1 {lx1[name]!r}"] if o1 is None else []
            what += [f"lx2 {lx2[name]!r}"] if o2 is None else []
            what += [f"entry {e}" for e in unread.get(name, [])]
            say(f"  {name!r}: NOT A FIGURE ({'; '.join(what)})")
            continue
        move = moves.get(name, Decimal(0))
        if o1 + move == o2:
            say(f"  {name!r}: TIES ({o1} + {move} = {o2})")
        else:
            say(f"  {name!r}: DIFFERS by {o2 - (o1 + move)} ({o1} + {move} = {o1 + move}, lx2 {o2})")
else:
    for path in args:
        k = kind(path)
        if k == "M":
            marks(path)
        elif k == "C":
            rows = load(path).findall("CURRENCY")
            say(f"  CURRENCY rows: {len(rows)}")
            for r in rows:
                say(f"  NAME {r.get('NAME') if r.get('NAME') is not None else leaf(r, 'NAME', False)!r} "
                    f"MAILINGNAME {leaf(r, 'MAILINGNAME', False)!r} DECIMALPLACES {leaf(r, 'DECIMALPLACES')}")
        elif k == "V":
            vouchers(path)
        elif k == "X":
            ledgers(path)
        else:
            stop(f"{path}: no kind {k}")
