# Lab helper (#485 D/E, not product code): LAB-COMMON C4 reading of saved answers; prints row tag counts, identity-field
# presence per row, and GUID-set comparisons. Usage: python -I -X utf8 compare.py <ground-truth answer> <ledger> <candidate answer> [...]
import re
import sys
import xml.etree.ElementTree as ET

def load(path):
    b = open(path, 'rb').read()
    t = b[2:].decode('utf-16-le') if b[:2] == b'\xff\xfe' else b.decode('utf-16-le')
    t = re.sub('[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t)
    t = re.sub(r'&#(x[0-9A-Fa-f]+|[0-9]+);', lambda m: '' if (lambda c: c <= 8 or c in (11, 12) or 14 <= c <= 31)(
        int(m.group(1)[1:], 16) if m.group(1).startswith('x') else int(m.group(1))) else m.group(0), t)
    root = ET.fromstring(t)
    coll = root.find('BODY/DATA/COLLECTION')
    return [] if coll is None else list(coll)

def txt(r, f):
    e = r.find(f)
    return None if e is None else ''.join(e.itertext()).strip()

def describe(path):
    rows = load(path)
    tags = {}
    for r in rows:
        tags[r.tag] = tags.get(r.tag, 0) + 1
    v = [r for r in rows if r.tag == 'VOUCHER']
    missing = {f: sum(1 for r in v if not txt(r, f)) for f in ('GUID', 'ALTERID', 'MASTERID', 'DATE')}
    print(f'{path}: row tags {tags}; VOUCHER rows {len(v)}; empty-or-absent per field {missing}')
    return v

if __name__ == '__main__':
    gt, ledger, cands = sys.argv[1], sys.argv[2], sys.argv[3:]
    g = describe(gt)
    want = set()
    for r in g:
        names = [''.join(e.itertext()) for e in r.iter('LEDGERNAME')]
        if ledger in names:
            want.add(txt(r, 'GUID'))
    print(f'ground truth: vouchers with a "{ledger}" ledger line: {len(want)}')
    for c in cands:
        v = describe(c)
        got = [txt(r, 'GUID') for r in v]
        s = set(got)
        print(f'  {c}: GUIDs {len(got)} (distinct {len(s)}); equal sets: {s == want}; only in candidate {len(s - want)}; only in ground truth {len(want - s)}')
        alt = sorted(int(txt(r, 'ALTERID')) for r in v if txt(r, 'ALTERID'))
        if alt:
            print(f'  ALTERID range {alt[0]}..{alt[-1]}; dates {sorted(set(txt(r, "DATE") for r in v))}')
