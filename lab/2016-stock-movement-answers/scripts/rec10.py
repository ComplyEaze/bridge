# Lab helper (#2016 stock movement, not product code): the step 10 record of one raw answer, read by structure (LAB-COMMON
# C4: UTF-16LE, forbidden characters dropped for parsing only, parsed as XML, never a text search). ROWS are the VOUCHER
# children of ENVELOPE/BODY/DATA/COLLECTION. Prints, per VOUCHER, the header fields the sheet names and every computed
# BRIDGE* element; per ALLINVENTORYENTRIES.LIST (and INVENTORYENTRIESIN/OUT.LIST, and INVENTORYALLOCATIONS.LIST nested
# under a ledger entry) the item fields and each BATCHALLOCATIONS.LIST. Read only. Usage: python -I rec10.py <answer file>
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
print(f'{sys.argv[1].replace(chr(92), "/").split("/")[-1]}: {len(raw)} B; VOUCHER rows {len(rows)}')
HEAD = ['DATE', 'VOUCHERTYPENAME', 'VOUCHERNUMBER', 'ALTERID', 'ISOPTIONAL', 'ISCANCELLED', 'ISPOSTDATED']
INV = ['STOCKITEMNAME', 'ISDEEMEDPOSITIVE', 'ACTUALQTY', 'BILLEDQTY', 'RATE', 'AMOUNT', 'ISSCRAP']
BAT = ['GODOWNNAME', 'BATCHNAME', 'DESTINATIONGODOWNNAME', 'ACTUALQTY', 'AMOUNT']


def val(e, tag):
    c = e.find(tag)
    if c is None:
        return '<absent>'
    t = (c.text or '').strip()
    return t if t else '<empty>'


def inv_lines(e, path):
    print(f'    {path}: ' + ' '.join(f'{k}={val(e, k)!r}' for k in INV))
    for b in e.findall('BATCHALLOCATIONS.LIST'):
        print('      BATCHALLOCATIONS.LIST: ' + ' '.join(f'{k}={val(b, k)!r}' for k in BAT))


def sort_key(r):
    t = (r.findtext('ALTERID') or '0').strip()
    return int(t) if t.isdigit() else 0


for v in sorted(rows, key=sort_key):
    print('VOUCHER ' + ' '.join(f'{h}={val(v, h)!r}' for h in HEAD))
    comp = [(c.tag, (c.text or '').strip()) for c in v if c.tag.startswith('BRIDGE')]
    print('  computed: ' + (' '.join(f'{t}={x!r}' for t, x in comp) if comp else '<none>'))
    for tag in ('ALLINVENTORYENTRIES.LIST', 'INVENTORYENTRIESIN.LIST', 'INVENTORYENTRIESOUT.LIST', 'INVENTORYENTRIES.LIST'):
        found = v.findall(tag)
        print(f'  {tag}: {"present x" + str(len(found)) if found else "absent"}')
        for e in found:
            inv_lines(e, tag)
    for led in v.findall('ALLLEDGERENTRIES.LIST') + v.findall('LEDGERENTRIES.LIST'):
        nested = led.findall('INVENTORYALLOCATIONS.LIST')
        print(f'  {led.tag} {val(led, "LEDGERNAME")!r} AMOUNT={val(led, "AMOUNT")!r} INVENTORYALLOCATIONS.LIST: {"x" + str(len(nested)) if nested else "absent"}')
        for e in nested:
            inv_lines(e, led.tag + '/INVENTORYALLOCATIONS.LIST')
