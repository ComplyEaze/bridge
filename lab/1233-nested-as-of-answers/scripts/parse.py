# Lab helper (#1233 nested as-of, not product code): reads the saved answers by LAB-COMMON C4 (UTF-16LE, forbidden
# characters dropped, parsed as XML) and writes the sheet's tables: the STOCKITEM rows of a1 and b1 (children of
# ENVELOPE/BODY/DATA/COLLECTION) and every line of the a2 and b2 Stock Summary reports, with the two amount sums.
import re
import sys
import xml.etree.ElementTree as ET
from decimal import Decimal

E = '<lab folder>/evidence/'
FORBID = re.compile('[\x00-\x08\x0b\x0c\x0e-\x1f]')


def load(name):
    b = open(E + name + '.answer.xml', 'rb').read()
    t = b.decode('utf-16')
    t = re.sub(r'&#(x[0-9A-Fa-f]+|[0-9]+);', lambda m: '' if (int(m.group(1)[1:], 16) if m.group(1)[0] == 'x' else int(m.group(1))) in set(range(0, 9)) | {11, 12} | set(range(14, 32)) else m.group(0), t)
    return ET.fromstring(FORBID.sub('', t))


def text(e, tag):
    x = e.find(tag)
    if x is None:
        return '<absent>'
    v = x.text or ''
    return v.strip() if v.strip() else '<empty>'


def dec(v):
    try:
        return Decimal(v.replace(',', ''))
    except Exception:
        return None


out = []
sums = {}
for name in ('a1', 'b1'):
    rows = load(name).findall('./BODY/DATA/COLLECTION/STOCKITEM')
    out.append(f'{name}: STOCKITEM rows {len(rows)}')
    total = Decimal(0)
    for r in rows:
        nm = r.get('NAME') or text(r, 'NAME')
        cv = text(r, 'CLOSINGVALUE')
        out.append(f'  NAME={nm!r} PARENT={text(r, "PARENT")!r} CLOSINGBALANCE={text(r, "CLOSINGBALANCE")!r} CLOSINGVALUE={cv!r}')
        d = dec(cv) if not cv.startswith('<') else None
        if d is not None:
            total += d
    sums[name] = total
for name in ('a2', 'b2'):
    root = load(name)
    kids = list(root)
    lines, cur, total = [], None, Decimal(0)
    for e in kids:
        if e.tag in ('DSPACCNAME',):
            cur = {'name': text(e, 'DSPDISPNAME')}
        elif e.tag == 'DSPSTKINFO' and cur is not None:
            bal = e.find('DSPSTKCL')
            q = text(bal, 'DSPCLQTY') if bal is not None else '<absent>'
            a = text(bal, 'DSPCLAMTA') if bal is not None else '<absent>'
            cur.update(qty=q, amt=a)
            lines.append(cur)
            d = dec(a) if not a.startswith('<') else None
            if d is not None:
                total += d
            cur = None
    out.append(f'{name}: lines {len(lines)}')
    for l in lines:
        out.append(f"  DSPDISPNAME={l['name']!r} DSPCLQTY={l.get('qty')!r} DSPCLAMTA={l.get('amt')!r}")
    sums[name] = total
out.append(f"sum check 31 Aug: a2 DSPCLAMTA sum {sums['a2']} vs a1 CLOSINGVALUE sum {sums['a1']}: {'yes' if sums['a2'] == sums['a1'] else 'no'}")
out.append(f"sum check 30 Sep: b2 DSPCLAMTA sum {sums['b2']} vs b1 CLOSINGVALUE sum {sums['b1']}: {'yes' if sums['b2'] == sums['b1'] else 'no'}")
open(E + 'parsed-tables.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
print('\n'.join(out))
