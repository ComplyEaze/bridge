# Lab helper (#2006, not product code): reads a saved Contra capture by XML parsing (UTF-16LE, forbidden control
# characters dropped for parsing only) and prints, per VOUCHER, the fields the sheet lists and every BANKALLOCATIONS.LIST
# child; then compares each entry's sign and allocation type with a reference capture, and checks the references.
# Usage: python -I -X utf8 parse-contra.py <capture> <reference capture> <table out>
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
    return [r for r in coll if r.tag == 'VOUCHER']


def txt(e, f):
    x = e.find(f)
    return None if x is None else ''.join(x.itertext())


def entries(v):
    out = []
    for le in v.findall('ALLLEDGERENTRIES.LIST'):
        allocs = []
        for ba in le.findall('BANKALLOCATIONS.LIST'):
            allocs.append([(c.tag, ''.join(c.itertext())) for c in ba])
        out.append((txt(le, 'LEDGERNAME'), txt(le, 'AMOUNT'), txt(le, 'ISDEEMEDPOSITIVE'), allocs))
    return out


cap, ref, table = sys.argv[1], sys.argv[2], sys.argv[3]
vs = load(cap)
rs = {txt(v, 'VOUCHERNUMBER'): v for v in load(ref)}
lines = []
problems = []
for v in sorted(vs, key=lambda v: int(txt(v, 'VOUCHERNUMBER'))):
    num = txt(v, 'VOUCHERNUMBER')
    lines.append(f"VOUCHER {num} DATE={txt(v, 'DATE')} GUID={txt(v, 'GUID')} ALTERID={(txt(v, 'ALTERID') or '').strip()} "
                 f"NARRATION={txt(v, 'NARRATION')!r} REFERENCE={txt(v, 'REFERENCE')!r}")
    es = entries(v)
    for name, amt, pos, allocs in es:
        lines.append(f"   LEDGERNAME={name!r} AMOUNT={amt} ISDEEMEDPOSITIVE={pos}")
        for a in allocs:
            for tag, val in a:
                lines.append(f"      {tag}={val!r}")
    rv = rs.get(num)
    if rv is None:
        problems.append(f'voucher {num}: not in the reference')
        continue
    re_ = entries(rv)
    mine = sorted((n, a.startswith('-'), [dict(x).get('TRANSACTIONTYPE') for x in al]) for n, a, p, al in es)
    theirs = sorted((n, a.startswith('-'), [dict(x).get('TRANSACTIONTYPE') for x in al]) for n, a, p, al in re_)
    order_mine = [n for n, a, p, al in es]
    order_ref = [n for n, a, p, al in re_]
    lines.append(f"   COMPARE with reference voucher {num}: sign and type {'EQUAL' if mine == theirs else 'DIFFERENT'}; entry order {'EQUAL' if order_mine == order_ref else 'DIFFERENT'} ({order_mine} vs {order_ref}); date {'EQUAL' if txt(v, 'DATE') == txt(rv, 'DATE') else 'DIFFERENT'}")
    if mine != theirs:
        problems.append(f'voucher {num}: sign/type differ: {mine} vs {theirs}')
    insts = [dict(x).get('INSTRUMENTNUMBER') for n, a, p, al in es for x in al]
    lines.append(f"   INSTRUMENTNUMBER per allocation: {insts}")
    if num == '2' and insts != ['222222222222', '222222222222']:
        problems.append(f'voucher 2 instrument numbers {insts}')
    if num == '5' and not all(i == '123456' for i in insts):
        problems.append(f'voucher 5 instrument numbers {insts}')
    if num in ('1', '3', '4') and any(insts):
        problems.append(f'voucher {num} carries an instrument number {insts}')
    if num == '1' and txt(v, 'NARRATION') != 'UTR 111111111111':
        problems.append(f'voucher 1 narration {txt(v, "NARRATION")!r}')
    if num == '4' and txt(v, 'NARRATION') != 'UTR 444444444444':
        problems.append(f'voucher 4 narration {txt(v, "NARRATION")!r}')
lines.append(f'VOUCHERS {len(vs)}; PROBLEMS {len(problems)}')
lines.extend('PROBLEM ' + p for p in problems)
open(table, 'w', encoding='utf-8', newline='\n').write('\n'.join(lines) + '\n')
print('\n'.join(l for l in lines if l.startswith(('VOUCHER', '   COMPARE', '   INSTRUMENT', 'PROBLEM'))))
