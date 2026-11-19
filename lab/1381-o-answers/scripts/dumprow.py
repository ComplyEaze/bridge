# Lab helper (1381-O, not product code): prints every non-empty leaf of each STOCKITEM row of a candidate read, by
# path, in document order, so a row's every dated entry and value is on record. Reads rows only at
# ./BODY/DATA/COLLECTION/STOCKITEM, after dropping only the characters XML forbids (C4).
# Usage: python -I -X utf8 dumprow.py <answer> [<item name> ...]
import re, sys, xml.etree.ElementTree as ET
t = open(sys.argv[1], 'rb').read().decode('utf-16-le').lstrip('﻿')
t = re.sub(r'&#([0-8]|1[124-9]|2[0-9]|3[01]);', '', t); t = re.sub('[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t)
want = set(sys.argv[2:])
def walk(el, path):
    kids = list(el)
    txt = (el.text or '').strip()
    if not kids:
        if txt: print(f'  {path} = {txt!r}')
        return
    for i, k in enumerate(kids):
        walk(k, f'{path}/{k.tag}')
for it in ET.fromstring(t).findall('./BODY/DATA/COLLECTION/STOCKITEM'):
    name = it.get('NAME') or (it.findtext('NAME') or '')
    if want and name not in want: continue
    print(f'STOCKITEM {name!r}')
    for k in it: walk(k, k.tag)
