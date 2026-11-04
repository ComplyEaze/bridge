# Lab helper (699-C16 Phase B step 6, not product code): checks the vouchers page against the vouchers on record:
# the 34 of A5-VOUCHERS (26 Oct, copied in 699-R1's evidence) and the 19 of this row's A6 table. Compares type, number
# and date as sets, the GUID prefix of every row, and, for C1 to C19, each voucher's ledgers and amounts as keyed.
# Usage: python -I -X utf8 check-c6.py
import collections, datetime, json, re
R = '<LAB_DIR>'
GUID = '00f0c618-b0df-43bd-bd69-b4c7b92c616f'
def d8(s): return datetime.datetime.strptime(s.strip(), '%d-%b-%Y').strftime('%Y%m%d')
rec = []
for line in open('<lab folder>/699r1/evidence/A5-VOUCHERS-26oct.md', encoding='utf-8'):
    c = [x.strip() for x in line.split('|')]
    if len(c) > 5 and re.fullmatch(r'\d+', c[1]):
        t, n = c[3].rsplit(' ', 1); rec.append((t, n, d8(c[4]), 'A5 #' + c[1]))
a6 = []
for row in list(open('<lab folder>/699c16/evidence/a6-table.tsv', encoding='utf-8'))[1:]:
    c = row.rstrip('\n').split('\t')
    t = c[1].split(' ')[0]; rec.append((t, c[2], d8(c[3]), c[0])); a6.append(c)
j = json.load(open(R + '/run/answers/004-c6-vouchers-p0.json', encoding='utf-8'))
res = j['response']['result']['structuredContent']['result']; items = res['items']
read = [(v['voucher_type'], v['voucher_number'], v['date']) for v in items]
out = []
out.append(f'on record: {len(rec)} (A5 {len(rec) - len(a6)}, A6 {len(a6)}) | read rows {len(items)} | total {res["total"]} | state {res["state"]} | next_offset {res.get("next_offset")}')
out.append('read by type: ' + ', '.join(f'{k} {v}' for k, v in sorted(collections.Counter(r[0] for r in read).items())))
out.append('on record by type: ' + ', '.join(f'{k} {v}' for k, v in sorted(collections.Counter(r[0] for r in rec).items())))
out.append(f'every row with this GUID\'s prefix: {all(v["guid"].startswith(GUID + "-") for v in items)}')
rs, ks = collections.Counter(read), collections.Counter(r[:3] for r in rec)
out.append(f'on record, not read: {sorted((ks - rs).elements())}')
out.append(f'read, not on record: {sorted((rs - ks).elements())}')
out.append(f'type, number and date equal as multisets: {rs == ks}')
by = {(v['voucher_type'], v['voucher_number'], v['date']): v for v in items}
for c in a6:
    t = c[1].split(' ')[0]; v = by.get((t, c[2], d8(c[3])))
    if not v: out.append(f'{c[0]}: not read'); continue
    lines = sorted((a['ledger'], a['amount']) for a in v['amounts'])
    out.append(f'{c[0]} {t} {c[2]} {v["date"]} narration {v["narration"]!r} cancelled {v["cancelled"]} optional {v["optional"]} lines {lines}')
open(R + '/evidence/r5v-check.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
print('\n'.join(out))
