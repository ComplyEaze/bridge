#!/usr/bin/env python3
# wx.py: row 2480-MW's reader for its voucher reads (the Vb-, Vm- and Vw- answers): row 2480-MX's, with the one change
# marked MW below (MX's wx.py STOPped on a bill allocation that holds only AMOUNT, the shape master's movement read is
# answered with, ledger-scoped-vouchers/movement-dn-party-month.answer included). It reads
# each answer by an XML parser (LAB-COMMON C4), never a text search; prints never more than CAP lines a call
# (AGENTS.md P5); and STOPs (exit 1) on an answer it cannot read. It never prints a VOUCHERKEY's value.
#
# What an answer is, decided once, the same way in every mode:
#   EMPTY ANSWER   no bytes (a 0-byte file, or no file), and its headers file a-<label>-<file>.headers ends with an
#                  HTTP 200 status line and exactly one CONTENT-LENGTH, 0. Tally answered, with nothing.
#   a failed send  no bytes, and the headers file missing, or its last status line not 200, or its CONTENT-LENGTH not
#                  exactly one 0. A STOP: it is never read as an empty answer, nor as "no voucher".
#   STATUS <s>     HEADER STATUS not 1, each LINEERROR printed whole.
#   VOUCHERS <n>   STATUS 1 and one DATA COLLECTION of n VOUCHER rows (n may be 0: an empty collection).
#   A STOP besides: not UTF-16LE, not XML, not one ENVELOPE with one HEADER STATUS; a LINEERROR under STATUS 1; an
#   element in the collection other than VOUCHER; a voucher without exactly one VOUCHERKEY (a whole number), GUID,
#   DATE and VOUCHERNUMBER; a GUID that does not begin with BOOK's (C4); a field below named twice in one voucher; an
#   entry without exactly one LEDGERNAME; a bill allocation without exactly one NAME, unless it is empty or holds one
#   AMOUNT and nothing else (MW); a VOUCHERKEY or GUID listed twice.
#   A bill allocation is read as one of three (MW): empty (no child), skipped, as MX did; AMOUNT-ONLY (exactly one
#   child, AMOUNT, itself with no child), kept as a bill with no NAME and no BILLTYPE, printed <no name>; or a bill
#   (one NAME, any others).
#
# Modes:
#   wx.py ANSWER     the line above, then for VOUCHERS one line a voucher: number, date, AlterID, entry count.
#   wx.py --same A B SAME: EQUAL or SAME: DIFFER, with both sizes and sha256 (exit 0). Two empty answers are EQUAL; a
#                    failed send is a STOP.
#   wx.py --match ID FIELDS SCOPED BOOK
#                    ONE line: does SCOPED, the ledger-scoped read (FIELDS movement for a Vm- answer, window for a Vw-
#                    answer) of ID's ledger, hold exactly the vouchers of BOOK, the book-wide read (Vb-book) of the
#                    same day, that carry an entry on ID's ledger, each the same? Vouchers are matched by VOUCHERKEY.
#                    "The same" is, for each matched voucher:
#                      - its own fields OWN (and, for window, OWN_WINDOW), each the same text (names exactly, the
#                        others trimmed), a field absent from both counting as the same;
#                      - its lines on ID's ledger, in order: each one's LEDGERNAME, ISDEEMEDPOSITIVE and AMOUNT (the
#                        movement read's three entry fields), and, for window, each line's bill allocations, each as
#                        NAME, BILLTYPE and AMOUNT.
#                    BOOK must hold exactly one voucher on ID's ledger, numbered BP/26-27/<ID's number> and dated
#                    20260801: the invoice part A created. Otherwise the line is NOT JUDGED, whatever SCOPED holds.
#                    The line is one of:
#                      MATCH <id> <fields>: EQUAL (...)          exit 0
#                      MATCH <id> <fields>: DIFFER (...)         exit 3: what is missing, what is extra (and whether
#                                                                 an extra voucher is another id's), which field
#                      MATCH <id> <fields>: EMPTY ANSWER         exit 3
#                      MATCH <id> <fields>: STATUS <s>           exit 3
#                      MATCH <id> <fields>: NOT JUDGED (why)     exit 3
#                    A STOP of either file is exit 1.
# Example: python3 -I wx.py --match ga window a-w2a-Vw-v-ga.xml a-k7-Vb-book.xml
import hashlib, pathlib, re, sys
import xml.etree.ElementTree as ET
CAP = 200
BOOK = "6b43e498-430c-4d5c-bfef-d32e2ab93c85"  # BRIDGE PILOT LAB, as nm.py, st.py and count20.py hold it
DAY = "20260801"
OWN = ("DATE", "VOUCHERNUMBER", "VOUCHERTYPENAME", "PARTYLEDGERNAME", "NARRATION", "GUID", "ALTERID", "MASTERID",
       "ISCANCELLED", "ISOPTIONAL")  # the voucher fields of master's movement FETCH
