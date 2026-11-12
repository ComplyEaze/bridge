# Lab helper (row 1761-S4 v3, new; not product code): writes the step 4 and step 5 call arguments to args/ by json.dump,
# from the sheet's step text and the batch_id C that step 3.2's answer (answers/014-s3-build.json) holds. Stops if a file
# exists. Usage: python -I -X utf8 mk-args4.py
import json, os
R = '<LAB_DIR>/'; G = 'b52de336-8552-4d40-9221-60aa64d3c1f1'
a = json.load(open(R + 'answers/014-s3-build.json', encoding='utf-8'))['response']['result']
C = a['structuredContent']['result']['batch_id']
files = {
    'post.json': {'company_guid': G, 'batch_id': C},
    'verify.json': {'company_guid': G, 'batch_id': C},
    'cash-move.json': {'company_guid': G, 'ledger': 'Cash', 'from': '2025-04-01', 'to': '2025-04-30'},
    'bank-move.json': {'company_guid': G, 'ledger': 'BRIDGE Bank A', 'from': '2025-04-01', 'to': '2025-04-30'},
}
for n in files:
    if os.path.exists(R + 'args/' + n): raise SystemExit(n + ' exists')
for n, d in files.items():
    with open(R + 'args/' + n, 'w', encoding='utf-8', newline='\n') as f: json.dump(d, f, indent=1); f.write('\n')
    print('written', n, C)
