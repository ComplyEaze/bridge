# Lab helper (#1233 TIEOUT D end date, not product code): reads t1 and t2 by LAB-COMMON C4 (UTF-16LE; the characters XML
# forbids dropped, raw or as references; parsed as XML) and writes the parser's table: t1's STOCKITEM rows and every t2 line.
import re
import xml.etree.ElementTree as ET
from decimal import Decimal

E = '<lab folder>/evidence/'
BAD = set(range(0, 9)) | {11, 12} | set(range(14, 32))


def load(name):
    t = open(E + name + '.answer.xml', 'rb').read().decode('utf-16')
    t = re.sub(r'&#(x[0-9A-Fa-f]+|[0-9]+);', lambda m: '' if (int(m.group(1)[1:], 16) if m.group(1)[0] == 'x' else int(m.group(1))) in BAD else m.group(0), t)
    return ET.fromstring(re.sub('[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t))


def tx(e, tag):
    x = e.find(tag)
    if x is None:
        return '<absent>'
    return (x.text or '').strip() or '<empty>'


out = []
rows = load('t1').findall('./BODY/DATA/COLLECTION/STOCKITEM')
out.append(f't1: STOCKITEM rows {len(rows)}')
cv = None
for r in rows:
    out.append(f"  NAME={(r.get('NAME') or tx(r, 'NAME'))!r} PARENT={tx(r, 'PARENT')!r} CLOSINGBALANCE={tx(r, 'CLOSINGBALANCE')!r} CLOSINGVALUE={tx(r, 'CLOSINGVALUE')!r}")
    cv = tx(r, 'CLOSINGVALUE')
lines, cur = [], None
for e in load('t2'):
    if e.tag == 'DSPACCNAME':
        cur = {'n': tx(e, 'DSPDISPNAME')}
    elif e.tag == 'DSPSTKINFO' and cur is not None:
        b = e.find('DSPSTKCL')
        cur.update(q=tx(b, 'DSPCLQTY'), a=tx(b, 'DSPCLAMTA'))
        lines.append(cur)
        cur = None
out.append(f't2: lines {len(lines)}')
for l in lines:
    out.append(f"  DSPDISPNAME={l['n']!r} DSPCLQTY={l['q']!r} DSPCLAMTA={l['a']!r}")
eq = len(lines) == 1 and cv not in (None, '<empty>', '<absent>') and Decimal(lines[0]['a']) == Decimal(cv)
out.append(f"t2 DSPCLAMTA equals t1 CLOSINGVALUE (exact decimals): {'yes' if eq else 'no'}")
open(E + 'parsed-tables.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
print('\n'.join(out))
