#!/usr/bin/env python3 -I
"""Compare one window's census answers (#2145).

Run: python -I compare2145.py RECORDED_SHA256|none M UNSPANNED_ANSWER SPANNED_ANSWER

RECORDED_SHA256 is the SHA-256 of the census answer the replay holds for that window (in the sheet), or `none` where the
replay holds none. M is the span's top. Each answer is read as bytes and hashed, then decoded (UTF-16LE, with or without
a BOM), stripped ONLY of the characters XML forbids (LAB-COMMON C4) and parsed as XML. Rows are the VOUCHER children of
ENVELOPE/BODY/DATA/COLLECTION, each listed as (GUID, ALTERID, DATE), never trimmed.

It prints both answers' sizes, hashes and rows, then the verdicts:
- A: the unspanned answer equals the recorded answer, byte for byte (the window is as it was when recorded);
- B: the spanned answer equals the recorded answer, byte for byte (the replay can carry the recorded answer for the
  spanned request, by this measurement);
- C: every row of the spanned answer has ALTERID at most M.
A and B are skipped (printed as SKIPPED) when RECORDED_SHA256 is `none`. A row of the UNSPANNED answer above M is printed
as a count, not a verdict: the book may have moved since it was recorded.

Exit 0 when every verdict that ran holds; 1 when one does not (each listed); 2 for an error, never a verdict: a file
that cannot be read, an answer that does not parse, a root other than ENVELOPE, or a VOUCHER row without a GUID, an
ALTERID that is not a number, or a GUID repeated.
"""
import hashlib
import re
import sys
import xml.etree.ElementTree as ET

FORBIDDEN_RAW = re.compile('[\x00-\x08\x0b\x0c\x0e-\x1f]')
FORBIDDEN_REF = re.compile(r'&#(?:x0*(?:[0-8bBcCeEfF]|1[0-9a-fA-F])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));')


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
    # Tally's answers may carry user-defined fields with an undeclared UDF: prefix (#899).
    text = text.replace('<ENVELOPE', '<ENVELOPE xmlns:UDF="urn:x-tally-udf"', 1)
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
    if len(sys.argv) != 5 or not sys.argv[2].isdigit():
        print(__doc__)
        return 2
    recorded, top, unspanned_path, spanned_path = sys.argv[1].lower(), int(sys.argv[2]), sys.argv[3], sys.argv[4]
    if recorded != 'none' and not re.fullmatch('[0-9a-f]{64}', recorded):
        print(__doc__)
        return 2
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
    print('unspanned rows above %d: %d' % (top, sum(1 for _, alter, _ in unspanned_rows if alter > top)))
    verdicts = []
    if recorded == 'none':
        print('SKIPPED: A and B (no recorded answer)')
    else:
        verdicts.append(('A: the unspanned answer equals the recorded one, byte for byte', hashlib.sha256(unspanned).hexdigest() == recorded))
        verdicts.append(('B: the spanned answer equals the recorded one, byte for byte', hashlib.sha256(spanned).hexdigest() == recorded))
    verdicts.append(('C: every row of the spanned answer is inside the span (ALTERID <= %d)' % top, all(alter <= top for _, alter, _ in spanned_rows)))
    for text, held in verdicts:
        print('%s: %s' % ('HOLDS' if held else 'DOES NOT HOLD', text))
    return 0 if all(held for _, held in verdicts) else 1


if __name__ == '__main__':
    sys.exit(main())
