# W18 expectations (b), (c), (e): parse answers (control chars replaced before parsing) and compare with W16 / W14.
import re, sys, hashlib, xml.etree.ElementTree as ET
A = 'C:/bridge-lab/w18/answers/'
W16 = 'C:/bridge-lab/w16/answers/'
W14 = 'C:/bridge-lab/w14/answers/'

def load(p):
    b = open(p, 'rb').read()
    t = b.decode('utf-16') if b[1:2] == b'\x00' or b[:2] == b'\xff\xfe' else b.decode('utf-8', 'replace')
    t = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '?', t)
    t = re.sub(r'&#(\d+);', lambda m: '?' if int(m.group(1)) < 32 and int(m.group(1)) not in (9, 10, 13) else m.group(0), t)
    return ET.fromstring(t.lstrip('\ufeff'))

def rows(p, tag):
    r = load(p)
    c = r.find('BODY/DATA/COLLECTION')
    return [] if c is None else c.findall(tag)

def names(p, tag='LEDGER'):
    out = []
    for e in rows(p, tag):
        n = e.get('NAME') or e.findtext('NAME')
        out.append((n or '').strip())
    return out

w16_rates = names(W16 + 'aw16-ledgers-rates-hsn-0.xml')
print('W16 rates ledgers:', len(w16_rates), sorted(w16_rates))
for f in ['aw18-ledgers-compliance.xml', 'aw18-ledgers-paired.xml', 'aw18-ledger-catalogue-v2.xml', 'aw18-ledger-rates-hsn.xml']:
    n = names(A + f)
    missing = sorted(set(w16_rates) - set(n)); extra = sorted(set(n) - set(w16_rates))
    print(f'{f}: {len(n)} LEDGER rows; missing vs W16 rates: {missing}; extra: {extra}')
g = names(A + 'aw18-groups.xml', 'GROUP')
print('groups:', len(g), 'BRIDGE Sales Sub present:', 'BRIDGE Sales Sub' in g)

def ledger_xml(p, name):
    for e in rows(p, 'LEDGER'):
        if (e.get('NAME') or '').strip() == name:
            return ET.tostring(e, encoding='unicode')
    return None
a = ledger_xml(A + 'aw18-ledger-rates-hsn.xml', 'BRIDGE Svc 4D 5%')
b = ledger_xml(W16 + 'aw16-ledgers-rates-hsn-0.xml', 'BRIDGE Svc 4D 5%')
print('4D row W18 == W16:', a == b, 'len', len(a or ''), len(b or ''))
if a != b and a and b:
    import difflib
    for l in difflib.unified_diff(b.splitlines(), a.splitlines(), lineterm='', n=0):
        print('  ', l[:200])
if a:
    for k in ['HSNCODE', 'HSN', 'APPLICABLEFROM', 'SRCOFGSTDETAILS', 'GSTTYPEOFSUPPLY', 'HSNSOURCETYPE', 'HSNMASTERNAME']:
        v = re.findall(r'<' + k + r'[^>]*>([^<]*)<', a)
        if v: print('  4D', k, v[:4])

def sha(p): return hashlib.sha256(open(p, 'rb').read()).hexdigest()
co18 = A + 'aw18-company-object.xml'
for ref in [W16 + 'aw16-company-object-0.xml', W14 + 'aw14-company-object-0.xml']:
    same = open(co18, 'rb').read() == open(ref, 'rb').read()
    print('company object vs', ref.split('/')[-1], 'byte-identical:', same, sha(co18)[:8], sha(ref)[:8])
    if not same:
        x = load(co18); y = load(ref)
        tx = [ (e.tag, (e.text or '').strip()) for e in x.iter() ]; ty = [ (e.tag, (e.text or '').strip()) for e in y.iter() ]
        import difflib
        d = [l for l in difflib.unified_diff([f'{a}={b}' for a, b in ty], [f'{a}={b}' for a, b in tx], lineterm='', n=0) if not l.startswith(('---', '+++', '@@'))]
        print('  element/text differences:', len(d))
        for l in d[:40]: print('   ', l[:160])
