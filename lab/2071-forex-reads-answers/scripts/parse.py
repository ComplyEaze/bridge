# Lab helper (#2071 step 10, not product code; rows are children of ENVELOPE/BODY/DATA/COLLECTION, never the CMPINFO count lines): the raw answers of one round read with an XML parser (characters XML forbids
# dropped for parsing only; the saved bytes are unchanged). Bills reports: every row in the order sent, every child element
# with its value as sent (empty marked); which report holds bill 1, bill 2, R1, P1 and P2. tb-currency, both ledger
# snapshots and the group snapshot: the named fields per row, as sent. Usage: python -I -X utf8 parse.py <prefix: a|b1|b2>
import re, sys, xml.etree.ElementTree as ET
E = '<LAB_DIR>/evidence'; pre = sys.argv[1]
def load(name):
    b = open(f'{E}/{pre}-{name}.response', 'rb').read(); t = (b[2:] if b[:2] == b'\xff\xfe' else b).decode('utf-16-le')
    t = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t)
    t = re.sub(r'&#(x[0-9a-fA-F]+|[0-9]+);', lambda m: '' if (int(m.group(1)[1:], 16) if m.group(1)[0] == 'x' else int(m.group(1))) in set(range(0, 9)) | {11, 12} | set(range(14, 32)) else m.group(0), t)
    return ET.fromstring(t)
def v(e):
    s = e.text if e.text is not None else ''
    return '(empty)' if s.strip() == '' and len(e) == 0 else repr(s.strip() if len(e) == 0 else '')
out = [f'Round {pre}: raw answers parsed with an XML parser.']
where = {}
for rep in ('bills-receivable', 'bills-payable'):
    root = load(rep)
    body = root.find('BODY') if root.find('BODY') is not None else root
    # rows: the BILLFIXED groups in the order sent; each row is a BILLFIXED element with the sibling elements that follow it
    kids = list(root) if root.find('BILLFIXED') is not None else [x for x in root.iter() if x is not root]
    rows, cur = [], None
    for x in list(root):
        if x.tag == 'BILLFIXED': cur = [x]; rows.append(cur)
        elif cur is not None: cur.append(x)
    out.append(f'{rep}: {len(rows)} bill row(s)')
    for i, r in enumerate(rows, 1):
        parts = []
        for x in r:
            if len(x): parts.append(x.tag + '{' + ', '.join(f'{c.tag} {v(c)}' for c in x) + '}')
            else: parts.append(f'{x.tag} {v(x)}')
        out.append(f'  row {i}: ' + '; '.join(parts))
        ref = r[0].findtext('BILLREF') or ''
        where.setdefault(ref.strip(), []).append(f'{rep} row {i}')
for b in ('1', '2', 'R1', 'P1', 'P2'): out.append(f'bill {b}: ' + (', '.join(where[b]) if b in where else 'in neither report'))
def rows(name, tag):
    root = load(name); c = root.find('./BODY/DATA/COLLECTION')
    return [x for x in (c if c is not None else root.iter(tag)) if x.tag == tag]
for x in rows('tb-currency', 'LEDGER') if False else []: pass
root = load('tb-currency')
tb = [x for x in root.iter() if x.tag in ('LEDGER', 'DSPACCNAME')]
out.append('tb-currency: per row (as sent)')
# the Trial Balance answer is a flat list: each ledger's fields follow its name element
flat = list(root.iter())
names = [x for x in flat if x.tag in ('DSPDISPNAME', 'NAME') or x.tag == 'LEDGER']
led = root.findall('./BODY/DATA/COLLECTION/LEDGER')
if led:
    for x in led:
        f = {k: (x.findtext(k) if x.find(k) is not None else 'absent') for k in ('PARENT', 'TBALOPENING', 'DEBITTOTALS', 'CREDITTOTALS', 'TBALCLOSING', 'CURRENCYNAME')}
        out.append(f'  NAME {x.get("NAME") or x.findtext("NAME")!r}; ' + '; '.join(f'{k} {("(empty)" if (w or "").strip() == "" else repr(w.strip())) if w != "absent" else "absent"}' for k, w in f.items()))
for name in ('ledger-snapshot', 'ledger-snapshot-compliance'):
    out.append(f'{name}: per ledger (as sent)')
    for x in load(name).findall('./BODY/DATA/COLLECTION/LEDGER'):
        f = {k: (x.findtext(k) if x.find(k) is not None else 'absent') for k in ('PARENT', 'CLOSINGBALANCE', 'OPENINGBALANCE', 'ISBILLWISEON', 'CURRENCYNAME')}
        out.append(f'  NAME {x.get("NAME") or x.findtext("NAME")!r}; ' + '; '.join(f'{k} {("(empty)" if (w or "").strip() == "" else repr(w.strip())) if w != "absent" else "absent"}' for k, w in f.items()))
g = load('group-snapshot').findall('./BODY/DATA/COLLECTION/GROUP')
out.append(f'group-snapshot: {len(g)} group row(s)')
for x in g:
    out.append(f'  NAME {x.get("NAME") or x.findtext("NAME")!r}; PARENT {(x.findtext("PARENT") or "")!r}; RESERVEDNAME {x.get("RESERVEDNAME") if x.get("RESERVEDNAME") is not None else (x.findtext("RESERVEDNAME") if x.find("RESERVEDNAME") is not None else "absent")!r}')
open(f'{E}/parsed-{pre}.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n'); print('\n'.join(out))
