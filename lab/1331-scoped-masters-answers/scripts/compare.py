# Lab helper (#1331, not product code): the sheet's "equal to the control's row", by parsing both answers (C4): each LEDGER row
# of ENVELOPE/BODY/DATA/COLLECTION is flattened to its element paths with their values (attributes included, text not trimmed)
# and compared with the control's row of the same key (GUID, or NAME for the balance rows). A difference is listed by element
# path, never by value. Also: the catalogue's ledgers under named parents against a parent-filtered answer's rows.
# Writes evidence/compare.txt. Usage: python -I -X utf8 compare.py
import re, xml.etree.ElementTree as ET
E = '<LAB_DIR>/evidence'
def parse(label):
    b = open(f'{E}/{label}.xml', 'rb').read(); t = (b[2:] if b[:2] == b'\xff\xfe' else b).decode('utf-16-le')
    t = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t)
    t = re.sub(r'&#(x[0-9a-fA-F]+|[0-9]+);', lambda m: '' if (int(m.group(1)[1:], 16) if m.group(1)[0] == 'x' else int(m.group(1))) in set(range(0, 9)) | {11, 12} | set(range(14, 32)) else m.group(0), t)
    c = ET.fromstring(t).find('./BODY/DATA/COLLECTION')
    return [x for x in c if x.tag == 'LEDGER'] if c is not None else []
def flat(e, p=''):
    out = []
    path = f'{p}/{e.tag}'
    for k, v in sorted(e.attrib.items()): out.append((f'{path}@{k}', v))
    out.append((path + '#text', e.text or ''))
    seen = {}
    for c in e:
        i = seen.get(c.tag, 0); seen[c.tag] = i + 1
        out += flat(c, f'{path}[{i}]' if False else path + f'[{i}]')
    return out
def key(e, by):
    if by == 'GUID': return (e.findtext('GUID') or '').strip()
    return e.get('NAME') if e.get('NAME') is not None else (e.findtext('NAME') or '')
lines = []
def cmp(label, control, by, expect=None):
    rows, ctl = parse(label), {key(x, by): x for x in parse(control)}
    keys = [key(x, by) for x in rows]
    lines.append(f'{label} against {control} by {by}: {len(rows)} row(s): {keys}' + (f'; expected {expect}: {"equal" if expect is not None and keys == expect else "DIFFERENT"}' if expect is not None else ''))
    for x in rows:
        k = key(x, by)
        if k not in ctl: lines.append(f'  {k}: NOT IN THE CONTROL'); continue
        a, b = flat(x), flat(ctl[k])
        if a == b: lines.append(f'  {k}: equal ({len(a)} paths)')
        else:
            da, db = dict(a), dict(b)
            diff = sorted(set(p for p in set(da) | set(db) if da.get(p) != db.get(p)))
            lines.append(f'  {k}: DIFFERS at {len(diff)} path(s): {diff[:30]}' + (f'; path lists differ in order or length ({len(a)} vs {len(b)})' if not diff else ''))
G = '6b43e498-430c-4d5c-bfef-d32e2ab93c85-'
five = [G + s for s in ('000000d0', '000000d3', '000000d5', '000000d7', '000000d8')]
def catalogue_under(parents):
    return sorted((x.findtext('GUID') or '').strip() for x in parse('A2-catalogue') if (x.findtext('PARENT') or '') in parents)
import sys
pre = sys.argv[1] if len(sys.argv) > 1 else 'A'
if pre == 'A':
    for lab in ('A5-g1', 'A6-g2', 'A9-n1', 'A10-n2'): cmp(lab, 'A4-p1', 'GUID', {'A5-g1': sorted(five), 'A6-g2': sorted(five[:4]), 'A9-n1': sorted(five)}.get(lab) and None)
    for lab, exp in (('A5-g1', five), ('A6-g2', five[:4]), ('A9-n1', five)):
        got = sorted((x.findtext('GUID') or '').strip() for x in parse(lab))
        lines.append(f'{lab}: GUID set {"equal to" if got == sorted(exp) else "DIFFERENT from"} the expected {len(exp)}')
    cmp('A9-n1', 'A5-g1', 'GUID')
    cmp('A8-g4', 'A4-c-master-whole', 'GUID')
    got = sorted((x.findtext('GUID') or '').strip() for x in parse('A8-g4')); lines.append(f'A8-g4: GUID set {"equal to" if got == sorted(five) else "DIFFERENT from"} the five')
    cmp('A8b-g5', 'A4-c-balance-whole', 'NAME')
    for lab, ps in (('A4-p1', ('Sundry Debtors', 'Sales Accounts', 'Duties & Taxes', 'Indirect Expenses')), ('A12-p2', ('Sales Accounts', 'Duties & Taxes', 'Indirect Expenses'))):
        got = sorted((x.findtext('GUID') or '').strip() for x in parse(lab)); want = catalogue_under(ps)
        lines.append(f'{lab}: {len(got)} rows; the catalogue holds {len(want)} ledgers under {list(ps)}: {"the same GUIDs" if got == want else "DIFFERENT GUIDs"}')
open(f'{E}/compare-{pre}.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(lines) + '\n')
print('\n'.join(lines))
