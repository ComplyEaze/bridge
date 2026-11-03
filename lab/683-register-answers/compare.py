# Lab helper (683-R1, not product code): sets the class-entry window answer (exchange 108) against the plain
# entry window answer (exchange 22), voucher by voucher (by GUID) and entry by entry, with an XML parser. For each
# element named AMOUNT or VATEXPAMOUNT it records the path under the voucher, and says whether its text is equal in
# the two answers. Prints counts and text shapes only (each digit shown as 9, letters as a), never a figure.
# Usage: python -I -X utf8 compare.py <plain exchange> <class exchange>
import re, sys
import xml.etree.ElementTree as ET
R = '<lab folder>/relay/'
def load(n):
    raw = open(f'{R}exchange-{n}.answer', 'rb').read(); b = raw[raw.find(b'\r\n\r\n') + 4:]
    t = b.decode('utf-16-le') if b[1:2] == b'\0' else b.decode('utf-8')
    t = t.strip().replace('&#4;', '').replace('\x03', '').replace('\r', '&#13;')
    return ET.fromstring(re.sub(r'<\?xml[^>]*\?>', '', t))
def shape(s): return re.sub(r'[A-Za-z]', 'a', re.sub(r'[0-9]', '9', s))
def amounts(v):
    out = []
    def walk(e, path):
        for c in e:
            p = path + '/' + c.tag
            if c.tag in ('AMOUNT', 'VATEXPAMOUNT') and c.text and c.text.strip(): out.append((p, c.text.strip()))
            walk(c, p)
    walk(v, '')
    return out
a, b = load(sys.argv[1]), load(sys.argv[2])
va = {v.findtext('GUID').strip(): v for v in a.iter('VOUCHER') if v.findtext('GUID')}
vb = {v.findtext('GUID').strip(): v for v in b.iter('VOUCHER') if v.findtext('GUID')}
print('vouchers plain', len(va), 'class', len(vb), 'same GUIDs', set(va) == set(vb))
eq = ne = 0; paths = {}; shapes = {}; only = 0
for g in va:
    pa, pb = amounts(va[g]), amounts(vb.get(g, ET.Element('x')))
    if [p for p, _ in pa] != [p for p, _ in pb]: only += 1
    for (p1, t1), (p2, t2) in zip(pa, pb):
        if t1 == t2: eq += 1
        else: ne += 1
        key = p1 + (' composite' if ' @ ' in t1 else '')
        paths[key] = paths.get(key, 0) + 1
        if ' @ ' in t1: shapes[shape(t1)] = shapes.get(shape(t1), 0) + 1
print('amount texts equal', eq, 'different', ne, 'vouchers whose amount paths differ', only)
for k, n in sorted(paths.items()): print('  path', k, n)
for k, n in sorted(shapes.items()): print('  composite text shape', repr(k), n)
