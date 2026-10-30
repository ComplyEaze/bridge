# Lab helper (#2017 step 8, not product code): parse5.py's reading (C4: forbidden characters dropped for parsing only,
# parsed as XML, never a text search), with the shapes Part B's answers added. parse5 folded every DSPSTKINFO after a
# DSPACCNAME into that line, so an item followed by SSBATCHNAME sub-lines showed the LAST sub-line's values: superseded.
# Here a DSPACCNAME + its first DSPSTKINFO is a line; an SSBATCHNAME (SSBATCH, SSGODOWN) + the DSPSTKINFO after it is a
# sub-line of the line before it, recorded with its batch and godown as sent. A Stock Query answer (STQGODOWNNAME,
# STQBATCHNAME, STQGODOWNQTY triples) is listed triple by triple. TOP/CHILD against the plain answer as in parse5; the
# sum of the TOP line amounts is printed (wire sign kept); sub-line amounts are summed per line and printed beside it.
# Usage: python -I -X utf8 parse6.py <answer label> [<plain answer label>]   (labels without .answer.xml)
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
def vals(info):
    cl = info.find('DSPSTKCL'); d = {}
    for k in ('DSPCLQTY', 'DSPCLRATE', 'DSPCLAMTA'):
        v = cl.findtext(k) if cl is not None and cl.find(k) is not None else None
        d[k] = 'absent' if v is None else ('(empty)' if v.strip() == '' else v.strip())
    extra = [c.tag for c in cl if c.tag not in ('DSPCLQTY', 'DSPCLRATE', 'DSPCLAMTA')] if cl is not None else []
    return d, extra + [c.tag for c in info if c.tag != 'DSPSTKCL']
def txt(el, k):
    v = el.findtext(k)
    return 'absent' if v is None else ('(empty)' if v.strip() == '' else v.strip())
def lines(root):
    out, other, cur, pend = [], [], None, None
    for el in list(root):
        if el.tag == 'DSPACCNAME':
            cur = {'name': el.findtext('DSPDISPNAME'), 'extra': [c.tag for c in el if c.tag != 'DSPDISPNAME'], 'subs': [], 'v': None}; out.append(cur); pend = cur
        elif el.tag == 'SSBATCHNAME' and cur is not None:
            sub = {'batch': txt(el, 'SSBATCH'), 'godown': txt(el, 'SSGODOWN'), 'extra': [c.tag for c in el if c.tag not in ('SSBATCH', 'SSGODOWN')], 'v': None}
            cur['subs'].append(sub); pend = sub
        elif el.tag == 'DSPSTKINFO' and pend is not None and pend['v'] is None:
            pend['v'], ex = vals(el); pend['extra'] += ex
        else:
            other.append(el.tag)
    return out, other
def fmt(v): return f'qty {v["DSPCLQTY"]}; rate {v["DSPCLRATE"]}; amount {v["DSPCLAMTA"]}' if v else 'no DSPSTKINFO'
def amt(v): return Decimal(v['DSPCLAMTA']) if v and v['DSPCLAMTA'] not in ('absent', '(empty)') else None
label = sys.argv[1]; plain = sys.argv[2] if len(sys.argv) > 2 else None
root = load(label)
res = [f'{label} (parse6):']
if root is None: res.append('  0-byte answer')
else:
    seq = [c.tag for c in root]
    res.append(f'  top-level element names ({len(seq)}): ' + ' '.join(seq[:40]) + (' ...' if len(seq) > 40 else ''))
    st = root.find('.//STATUS'); le = [x.text for x in root.iter('LINEERROR')]
    if st is not None or le: res.append(f'  STATUS {st.text if st is not None else "absent"}; LINEERROR {le}')
    if any(t == 'STQGODOWNNAME' for t in seq):
        kids = list(root); i = 0; other = []
        while i < len(kids):
            if kids[i].tag == 'STQGODOWNNAME' and i + 2 < len(kids) and kids[i + 1].tag == 'STQBATCHNAME' and kids[i + 2].tag == 'STQGODOWNQTY':
                res.append(f'  godown {kids[i].text!r}; batch {kids[i + 1].text!r}; quantity {kids[i + 2].text!r}'); i += 3
            else: other.append(kids[i].tag); i += 1
        if other: res.append(f'  elements outside the triple shape: {other}')
    else:
        ls, other = lines(root)
        tops = None
        if plain:
            pl, _ = lines(load(plain)); tops = [x['name'] for x in pl]
        total = Decimal(0)
        for x in ls:
            kind = '' if tops is None else ('TOP   ' if x['name'] in tops else 'CHILD ')
            res.append(f'  {kind}{x["name"]!r}: {fmt(x["v"])}' + (f'; other elements {x["extra"]}' if x['extra'] else ''))
            ss = Decimal(0); n = 0
            for s in x['subs']:
                res.append(f'      sub-line batch {s["batch"]!r} godown {s["godown"]!r}: {fmt(s["v"])}' + (f'; other elements {s["extra"]}' if s['extra'] else ''))
                if amt(s['v']) is not None: ss += amt(s['v']); n += 1
            if x['subs']: res.append(f'      {len(x["subs"])} sub-lines; their amounts sum to {ss} (line amount {x["v"]["DSPCLAMTA"] if x["v"] else "none"})')
            if (tops is None or x['name'] in tops) and amt(x['v']) is not None: total += amt(x['v'])
        if other: res.append(f'  elements outside the line shape: {other}')
        res.append(f'  lines {len(ls)}; sum of the ' + ('TOP ' if tops else '') + f'line amounts (wire sign) {total}')
open(f'{E}/{label}.parsed.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(res) + '\n')
print('\n'.join(res))
