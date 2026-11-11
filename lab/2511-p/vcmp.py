#!/usr/bin/env python3
# vcmp.py: row 2511-P's window compare. Two movement answers (V- requests), VOUCHER row by VOUCHER row, matched by
# GUID, by an XML parser (LAB-COMMON C4), never a text search, and never more than CAP lines per call (AGENTS.md P5).
# Usage: python3 -I vcmp.py A B       (A the earlier answer, B the later one)
# Each answer prints HEADER/STATUS and every LINEERROR first. An answer that does not decode or parse, has no
# BODY/DATA/COLLECTION, has a VOUCHER row with no GUID or a GUID on two rows, or a GUID that does not begin with
# BOOK, is a failed read: vcmp.py prints STOP and exits 1, never "0 rows".
# Then, for each GUID in both answers, sorted: SAME when the two VOUCHER elements serialise to equal text (every
# attribute, child, text and the whitespace between children; the element's own tail is set aside), else DIFFERS,
# with each direct child that differs, by position: its tag in A and in B, the character at which the two
# serialised texts first differ, and each text from BEFORE characters ahead of that point, cut to CUT characters. Then every GUID found in one answer only, with its DATE, VOUCHERTYPENAME, VOUCHERNUMBER and
# NARRATION. It ends with one line: COMMON SAME (exit 0) or COMMON DIFFERS (exit 2), with the three counts.
# A GUID in one answer only does not by itself make COMMON DIFFERS: the row's step says what it EXPECTs there.
# Example: python3 -I vcmp.py a-mv0-V-mvy-20250401-20260331.xml a-mv1-V-mvy-20250401-20260331.xml
import copy, pathlib, re, sys
import xml.etree.ElementTree as ET
BOOK = "1bb48be5-baba-45b2-90d8-7593244915a9"  # BRIDGE FOREXGROUP LAB, as lm.py names it
CAP = 400
CUT = 300
BEFORE = 120
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

def load(path):
    try:
        raw = pathlib.Path(path).read_bytes()
    except OSError as error:
        stop(f"{path} cannot be read: {error}")
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
    say(f"{path}: HEADER/STATUS {status.text.strip() if status is not None and status.text else '<absent>'}")
    for error in root.iter("LINEERROR"):
        say(f"  LINEERROR {error.text!r}")
    collection = root.find("BODY/DATA/COLLECTION")
    if collection is None:
        stop(f"{path} has no BODY/DATA/COLLECTION: the request failed")
    return collection

def leaf(row, tag):
    found = row.find(tag)
    if found is None:
        return "<absent>"
    return found.text or ""

def text_of(element):
    """The element serialised, without its own tail."""
    alone = copy.deepcopy(element)
    alone.tail = None
    return ET.tostring(alone, encoding="unicode")

def rows(path):
    out = {}
    found = load(path).findall("VOUCHER")
    for row in found:
        guid = leaf(row, "GUID").strip()
        if guid in ("<absent>", ""):
            stop(f"{path}: a VOUCHER row has no GUID")
        if not guid.startswith(BOOK):
            stop(f"{path}: GUID {guid} does not begin with the book's GUID")
        if guid in out:
            stop(f"{path}: GUID {guid} on two VOUCHER rows")
        out[guid] = row
    say(f"  VOUCHER rows: {len(found)}")
    return out

def first_difference(one, two):
    for index, (x, y) in enumerate(zip(one, two)):
        if x != y:
            return index
    return min(len(one), len(two))

def around(text, at):
    start = max(0, at - BEFORE)
    part = text[start:start + CUT]
    return ("..." if start else "") + repr(part) + ("..." if start + CUT < len(text) else "") + f" ({len(text)} characters)"

def children_differ(a, b):
    left, right = list(a), list(b)
    if len(left) != len(right):
        say(f"    children: {len(left)} in A, {len(right)} in B")
    for index in range(max(len(left), len(right))):
        one = left[index] if index < len(left) else None
        two = right[index] if index < len(right) else None
        one_text = text_of(one) if one is not None else "<absent>"
        two_text = text_of(two) if two is not None else "<absent>"
        if one_text != two_text:
            say(f"    child {index}: {one.tag if one is not None else '<absent>'} in A, "
                f"{two.tag if two is not None else '<absent>'} in B")
            at = first_difference(one_text, two_text)
            say(f"      first difference at character {at}")
            say(f"      A {around(one_text, at)}")
            say(f"      B {around(two_text, at)}")
    if (a.text or "") != (b.text or "") or a.attrib != b.attrib:
        say(f"    the VOUCHER element's own text or attributes differ: A {a.attrib!r} {around(a.text or '', 0)}, "
            f"B {b.attrib!r} {around(b.text or '', 0)}")

def only(label, guid, row):
    say(f"  only in {label}: {guid} {leaf(row, 'DATE').strip()} {leaf(row, 'VOUCHERTYPENAME')!r} "
        f"{leaf(row, 'VOUCHERNUMBER')!r} {leaf(row, 'NARRATION')!r}")

def main():
    if len(sys.argv) != 3:
        stop("usage: vcmp.py A B")
    a, b = rows(sys.argv[1]), rows(sys.argv[2])
    common = sorted(set(a) & set(b))
    differs = 0
    for guid in common:
        if text_of(a[guid]) == text_of(b[guid]):
            say(f"  SAME {guid}")
        else:
            differs += 1
            say(f"  DIFFERS {guid}")
            children_differ(a[guid], b[guid])
    for guid in sorted(set(a) - set(b)):
        only("A", guid, a[guid])
    for guid in sorted(set(b) - set(a)):
        only("B", guid, b[guid])
    counts = f"common {len(common)}, only in A {len(set(a) - set(b))}, only in B {len(set(b) - set(a))}"
    if differs:
        say(f"COMMON DIFFERS: {differs} of {len(common)} common rows differ ({counts})")
        raise SystemExit(2)
    say(f"COMMON SAME ({counts})")

main()