OWN_WINDOW = ("ISPOSTDATED", "REFERENCE", "ISINVOICE", "PARTYGSTIN")  # and the window FETCH's others
NAMES = ("PARTYLEDGERNAME", "NARRATION", "LEDGERNAME", "NAME")  # never trimmed (C4)
here = pathlib.Path(__file__).resolve().parent
# st.py's, nm.py's, rf.py's and row 2480-CV's cv.py's: a character XML forbids, raw or as a reference, kept as U+FFFD.
FORBIDDEN = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));", re.I)
lines = 0
where = ""  # the answer being read, for a STOP's message

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

def text_of(element, tag):
    found = element.findall(tag)
    if len(found) > 1:
        stop(f"{where}: {tag} named {len(found)} times in one element")
    if not found:
        return None
    value = found[0].text or ""
    return value if tag in NAMES else value.strip()

def bytes_or_empty(path):
    """The answer's bytes, or None for an empty answer. A failed send is a STOP."""
    p = pathlib.Path(path)
    raw = p.read_bytes() if p.is_file() else b""
    if raw:
        return raw
    head = pathlib.Path(str(path) + ".headers")
    if not head.is_file():
        stop(f"{path}: no bytes and no headers file: the send failed or was not made, not an empty answer")
    blocks = [b for b in re.split(r"\r?\n\r?\n", head.read_text("latin-1")) if b.strip()]
    last = blocks[-1].splitlines() if blocks else []
    status = last[0].split() if last else []
    length = [l.split(":", 1)[1].strip() for l in last[1:] if l.lower().startswith("content-length:")]
    if len(status) < 2 or not status[0].startswith("HTTP/") or status[1] != "200" or length != ["0"]:
        stop(f"{path}: no bytes, and the headers say {' '.join(status) or 'nothing'}, CONTENT-LENGTH {length}: "
             "the send failed, not an empty answer")
    return None

def read(path):
    """('empty', None) | ('status', (s, errors)) | ('vouchers', [voucher dict, ...])."""
    global where
    where = path
    raw = bytes_or_empty(path)
    if raw is None:
        return "empty", None
    if raw[:2] == b"\xff\xfe":
        raw = raw[2:]
    try:
        text = raw.decode("utf-16-le")
    except UnicodeDecodeError as error:
        stop(f"{path} does not decode as UTF-16LE: {error}")
    try:
        root = ET.fromstring(FORBIDDEN.sub("�", text))
    except ET.ParseError as error:
        stop(f"{path} does not parse: {error}")
    if root.tag != "ENVELOPE":
        stop(f"{path}: the root is {root.tag}, not ENVELOPE")
    status = [(e.text or "").strip() for e in root.findall("./HEADER/STATUS")]
    if len(status) != 1:
        stop(f"{path}: {len(status)} HEADER STATUS elements, not one")
    errors = [" ".join((e.text or "").split()) for e in root.iter("LINEERROR")]
    if status[0] != "1":
        return "status", (status[0], errors)
    if errors:
        stop(f"{path}: STATUS 1 with {len(errors)} LINEERROR: " + " | ".join(errors)[:300])
    collections = root.findall("./BODY/DATA/COLLECTION")
    if len(collections) != 1 or len(list(root.iter("COLLECTION"))) != 1:
        stop(f"{path}: not one DATA COLLECTION")
    rows = []
    for child in collections[0]:
        if child.tag != "VOUCHER":
            stop(f"{path}: a {child.tag} element in the collection")
        key, guid = text_of(child, "VOUCHERKEY"), text_of(child, "GUID")
        number, date = text_of(child, "VOUCHERNUMBER"), text_of(child, "DATE")
        if not key or not (key.isdigit() and key.isascii()) or not guid or not number or not date:
            stop(f"{path}: a voucher without one whole-number VOUCHERKEY, one GUID, one VOUCHERNUMBER and one DATE")
        if not guid.startswith(BOOK + "-"):
            stop(f"{path}: a voucher GUID that is not the book's")
        own = {tag: text_of(child, tag) for tag in OWN + OWN_WINDOW}
        entries = []
        for entry in child.findall("ALLLEDGERENTRIES.LIST"):
            ledger = text_of(entry, "LEDGERNAME")
            if ledger is None:
                stop(f"{path}: {number}: an entry without one LEDGERNAME")
            bills = []
            for bill in entry.findall("BILLALLOCATIONS.LIST"):
                if not len(bill):
                    continue  # Tally lists an empty BILLALLOCATIONS.LIST on a line with no bill
                if [c.tag for c in bill] == ["AMOUNT"] and not len(bill[0]):  # MW: the movement read's shape: one AMOUNT, nothing else
                    bills.append((None, None, text_of(bill, "AMOUNT")))
                    continue
                name = text_of(bill, "NAME")
                if name is None:
                    stop(f"{path}: {number}: a bill allocation without one NAME")
                bills.append((name, text_of(bill, "BILLTYPE"), text_of(bill, "AMOUNT")))
            entries.append({"line": (ledger, text_of(entry, "ISDEEMEDPOSITIVE"), text_of(entry, "AMOUNT")),
                            "bills": bills})
        rows.append({"key": key, "number": number, "date": date, "own": own, "entries": entries})
    if len({r["key"] for r in rows}) != len(rows) or len({r["own"]["GUID"] for r in rows}) != len(rows):
        stop(f"{path}: a VOUCHERKEY or a GUID listed twice ({len(rows)} vouchers)")
    return "vouchers", rows

