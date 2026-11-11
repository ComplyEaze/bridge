#!/usr/bin/env python3
# vcmp.py: row 2511-P's window compare. Two movement answers (V- requests), VOUCHER row by VOUCHER row, matched by
# GUID, by an XML parser (LAB-COMMON C4), never a text search, with its detail lines capped (AGENTS.md P5).
# Usage: python3 -I vcmp.py A B [--only-a GUID]... [--only-b GUID]...
#   A is the earlier answer, B the later one. --only-a and --only-b name the GUIDs the step EXPECTs in that answer
#   only; a side not named EXPECTs none. Run it alone, never into a pipe, read its exit code, copy the whole output.
# Each answer first prints its bytes, sha256 and how many forbidden characters were replaced. Then it is a failed
# read, and vcmp.py prints STOP and exits 1, never "0 rows", when it: cannot be read; does not decode as UTF-16LE;
# does not parse; has HEADER/STATUS other than exactly 1, or none; holds any LINEERROR; has no BODY/DATA/COLLECTION;
# has a COLLECTION child that is not a VOUCHER; has no VOUCHER row; has a VOUCHER row whose GUID is absent, is
# not BOOK's GUID plus "-" and 8 lowercase hex digits, or is on two rows.
# Forbidden characters (C4's list, raw or as a reference) are never deleted: each becomes the visible token
# TOKEN followed by its two hex digits, so two answers that differ only there still differ.
# The compare: for each GUID in both answers, the two VOUCHER elements, each serialised by the parser without its
# own tail, must be equal. That is equality after the parse's own normalisations, which ROW.md names as accepted:
# a line break written CR LF reads as LF, an empty element reads the same in either form, and a character reads the
# same written raw or as a reference. It prints one counted line for the rows that are SAME, then a DIFFERS block
# per row that is not (each differing child by position, the character where its text first differs and the text
# around it, or that the row differs only between children), then every GUID found in one answer only.
# Detail lines stop after CAP; the result is decided before anything is printed, so the cap never changes it.
# The last line is the result:
#   COMMON SAME: every common row equal and the GUIDs in one answer only exactly as named (exit 0);
#   COMMON DIFFERS: a common row differs, or a GUID in one answer only was not named, or a named one is missing
#   (exit 2).
# Example: python3 -I vcmp.py a-mv0-V-mvy-20250401-20260331.xml a-mv1-V-mvy-20250401-20260331.xml --only-b <GUID>
import copy, hashlib, pathlib, re, sys
import xml.etree.ElementTree as ET
BOOK = "1bb48be5-baba-45b2-90d8-7593244915a9"  # BRIDGE FOREXGROUP LAB, as lm.py names it
GUID = re.compile(re.escape(BOOK) + r"-[0-9a-f]{8}")
CAP = 300
CUT = 300
BEFORE = 120
TOKEN = "�#"
FORBIDDEN = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));", re.I)
detail = 0

def stop(text):
    print(f"STOP: {text}")
    raise SystemExit(1)

def say_detail(text):
    global detail
    detail += 1
    if detail <= CAP:
        print(text)
    elif detail == CAP + 1:
        print(f"... more than {CAP} detail lines: the rest is not printed; the result below is unchanged")

def visible(match):
    found = match.group(0)
    if found.startswith("&#"):
        body = found[2:-1]
        code = int(body[1:], 16) if body[:1] in "xX" else int(body)
    else:
        code = ord(found)
    return f"{TOKEN}{code:02x}"

def load(path):
    try:
        raw = pathlib.Path(path).read_bytes()
    except OSError as error:
        stop(f"{path} cannot be read: {error}")
    print(f"{path}: {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}")
    body = raw[2:] if raw[:2] == b"\xff\xfe" else raw
    try:
        text = body.decode("utf-16-le")
    except UnicodeDecodeError as error:
        stop(f"{path} does not decode as UTF-16LE: {error}")
    text, replaced = FORBIDDEN.subn(visible, text)
    print(f"  forbidden characters replaced by {TOKEN!r}<hex>: {replaced}")
    try:
        root = ET.fromstring(text)
    except ET.ParseError as error:
        stop(f"{path} does not parse: {error}")
    status = root.find("HEADER/STATUS")
    status_text = status.text.strip() if status is not None and status.text else "<absent>"
    print(f"  HEADER/STATUS {status_text}")
    if status_text != "1":
        stop(f"{path}: HEADER/STATUS is {status_text}, not 1")
    errors = [e.text for e in root.iter("LINEERROR")]
    if errors:
        stop(f"{path}: LINEERROR {errors[0]!r} ({len(errors)} in all)")
    collection = root.find("BODY/DATA/COLLECTION")
    if collection is None:
        stop(f"{path} has no BODY/DATA/COLLECTION: the request failed")
    others = sorted({child.tag for child in collection if child.tag != "VOUCHER"})
    if others:
        stop(f"{path}: COLLECTION holds children other than VOUCHER: {others}")
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
    if not found:
        stop(f"{path}: no VOUCHER row: an empty answer is a failed read, never a match")
    for row in found:
        guid = leaf(row, "GUID").strip()
        if guid in ("<absent>", ""):
            stop(f"{path}: a VOUCHER row has no GUID")
        if not GUID.fullmatch(guid):
            stop(f"{path}: GUID {guid!r} is not the book's GUID, '-' and 8 lowercase hex digits")
        if guid in out:
            stop(f"{path}: GUID {guid} on two VOUCHER rows")
        out[guid] = row
    print(f"  VOUCHER rows: {len(found)}")
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

