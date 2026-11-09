#!/usr/bin/env python3
# n3.py: row N3's helpers (lab item #2433, not product code). Run with python -I. Exit 3 = STOP, 2 = usage.
#   n3.py pattern                  prints both names this row types, with their characters, UTF-8 bytes and sha256:
#                                  A, 1,000 ASCII letters in 100 blocks of 10 (a two-letter block label, AA to DV,
#                                  then abcdefgh); D, 120 Devanagari consonants in 12 blocks of one consonant repeated
#                                  10 times (U+0915 to U+0920), 360 UTF-8 bytes. D holds no combining mark, so it is
#                                  the same under every Unicode normal form.
#   n3.py gen                      writes, beside this script, from L-ledgers-shape-lab.utf16le.xml:
#                                  L-ledgers-a1000.utf16le.xml  SVCURRENTCOMPANY holds A
#                                  L-ledgers-d120.utf16le.xml   SVCURRENTCOMPANY holds D
#                                  L-ledgers-d85.utf16le.xml    SVCURRENTCOMPANY holds D's first 85 characters (255
#                                                               UTF-8 bytes)
#                                  It asserts, for each, that the company name is the only change, byte for byte,
#                                  and prints each file's bytes and sha256. Neither name needs XML escaping.
#   n3.py native <answer> <guid> a|d
#                                  a raw Company collection answer: the GUID's one row (else STOP: the book guard),
#                                  its NAME attribute and NAME elements, each with its characters, UTF-8 bytes,
#                                  sha256, whether it is the whole name A or D, and, when it is a prefix of it, how
#                                  many characters and bytes, and the block it ends in. A value that is not a prefix
#                                  prints where it first differs, and is printed only if it holds letters of A's or
#                                  D's alphabet alone.
#   n3.py stored <answer> <guid> a|d
#                                  when the stored name is a prefix of A (or D) shorter than it, writes
#                                  L-ledgers-a-stored.utf16le.xml (or L-ledgers-d-stored.utf16le.xml), whose
#                                  SVCURRENTCOMPANY holds that prefix, cut from A or D in this script and never copied
#                                  from the answer, and appends its sha256 and bytes to SHA256-run.txt, which send.py
#                                  reads for those two files only. When the stored name is the whole name, or not a
#                                  prefix of it, it writes nothing and says so.
import hashlib
import pathlib
import re
import sys
import xml.etree.ElementTree as ET

BOUND = 2_000_000
SYNTHETIC = 'BRIDGE SHAPE LAB'
SOURCE = 'L-ledgers-shape-lab.utf16le.xml'
SOURCE_SHA = '61c9b17f265e3f1e9dc9d6997488dc41e65ae49a2850233e16df523abd041373'
UPPER = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'
A = ''.join(f'{UPPER[i // 26]}{UPPER[i % 26]}abcdefgh' for i in range(100))
D = ''.join(chr(0x0915 + i) * 10 for i in range(12))
NAMES = {'a': A, 'd': D}
STORED = {'a': 'L-ledgers-a-stored.utf16le.xml', 'd': 'L-ledgers-d-stored.utf16le.xml'}
TARGETS = (('L-ledgers-a1000.utf16le.xml', A), ('L-ledgers-d120.utf16le.xml', D), ('L-ledgers-d85.utf16le.xml', D[:85]))
# As measure.py and t2.py: control characters and their numeric references are removed before parsing.
FORBIDDEN_RAW = re.compile('[\x00-\x08\x0b\x0c\x0e-\x1f]')
FORBIDDEN_REF = re.compile(r'&#(?:x0*(?:[0-8bBcCeEfF]|1[0-9a-fA-F])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));')
here = pathlib.Path(__file__).resolve().parent


def stop(message):
    print(f'STOP: {message}')
    sys.exit(3)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def check_names():
    assert len(A) == 1000 and A.isascii() and A.isalpha(), 'A is 1,000 ASCII letters'
    assert len(D) == 120 and len(D.encode('utf-8')) == 360, 'D is 120 characters, 360 UTF-8 bytes'
    assert all('क' <= ch <= 'ठ' for ch in D), 'D holds consonants U+0915 to U+0920 only'
    assert len(D[:85].encode('utf-8')) == 255, 'D\'s first 85 characters are 255 bytes'


def pattern():
    check_names()
    for key, name in NAMES.items():
        raw = name.encode('utf-8')
        print(f'{key.upper()}: {len(name)} characters, {len(raw)} UTF-8 bytes, sha256 {sha(raw)}')
        print(f'{key.upper()} as typed: {name}')


def source():
    raw = (here / SOURCE).read_bytes()
    if sha(raw) != SOURCE_SHA:
        stop(f'{SOURCE} is not the file this script was written for')
    assert raw[:2] == b'\xff\xfe', 'the source starts with a UTF-16LE byte-order mark'
    text = raw[2:].decode('utf-16-le')
    old = f'<SVCURRENTCOMPANY>{SYNTHETIC}</SVCURRENTCOMPANY>'
    assert text.count(old) == 1, 'the source names the company once'
    assert text.count(SYNTHETIC) == 1, 'the name appears nowhere else'
    assert text.count('SVCURRENTCOMPANY') == 2, 'the source has one SVCURRENTCOMPANY element'
    name_at = raw.index(old.encode('utf-16-le')) + len('<SVCURRENTCOMPANY>'.encode('utf-16-le'))
    return raw, text, old, name_at, name_at + len(SYNTHETIC.encode('utf-16-le'))


