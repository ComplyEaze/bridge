# Lab helper (#2018 L1-L3, not product code): the sheet's tables, by parser. Stock Query answers (UTF-16LE, C4: only the
# characters XML forbids dropped): STQPURCDATE / STQSALESDATE groups and the STQGODOWNNAME, STQBATCHNAME, STQGODOWNQTY triples
# in document order, and whether each answer is byte-identical to the 28 October answer of the same request. Collections: each
# STOCKITEM row (children of ENVELOPE/BODY/DATA/COLLECTION) with CLOSINGBALANCE as sent (absent, empty or its text). The
# vouchers tool answer: each voucher and each goods line with its batches. Then L1, L2 and L3. Writes tables.md (item names
# appear only here, the raw answers and the tool answers) and prints the counts. Usage: python -I parse2018.py
import json, os, re, subprocess, xml.etree.ElementTree as ET
from decimal import Decimal
W = '<lab folder>'; A = W + '/answers/raw'
OLD = {'q-r-09-stock-query-soda-ash-light': 'a5b369bf:lab/2018-stock-query-items-periods-answers/raw/r-09-stock-query-soda-ash-light',
       'q-r-03-stock-query-cleaning-kit-a': 'a5b369bf:lab/2018-stock-query-items-periods-answers/raw/r-03-stock-query-cleaning-kit-a',
       'q-r-02-stock-query-caustic-soda-flakes': 'a5b369bf:lab/2018-stock-query-items-periods-answers/raw/r-02-stock-query-caustic-soda-flakes',
       'q-r-4-stock-query-stockitemname': '263cc72c:lab/2018-stock-query-with-item-answers/raw/r-4-stock-query-stockitemname',
       'q-r-12-stock-query-hcl-to-20250401': 'a5b369bf:lab/2018-stock-query-items-periods-answers/raw/r-12-stock-query-hcl-to-20250401',
       'q-r-14-stock-query-cleaning-kit-a-to-20250401': 'a5b369bf:lab/2018-stock-query-items-periods-answers/raw/r-14-stock-query-cleaning-kit-a-to-20250401',
       'q-r-13-stock-query-zero-stock-item-to-20250401': 'a5b369bf:lab/2018-stock-query-items-periods-answers/raw/r-13-stock-query-zero-stock-item-to-20250401',
       'q-r-04-stock-query-cleaning-kit-b': 'a5b369bf:lab/2018-stock-query-items-periods-answers/raw/r-04-stock-query-cleaning-kit-b'}
ITEM = {'q-r-09-stock-query-soda-ash-light': ('Soda Ash Light', '20260331'), 'q-r-03-stock-query-cleaning-kit-a': ('Cleaning Kit A', '20260331'),
        'q-r-02-stock-query-caustic-soda-flakes': ('Caustic Soda Flakes', '20260331'), 'q-r-4-stock-query-stockitemname': ('Hydrochloric Acid 33pc', '20260331'),
        'q-r-12-stock-query-hcl-to-20250401': ('Hydrochloric Acid 33pc', '20250401'), 'q-r-14-stock-query-cleaning-kit-a-to-20250401': ('Cleaning Kit A', '20250401'),
        'q-r-13-stock-query-zero-stock-item-to-20250401': ('Zero Stock Item', '20250401'), 'q-r-04-stock-query-cleaning-kit-b': ('Cleaning Kit B', '20260331')}
