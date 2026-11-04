# Lab helper (2057-C, not product code): sets this row's step 1 answers against the 31 Oct row's answers at a8bcd9f8
# (read by git show into a scratch folder given as argument). ledger_masters compliance: each ledger row (by name) equal,
# field by field; masters voucher_types: the Purchase row equal. Snapshot ids and read times are left out; any field that
# differs is printed by its path, with both values only when they are not names. Usage: python -I -X utf8 cmp1.py <old dir>
import json, sys
O = sys.argv[1].rstrip('/') + '/'; N = '<LAB_DIR>/srv/answers/'
def body(p):
    j = json.load(open(p, encoding='utf-8')); return j['response']['result']['structuredContent']['result']
def diff(a, b, path=''):
    out = []
    if isinstance(a, dict) and isinstance(b, dict):
        for k in sorted(set(a) | set(b)):
            if k in ('snapshot_id', 'read_at', 'snapshot', 'evidence'): continue
            out += diff(a.get(k, '<absent>'), b.get(k, '<absent>'), f'{path}.{k}')
    elif isinstance(a, list) and isinstance(b, list):
        if len(a) != len(b): out.append(f'{path}: list length {len(a)} vs {len(b)}')
        for i, (x, y) in enumerate(zip(a, b)): out += diff(x, y, f'{path}[{i}]')
    elif a != b: out.append(f'{path}: {json.dumps(a)} vs {json.dumps(b)}')
    return out
for new, old, key in (('003-s1-ledgers-gst.json', '003-s1-ledgers-gst.json', 'ledgers'), ('004-s1-vchtypes.json', '004-s1-vchtypes-call1.json', 'voucher_types')):
    a, b = body(N + new), body(O + old)
    rows_a = {r.get('name'): r for r in (a.get('items') or a.get(key) or a.get('rows') or [])}
    rows_b = {r.get('name'): r for r in (b.get('items') or b.get(key) or b.get('rows') or [])}
    print(new, 'rows now', len(rows_a), '31 Oct', len(rows_b), 'top keys', sorted(a)[:12])
    names = sorted(rows_a) if key == 'ledgers' else ['Purchase']
    for n in names:
        d = diff(rows_a.get(n), rows_b.get(n))
        print(f'  {n}: {"equal" if not d else str(len(d)) + " differences"}')
        for x in d[:20]: print('    ', x)