def write(target, name):
    raw, text, old, name_at, name_end = source()
    assert not set(name) & set('<>&"\''), 'nothing in the name needs XML escaping'
    out = b'\xff\xfe' + text.replace(old, f'<SVCURRENTCOMPANY>{name}</SVCURRENTCOMPANY>').encode('utf-16-le')
    # The only change: the name's bytes, and nothing else.
    assert out == raw[:name_at] + name.encode('utf-16-le') + raw[name_end:], 'only the name differs'
    (here / target).write_bytes(out)
    print(f'{target}: {len(out)} bytes, sha256 {sha(out)}, name {len(name)} characters, '
          f'{len(name.encode("utf-8"))} UTF-8 bytes')
    return out


def gen():
    check_names()
    print(f'{SOURCE}: {len((here / SOURCE).read_bytes())} bytes, sha256 {SOURCE_SHA}')
    for target, name in TARGETS:
        write(target, name)


def company_row(path, guid):
    data = pathlib.Path(path).read_bytes()
    if not data or len(data) > BOUND:
        stop(f'{path}: {len(data)} bytes, empty or above the bound')
    if data[:2] == b'\xff\xfe':
        data = data[2:]
    try:
        root = ET.fromstring(FORBIDDEN_REF.sub('', FORBIDDEN_RAW.sub('', data.decode('utf-16-le'))))
    except (UnicodeDecodeError, ET.ParseError) as error:
        stop(f'{path} does not parse as UTF-16LE XML ({error})')
    kids = lambda e, tag: [c for c in e if c.tag.upper() == tag]
    found = [c for b in kids(root, 'BODY') for d in kids(b, 'DATA') for c in kids(d, 'COLLECTION')]
    rows = [r for c in found for r in kids(c, 'COMPANY')]
    print(f'BODY/DATA/COLLECTION: {len(found)}; COMPANY rows: {len(rows)}')
    if len(found) != 1 or len(rows) != 1:
        stop(f'{path}: not one collection with one COMPANY row (THE COMPANY alone gives 1)')
    guids = [(g.text or '').strip().lower() for g in kids(rows[0], 'GUID')]
    if guids != [guid.lower()]:
        stop(f'{path}: the row\'s GUID is not {guid}: the book guard')
    print(f'GUID: {guids[0]} (matches)')
    attribute = next((v for k, v in rows[0].attrib.items() if k.upper() == 'NAME'), None)
    return attribute, [c.text or '' for c in kids(rows[0], 'NAME')]


def facts(label, value, name):
    raw = value.encode('utf-8')
    print(f'{label}: chars {len(value)}, utf8_bytes {len(raw)}, sha256 {sha(raw)}')
    common = 0
    while common < min(len(value), len(name)) and value[common] == name[common]:
        common += 1
    if value == name:
        print(f'{label}: the whole name, {len(name)} characters')
    elif common == len(value) and value:
        block, within = divmod(len(value) - 1, 10)
        print(f'{label}: a prefix of the name, {len(value)} of {len(name)} characters, {len(raw)} UTF-8 bytes; '
              f'ends in block {block + 1}, character {within + 1} of 10')
    else:
        print(f'{label}: NOT a prefix of the name; first difference at character {common + 1}')
    alphabet = set(UPPER + 'abcdefgh') if name is A else set(D)
    if value and set(value) <= alphabet:
        print(f'{label} as stored: {value}')
    else:
        print(f'{label}: not printed (it holds a character outside the name\'s alphabet, or is empty)')


def native(path, guid, key):
    check_names()
    attribute, elements = company_row(path, guid)
    print(f'NAME attribute present: {"yes" if attribute is not None else "no"}; NAME elements: {len(elements)}')
    if attribute is not None:
        facts('NAME attribute', attribute, NAMES[key])
    for i, element in enumerate(elements, 1):
        facts(f'NAME element {i}', element, NAMES[key])
    if attribute is not None and elements:
        print(f'attribute equal to element 1: {"yes" if attribute == elements[0] else "no"}')


def stored(path, guid, key):
    check_names()
    attribute, elements = company_row(path, guid)
    name, target = NAMES[key], STORED[key]
    values = {v for v in [attribute, *elements] if v is not None}
    if len(values) != 1:
        print(f'the NAME attribute and elements differ, or are absent: {target} is not written')
        return
    value = values.pop()
    if value == name:
        print(f'the stored name is {key.upper()} whole: {target} is not needed and is not written')
        return
    if not value or not name.startswith(value):
        print(f'the stored name is not a prefix of {key.upper()}: {target} is not written')
        return
    if (here / target).exists():
        stop(f'{target} already exists: it is written once')
    out = write(target, name[:len(value)])  # cut from the name here; nothing from the answer goes into the request
    with open(here / 'SHA256-run.txt', 'a', encoding='utf-8', newline='\n') as listing:
        listing.write(f'{sha(out)}  {len(out)}  {target}\n')
    print(f'appended to SHA256-run.txt: {target}, prefix of {key.upper()}, {len(value)} characters')


args = sys.argv[1:]
if args == ['pattern']:
    pattern()
elif args == ['gen']:
    gen()
elif len(args) == 4 and args[0] == 'native' and args[3] in NAMES:
    native(args[1], args[2], args[3])
elif len(args) == 4 and args[0] == 'stored' and args[3] in NAMES:
    stored(args[1], args[2], args[3])
else:
    print('usage: n3.py pattern | gen | native <answer> <guid> a|d | stored <answer> <guid> a|d')
    sys.exit(2)
