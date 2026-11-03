# Lab helper (683-R1, not product code): sets the class-entry window answer (exchange 108) against #2276's fixture
# answer vouchers-forex-unspaced-20250401.utf16le.xml at 75224fe3 (read with git show into a scratch folder given as
# argument), voucher by voucher (GUID) and AMOUNT/VATEXPAMOUNT element by element, with an XML parser. Prints counts
# and the fixture files' sha256 only. Usage: python -I -X utf8 compare2276.py <folder holding ans.xml and req.xml>
import hashlib, re, sys
import xml.etree.ElementTree as ET
S = sys.argv[1].rstrip('/') + '/'
def load(b):
    t = b[2:].decode('utf-16-le') if b[:2] == b'\xff\xfe' else b.decode('utf-16-le') if b[1:2] == b'\0' else b.decode('utf-8')
    t = t.strip().replace('&#4;', '').replace('\x03', '').replace('\r', '&#13;'); return ET.fromstring(re.sub(r'<\?xml[^>]*\?>', '', t))
def amounts(root):
    d = {}
    for v in root.iter('VOUCHER'):
        g = (v.findtext('GUID') or '').strip()
        if not g: continue
        rows = []
        def walk(e, p):
            for c in e:
                q = p + '/' + c.tag
                if c.tag in ('AMOUNT', 'VATEXPAMOUNT') and c.text and c.text.strip(): rows.append((q, c.text.strip()))
                walk(c, q)
        walk(v, ''); d[g] = rows
    return d
fa = open(S + 'ans.xml', 'rb').read(); fr = open(S + 'req.xml', 'rb').read()
print('fixture answer sha256', hashlib.sha256(fa).hexdigest(), 'request sha256', hashlib.sha256(fr).hexdigest())
fx = amounts(load(fa))
raw = open('<lab folder>/relay/exchange-108.answer', 'rb').read(); cl = amounts(load(raw[raw.find(b'\r\n\r\n') + 4:]))
print('fixture vouchers', len(fx), 'class-entry vouchers', len(cl), 'same GUIDs', set(fx) == set(cl))
eq = ne = pd = 0
for g in fx:
    if [p for p, _ in fx[g]] != [p for p, _ in cl.get(g, [])]: pd += 1
    for (_, t1), (_, t2) in zip(fx[g], cl.get(g, [])): eq += t1 == t2; ne += t1 != t2
print('amount texts equal', eq, 'different', ne, 'vouchers whose amount paths differ', pd)
for n in (22, 108):
    rq = open(f'<lab folder>/relay/exchange-{n}.request', 'rb').read(); b = rq[rq.find(b'\r\n\r\n') + 4:]
    print(f'exchange {n} request body equal to the fixture request:', b == fr, len(b), 'B against', len(fr), 'B')
