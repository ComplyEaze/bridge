# Lab helper (row 1761-S4 v2, new; not product code): writes the row's fixed call arguments to args/ by json.dump, from the
# sheet's step text, with THE COMPANY's GUID from row 1761-S4's RESULTS.md (checked against list_companies at step 0.3).
# The build arguments (proposals id and sha256) are written later from the parse answers. Stops if a file exists.
# Usage: python -I -X utf8 mk-args.py
import json, os
R = '<LAB_DIR>/args/'; G = 'b52de336-8552-4d40-9221-60aa64d3c1f1'
parse = {'statement_path': '<LAB_DIR>/statement.csv', 'controls_source': 'prior_statement',
         'opening_balance': '50,000.00', 'closing_balance': '35,000.00', 'account_label': 'Lab CSV 4321',
         'bank_ledger': 'BRIDGE Bank A', 'suspense_ledger': 'BRIDGE Suspense'}
a1, a2 = 'st-20250408-be621b1463b52f1b', 'st-20250409-ef7d14f34a7b0412'
def ans(cash1): return [{'bridge_txn_id': a1, 'answer': 'paid_to_someone', 'ledger': 'BRIDGE Contractor A', 'cash_ledger': cash1},
                        {'bridge_txn_id': a2, 'answer': 'paid_to_someone', 'ledger': 'BRIDGE Wages', 'cash_ledger': 'Cash'}]
files = {
    'marks.json': {'company_guid': G},
    'vt.json': {'company_guid': G, 'kind': 'voucher_types'},
    'seed-day.json': {'company_guid': G, 'from': '2025-04-01', 'to': '2025-04-01'},
    'stmt-days.json': {'company_guid': G, 'from': '2025-04-08', 'to': '2025-04-09'},
    's1-parse.json': parse,
    's2-parse.json': dict(parse, cash_answers=ans('BRIDGE Bank B')),
    's3-parse.json': dict(parse, cash_answers=ans('Cash')),
}
for n, d in files.items():
    if os.path.exists(R + n): raise SystemExit(n + ' exists')
for n, d in files.items():
    with open(R + n, 'w', encoding='utf-8', newline='\n') as f: json.dump(d, f, indent=1); f.write('\n')
    print('written', n)