def c4(b):
    t = b.decode('utf-16-le').lstrip('\ufeff')
    t = re.sub(r'&#(x[0-9A-Fa-f]+|[0-9]+);', lambda m: '' if (int(m.group(1)[1:], 16) if m.group(1)[0] == 'x' else int(m.group(1))) in set(range(0, 9)) | {11, 12} | set(range(14, 32)) else m.group(0), t)
    return re.sub('[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t).replace('<ENVELOPE', '<ENVELOPE xmlns:UDF="urn:x-tally-udf"', 1)
out = ['# Parser tables (#2018 L1-L3). Item names appear here, in the raw answers and in the tool answers only.\n']
sq = {}
out.append('## Stock Query answers\n| request | item, to | bytes | byte-identical to 28 Oct | purchase groups (dates) | sales groups (dates) | godown, batch, qty triples |\n|---|---|---|---|---|---|---|')
for stem, (item, to) in ITEM.items():
    b = open(f'{A}/{stem}.answer.xml', 'rb').read()
    old = subprocess.run(['git', '-C', 'D:/dev/bridge', 'show', OLD[stem]], capture_output=True).stdout
    root = ET.fromstring(c4(b)) if b.strip() else None
    els = [] if root is None else [(e.tag, (e.text or '')) for e in root.iter() if e.tag != 'ENVELOPE']
    purc = [t for g, t in els if g == 'STQPURCDATE']; sales = [t for g, t in els if g == 'STQSALESDATE']
    trip, cur = [], {}
    for g, t in els:
        if g in ('STQGODOWNNAME', 'STQBATCHNAME', 'STQGODOWNQTY'):
            cur[g] = t
            if g == 'STQGODOWNQTY': trip.append((cur.get('STQGODOWNNAME'), cur.get('STQBATCHNAME'), t)); cur = {}
    sq[stem] = {'item': item, 'to': to, 'purc': purc, 'sales': sales, 'trip': trip, 'empty': not els}
    out.append(f"| {stem} | {item}, {to} | {len(b)} | {'yes' if b == old else ('no' if old else 'no 28 Oct file')} | {len(purc)} ({', '.join(purc)}) | {len(sales)} ({', '.join(sales)}) | {'; '.join(f'{x!r}, {y!r}, {z!r}' for x, y, z in trip) or ('empty answer' if not els else 'none')} |")
coll = {}
for stem, to in (('c-a-01-items-to-20250401', '20250401'), ('c-a-03-items-to-20260331', '20260331')):
    root = ET.fromstring(c4(open(f'{A}/{stem}.answer.xml', 'rb').read())); col = root.find('./BODY/DATA/COLLECTION')
    for r in (col if col is not None else []):
        if r.tag != 'STOCKITEM': continue
        cb = r.find('CLOSINGBALANCE'); coll[(r.get('NAME'), to)] = 'absent' if cb is None else ('empty' if not (cb.text or '').strip() else cb.text.strip())
out.append('\n## The stock-item collection: CLOSINGBALANCE as sent\n| item | to | CLOSINGBALANCE |\n|---|---|---|')
for (n, to), v in sorted(coll.items()):
    if n in {i for i, _ in ITEM.values()}: out.append(f'| {n} | {to} | {v} |')
w = json.load(open(W + '/answers-mcp/002-vouchers-p1.json', encoding='utf-8')); r = json.loads(w['response']['result']['content'][0]['text'])['result']
vs = r['items']
out.append(f"\n## The year's vouchers (one page; total {r['total']}, returned {len(vs)}, state {r['state']}, reason {r.get('reason')}, withheld {r.get('withheld_vouchers')})\n| date | type | number | cancelled | optional | goods lines (stock_item, actual_quantity, batches as godown/batch/qty) |\n|---|---|---|---|---|---|")
lines = []
def val(x): return x.get('value') if isinstance(x, dict) else x
for v in vs:
    gl = []
    for a in v.get('amounts', []):
        for g in a.get('goods_lines', []) or []:
            bs = g.get('batches')
            gl.append((val(g.get('stock_item')), val(g.get('actual_quantity')), [(val(x.get('godown')), val(x.get('batch')), val(x.get('actual_quantity'))) for x in bs] if bs else None))
            if 'goods_lines_unreadable' in g: gl.append(('UNREADABLE', g['goods_lines_unreadable'], None))
        if 'goods_lines_unreadable' in a: gl.append(('UNREADABLE', a['goods_lines_unreadable'], None))
    lines.append((v, gl))
    if gl or v.get('is_invoice'):
        out.append(f"| {v.get('date')} | {v.get('voucher_type')} | {v.get('voucher_number')} | {v.get('cancelled')} | {v.get('optional')} | " + '; '.join(f'{s!r} {q!r} ' + ('no batches' if b is None else ', '.join(f'{x!r}/{y!r}/{z!r}' for x, y, z in b)) for s, q, b in gl) + ' |')
def qty(s): m = re.match(r'\s*(-?[\d.]+)', s or ''); return Decimal(m.group(1)) if m else Decimal(0)
def live(v): return str(v.get('cancelled')).lower() not in ('true', 'yes') and str(v.get('optional')).lower() not in ('true', 'yes')
# L1
sal = [(v.get('date'), q) for v, gl in lines if live(v) and 'purchase' in str(v.get('voucher_type')).lower() for s, q, b in gl if s == 'Soda Ash Light']
out.append(f"\n## L1\nPurchase goods lines for Soda Ash Light (not cancelled, not optional): {len(sal)} ({', '.join(d for d, _ in sal)}); q-r-09 purchase groups: {len(sq['q-r-09-stock-query-soda-ash-light']['purc'])} ({', '.join(sq['q-r-09-stock-query-soda-ash-light']['purc'])})")
# L2
out.append('\n## L2\n| item | (godown, batch) pairs in the goods lines, signed sum | opening lines (1 Apr 2025) | year-end lines (31 Mar 2026) | opening + sum = year end, pair by pair | year-end lines in no voucher and no opening |\n|---|---|---|---|---|---|')
for item, ye, op in (('Soda Ash Light', 'q-r-09-stock-query-soda-ash-light', None), ('Cleaning Kit A', 'q-r-03-stock-query-cleaning-kit-a', 'q-r-14-stock-query-cleaning-kit-a-to-20250401'),
                     ('Caustic Soda Flakes', 'q-r-02-stock-query-caustic-soda-flakes', None), ('Hydrochloric Acid 33pc', 'q-r-4-stock-query-stockitemname', 'q-r-12-stock-query-hcl-to-20250401')):
    pairs = {}
    for v, gl in lines:
        if not live(v): continue
        t = str(v.get('voucher_type')).lower(); sign = 1 if 'purchase' in t else (-1 if 'sales' in t else 0)
        for s, q, b in gl:
            if s != item or b is None: continue
            for g, bt, bq in b: pairs[(g, bt)] = pairs.get((g, bt), Decimal(0)) + sign * qty(bq)
    yl = {(g, b): qty(q) for g, b, q in sq[ye]['trip']}; ol = {(g, b): qty(q) for g, b, q in sq[op]['trip']} if op else {}
    tie = '-' if not op else '; '.join(f'{k}: {ol.get(k, 0)} + {pairs.get(k, 0)} = {ol.get(k, 0) + pairs.get(k, 0)} vs {yl.get(k, 0)} {"equal" if ol.get(k, 0) + pairs.get(k, 0) == yl.get(k, 0) else "NOT equal"}' for k in sorted(set(pairs) | set(ol) | set(yl), key=str))
    orphan = [k for k in yl if k not in pairs and k not in ol]
    out.append(f"| {item} | {len(pairs)}: {'; '.join(f'{k}: {s}' for k, s in sorted(pairs.items(), key=str))} | {'; '.join(f'{k}: {s}' for k, s in ol.items()) or ('not read' if not op else 'none')} | {'; '.join(f'{k}: {s}' for k, s in yl.items()) or 'none'} | {tie} | {orphan or 'none'} |")
# L3
out.append('\n## L3\n| item | date | Stock Query | CLOSINGBALANCE | reading |\n|---|---|---|---|---|')
for stem, (item, to) in ITEM.items():
    d = sq[stem]; cb = coll.get((item, to), 'item not in the collection')
    sqtxt = 'empty' if d['empty'] or not d['trip'] else '; '.join(f'{g}/{b}/{q}' for g, b, q in d['trip'])
    if sqtxt == 'empty': reading = 'zero-stock case' if re.match(r'^0(\.0+)?\b', cb) else ('no stock movement (closing ' + cb + ')' if cb in ('empty', 'absent') else 'DISAGREEMENT')
    else:
        tot = sum(qty(q) for _, _, q in d['trip']); reading = ('DISAGREEMENT (closing zero)' if re.match(r'^0(\.0+)?\b', cb) else f'lines sum {tot} vs closing {qty(cb)}: {"equal" if tot == qty(cb) else "not equal"}')
    out.append(f'| {item} | {to} | {sqtxt} | {cb} | {reading} |')
open(W + '/tables.md', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
print('\n'.join(out))
