# Lab helper (2162-S3a, not product code): the sheet's counts for one run.py label, every answer parsed as XML (only the
# characters XML forbids are dropped for parsing; the saved bytes are untouched). Prints and appends to evidence/check.txt.
# Usage: python -I -X utf8 check.py <label>
import glob, hashlib, os, re, sys, xml.etree.ElementTree as ET
G = '<lab folder>/go/'; label = sys.argv[1]; PARTY = 'BRIDGE Walk-in BW'
out = open('<lab folder>/evidence/check.txt', 'a', encoding='utf-8', newline='\n')
def p(*a):
    s = ' '.join(str(x) for x in a); print(s); out.write(s + '\n')
BAD = re.compile(r'[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:[0-8]|1[124-9]|2[0-9]|3[01]);')
def dec(b): return b.decode('utf-16-le').lstrip('\ufeff') if b[1:2] == b'\0' or b[:2] == b'\xff\xfe' else b.decode('utf-8')
def kids(e, tag): return [c for c in e if c.tag == tag]
def txt(e, tag): return [(c.text or '').strip() for c in e if c.tag == tag]
for f in sorted(glob.glob(f'{G}a-{label}-*')):
    b = open(f, 'rb').read(); name = os.path.basename(f); t = dec(b)
    p(f'== {name}: {len(b)} B sha256 {hashlib.sha256(b).hexdigest()}')
    if 'Unknown Request' in t: p('   UNKNOWN REQUEST'); continue
    root = ET.fromstring(BAD.sub('', t))
    if 'M-marks' in name:
        for c in root.iter('COMPANY'):
            if c.attrib or len(c): p('   COMPANY', c.get('NAME'), 'ALTVCHID', txt(c, 'ALTVCHID'), 'ALTMSTID', txt(c, 'ALTMSTID'))
    elif 'L-ledger' in name or 'I-00' in name:
        p('  ', {k: [(e.text or '').strip() for e in root.iter(k)] for k in ('CREATED', 'ALTERED', 'ERRORS', 'EXCEPTIONS', 'LASTVCHID')}, 'LINEERROR', [(e.text or '').strip() for e in root.iter('LINEERROR')])
    elif 'R-00' in name:
        vs = [v for v in root.iter('VOUCHER') if v.attrib or len(v)]
        p('   VOUCHER elements', len(vs))
        for v in vs:
            p('   number', txt(v, 'VOUCHERNUMBER'), 'date', txt(v, 'DATE'), 'ALTERID', txt(v, 'ALTERID'), 'GUID', txt(v, 'GUID'),
              'INCLUDED', txt(v, 'VCHGSTSTATUSISINCLUDED'), 'UNCERTAIN', txt(v, 'VCHGSTSTATUSISUNCERTAIN'), 'EXCLUDED', txt(v, 'VCHGSTSTATUSISEXCLUDED'))
            legs = kids(v, 'ALLLEDGERENTRIES.LIST') + kids(v, 'LEDGERENTRIES.LIST')
            party = [l for l in legs if txt(l, 'LEDGERNAME') == [PARTY]]
            p('   legs', len(legs), '| party legs', len(party), [l.tag for l in party])
            for l in party:
                ba = kids(l, 'BILLALLOCATIONS.LIST')
                p('     party leg AMOUNT', txt(l, 'AMOUNT'), '| BILLALLOCATIONS.LIST direct children', len(ba))
                for a in ba: p('       NAME', txt(a, 'NAME'), 'BILLTYPE', txt(a, 'BILLTYPE'), 'AMOUNT', txt(a, 'AMOUNT'))
            p('   BILLALLOCATIONS.LIST under every other leg', sum(len(list(l.iter('BILLALLOCATIONS.LIST'))) for l in legs if l not in party))
    elif 'B-bills' in name:
        els = list(root.iter()); groups = 0
        for i, e in enumerate(els):
            if e.tag == 'BILLFIXED':
                party = [(c.text or '').strip() for c in e if c.tag == 'BILLPARTY']
                if party == [PARTY]:
                    groups += 1
                    nxt = next((x for x in els[i + 1:] if x.tag == 'BILLCL'), None)
                    p('   group BILLREF', txt(e, 'BILLREF'), 'BILLDATE', txt(e, 'BILLDATE'), 'BILLCL after it', (nxt.text or '').strip() if nxt is not None else None)
        p('   BILLFIXED groups for', PARTY, groups, '| all BILLFIXED groups', sum(1 for e in els if e.tag == 'BILLFIXED'))
