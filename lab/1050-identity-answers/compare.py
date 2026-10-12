# 1050-B comparison by structure: the voucher dated 20260710 (N1 in a1/a2, K in a3) from the b2 (candidates) answers.
# Flattens the VOUCHER element (attributes as @name, child elements by path with sibling index), then writes
# a markdown table: field | a1 (N1) | a2 (N1 after restore) | a3 (K) | a1 vs a2 | a1 vs a3.
# Usage: python -I compare.py <a1-candidates> <a2-candidates> <a3-candidates> <out.md>
import re
import sys
import xml.etree.ElementTree as ET

sys.stdout.reconfigure(encoding='utf-8')


def voucher(p):
    raw = open(p, 'rb').read()
    t = raw.decode('utf-16' if raw[:2] in (b'\xff\xfe', b'\xfe\xff') else 'utf-16-le').lstrip('﻿').replace('&#4;', '')
    t = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t)
    vs = [v for v in ET.fromstring(t).findall('.//COLLECTION/VOUCHER') if (v.findtext('DATE') or '').strip() == '20260710']
    if len(vs) != 1:
        sys.exit('%s: %d vouchers dated 20260710' % (p, len(vs)))
    return vs[0]


def flat(el, path, out):
    for k, v in el.attrib.items():
        out[path + '/@' + k] = v
    kids = list(el)
    if not kids:
        out[path] = (el.text or '')
        return
    seen = {}
    for c in kids:
        i = seen.get(c.tag, 0)
        seen[c.tag] = i + 1
        flat(c, '%s/%s[%d]' % (path, c.tag, i), out)


cols = []
for p in sys.argv[1:4]:
    d = {}
    flat(voucher(p), 'VOUCHER', d)
    cols.append(d)
keys = sorted(set().union(*cols), key=lambda k: (0 if '/@' in k.split('/')[1] else 1, k))
rows = ['| field | a1 (N1) | a2 (N1 after harmless restore) | a3 (K, hand re-key) | a1 vs a2 | a1 vs a3 |', '|---|---|---|---|---|---|']
nd12 = nd13 = 0
diffs13 = []
for k in keys:
    a, b, c = (col.get(k, '<absent>') for col in cols)
    e12 = 'equal' if a == b else 'DIFFERENT'
    e13 = 'equal' if a == c else 'DIFFERENT'
    nd12 += a != b
    nd13 += a != c
    if a != c:
        diffs13.append(k)
    rows.append('| %s | %r | %r | %r | %s | %s |' % (k.replace('|', '/'), a, b, c, e12, e13))
summary = ['', 'fields: a1 %d, a2 %d, a3 %d; differing a1 vs a2: %d; a1 vs a3: %d' % (len(cols[0]), len(cols[1]), len(cols[2]), nd12, nd13),
           'a1 vs a3 differing fields: ' + ', '.join(diffs13)]
open(sys.argv[4], 'w', encoding='utf-8', newline='\n').write('\n'.join(rows + summary) + '\n')
print('\n'.join(summary))
