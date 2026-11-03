# Lab helper (2162-S3b, not product code): the sheet's counts for one run.py label, every answer parsed as XML (only the
# characters XML forbids are dropped for parsing; the saved bytes are untouched). Bill allocations are counted by content:
# a BILLALLOCATIONS.LIST with a NAME child is an allocation; whitespace-only ones are counted apart. Appends to
# evidence/check.txt. Usage: python -I -X utf8 check.py <label> [customer for R]
import glob, hashlib, os, re, sys, xml.etree.ElementTree as ET
G = '<lab folder>/go/'; label = sys.argv[1]
CUST = ['BRIDGE Walk-in BJ', 'BRIDGE Walk-in BS', 'BRIDGE Walk-in BO']
RCUST = {'s5r': 'BRIDGE Walk-in BJ', 's6r': 'BRIDGE Walk-in BS', 's7r': 'BRIDGE Walk-in BO'}
out = open('<lab folder>/evidence/check.txt', 'a', encoding='utf-8', newline='\n')
def p(*a):
    s = ' '.join(str(x) for x in a); print(s); out.write(s + '\n')
BAD = re.compile(r'[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:[0-8]|1[124-9]|2[0-9]|3[01]);')
def dec(b): return b.decode('utf-16-le').lstrip('\ufeff') if b[1:2] == b'\0' or b[:2] == b'\xff\xfe' else b.decode('utf-8')
def txt(e, tag): return [(c.text or '').strip() for c in e if c.tag == tag]
def legs(v): return [c for c in v if c.tag in ('ALLLEDGERENTRIES.LIST', 'LEDGERENTRIES.LIST')]
def allocs(leg):
    ba = [c for c in leg if c.tag == 'BILLALLOCATIONS.LIST']
    named = [a for a in ba if any(c.tag == 'NAME' for c in a)]
    blank = [a for a in ba if len(a) == 0 and not (a.text or '').strip()]
    return named, blank, len(ba)
for f in sorted(glob.glob(f'{G}a-{label}-*')):
    b = open(f, 'rb').read(); name = os.path.basename(f); t = dec(b)
    p(f'== {name}: {len(b)} B sha256 {hashlib.sha256(b).hexdigest()}')
    if 'Unknown Request' in t: p('   UNKNOWN REQUEST'); continue
    root = ET.fromstring(BAD.sub('', t))
    if 'M-marks' in name:
        for c in root.iter('COMPANY'):
            if c.attrib or len(c): p('   COMPANY', c.get('NAME'), 'ALTVCHID', txt(c, 'ALTVCHID'), 'ALTMSTID', txt(c, 'ALTMSTID'))
    elif 'L-ledgers' in name or 'I-00' in name:
        p('  ', {k: [(e.text or '').strip() for e in root.iter(k)] for k in ('CREATED', 'ALTERED', 'ERRORS', 'EXCEPTIONS', 'LASTVCHID')}, 'LINEERROR', [(e.text or '').strip() for e in root.iter('LINEERROR')])
    elif 'B-bills' in name:
        els = list(root.iter())
        p('   all BILLFIXED groups', sum(1 for e in els if e.tag == 'BILLFIXED'))
        for cu in CUST:
            n = 0
            for i, e in enumerate(els):
                if e.tag == 'BILLFIXED' and txt(e, 'BILLPARTY') == [cu]:
                    n += 1; nxt = next((x for x in els[i + 1:] if x.tag == 'BILLCL'), None)
                    p(f'   {cu}: BILLREF {txt(e, "BILLREF")} BILLDATE {txt(e, "BILLDATE")} BILLCL after it {(nxt.text or "").strip() if nxt is not None else None}')
            p(f'   {cu}: groups {n}')
    elif 'V-days' in name:
        vs = [v for v in root.iter('VOUCHER') if v.attrib or len(v)]
        p('   VOUCHER elements', len(vs))
        for v in vs:
            p('   voucher', txt(v, 'VOUCHERTYPENAME'), 'no', txt(v, 'VOUCHERNUMBER'), 'date', txt(v, 'DATE'), 'ALTERID', txt(v, 'ALTERID'), 'legs', [txt(l, 'LEDGERNAME') + txt(l, 'AMOUNT') for l in legs(v)])
            for l in legs(v):
                if txt(l, 'LEDGERNAME') and txt(l, 'LEDGERNAME')[0] in CUST:
                    named, blank, n = allocs(l)
                    p(f'     leg {txt(l, "LEDGERNAME")} AMOUNT {txt(l, "AMOUNT")}: allocations with NAME {len(named)}, whitespace-only {len(blank)}, elements {n}')
                    for a in named: p('       NAME', txt(a, 'NAME'), 'BILLTYPE', txt(a, 'BILLTYPE'), 'AMOUNT', txt(a, 'AMOUNT'))
    elif 'R-00' in name:
        cu = RCUST.get(label); vs = [v for v in root.iter('VOUCHER') if v.attrib or len(v)]
        p('   VOUCHER elements', len(vs), '| customer', cu)
        for v in vs:
            p('   number', txt(v, 'VOUCHERNUMBER'), 'date', txt(v, 'DATE'), 'ALTERID', txt(v, 'ALTERID'), 'GUID', txt(v, 'GUID'),
              'INCLUDED', txt(v, 'VCHGSTSTATUSISINCLUDED'), 'UNCERTAIN', txt(v, 'VCHGSTSTATUSISUNCERTAIN'), 'EXCLUDED', txt(v, 'VCHGSTSTATUSISEXCLUDED'))
            party = [l for l in legs(v) if txt(l, 'LEDGERNAME') == [cu]]; other = [l for l in legs(v) if l not in party]
            p('   legs', len(legs(v)), '| customer legs', len(party))
            for l in party:
                named, blank, n = allocs(l)
                p(f'     customer leg AMOUNT {txt(l, "AMOUNT")}: allocations with NAME {len(named)}, whitespace-only {len(blank)}')
                for a in named: p('       NAME', txt(a, 'NAME'), 'BILLTYPE', txt(a, 'BILLTYPE'), 'AMOUNT', txt(a, 'AMOUNT'))
            p('   other legs: allocations with NAME', sum(len(allocs(l)[0]) for l in other), '| whitespace-only', sum(len(allocs(l)[1]) for l in other))
