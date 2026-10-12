# Compare two status-window answers voucher by voucher (leaf fields). Usage: python -I wdiff.py <old> <new>
import sys, xml.etree.ElementTree as ET
def load(p):
    t = open(p, 'rb').read().decode('utf-16-le').lstrip('﻿').replace('&#4;', '')
    out = {}
    for v in ET.fromstring(t).findall('.//COLLECTION/VOUCHER'):
        out[(v.findtext('VOUCHERNUMBER') or '').strip()] = {c.tag: (c.text or '').strip() for c in v if len(c) == 0}
    return out
a, b = load(sys.argv[1]), load(sys.argv[2])
print('vouchers', len(a), len(b))
for k in sorted(set(a) | set(b)):
    if k not in a or k not in b: print('ONLY IN ONE', k); continue
    d = {f: (a[k].get(f), b[k].get(f)) for f in set(a[k]) | set(b[k]) if a[k].get(f) != b[k].get(f)}
    if d: print(k, d)
