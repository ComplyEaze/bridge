#!/usr/bin/env python3 -I
"""Summarise one answer of this sheet (#2145, item 2145-R).

Run: python -I summarise2145r.py ANSWER [M]

Decode the answer (UTF-16LE, with or without a BOM), drop ONLY the characters XML forbids, raw or as references
(LAB-COMMON C4), and parse it as XML. Rows are the children of ENVELOPE/BODY/DATA/COLLECTION; the CMPINFO block above
them holds count lines with the same tag names, which are not rows.

It prints the answer's size, its sha256, its BODY digest (the SHA-256 of its text with the one CMPINFO element removed,
encoded as UTF-8; CMPINFO holds the session's object counts, which differ between sessions even when every row is the
same) and its rows, sorted by ALTERID as a number:
- a COMPANY row as (NAME, GUID, ALTVCHID, ALTMSTID);
- a VOUCHER row as (GUID, ALTERID, DATE, VOUCHERTYPENAME, VOUCHERNUMBER, ISCANCELLED, ISOPTIONAL), each as Tally sent it
  (a field Tally did not send prints as None);
- any other row by its tag and NAME.
With M (a census answer), it also checks that every VOUCHER row's ALTERID is at most M.

Exit 0: parsed, and with M every row is inside the span. Exit 1: with M, a row lies above it (listed). Exit 2: an
error, never a result: the file cannot be read, does not parse, has a root other than ENVELOPE, more than one CMPINFO, no
COLLECTION, or a VOUCHER row without a GUID or with an ALTERID that is not a number.
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


def field(row, name):
    value = row.findtext(name)
    return None if value is None else value.strip()


def main():
    if len(sys.argv) not in (2, 3):
        print(__doc__)
        return 2
    path = sys.argv[1]
    span = int(sys.argv[2]) if len(sys.argv) == 3 else None
    try:
        data = open(path, 'rb').read()
        body = data[2:] if data[:2] == b'\xff\xfe' else data
        text = body.decode('utf-16-le')
        found = CMPINFO.findall(text)
        if len(found) > 1 or text.count('<CMPINFO>') != len(found):
            raise Refused('CMPINFO is not one whole element')
        digest = hashlib.sha256(CMPINFO.sub('', text, count=1).encode('utf-8')).hexdigest()
        root = ET.fromstring(FORBIDDEN_REF.sub('', FORBIDDEN_RAW.sub('', text)))
        if root.tag != 'ENVELOPE':
            raise Refused('root is %s' % root.tag)
        collection = root.find('BODY/DATA/COLLECTION')
        if collection is None:
            raise Refused('no ENVELOPE/BODY/DATA/COLLECTION')
        rows = []
        for row in collection:
            if row.tag == 'VOUCHER':
                guid, alterid = field(row, 'GUID'), field(row, 'ALTERID')
                if not guid or not (alterid or '').isdigit():
                    raise Refused('a VOUCHER row without a GUID or a numeric ALTERID')
                rows.append((int(alterid), ('VOUCHER', guid, int(alterid), field(row, 'DATE'),
                                            field(row, 'VOUCHERTYPENAME'), field(row, 'VOUCHERNUMBER'),
                                            field(row, 'ISCANCELLED'), field(row, 'ISOPTIONAL'))))
            elif row.tag == 'COMPANY':
                rows.append((0, ('COMPANY', row.get('NAME') or field(row, 'NAME'), field(row, 'GUID'),
                                 field(row, 'ALTVCHID'), field(row, 'ALTMSTID'))))
            else:
                rows.append((0, (row.tag, row.get('NAME') or field(row, 'NAME'))))
    except (OSError, UnicodeDecodeError, ET.ParseError, Refused) as error:
        print('ERROR %s: %s' % (path, error))
        return 2
    print('%s: %d bytes, sha256 %s, BODY digest %s, %d rows' % (
        path, len(data), hashlib.sha256(data).hexdigest(), digest, len(rows)))
    for _, row in sorted(rows, key=lambda item: item[0]):
        print('  %r' % (row,))
    if span is None:
        return 0
    above = [row for key, row in rows if row[0] == 'VOUCHER' and key > span]
    for row in above:
        print('ABOVE THE SPAN %d: %r' % (span, row))
    print('rows above %d: %d' % (span, len(above)))
    return 1 if above else 0


if __name__ == '__main__':
    sys.exit(main())
