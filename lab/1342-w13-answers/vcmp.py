# Structural field-by-field comparison of the VOUCHER element in two W11 read-back answers (UTF-16LE).
# Flattens each element to path -> text (repeated siblings get [i]); prints paths whose values differ or exist on one side only.
# Usage: python -I vcmp.py <old answer> <new answer>
import re
import sys
import xml.etree.ElementTree as ET

sys.stdout.reconfigure(encoding='utf-8')


def voucher(p):
    t = open(p, 'rb').read().decode('utf-16-le').lstrip('﻿').replace('&#4;', '')
    t = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t)
    root = ET.fromstring(t)
    vs = root.findall('.//COLLECTION/VOUCHER')
    if len(vs) != 1:
        sys.exit('%s: %d VOUCHER elements' % (p, len(vs)))
    return vs[0]


def flat(el, path, out):
    for k, v in el.attrib.items():
        out[path + '/@' + k] = v
    kids = list(el)
    if not kids:
        out[path] = (el.text or '').strip()
        return
    seen = {}
    for c in kids:
        i = seen.get(c.tag, 0)
        seen[c.tag] = i + 1
        flat(c, '%s/%s[%d]' % (path, c.tag, i), out)


a, b = {}, {}
flat(voucher(sys.argv[1]), 'VOUCHER', a)
flat(voucher(sys.argv[2]), 'VOUCHER', b)
n = 0
for k in sorted(set(a) | set(b)):
    if a.get(k) != b.get(k):
        n += 1
        print('%s\n    old: %r\n    new: %r' % (k, a.get(k, '<absent>'), b.get(k, '<absent>')))
print('fields old %d new %d differing %d' % (len(a), len(b), n))
