#!/usr/bin/env python3
# cv.py: row 2480-CV's census reader. New for this row. It reads a census answer (a-*-C-*) by an XML parser (LAB-COMMON
# C4), never a text search; prints never more than CAP lines a call (AGENTS.md P5); and STOPs (exit 1) on an answer it
# cannot read. "No bytes" (a 0-byte file), "an empty collection" and "the request failed" are three different lines.
# Modes:
#   cv.py ANSWER        ANSWER: NO BYTES (a 0-byte file; whether the empty-answer exception covers it is decided by
#                       its headers file, as ROW.md says) | ANSWER: STATUS <s> (HEADER STATUS not 1, each LINEERROR
#                       printed whole) | ANSWER: VOUCHERS <n>, then one line a voucher: GUID, ALTERID, DATE. Exit 0.
#                       A STOP (exit 1): the file missing, not UTF-16LE, not XML, not one ENVELOPE with one DATA
#                       COLLECTION, a LINEERROR in an answer whose STATUS is 1 (a failure is never read as "no voucher"),
#                       an element in the collection other than VOUCHER, a voucher without exactly one GUID, one
#                       whole-number ALTERID and one DATE, or a GUID listed twice (#485 has not observed one; it is the
#                       coordinator's to read). A BOM is optional, and the STATUS text is read with its spaces stripped.
#   cv.py --same A B    SAME: EQUAL or SAME: DIFFER, with both sizes and sha256 (exit 0). A missing file is a STOP.
#   cv.py --own ID ANSWER BEFORE AFTER CREATED
#                       Is ANSWER exactly the voucher of ID's own invoice? BEFORE and AFTER are book-wide census answers
#                       (C-book) from before any invoice of this row and after the last one; CREATED is the ids whose
#                       invoice st.py passed as CREATED ONE, comma-separated, in names.tsv's order. The new vouchers are
#                       AFTER's less BEFORE's, by GUID. There must be one for each id of CREATED, each dated 20260801,
#                       and each AlterID its own; ordered by AlterID they are CREATED's invoices in the order sent, so
#                       ID's is the one at ID's place. OWN: YES (exit 0) if ANSWER holds that voucher and nothing else.
#                       OWN: NO (exit 3) names what ANSWER holds instead; OWN: NOT JUDGED (exit 3) names why the book
#                       census does not allow a judgement. A STOP of any file read is exit 1.
#                       It also prints, as an observation and never a check, whether that GUID is I-<ID>.xml's REMOTEID.
# Example: python3 -I cv.py --own cm a-r2a-C-v-cm.xml a-s2-C-book.xml a-s6ka-C-book.xml pl,cm,us
import hashlib, pathlib, re, sys
import xml.etree.ElementTree as ET
CAP = 200
DAY = "20260801"
here = pathlib.Path(__file__).resolve().parent
# st.py's, nm.py's and rf.py's: a character XML forbids, raw or as a reference, kept as U+FFFD, as master's sanitizer.
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

def raw_of(path):
    p = pathlib.Path(path)
    if not p.is_file():
        stop(f"{path} is missing")
    return p.read_bytes()

def read(path):
    """('bytes', None) for a 0-byte file, ('status', (s, errors)) or ('vouchers', [(guid, alterid, date)])."""
    raw = raw_of(path)
    if not raw:
        return "none", None
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
        found = {tag: child.findall(tag) for tag in ("GUID", "ALTERID", "DATE")}
        if any(len(v) != 1 for v in found.values()):
            stop(f"{path}: a voucher without exactly one GUID, ALTERID and DATE")
        guid = (found["GUID"][0].text or "").strip()
        alter = (found["ALTERID"][0].text or "").strip()
        date = (found["DATE"][0].text or "").strip()
        if not guid or not (alter.isdigit() and alter.isascii()) or not date:
            stop(f"{path}: a voucher with an empty GUID or DATE, or an ALTERID that is not a whole number")
        rows.append((guid, int(alter), date))
    if len({g for g, _, _ in rows}) != len(rows):
        stop(f"{path}: a GUID listed twice ({len(rows)} vouchers, {len({g for g, _, _ in rows})} GUIDs)")
    return "vouchers", rows

