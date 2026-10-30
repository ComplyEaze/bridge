# Lab helper (#2017, not product code): the sheet's marks read (rule 4): ledger_masters fields=compliance, first page,
# no snapshot_id, through call.py; prints snapshot.master_alter_id (M), voucher_alter_id (V) and reused, checks them
# against the expected pair, and appends a line to evidence\marks.txt. Usage: python -I -X utf8 tm.py <label> <M> <V>
import json, os, subprocess, sys
W = '<LAB_DIR>'; SD = W + '/srv'
label, em, ev = sys.argv[1], sys.argv[2], sys.argv[3]
G = '3a6bd6e1-b835-4bff-89dd-8a6af138c346'
subprocess.run(['python', '-I', '-X', 'utf8', W + '/call.py', f'marks-{label}', 'ledger_masters', json.dumps({'company_guid': G, 'fields': 'compliance'})], capture_output=True, text=True)
fn = [f for f in sorted(os.listdir(SD + '/answers')) if f.endswith(f'-marks-{label}.json')][-1]
j = json.load(open(f'{SD}/answers/{fn}', encoding='utf-8'))
res = j['response'].get('result') or {}; sc = res.get('structuredContent') or {}; body = sc.get('result') or {}
snap = body.get('snapshot') or {}
m, v, r = snap.get('master_alter_id'), snap.get('voucher_alter_id'), snap.get('reused')
ok = str(m) == em and str(v) == ev and r is False and not res.get('isError')
line = f'{j["call_sent_utc"]} marks {label} ({fn}): M {m} V {v} reused {r} isError {res.get("isError")}; expected {em}/{ev}: {"equal" if ok else "DIFFERENT"}'
with open(W + '/evidence/marks.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(line + '\n')
print(line)
if not ok: sys.exit(2)
