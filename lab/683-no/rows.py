#!/usr/bin/env python3
# rows.py: row 683-XF's content comparison of two answers of one request (a Trial Balance or an opening export).
# It reads each answer by an XML parser (LAB-COMMON C4), never a text search, and prints never more than CAP lines per
# call (AGENTS.md P5). Decoding and the forbidden-character rule are tb.py's, line for line.
# Why: row 683-XF's first sitting stopped on a Trial Balance whose rows held every value of row 683-TW's, with
# TBALOPENING written before TBALCLOSING instead of after it. Byte equality then failed on order alone. Master's
# Trial Balance parse reads each child of a row by its name, so its result does not depend on that order; this script
# compares what master reads, and reports the order apart from it.
# ROWS are the LEDGER elements that are children of ENVELOPE/BODY/DATA/COLLECTION, each keyed by its GUID text exactly
# as sent. A row's CONTENT is its attributes and every element below it: each element's tag, attributes and text
# exactly as sent (never trimmed), compared as a set at every level, so the order of siblings does not count.
# Whitespace-only text of an element that has children is layout, and is not compared. CMPINFO and everything outside
# the rows is not compared here (cmpinfo.py compares bytes).
# STOP, exit 1, never "nothing differs", if either answer: does not decode as UTF-16LE; carries a DOCTYPE; does not
# parse; has a LINEERROR or ERROR element; has a HEADER/STATUS other than one STATUS of exactly "1"; has no
# BODY/DATA/COLLECTION or more than one; has no LEDGER row; has a row with no GUID, more than one GUID, or an empty one;
# or has two rows with one GUID (any case).
# Output: each answer's size and sha256 and its row count; each GUID found in one answer only; for each row whose
# content differs, its NAME and each element path whose values differ (A -> B); then
#   CONTENT SAME or CONTENT DIFFER                    (the rows, as sets of elements),
#   ROW ORDER SAME or ROW ORDER DIFFERS               (the GUIDs in the order sent),
#   ELEMENT ORDER SAME or ELEMENT ORDER DIFFERS (n)   (the tags below each row, in the order sent; n rows differ),
# with each distinct pair of element orders printed once.
# Exit: 0 CONTENT SAME with both orders SAME; 3 CONTENT SAME with an order that DIFFERS; 2 CONTENT DIFFER; 1 STOP.
# Example: python3 -I rows.py a-twtb3-T-tb3-20250401-20250401.xml a-tb3a-T-tb3-20250401-20250401.xml
import hashlib, pathlib, re, sys
import xml.etree.ElementTree as ET
CAP = 400
FORBIDDEN = re.compile(r"&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));", re.I)
RAW_C0 = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]")
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

def reference_number(reference):  # &#n; or &#xh; as its decimal number (tb.py)
    body = reference[2:-1]
    return int(body[1:], 16) if body[:1] in "xX" else int(body)

def canon(element):  # the element's content, with the order of its children not counted
    children = list(element)
    text = element.text or ""
    if children and not text.strip():
        text = ""
    return (element.tag, tuple(sorted(element.attrib.items())), text,
            tuple(sorted(canon(child) for child in children)))

def leaves(element, path=""):  # (path, attributes, text) for every element below a row, in a stable order
    out = []
    for child in element:
        here = f"{path}/{child.tag}"
        if len(child):
            out += leaves(child, here)
        else:
            out.append((here, tuple(sorted(child.attrib.items())), child.text or ""))
    return sorted(out)

def order(element):  # the tags below the row, in the order sent
    return tuple(child.tag if not len(child) else (child.tag, order(child)) for child in element)

