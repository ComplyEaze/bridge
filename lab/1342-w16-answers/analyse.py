# W16: parse answers (raw control chars such as \x03 replaced by '|' for parsing only) and compare control vs new.
import re, sys, xml.etree.ElementTree as ET
def load(p):
    t = open(p, 'rb').read().decode('utf-16-le').lstrip('\ufeff').replace('&#4;', '\u00a4')
    t = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '|', t)
    return ET.fromstring(t)
def rows(r, tag):
    out = {}
    for e in r.findall('.//COLLECTION/' + tag):
        out[e.get('NAME') or (e.findtext('NAME') or '').strip()] = e
    return out
def hsn(e):
    res = []
    for h in e.iter('HSNDETAILS.LIST'):
        res.append({c.tag: (c.text or '').strip() for c in h if len(c) == 0})
    return res
P = 'C:/bridge-lab/w16/answers/'
for ctl, new, tag in [('aw16-ledgers-rates-0.xml', 'aw16-ledgers-rates-hsn-0.xml', 'LEDGER'), ('aw16-groups-0.xml', 'aw16-groups-hsn-0.xml', 'GROUP')]:
    a, b = load(P + ctl), load(P + new)
    ra, rb = rows(a, tag), rows(b, tag)
    print('==', tag, 'control rows', len(ra), 'new rows', len(rb), 'same names:', set(ra) == set(rb), '| LINEERROR', [x.text for x in b.iter('LINEERROR')])
    added = set()
    for n in rb:
        ta = {c.tag for c in ra.get(n, [])}; tb = {c.tag for c in rb[n]}
        added |= (tb - ta)
    print('   new top-level child tags vs control:', sorted(added))
    for n in sorted(rb):
        e = rb[n]; h = hsn(e)
        extra = {k: (e.findtext(k) or '').strip() for k in ('GSTAPPLICABLE', 'GSTTYPEOFSUPPLY') if e.find(k) is not None}
        if h or extra or 'BRIDGE' in n or n in ('Sales Accounts',):
            print('   ', repr(n), 'HSN rows', len(h), h[:3], extra)
r = load(P + 'aw16-company-object-0.xml')
print('== company object: root', r.tag, 'BODY', r.find('BODY') is not None, 'LINEERROR', [x.text for x in r.iter('LINEERROR')], 'elements', sum(1 for _ in r.iter()))
for L in ('HSNDETAILS.LIST', 'CMPGSTDETAILS.LIST'):
    for e in r.iter(L): print('  ', L, {c.tag: (c.text or '').strip() for c in e if len(c) == 0})
print('   CMPTAXAMOUNTTOLERENCE', [x.text for x in r.iter('CMPTAXAMOUNTTOLERENCE')])
m = open(P + 'aw16-marks-0.xml', 'rb').read().decode('utf-16-le')
print('== marks', re.findall(r'<(ALTVCHID|ALTMSTID)[^>]*>\s*([^<]*)', m))
