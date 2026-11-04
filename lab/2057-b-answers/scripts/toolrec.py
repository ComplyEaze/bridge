# Lab helper (2057-B, not product code): the step 14-16 records of the tool answers, by structure: per vouchers answer,
# state, total and each voucher's entries (ledger, amount, goods lines); the purchase_register answer's state, totals, items
# (status, tax_in_books, taxable_entries, has_taxable_entry, party_entries, other_entries) and the length of every side
# list, with any voucher in a side list named; ledger_masters' compliance rows for the five ledgers; masters' Purchase row.
# Usage: python -I -X utf8 toolrec.py <answer file> [...]
import json, sys
def body(p):
    j = json.load(open(p, encoding='utf-8')); r = j['response']['result']
    return r.get('isError'), r['structuredContent']['result']
def v(x):
    return x.get('value') if isinstance(x, dict) and 'state' in x else x
for p in sys.argv[1:]:
    err, b = body(p); n = p.replace(chr(92), '/').split('/')[-1]
    snap = b.get('snapshot') or {}
    print(f'== {n}: isError {err}; state {b.get("state")}; total {b.get("total")}; snapshot {snap.get("master_alter_id")}/{snap.get("voucher_alter_id")} reused {snap.get("reused")}')
    if 'vouchers' in n:
        for it in b['items']:
            print(f'  {it["date"]} {it["voucher_type"]} {it["voucher_number"]} ref {it["reference"]} party {it["party"]} gstin {it["party_gstin"]} is_invoice {it["is_invoice"]} cancelled {it["cancelled"]}')
            for a in it['amounts']:
                bills = [(x['bill_type'], (x.get('reference') or {}).get('name'), x['amount']) for x in a.get('bill_allocations', [])]
                print(f'    {a["ledger"]} {a["amount"]} deemed_positive {a["is_deemed_positive"]} bills {bills}')
                for g in a.get('goods_lines', []):
                    bt = [(v(x['godown']), v(x['batch']), v(x['amount'])) for x in g.get('batches', [])]
                    print(f'      goods {g["stock_item"]} {v(g["actual_quantity"])} @ {v(g["rate"])} = {v(g["amount"])} batches {bt}')
    elif 'purchase-register' in n:
        for k in sorted(b):
            if isinstance(b[k], list) and k != 'items':
                print(f'  {k}: {len(b[k])}' + ''.join(f'\n    {json.dumps(x, ensure_ascii=False)[:400]}' for x in b[k]))
            elif not isinstance(b[k], (list, dict)):
                print(f'  {k}: {b[k]}')
            elif k not in ('items', 'snapshot', 'window'):
                print(f'  {k}: {json.dumps(b[k], ensure_ascii=False)[:400]}')
        for it in b['items']:
            print(f'  ITEM {json.dumps({k: it[k] for k in it if k not in ("evidence",)}, ensure_ascii=False)}')
    elif 'ledgers' in n:
        for it in b['items']:
            print(f'  {it.get("name")}: {json.dumps(it.get("compliance"), ensure_ascii=False)}')
    elif 'vchtypes' in n:
        for it in b.get('masters') or []:
            if it.get('name') == 'Purchase': print(f'  Purchase: {json.dumps(it, ensure_ascii=False)}')
