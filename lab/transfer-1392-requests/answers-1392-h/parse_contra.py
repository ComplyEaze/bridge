# Parse the 1392-H raw capture per VOUCHER (structure only, no text search).
# Usage: python -I parse_contra.py <response file>
import sys
import xml.etree.ElementTree as ET

sys.stdout.reconfigure(encoding="utf-8")

raw = open(sys.argv[1], 'rb').read()
enc = 'utf-16-le' if raw[:2] in (b'<\x00', b'\xff\xfe') else 'utf-8'
text = raw.decode(enc).lstrip('﻿')
# Tally writes &#4; (U+0004, not allowed in XML 1.0) before reserved names; replace it with the
# visible marker U+FFFD (the same form Bridge uses) so the document parses. Counted and reported.
n4 = text.count('&#4;')
text = text.replace('&#4;', '�#4;')
root = ET.fromstring(text)
print('&#4; references replaced with U+FFFD#4; for parsing:', n4)
print('encoding', enc, '; HEADER/STATUS', root.findtext('HEADER/STATUS'))
vouchers = list(root.iter('VOUCHER'))
print('VOUCHER count', len(vouchers))


def leaf(e):
    return (e.text or '').strip() if len(list(e)) == 0 else None


def dump_all(e, indent):
    for c in e:
        v = leaf(c)
        if v is None:
            print(f'{indent}{c.tag}:')
            dump_all(c, indent + '  ')
        elif v != '' or c.attrib:
            print(f'{indent}{c.tag} = {v!r}')


for v in vouchers:
    print('---')
    print('VOUCHER attrs', dict(v.attrib))
    for tag in ('VOUCHERNUMBER', 'DATE', 'GUID', 'ALTERID', 'MASTERID', 'VOUCHERTYPENAME', 'NARRATION', 'REFERENCE', 'REFERENCEDATE', 'PARTYLEDGERNAME'):
        els = v.findall(tag)
        print(f'  {tag} =', [leaf(e) for e in els] if els else 'ABSENT')
    entries = v.findall('ALLLEDGERENTRIES.LIST')
    print('  ALLLEDGERENTRIES.LIST count', len(entries))
    for i, le in enumerate(entries, 1):
        print(f'  entry {i}: LEDGERNAME={le.findtext("LEDGERNAME")!r} AMOUNT={le.findtext("AMOUNT")!r} ISDEEMEDPOSITIVE={le.findtext("ISDEEMEDPOSITIVE")!r}')
        bals = le.findall('BANKALLOCATIONS.LIST')
        print(f'    BANKALLOCATIONS.LIST count {len(bals)}')
        for j, b in enumerate(bals, 1):
            print(f'    bank allocation {j} (every element, empty ones listed by name):')
            for c in b:
                val = leaf(c)
                if val is None:
                    print(f'      {c.tag}: <{len(list(c))} children>')
                    dump_all(c, '        ')
                else:
                    print(f'      {c.tag} = {val!r}')
