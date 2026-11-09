#!/usr/bin/env python3
# t2.py: row T2's helpers (lab item #2433, not product code). Run with python -I. Exit 3 = STOP, 2 = usage.
#   t2.py gen                      writes L-ledgers-spaced.utf16le.xml from L-ledgers-shape-lab.utf16le.xml, beside
#                                  this script: SVCURRENTCOMPANY holds two spaces, the name, two spaces. It asserts
#                                  that this is the only change, byte for byte, and prints both files' sha256.
#   t2.py name <answer> <guid>     a raw Company collection answer (as LAB-COMMON C4 reads it): the GUID's row's NAME
#                                  attribute and NAME elements, each printed quoted (edge spaces visible), with its
#                                  characters, UTF-8 bytes, first and last 4 bytes in hex, and sha256. The text is
#                                  printed only when, without its edge spaces, it is the synthetic BRIDGE SHAPE LAB.
#   t2.py ledgers <answer> <guid>  a raw List of Ledgers answer: HEADER/STATUS, the number of BODY/DATA/COLLECTION
#                                  elements, the rows under them and their tags, how many rows hold one GUID, how many
#                                  of those begin with <guid>-, how many hold a BRIDGECOMPANYGUID equal to <guid>, the
#                                  distinct BRIDGECOMPANYGUID values, the CMPINFO counts that are not 0 (and how many
#                                  are) and every LINEERROR. No ledger
#                                  name is printed. Nothing is judged: 0 rows is printed, not a STOP. A file that does
#                                  not parse is a STOP.
import hashlib
import json
import pathlib
import re
import sys
import xml.etree.ElementTree as ET

BOUND = 2_000_000
SYNTHETIC = 'BRIDGE SHAPE LAB'
SOURCE, TARGET = 'L-ledgers-shape-lab.utf16le.xml', 'L-ledgers-spaced.utf16le.xml'
SOURCE_SHA = '61c9b17f265e3f1e9dc9d6997488dc41e65ae49a2850233e16df523abd041373'
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
    assert text.count(SYNTHETIC) == 1, 'the name appears nowhere else'
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


def collections(root):
    return [c for body in children(root, 'BODY') for data in children(body, 'DATA') for c in children(data, 'COLLECTION')]


def show(label, value):
    raw = value.encode('utf-8')
    shown = json.dumps(value) if value.strip(' ') == SYNTHETIC else '(not printed: not the synthetic name)'
    print(f'{label}: {shown}; chars {len(value)}; utf8_bytes {len(raw)}; first4 {raw[:4].hex(" ")}; '
          f'last4 {raw[-4:].hex(" ")}; sha256 {hashlib.sha256(raw).hexdigest()}')


def name(path, guid):
    found = collections(parse(path))
    rows = [row for c in found for row in children(c, 'COMPANY')]
    print(f'BODY/DATA/COLLECTION: {len(found)}; COMPANY rows: {len(rows)}')
    match = [r for r in rows if [(g.text or '').strip().lower() for g in children(r, 'GUID')] == [guid.lower()]]
    if len(found) != 1 or len(match) != 1:
        stop(f'{path}: not one collection with one row for GUID {guid}')
    row = match[0]
    attribute = next((v for k, v in row.attrib.items() if k.upper() == 'NAME'), None)
    if attribute is None:
        print('NAME attribute: absent')
    else:
        show('NAME attribute', attribute)
    for i, element in enumerate(children(row, 'NAME'), 1):
        show(f'NAME element {i}', element.text or '')


def ledgers(path, guid):
    root = parse(path)
    guid = guid.lower()
    status = [(s.text or '').strip() for h in children(root, 'HEADER') for s in children(h, 'STATUS')]
    found = collections(root)
    rows = [row for c in found for row in c]
    tags = sorted({row.tag for row in rows})
    one_guid = [r for r in rows if len(children(r, 'GUID')) == 1]
    prefixed = [r for r in one_guid if (children(r, 'GUID')[0].text or '').strip().lower().startswith(guid + '-')]
    computed = [(c.text or '').strip().lower() for r in rows for c in children(r, 'BRIDGECOMPANYGUID')]
    print(f'HEADER/STATUS: {status}; BODY/DATA/COLLECTION: {len(found)}; rows: {len(rows)}; row tags: {tags}')
    print(f'rows with one GUID: {len(one_guid)}; of those, beginning with {guid}-: {len(prefixed)}')
    print(f'rows with BRIDGECOMPANYGUID equal to {guid}: {computed.count(guid)}; '
          f'distinct BRIDGECOMPANYGUID values: {sorted(set(computed))}')
    for info in root.iter():
        if info.tag.upper() == 'CMPINFO':
            counts = [(c.tag, (c.text or '').strip()) for c in info]
            shown = [f'{tag} {value}' for tag, value in counts if value != '0']
            print(f'CMPINFO: {len(counts)} counts, {len(counts) - len(shown)} of them 0; the others: '
                  + (', '.join(shown) or 'none'))
    for error in root.iter():
        if error.tag.upper() == 'LINEERROR':
            print(f'LINEERROR: {json.dumps(error.text or "")}')


args = sys.argv[1:]
if args == ['gen']:
    gen()
elif len(args) == 3 and args[0] == 'name':
    name(args[1], args[2])
elif len(args) == 3 and args[0] == 'ledgers':
    ledgers(args[1], args[2])
else:
    print('usage: t2.py gen | name <answer> <guid> | ledgers <answer> <guid>')
    sys.exit(2)
