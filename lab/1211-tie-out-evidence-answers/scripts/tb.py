# Lab helper (#1211, not product code): one trial_balance read (every page) through call.py, from 2026-04-01 to the date
# given; prints each named ledger's row whole and its closing.value, and appends a summary to evidence\tb-log.txt.
# Usage: python -I -X utf8 tb.py <label> <to yyyy-mm-dd> <ledger> [<ledger> ...]
import json, os, subprocess, sys
W = '<LAB_DIR>'; SD = W + '/srv'
G = '9c97970c-e25d-4348-9740-6ddb1caaa56d'
label, to, names = sys.argv[1], sys.argv[2], sys.argv[3:]
def log(s):
    with open(W + '/evidence/tb-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
rows, offset, page = [], None, 0
while True:
    args = {'company_guid': G, 'from': '2026-04-01', 'to': to}
    if offset is not None: args['offset'] = offset
    name = f'{label}' + (f'-p{page}' if page else '')
    subprocess.run(['python', '-I', '-X', 'utf8', W + '/call.py', name, 'trial_balance', json.dumps(args)], capture_output=True, text=True)
    fn = [f for f in sorted(os.listdir(SD + '/answers')) if f.endswith(f'-{name}.json')][-1]
    j = json.load(open(f'{SD}/answers/{fn}', encoding='utf-8'))
    r = j['response'].get('result') or {}; sc = r.get('structuredContent') or {}; b = sc.get('result') if isinstance(sc.get('result'), dict) else {}
    if r.get('isError') or not b:
        log(f'{label} to {to}: {fn} isError {r.get("isError")}: {json.dumps(r)[:2500]}'); sys.exit(3)
    found = b.get('ledgers') or b.get('rows') or []
    rows += found
    log(f'{label} to {to}: {fn} page {page}: {len(found)} rows; keys {[k for k in b if k not in ("ledgers", "rows")]}; state {b.get("state")}; evidence.state {(sc.get("evidence") or {}).get("state")}')
    offset = b.get('next_offset')
    if not offset: break
    page += 1
for n in names:
    hit = [x for x in rows if (x.get('ledger') or x.get('name')) == n]
    if not hit: log(f'  {n}: no row'); continue
    for x in hit: log(f'  {n}: closing.value {(x.get("closing") or {}).get("value") if isinstance(x.get("closing"), dict) else x.get("closing")}; row {json.dumps(x)}')
