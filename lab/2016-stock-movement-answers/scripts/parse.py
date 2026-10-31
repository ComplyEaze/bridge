# Lab helper (#2016 stock movement, not product code): reads one raw answer by structure (LAB-COMMON C4: UTF-16LE,
# forbidden characters dropped, parsed as XML, never a text search) and prints, per VOUCHER row of
# ENVELOPE/BODY/DATA/COLLECTION, its header fields and every list under it with its element path, the inventory and
# ledger lists' fields, and empty values shown as <empty>. Read only. Usage: python -I parse.py <answer file>
import re
import sys
import xml.etree.ElementTree as ET

raw = open(sys.argv[1], 'rb').read()
text = raw.decode('utf-16-le')
if text.startswith('﻿'):
    text = text[1:]
text = re.sub(r'&#(0*([0-8]|1[124]|1[4-9]|2[0-9]|3[01]));', '', text)
text = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', text)
root = ET.fromstring(text)
coll = root.find('./BODY/DATA/COLLECTION')
rows = [] if coll is None else [e for e in coll if e.tag == 'VOUCHER']
print(f'{sys.argv[1]}: {len(raw)} B; VOUCHER rows {len(rows)}')
HEAD = ['DATE', 'VOUCHERTYPENAME', 'VOUCHERNUMBER', 'PARTYLEDGERNAME', 'MASTERID', 'ALTERID', 'ISINVOICE',
        'ISCANCELLED', 'ISOPTIONAL', 'NARRATION']
FIELDS = ['STOCKITEMNAME', 'LEDGERNAME', 'ISDEEMEDPOSITIVE', 'ACTUALQTY', 'BILLEDQTY', 'RATE', 'AMOUNT', 'GODOWNNAME',
          'BATCHNAME', 'EXPIRYPERIOD', 'DESTINATIONGODOWNNAME', 'ISLASTDEEMEDPOSITIVE', 'MFDON', 'DISCOUNT']


def val(e, tag):
    c = e.find(tag)
    if c is None:
        return None
    t = (c.text or '').strip()
    return t if t else '<empty>'


def walk(e, path, depth):
    for c in e:
        if c.tag.endswith('.LIST'):
            p = f'{path}/{c.tag}'
            vals = {f: val(c, f) for f in FIELDS if c.find(f) is not None}
            kids = sum(1 for _ in c)
            print(f'{"  " * depth}{p} ({kids} children) ' + ' '.join(f'{k}={v!r}' for k, v in vals.items()))
            walk(c, p, depth + 1)


for v in sorted(rows, key=lambda r: int((r.findtext('MASTERID') or '0').strip() or 0)):
    print('VOUCHER ' + ' '.join(f'{h}={val(v, h)!r}' for h in HEAD) + f' attrs={dict(v.attrib)}')
    lists = {}
    for c in v:
        if c.tag.endswith('.LIST'):
            lists[c.tag] = lists.get(c.tag, 0) + 1
    print('  top-level lists: ' + ', '.join(f'{k} x{n}' for k, n in sorted(lists.items())))
    walk(v, '', 1)
