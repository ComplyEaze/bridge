# Lab helper (1381-O, not product code): for each probe, every leaf the request sent under its STOCKITEM, by path and in
# document order, is looked for in the item's stored row (the a1 read named). A leaf counts as kept when the stored row
# has the same path with the same text, or with a number equal in value (2.5 and 2.50). Each sent leaf is used once
# (multiset). A sent NAME is looked for in LANGUAGENAME.LIST/NAME.LIST/NAME, where Tally keeps it. The marker character &#4; is shown as <4>. Prints each probe's counts and every leaf not kept.
# Usage: python -I -X utf8 sent_vs_stored.py <a1 answer> <request> <item name> [<request> <item name> ...]
import re, sys, xml.etree.ElementTree as ET
def load(path):
    t = open(path, 'rb').read().decode('utf-16-le').lstrip('﻿')
    t = t.replace('&#4;', '&lt;4&gt;')
    t = re.sub(r'&#([0-8]|1[124-9]|2[0-9]|3[01]);', '', t); t = re.sub('[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t)
    return ET.fromstring(t)
def leaves(el, path, out):
    kids = list(el)
    if not kids:
        txt = (el.text or '').strip()
        if txt: out.append((path, txt))
        return out
    for k in kids: leaves(k, f'{path}/{k.tag}', out)
    return out
def same(a, b):
    if a == b: return True
    try: return float(a) == float(b)
    except ValueError: return False
a1 = load(sys.argv[1]); rows = {}
for it in a1.findall('./BODY/DATA/COLLECTION/STOCKITEM'):
    rows[it.get('NAME') or (it.findtext('NAME') or '')] = it
args = sys.argv[2:]
for i in range(0, len(args), 2):
    req, name = args[i], args[i + 1]
    sent_el = load(req).find('.//STOCKITEM')
    sent = [x for k in sent_el for x in leaves(k, k.tag, [])]
    if name not in rows: print(f'{req} {name!r}: no stored row in {sys.argv[1]}'); continue
    stored = [x for k in rows[name] for x in leaves(k, k.tag, [])]
    # Tally keeps an item's name in LANGUAGENAME.LIST/NAME.LIST/NAME, not as a direct NAME leaf: a sent NAME is looked for there.
    stored = [('NAME', w) if q == 'LANGUAGENAME.LIST/NAME.LIST/NAME' else (q, w) for q, w in stored]
    pool = list(stored); missing = []; changed = []
    for p, v in sent:
        hit = next((j for j, (q, w) in enumerate(pool) if q == p and same(v, w)), None)
        if hit is None: missing.append((p, v)); continue
        if pool[hit][1] != v: changed.append((p, v, pool[hit][1]))
        pool.pop(hit)
    print(f'{req} {name!r}: sent leaves={len(sent)}, kept={len(sent) - len(missing)}, not kept={len(missing)}, kept with another spelling={len(changed)}')
    for p, v in missing: print(f'  not kept: {p} = {v!r}')
    for p, v, w in changed: print(f'  kept as: {p} sent {v!r} stored {w!r}')