def explain(guid, a, b):
    say_detail(f"  DIFFERS {guid}")
    shown = False
    if (a.text or "") != (b.text or "") or a.attrib != b.attrib:
        shown = True
        say_detail(f"    the VOUCHER element's own text or attributes differ: A {a.attrib!r} {around(a.text or '', 0)}, "
                   f"B {b.attrib!r} {around(b.text or '', 0)}")
    left, right = list(a), list(b)
    if len(left) != len(right):
        shown = True
        say_detail(f"    children: {len(left)} in A, {len(right)} in B")
    for index in range(max(len(left), len(right))):
        one = left[index] if index < len(left) else None
        two = right[index] if index < len(right) else None
        one_text = text_of(one) if one is not None else "<absent>"
        two_text = text_of(two) if two is not None else "<absent>"
        if one_text != two_text:
            shown = True
            at = first_difference(one_text, two_text)
            say_detail(f"    child {index}: {one.tag if one is not None else '<absent>'} in A, "
                       f"{two.tag if two is not None else '<absent>'} in B; first difference at character {at}")
            say_detail(f"      A {around(one_text, at)}")
            say_detail(f"      B {around(two_text, at)}")
    if not shown:
        say_detail("    it differs only in the text between its children")

def described(guid, row):
    return (f"{guid} {leaf(row, 'DATE').strip()} {leaf(row, 'VOUCHERTYPENAME')!r} "
            f"{leaf(row, 'VOUCHERNUMBER')!r} {leaf(row, 'NARRATION')!r}")

def arguments(argv):
    files, named = [], {"--only-a": set(), "--only-b": set()}
    index = 0
    while index < len(argv):
        word = argv[index]
        if word in named:
            if index + 1 >= len(argv):
                stop(f"{word} needs a GUID")
            guid = argv[index + 1]
            if not GUID.fullmatch(guid):
                stop(f"{word} {guid!r} is not the book's GUID, '-' and 8 lowercase hex digits")
            named[word].add(guid)
            index += 2
        elif word.startswith("--"):
            stop(f"unknown option {word}")
        else:
            files.append(word)
            index += 1
    if len(files) != 2:
        stop("usage: vcmp.py A B [--only-a GUID]... [--only-b GUID]...")
    return files, named["--only-a"], named["--only-b"]

def main():
    (path_a, path_b), want_a, want_b = arguments(sys.argv[1:])
    a, b = rows(path_a), rows(path_b)
    common = sorted(set(a) & set(b))
    differs = [guid for guid in common if text_of(a[guid]) != text_of(b[guid])]
    only_a, only_b = set(a) - set(b), set(b) - set(a)
    unnamed_a, unnamed_b = only_a - want_a, only_b - want_b
    missing_a, missing_b = want_a - only_a, want_b - only_b
    failed = bool(differs or unnamed_a or unnamed_b or missing_a or missing_b)

    print(f"  SAME: {len(common) - len(differs)} of {len(common)} common rows")
    for guid in differs:
        explain(guid, a[guid], b[guid])
    for label, found, wanted, table in (("A", only_a, want_a, a), ("B", only_b, want_b, b)):
        for guid in sorted(found):
            say_detail(f"  only in {label} ({'named' if guid in wanted else 'NOT NAMED'}): {described(guid, table[guid])}")
        for guid in sorted(wanted - found):
            say_detail(f"  named as only in {label}, NOT FOUND there: {guid}")
    counts = (f"common {len(common)}, differ {len(differs)}, only in A {len(only_a)} ({len(unnamed_a)} not named), "
              f"only in B {len(only_b)} ({len(unnamed_b)} not named), named but missing {len(missing_a) + len(missing_b)}")
    if failed:
        print(f"COMMON DIFFERS ({counts})")
        raise SystemExit(2)
    print(f"COMMON SAME ({counts})")

main()
