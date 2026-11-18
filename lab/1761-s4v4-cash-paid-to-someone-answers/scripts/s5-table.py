# Lab helper (row 1761-S4 v4, new; not product code): writes evidence/s5-voucher-table.txt from step 5.2's vouchers answer
# (one line per voucher: date, type, number, master id, each ledger entry, narration with its length) and checks each line
# against the sheet's table; prints the movement figures of steps 5.3 and 5.4. Reads only; stops if the table exists.
# Usage: python -I -X utf8 s5-table.py
import json, os
R = '<LAB_DIR>/'
OUT = R + 'evidence/s5-voucher-table.txt'
if os.path.exists(OUT): raise SystemExit('table exists')
def res(p): return json.load(open(R + 'answers/' + p, encoding='utf-8'))['response']['result']['structuredContent']['result']
v = res('021-s5-2-vouchers.json')
EXP = [
 ('20250408', 'Contra', {'Cash': 'Dr', 'BRIDGE Bank A': 'Cr'}, '12000.00', 'to Cash | Lab CSV 4321 | 08-Apr-2025'),
 ('20250408', 'Payment', {'BRIDGE Contractor A': 'Dr', 'Cash': 'Cr'}, '12000.00', 'Cash paid to BRIDGE Contractor A from Cash | withdrawn | Lab CSV 4321 | 08-Apr-2025 | Bridge: cash to one payee in a day over 10000; check s.40A(3)'),
 ('20250409', 'Contra', {'Cash': 'Dr', 'BRIDGE Bank A': 'Cr'}, '3000.00', 'to Cash | Lab CSV 4321 | 09-Apr-2025'),
 ('20250409', 'Payment', {'BRIDGE Wages': 'Dr', 'Cash': 'Cr'}, '3000.00', 'Cash paid to BRIDGE Wages from Cash | withdrawn | Lab CSV 4321 | 09-Apr-2025'),
]
lines = [f"state {v.get('state')} reason {v.get('reason')} total {v.get('total')} snapshot V {v['snapshot']['voucher_alter_id']}"]
items = sorted(v['items'], key=lambda i: int(i['master_id']))
found = []
for i in items:
    ent = []
    sides = {}
    for a in i['amounts']:
        side = 'Dr' if a['is_deemed_positive'] == 'Yes' else 'Cr'
        sides[a['ledger']] = (side, a['amount'].lstrip('-'))
        ent.append(f"{a['ledger']} {a['amount']} {side}")
    n = i['narration']
    lines.append(f"{i['date']} | {i['voucher_type']} | no. {i['voucher_number']} | master_id {i['master_id']} | guid {i['guid']} | "
                 + '; '.join(ent) + f" | narration {json.dumps(n, ensure_ascii=False)} ({len(n)} characters)")
    found.append((i['date'], i['voucher_type'], {k: s[0] for k, s in sides.items()}, sorted({s[1] for s in sides.values()}), n))
ok = len(found) == 4
for e in EXP:
    m = [f for f in found if f[0] == e[0] and f[1] == e[1] and f[2] == e[2] and f[3] == [e[3]]]
    nar = [f for f in m if f[4] == e[4]]
    lines.append(f"EXPECT {e[0]} {e[1]} {e[3]}: voucher {'found' if m else 'MISSING'}; narration {'exact' if nar else 'DIFFERS'}")
    ok = ok and bool(m) and bool(nar)
lines.append('table check: ' + ('PASS' if ok else 'FAIL'))
for f in ('022-s5-3-cash.json', '023-s5-4-bank.json'):
    r = res(f)
    keep = {k: r.get(k) for k in r if k not in ('limitations', 'next_step', 'warnings', 'items', 'rows', 'days')}
    lines.append(f + ' ' + json.dumps(keep, ensure_ascii=False)[:900])
open(OUT, 'w', encoding='utf-8', newline='\n').write('\n'.join(lines) + '\n')
print('\n'.join(lines))
