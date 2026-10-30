# Lab helper (#2017 steps 5 and 8, not product code): each raw answer read by structure (LAB-COMMON C4: forbidden
# characters dropped for parsing only, parsed as XML, never a text search). Records the sequence of top-level element
# names, and for each line (a DSPACCNAME followed by its DSPSTKINFO) the display name and DSPCLQTY, DSPCLRATE and DSPCLAMTA
# as sent, empty values marked; any element not in that shape is listed by path. For an exploded answer, each line is
# marked TOP when its name is one of the plain answer's lines for the same report and date, else CHILD (position tells
# its parent: the TOP line before it). The sum of the TOP amounts is printed (the wire sign kept).
# Usage: python -I -X utf8 parse5.py <answer label> [<plain answer label>]   (labels without .answer.xml)
import re, sys, xml.etree.ElementTree as ET
from decimal import Decimal
E = '<LAB_DIR>/evidence'
def load(label):
    b = open(f'{E}/{label}.answer.xml', 'rb').read()
    if not b: return None
    t = (b[2:] if b[:2] == b'\xff\xfe' else b).decode('utf-16-le')
    t = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t)
    t = re.sub(r'&#(x[0-9a-fA-F]+|[0-9]+);', lambda m: '' if (int(m.group(1)[1:], 16) if m.group(1)[0] == 'x' else int(m.group(1))) in set(range(0, 9)) | {11, 12} | set(range(14, 32)) else m.group(0), t)
    return ET.fromstring(t)
def lines(root):
    out, other, cur = [], [], None
    for el in list(root):
        if el.tag == 'DSPACCNAME':
            cur = {'name': el.findtext('DSPDISPNAME'), 'extra': [c.tag for c in el if c.tag != 'DSPDISPNAME']}; out.append(cur)
        elif el.tag == 'DSPSTKINFO' and cur is not None:
            cl = el.find('DSPSTKCL')
            for k in ('DSPCLQTY', 'DSPCLRATE', 'DSPCLAMTA'):
                v = cl.findtext(k) if cl is not None and cl.find(k) is not None else None
                cur[k] = 'absent' if v is None else ('(empty)' if v.strip() == '' else v.strip())
            extra = [c.tag for c in (cl if cl is not None else [])] if cl is not None else []
            cur['extra'] += [x for x in extra if x not in ('DSPCLQTY', 'DSPCLRATE', 'DSPCLAMTA')] + [c.tag for c in el if c.tag != 'DSPSTKCL']
        else:
            other.append(el.tag)
    return out, other
label = sys.argv[1]; plain = sys.argv[2] if len(sys.argv) > 2 else None
root = load(label)
res = [f'{label}:']
if root is None: res.append('  0-byte answer')
else:
    seq = [c.tag for c in root]
    res.append(f'  top-level element names ({len(seq)}): ' + ' '.join(seq[:40]) + (' ...' if len(seq) > 40 else ''))
    st = root.find('.//STATUS'); le = [x.text for x in root.iter('LINEERROR')]
    if st is not None or le: res.append(f'  STATUS {st.text if st is not None else "absent"}; LINEERROR {le}')
    ls, other = lines(root)
    tops = None
    if plain:
        pl, _ = lines(load(plain)); tops = [x['name'] for x in pl]
    total = Decimal(0); tl = []
    for x in ls:
        kind = '' if tops is None else ('TOP   ' if x['name'] in tops else 'CHILD ')
        res.append(f'  {kind}{x["name"]!r}: qty {x.get("DSPCLQTY")}; rate {x.get("DSPCLRATE")}; amount {x.get("DSPCLAMTA")}' + (f'; other elements {x["extra"]}' if x['extra'] else ''))
        if (tops is None or x['name'] in tops) and x.get('DSPCLAMTA') not in (None, 'absent', '(empty)'):
            total += Decimal(x['DSPCLAMTA']); tl.append(x['name'])
    if other: res.append(f'  elements outside the line shape: {other}')
    res.append(f'  lines {len(ls)}; sum of the ' + ('TOP ' if tops else '') + f'line amounts (wire sign) {total}')
open(f'{E}/{label}.parsed.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(res) + '\n')
print('\n'.join(res))