def show(path):
    kind, value = read(path)
    if kind == "empty":
        say(f"{path}: ANSWER: EMPTY ANSWER (HTTP 200, CONTENT-LENGTH 0)")
    elif kind == "status":
        say(f"{path}: ANSWER: STATUS {value[0]}, LINEERROR {len(value[1])}")
        for text in value[1]:
            say(f"  LINEERROR: {text}")
    else:
        say(f"{path}: ANSWER: VOUCHERS {len(value)}")
        for r in value:
            say(f"  {r['number']} DATE {r['date']} ALTERID {r['own']['ALTERID']} entries {len(r['entries'])}")
    return kind, value

def table():
    rows = [line.split("\t") for line in (here / "names.tsv").read_text(encoding="utf-8").splitlines()]
    return {i: (number, name) for i, number, name in rows}

def match(ident, fields, scoped, book):
    names = table()
    if ident not in names or fields not in ("movement", "window"):
        stop("usage: --match ID FIELDS SCOPED BOOK, with ID in names.tsv and FIELDS movement or window")
    number, name = names[ident]
    head = f"MATCH {ident} {fields}"
    book_kind, book_rows = read(book)
    kind, rows = read(scoped)
    if book_kind != "vouchers":
        return f"{head}: NOT JUDGED (the book-wide read is {'an empty answer' if book_kind == 'empty' else 'STATUS ' + book_rows[0]})", 3
    on = lambda r, who: [e for e in r["entries"] if e["line"][0] == who]
    expected = [r for r in book_rows if on(r, name)]
    if len(expected) != 1:
        return f"{head}: NOT JUDGED (the book-wide read holds {len(expected)} vouchers on this ledger, not one)", 3
    if expected[0]["number"] != f"BP/26-27/{number}" or expected[0]["date"] != DAY:
        return f"{head}: NOT JUDGED (the book-wide read's voucher on this ledger is not BP/26-27/{number} of {DAY})", 3
    if kind == "empty":
        return f"{head}: EMPTY ANSWER", 3
    if kind == "status":
        return f"{head}: STATUS {rows[0]}", 3
    own_fields = OWN + (OWN_WINDOW if fields == "window" else ())
    want = {r["key"]: r for r in expected}
    got = {r["key"]: r for r in rows}
    missing = [want[k]["number"] for k in want if k not in got]
    extra = [got[k] for k in got if k not in want]
    others = {n for i, (_, n) in names.items() if i != ident}
    extra_ids = sum(1 for r in extra if any(on(r, n) for n in others))
    differs = []
    for k in want.keys() & got.keys():
        a, b = want[k], got[k]
        differs += [f"{a['number']} {f}" for f in own_fields if a["own"][f] != b["own"][f]]
        la, lb = on(a, name), on(b, name)
        if [e["line"] for e in la] != [e["line"] for e in lb]:
            differs.append(f"{a['number']} its lines on the ledger ({len(la)} in the book, {len(lb)} here, not the same)")
        elif fields == "window" and [e["bills"] for e in la] != [e["bills"] for e in lb]:
            differs.append(f"{a['number']} the bill allocations of its lines on the ledger")
    if missing or extra or differs:
        why = []
        if missing:
            why.append(f"missing {', '.join(missing)}")
        if extra:
            why.append(f"{len(extra)} other vouchers, {extra_ids} of them another id's")
        if differs:
            why.append("differs: " + "; ".join(differs[:6]) + (f"; and {len(differs) - 6} more" if len(differs) > 6 else ""))
        return f"{head}: DIFFER ({'; '.join(why)})", 3
    line = on(expected[0], name)[0]
    bills = ", ".join(f"{'<no name>' if n is None else n} {t} {a}" for n, t, a in line["bills"]) if fields == "window" else ""
    return (f"{head}: EQUAL (1 voucher, {expected[0]['number']} of {DAY}, matched by VOUCHERKEY; its line "
            f"{line['line'][1]} {line['line'][2]}{'; bill ' + bills if bills else ''}; "
            f"{len(own_fields)} own fields equal)"), 0

args = sys.argv[1:]
if len(args) == 1 and not args[0].startswith("--"):
    show(args[0])
elif len(args) == 3 and args[0] == "--same":
    a, b = bytes_or_empty(args[1]) or b"", bytes_or_empty(args[2]) or b""
    verdict = "EQUAL" if a == b else "DIFFER"
    say(f"SAME: {verdict} ({len(a)} B sha256 {hashlib.sha256(a).hexdigest()[:16]}; "
        f"{len(b)} B sha256 {hashlib.sha256(b).hexdigest()[:16]})")
elif len(args) == 5 and args[0] == "--match":
    text, code = match(*args[1:])
    say(text)
    raise SystemExit(code)
else:
    stop("usage: wx.py ANSWER | --same A B | --match ID FIELDS SCOPED BOOK")
