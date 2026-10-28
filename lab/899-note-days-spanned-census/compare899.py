#!/usr/bin/env python3 -I
"""Compare one day's census answers (#899, the note-day records).

Run: python -I compare899.py RECORDED_SHA256 UNSPANNED_ANSWER SPANNED_ANSWER

RECORDED_SHA256 is the SHA-256 of the census answer the record holds for that day (in the sheet).
Each answer is read as bytes and hashed, then decoded (UTF-16LE, with or without a BOM), stripped
ONLY of the characters XML forbids (LAB-COMMON C4) and parsed as XML. Rows are the VOUCHER
children of ENVELOPE/BODY/DATA/COLLECTION, each listed as (GUID, ALTERID, DATE), never trimmed.

It prints both answers' sizes, hashes and rows, then three verdicts:
- the unspanned answer equals the recorded answer, byte for byte;
- the spanned answer equals the unspanned answer, byte for byte;
- every row's ALTERID is at most 111, the span's top.

Exit 0 when all three hold; 1 when one does not (each listed); 2 for an error, never a verdict: a
file that cannot be read, an answer that does not parse, a root other than ENVELOPE, or a VOUCHER
row without a GUID, an ALTERID that is not a number, or a GUID repeated.
"""
import hashlib
import re
import sys
import xml.etree.ElementTree as ET

FORBIDDEN_RAW = re.compile('[\x00-\x08\x0b\x0c\x0e-\x1f]')
FORBIDDEN_REF = re.compile(r'&#(?:x0*(?:[0-8bBcCeEfF]|1[0-9a-fA-F])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));')
SPAN_TOP = 111


class Refused(Exception):
    pass


def load(path):
    try:
        data = open(path, 'rb').read()
    except OSError as error:
        raise Refused('%s cannot be read: %s' % (path, error))
    body = data[2:] if data[:2] == b'\xff\xfe' else data
    try:
        text = body.decode('utf-16-le')
    except UnicodeDecodeError as error:
        raise Refused('%s is not UTF-16LE: %s' % (path, error))
    text = FORBIDDEN_REF.sub('', FORBIDDEN_RAW.sub('', text))
    try:
        root = ET.fromstring(text)
    except ET.ParseError as error:
        raise Refused('%s does not parse: %s' % (path, error))
    if root.tag != 'ENVELOPE':
        raise Refused('%s: the root is %s, not ENVELOPE' % (path, root.tag))
    rows = []
    seen = set()
    for voucher in root.findall('./BODY/DATA/COLLECTION/VOUCHER'):
        def field(tag):
            found = voucher.find(tag)
            return None if found is None else (found.text or '')
        guid = (field('GUID') or '').strip()
        alter = (field('ALTERID') or '').strip()
        if not guid:
            raise Refused('%s: a VOUCHER has no GUID' % path)
        if guid in seen:
            raise Refused('%s: GUID %s is repeated' % (path, guid))
        if not alter.isdigit():
            raise Refused('%s: GUID %s has ALTERID %r' % (path, guid, alter))
        seen.add(guid)
        rows.append((guid, int(alter), field('DATE')))
    return data, rows


def main():
    if len(sys.argv) != 4:
        print(__doc__)
        return 2
    recorded, unspanned_path, spanned_path = sys.argv[1:]
    try:
        unspanned, unspanned_rows = load(unspanned_path)
        spanned, spanned_rows = load(spanned_path)
    except Refused as error:
        print('ERROR: %s' % error)
        return 2
    for name, data, rows in [('unspanned', unspanned, unspanned_rows), ('spanned', spanned, spanned_rows)]:
        print('%-9s %d bytes, sha256 %s, %d VOUCHER rows' % (name, len(data), hashlib.sha256(data).hexdigest(), len(rows)))
        for row in rows:
            print('  %r' % (row,))
    verdicts = [
        ('the unspanned answer equals the recorded one, byte for byte', hashlib.sha256(unspanned).hexdigest() == recorded.lower()),
        ('the spanned answer equals the unspanned one, byte for byte', spanned == unspanned),
        ('every row of both is inside the span (ALTERID <= %d)' % SPAN_TOP, all(alter <= SPAN_TOP for _, alter, _ in unspanned_rows + spanned_rows)),
    ]
    for text, held in verdicts:
        print('%s: %s' % ('HOLDS' if held else 'DOES NOT HOLD', text))
    return 0 if all(held for _, held in verdicts) else 1


if __name__ == '__main__':
    sys.exit(main())
