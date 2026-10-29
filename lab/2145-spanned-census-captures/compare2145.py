#!/usr/bin/env python3 -I
"""Compare one window's census answers (#2145).

Run: python -I compare2145.py RECORDED|none M UNSPANNED_ANSWER SPANNED_ANSWER

RECORDED is the BODY digest of the census answer the replay holds for that window (in the sheet), or `none` where the
replay holds none. M is the span's top.

The BODY digest of an answer: decode it (UTF-16LE, with or without a BOM), remove the one CMPINFO element (from
`<CMPINFO>` to `</CMPINFO>` inclusive; an answer with none keeps its text, one with more than one is an error), and take
the SHA-256 of what is left, encoded as UTF-8. CMPINFO holds the Tally session's object counts, which differ between
sessions and with the companies loaded even when every row is the same, so it is left out of the comparison and printed.

Each answer is also stripped ONLY of the characters XML forbids (LAB-COMMON C4) and parsed as XML. Rows are the VOUCHER
children of ENVELOPE/BODY/DATA/COLLECTION, each listed as (GUID, ALTERID, DATE), never trimmed.

It prints both answers' sizes, byte hashes, BODY digests, CMPINFO blocks and rows, whether the two answers are equal byte
for byte, and the number of control rows above M (the book may have moved since it was recorded), then the verdicts:
- A: the control's BODY digest equals the recorded one (the window is as it was when recorded);
- B: the spanned answer's BODY digest equals the recorded one (the replay's census answer stands for the spanned request,
  by this measurement);
- C: every row of the spanned answer has ALTERID at most M;
- D: each answer holds ENVELOPE/BODY/DATA/COLLECTION and at least one VOUCHER row (every window this sheet reads held one
  when recorded; an empty or error answer must not pass).
A and B are SKIPPED when RECORDED is `none`.

Exit 0 when every verdict that ran holds; 1 when one does not (each listed); 2 for an error, never a verdict: a file
that cannot be read, an answer that does not parse, a root other than ENVELOPE, more than one CMPINFO, or a VOUCHER row
without a GUID, an ALTERID that is not a number, or a GUID repeated.
"""
import hashlib
import re
import sys
import xml.etree.ElementTree as ET

FORBIDDEN_RAW = re.compile('[\x00-\x08\x0b\x0c\x0e-\x1f]')
FORBIDDEN_REF = re.compile(r'&#(?:x0*(?:[0-8bBcCeEfF]|1[0-9a-fA-F])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));')
CMPINFO = re.compile(r'<CMPINFO>.*?</CMPINFO>', re.S)


class Refused(Exception):
    pass


def body_digest(text, path):
    found = CMPINFO.findall(text)
    if len(found) > 1 or text.count('<CMPINFO>') != len(found):
        raise Refused('%s: CMPINFO is not one whole element' % path)
    stripped = CMPINFO.sub('', text, count=1)
    return hashlib.sha256(stripped.encode('utf-8')).hexdigest(), (found[0] if found else '')


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
    digest, cmpinfo = body_digest(text, path)
    clean = FORBIDDEN_REF.sub('', FORBIDDEN_RAW.sub('', text))
    # Tally's answers may carry user-defined fields with an undeclared UDF: prefix (#899).
    clean = clean.replace('<ENVELOPE', '<ENVELOPE xmlns:UDF="urn:x-tally-udf"', 1)
    try:
        root = ET.fromstring(clean)
    except ET.ParseError as error:
        raise Refused('%s does not parse: %s' % (path, error))
    if root.tag != 'ENVELOPE':
        raise Refused('%s: the root is %s, not ENVELOPE' % (path, root.tag))
    has_collection = root.find('./BODY/DATA/COLLECTION') is not None
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
    return data, digest, cmpinfo, has_collection, rows


def main():
    if len(sys.argv) != 5 or not sys.argv[2].isdigit():
        print(__doc__)
        return 2
    recorded, top, unspanned_path, spanned_path = sys.argv[1].lower(), int(sys.argv[2]), sys.argv[3], sys.argv[4]
    if recorded != 'none' and not re.fullmatch('[0-9a-f]{64}', recorded):
        print(__doc__)
        return 2
    try:
        unspanned = load(unspanned_path)
        spanned = load(spanned_path)
    except Refused as error:
        print('ERROR: %s' % error)
        return 2
    for name, (data, digest, cmpinfo, _, rows) in [('unspanned', unspanned), ('spanned', spanned)]:
        print('%-9s %d bytes, sha256 %s, BODY digest %s, %d VOUCHER rows' % (name, len(data), hashlib.sha256(data).hexdigest(), digest, len(rows)))
        print('  CMPINFO: %s' % ' '.join(cmpinfo.split()))
        for row in rows:
            print('  %r' % (row,))
    print('the two answers are equal byte for byte: %s' % ('yes' if unspanned[0] == spanned[0] else 'no'))
    print('unspanned rows above %d: %d' % (top, sum(1 for _, alter, _ in unspanned[4] if alter > top)))
    verdicts = []
    if recorded == 'none':
        print('SKIPPED: A and B (no recorded answer)')
    else:
        verdicts.append(("A: the control's BODY digest equals the recorded one", unspanned[1] == recorded))
        verdicts.append(("B: the spanned answer's BODY digest equals the recorded one", spanned[1] == recorded))
    verdicts.append(('C: every row of the spanned answer is inside the span (ALTERID <= %d)' % top, all(alter <= top for _, alter, _ in spanned[4])))
    verdicts.append(('D: each answer holds a COLLECTION and at least one VOUCHER row', all(answer[3] and answer[4] for answer in (unspanned, spanned))))
    for text, held in verdicts:
        print('%s: %s' % ('HOLDS' if held else 'DOES NOT HOLD', text))
    return 0 if all(held for _, held in verdicts) else 1


if __name__ == '__main__':
    sys.exit(main())