def show(path):
    kind, value = read(path)
    if kind == "none":
        say(f"{path}: ANSWER: NO BYTES")
    elif kind == "status":
        say(f"{path}: ANSWER: STATUS {value[0]}, LINEERROR {len(value[1])}")
        for text in value[1]:
            say(f"  LINEERROR: {text}")
    else:
        say(f"{path}: ANSWER: VOUCHERS {len(value)}")
        for guid, alter, date in value:
            say(f"  {guid} ALTERID {alter} DATE {date}")
    return kind, value

def census(path, role):
    kind, value = read(path)
    if kind != "vouchers":
        return None, f"{role} {path} is not a census with vouchers ({'no bytes' if kind == 'none' else 'STATUS ' + value[0]})"
    guids = [g for g, _, _ in value]
    if len(set(guids)) != len(guids):
        return None, f"{role} {path} lists a GUID twice"
    return value, None

args = sys.argv[1:]
if len(args) == 1 and not args[0].startswith("--"):
    show(args[0])
elif len(args) == 3 and args[0] == "--same":
    a, b = raw_of(args[1]), raw_of(args[2])
    verdict = "EQUAL" if a == b else "DIFFER"
    say(f"SAME: {verdict} ({len(a)} B sha256 {hashlib.sha256(a).hexdigest()[:16]}; "
        f"{len(b)} B sha256 {hashlib.sha256(b).hexdigest()[:16]})")
elif len(args) == 6 and args[0] == "--own":
    ident, answer, before, after, created = args[1:]
    order = [line.split("\t")[0] for line in (here / "names.tsv").read_text(encoding="utf-8").splitlines()]
    created = created.split(",")
    if ident not in created or any(i not in order for i in created) or len(set(created)) != len(created) \
            or created != sorted(created, key=order.index):
        stop("CREATED must hold ID, and only ids of names.tsv, once each, in its order")
    kind, held = show(answer)
    rows_before, why = census(before, "BEFORE")
    rows_after, why_after = (None, None) if why else census(after, "AFTER")
    why = why or why_after
    if not why:
        before_guids = {g for g, _, _ in rows_before}
        if not before_guids <= {g for g, _, _ in rows_after}:
            why = "a voucher of BEFORE is not in AFTER"
    if not why:
        new = sorted((r for r in rows_after if r[0] not in before_guids), key=lambda r: r[1])
        if len(new) != len(created):
            why = f"{len(new)} new vouchers in AFTER, for {len(created)} created invoices"
        elif any(d != DAY for _, _, d in new):
            why = f"a new voucher not dated {DAY}"
        elif len({a for _, a, _ in new}) != len(new):
            why = "two new vouchers share an AlterID"
    if why:
        say(f"OWN: NOT JUDGED ({why})")
        raise SystemExit(3)
    own = dict(zip(created, new))[ident]
    say(f"{ident}'s invoice: {own[0]} ALTERID {own[1]} DATE {own[2]} (new vouchers {len(new)})")
    remote = re.findall(r'REMOTEID="([^"]*)"', (here / f"I-{ident}.xml").read_bytes()[2:].decode("utf-16-le"))
    say(f"  observation: its GUID {'is' if remote == [own[0]] else 'is not'} I-{ident}.xml's REMOTEID")
    if kind == "vouchers" and held == [own]:
        say("OWN: YES")
        raise SystemExit(0)
    if kind == "vouchers":
        others = [g for g, _, _ in held if g != own[0]]
        say(f"OWN: NO (it holds {len(held)} vouchers, {len(others)} not this id's"
            f"{', ' + str(len([g for g in others if g in {r[0] for r in new}])) + ' of them another id' if others else ''})")
    else:
        say(f"OWN: NO ({'no bytes' if kind == 'none' else 'STATUS ' + held[0]})")
    raise SystemExit(3)
else:
    stop("usage: cv.py ANSWER | --same A B | --own ID ANSWER BEFORE AFTER CREATED")
