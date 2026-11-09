#!/usr/bin/env python3
# t2.py: row T2's two helpers (lab item #2433, not product code). Run with python -I. Exit 3 = STOP, 2 = usage.
#   t2.py gen                      writes M-marks-spaced.utf16le.xml from M-marks-shape-lab.utf16le.xml, beside
#                                  this script: SVCURRENTCOMPANY holds two spaces, the name, two spaces. It asserts
#                                  that this is the only change, byte for byte, and prints both files' sha256.
#   t2.py name <answer> <guid>     a raw Company collection answer (as LAB-COMMON C4 reads it): the GUID's row's NAME
#                                  attribute and NAME elements, each printed quoted (edge spaces visible), with its
#                                  characters, UTF-8 bytes, first and last 4 bytes in hex, and sha256. The text is
#                                  printed only when, without its edge spaces, it is the synthetic BRIDGE SHAPE LAB.
#   t2.py rows <answer>            any raw answer: HEADER/STATUS, the number of BODY/DATA/COLLECTION elements, each
#                                  COMPANY row's GUID, ALTVCHID and ALTMSTID, and every LINEERROR. Nothing is judged:
#                                  a row count of 0 is printed, not a STOP. A file that does not parse is a STOP.
import hashlib
import json
import pathlib
import re
import sys
import xml.etree.ElementTree as ET

BOUND = 2_000_000
SYNTHETIC = 'BRIDGE SHAPE LAB'
SOURCE, TARGET = 'M-marks-shape-lab.utf16le.xml', 'M-marks-spaced.utf16le.xml'
SOURCE_SHA = 'f556551d09682fd94d41b40bcf539d5d34bf06260d6da4bb44ad7023c598fcb1'
FORBIDDEN_RAW = re.compile('[\x00-\x08\x0b\x0c\x0e-\x1f]')
FORBIDDEN_REF = re.compile(r'&#(?:x0*(?:[0-8bBcCeEfF]|1[0-9a-fA-F])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));')
here = pathlib.Path(__file__).resolve().parent


def stop(message):
    print(f'STOP: {message}')
    sys.exit(3)


def gen():
    raw = (here / SOURCE).read_bytes()
    if hashlib.sha256(raw).hexdigest() != SOURCE_SHA:
        stop(f'{SOURCE} is not the file this script was written for')
    assert raw[:2] == b'\xff\xfe', 'the source starts with a UTF-16LE byte-order mark'
    text = raw[2:].decode('utf-16-le')
    old = f'<SVCURRENTCOMPANY>{SYNTHETIC}</SVCURRENTCOMPANY>'
    new = f'<SVCURRENTCOMPANY>  {SYNTHETIC}  </SVCURRENTCOMPANY>'
    assert text.count(old) == 1, 'the source names the company once'
    assert text.count('SVCURRENTCOMPANY') == 2, 'the source has one SVCURRENTCOMPANY element'
    out = b'\xff\xfe' + text.replace(old, new).encode('utf-16-le')
    # The only change: four spaces (eight bytes) inserted, two each side of the name, and nothing else.
    at = raw.index(old.encode('utf-16-le'))
    name_at = at + len('<SVCURRENTCOMPANY>'.encode('utf-16-le'))
    name_end = name_at + len(SYNTHETIC.encode('utf-16-le'))
    assert out == raw[:name_at] + ' '.encode('utf-16-le') * 2 + raw[name_at:name_end] \
        + ' '.encode('utf-16-le') * 2 + raw[name_end:], 'only the four spaces differ'
    assert len(out) == len(raw) + 8
    (here / TARGET).write_bytes(out)
    print(f'{SOURCE}: {len(raw)} bytes, sha256 {SOURCE_SHA}')
    print(f'{TARGET}: {len(out)} bytes, sha256 {hashlib.sha256(out).hexdigest()}')


def parse(path):
    data = pathlib.Path(path).read_bytes()
    if len(data) > BOUND or not data:
        stop(f'{path}: {len(data)} bytes, empty or above the bound')
    if data[:2] == b'\xff\xfe':
        data = data[2:]
    try:
        text = data.decode('utf-16-le')
    except UnicodeDecodeError:
        stop(f'{path} is not UTF-16LE')
    text = FORBIDDEN_REF.sub('', FORBIDDEN_RAW.sub('', text))
    try:
        return ET.fromstring(text)
    except ET.ParseError as error:
        stop(f'{path} does not parse as XML ({error})')


def children(element, tag):
    return [c for c in element if c.tag.upper() == tag]


def company_rows(root):
    collections = [c for body in children(root, 'BODY') for data in children(body, 'DATA')
                   for c in children(data, 'COLLECTION')]
    return collections, [row for c in collections for row in children(c, 'COMPANY')]


def show(label, value):
    raw = value.encode('utf-8')
    shown = json.dumps(value) if value.strip(' ') == SYNTHETIC else '(not printed: not the synthetic name)'
    print(f'{label}: {shown}; chars {len(value)}; utf8_bytes {len(raw)}; first4 {raw[:4].hex(" ")}; '
          f'last4 {raw[-4:].hex(" ")}; sha256 {hashlib.sha256(raw).hexdigest()}')


def name(path, guid):
    collections, rows = company_rows(parse(path))
    print(f'BODY/DATA/COLLECTION: {len(collections)}; COMPANY rows: {len(rows)}')
    match = [r for r in rows if [(g.text or '').strip().lower() for g in children(r, 'GUID')] == [guid.lower()]]
    if len(collections) != 1 or len(match) != 1:
        stop(f'{path}: not one collection with one row for GUID {guid}')
    row = match[0]
    attribute = next((v for k, v in row.attrib.items() if k.upper() == 'NAME'), None)
    if attribute is None:
        print('NAME attribute: absent')
    else:
        show('NAME attribute', attribute)
    for i, element in enumerate(children(row, 'NAME'), 1):
        show(f'NAME element {i}', element.text or '')


def rows(path):
    root = parse(path)
    status = [(s.text or '').strip() for h in children(root, 'HEADER') for s in children(h, 'STATUS')]
    collections, found = company_rows(root)
    print(f'HEADER/STATUS: {status}; BODY/DATA/COLLECTION: {len(collections)}; COMPANY rows: {len(found)}')
    for i, row in enumerate(found, 1):
        fields = {tag: [(c.text or '').strip() for c in children(row, tag)] for tag in ('GUID', 'ALTVCHID', 'ALTMSTID')}
        print(f'row {i}: GUID {fields["GUID"]}; ALTVCHID {fields["ALTVCHID"]}; ALTMSTID {fields["ALTMSTID"]}')
    for error in root.iter():
        if error.tag.upper() == 'LINEERROR':
            print(f'LINEERROR: {json.dumps(error.text or "")}')


args = sys.argv[1:]
if args == ['gen']:
    gen()
elif len(args) == 3 and args[0] == 'name':
    name(args[1], args[2])
elif len(args) == 2 and args[0] == 'rows':
    rows(args[1])
else:
    print('usage: t2.py gen | name <answer> <guid> | rows <answer>')
    sys.exit(2)
