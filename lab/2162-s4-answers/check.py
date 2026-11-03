# Lab helper (2162-S4, not product code): the sheet's counts for one run.py label, every answer parsed as XML (only the
# characters XML forbids are dropped for parsing; the saved bytes are untouched). Appends to evidence/check.txt.
# Usage: python -I -X utf8 check.py <label>
import glob, hashlib, os, re, sys, xml.etree.ElementTree as ET
G = '<lab folder>/go/'; label = sys.argv[1]
out = open('<lab folder>/evidence/check.txt', 'a', encoding='utf-8', newline='\n')
def p(*a):
    s = ' '.join(str(x) for x in a); print(s); out.write(s + '\n')
BAD = re.compile(r'[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:[0-8]|1[124-9]|2[0-9]|3[01]);')
def dec(b): return b.decode('utf-16-le').lstrip('\ufeff') if b[1:2] == b'\0' or b[:2] == b'\xff\xfe' else b.decode('utf-8')
def txt(e, tag): return [(c.text or '').strip() for c in e if c.tag == tag]
for f in sorted(glob.glob(f'{G}a-{label}-*')):
    b = open(f, 'rb').read(); name = os.path.basename(f); t = dec(b)
    p(f'== {name}: {len(b)} B sha256 {hashlib.sha256(b).hexdigest()}')
    if 'Unknown Request' in t: p('   UNKNOWN REQUEST'); continue
    root = ET.fromstring(BAD.sub('', t))
    if 'M-marks' in name:
        for c in root.iter('COMPANY'):
            if c.attrib or len(c): p('   COMPANY', c.get('NAME'), 'ALTVCHID', txt(c, 'ALTVCHID'), 'ALTMSTID', txt(c, 'ALTMSTID'))
    elif '-I-' in name:
        p('  ', {k: [(e.text or '').strip() for e in root.iter(k)] for k in ('CREATED', 'ALTERED', 'ERRORS', 'EXCEPTIONS', 'LASTVCHID')}, 'LINEERROR', [(e.text or '').strip() for e in root.iter('LINEERROR')])
    elif '-N-' in name:
        rows = [e for e in root.iter() if any(c.tag == 'VOUCHERNUMBER' for c in e)]
        p('   rows with a VOUCHERNUMBER child', len(rows))
        for r in rows: p('   ', r.tag, 'VOUCHERNUMBER', txt(r, 'VOUCHERNUMBER'), 'DATE', txt(r, 'DATE'), 'TYPE', txt(r, 'VOUCHERTYPENAME'), 'ISCANCELLED', txt(r, 'ISCANCELLED'), 'BRIDGEVCHISSALES', txt(r, 'BRIDGEVCHISSALES'))
    elif '-R-' in name:
        vs = [v for v in root.iter('VOUCHER') if v.attrib or len(v)]
        p('   VOUCHER elements', len(vs))
        for v in vs: p('   ', 'VOUCHERNUMBER', txt(v, 'VOUCHERNUMBER'), 'DATE', txt(v, 'DATE'), 'ALTERID', txt(v, 'ALTERID'), 'GUID last 8', [g[-8:] for g in txt(v, 'GUID')])