def load(path):
    raw = pathlib.Path(path).read_bytes()
    say(f"{path}: {len(raw)} B sha256 {hashlib.sha256(raw).hexdigest()}")
    body = raw[2:] if raw[:2] == b"\xff\xfe" else raw
    try:
        text = body.decode("utf-16-le")
    except UnicodeDecodeError as error:
        stop(f"{path} does not decode as UTF-16LE: {error}")
    if "<!DOCTYPE" in text.upper():
        stop(f"{path} carries a DOCTYPE")
    text = RAW_C0.sub("�", FORBIDDEN.sub(lambda m: f"�#{reference_number(m.group(0))};", text))
    try:
        root = ET.fromstring(text)
    except ET.ParseError as error:
        stop(f"{path} does not parse: {error}")
    if root.tag != "ENVELOPE":
        stop(f"{path}: the root is {root.tag}, not ENVELOPE")
    errors = [e for e in root.iter() if e.tag in ("LINEERROR", "ERROR")]
    if errors:
        stop(f"{path} carries {len(errors)} LINEERROR or ERROR element(s): the request failed")
    statuses = root.findall("HEADER/STATUS")
    if len(statuses) != 1 or (statuses[0].text or "") != "1":
        stop(f"{path}: HEADER/STATUS is not one STATUS of exactly '1'")
    collections = root.findall("BODY/DATA/COLLECTION")
    if len(collections) != 1:
        stop(f"{path}: {len(collections)} BODY/DATA/COLLECTION elements, not 1")
    rows, seen = {}, set()
    sequence = []
    for row in collections[0].findall("LEDGER"):
        guids = row.findall("GUID")
        if len(guids) != 1 or not (guids[0].text or ""):
            stop(f"{path}: a LEDGER row {row.get('NAME')!r} has {len(guids)} GUID elements or an empty one")
        guid = guids[0].text
        if guid.lower() in seen:
            stop(f"{path}: two rows share GUID {guid!r}")
        seen.add(guid.lower())
        rows[guid] = row
        sequence.append(guid)
    if not rows:
        stop(f"{path}: no LEDGER row")
    say(f"  rows: {len(rows)}")
    return rows, sequence

if len(sys.argv) != 3:
    stop("usage: rows.py A B")
(rows_a, seq_a), (rows_b, seq_b) = load(sys.argv[1]), load(sys.argv[2])
content_same = True
for guid in seq_a:
    if guid not in rows_b:
        content_same = False
        say(f"ONLY IN A: {guid} {rows_a[guid].get('NAME')!r}")
for guid in seq_b:
    if guid not in rows_a:
        content_same = False
        say(f"ONLY IN B: {guid} {rows_b[guid].get('NAME')!r}")
orders, order_rows = {}, 0
for guid in seq_a:
    if guid not in rows_b:
        continue
    a, b = rows_a[guid], rows_b[guid]
    if (a.tag, tuple(sorted(a.attrib.items()))) != (b.tag, tuple(sorted(b.attrib.items()))):
        content_same = False
        say(f"ROW {a.get('NAME')!r}: attributes {sorted(a.attrib.items())} -> {sorted(b.attrib.items())}")
    if canon(a) != canon(b):
        content_same = False
        la, lb = leaves(a), leaves(b)
        say(f"ROW {a.get('NAME')!r}: content differs")
        for item in sorted(set(la) ^ set(lb)):
            side = "A" if item in la else "B"
            say(f"  {side} only: {item[0]} {dict(item[1])} {item[2]!r}")
    if order(a) != order(b):
        order_rows += 1
        orders.setdefault((order(a), order(b)), []).append(a.get("NAME"))
for (oa, ob), names in orders.items():
    say(f"ELEMENT ORDER A: {oa}")
    say(f"ELEMENT ORDER B: {ob}")
    say(f"  in {len(names)} row(s): {names}")
say("CONTENT SAME" if content_same else "CONTENT DIFFER")
row_order_same = [g for g in seq_a if g in rows_b] == [g for g in seq_b if g in rows_a]
say("ROW ORDER SAME" if row_order_same else "ROW ORDER DIFFERS")
say("ELEMENT ORDER SAME" if not order_rows else f"ELEMENT ORDER DIFFERS ({order_rows})")
sys.exit(2 if not content_same else 0 if row_order_same and not order_rows else 3)
