# Lab helper (852-C2, from 1392-E, from #1713 Row S, not product code): one tool call through the running client: writes inbox\<seq>-<label>.json, waits for the answer with
# no timeout of its own but a printed line at 170 s, logs the round trip, prints the answer's main fields.
# Usage: python -I -X utf8 call.py <row dir> <label> <tool> <args file>
import json, os, sys, time
from datetime import datetime, timezone
R, label, tool, argf = sys.argv[1:5]
args = json.load(open(argf, encoding='utf-8'))
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
seq = len([f for f in os.listdir(R + '/inbox') if f.endswith('.json')]); fn = f'{seq:03d}-{label}.json'
tmp = f'{R}/{fn}.tmp'
with open(tmp, 'w', encoding='utf-8', newline='\n') as f: json.dump({'tool': tool, 'arguments': args}, f)
t0 = now(); m0 = time.monotonic(); os.replace(tmp, f'{R}/inbox/{fn}')
out = f'{R}/answers/{fn}'; warned = False
while not os.path.exists(out):
    time.sleep(0.1)
    if not warned and time.monotonic() - m0 > 170: print('no answer in 170 s; still waiting', flush=True); warned = True
time.sleep(0.3)
while True:
    try: j = json.load(open(out, encoding='utf-8')); break
    except ValueError: time.sleep(0.2)
r = j['response'].get('result') or {}; sc = r.get('structuredContent'); res = (sc or {}).get('result') if isinstance(sc, dict) else None
print(f'{fn}: {tool} sent {t0} secs {j.get("round_trip_seconds")} bytes {os.path.getsize(out)} isError {r.get("isError")} rpc {bool(j["response"].get("error"))}', flush=True)
with open(R + '/evidence/tools-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(f'{t0} {fn} {tool} secs {j.get("round_trip_seconds")} bytes {os.path.getsize(out)} isError {r.get("isError")}\n')
if tool == 'list':
    print(len(r.get('tools', [])), [t['name'] for t in r.get('tools', []) if 'ledger_create' in t['name'] or 'post' in t['name']]); sys.exit(0)
if isinstance(res, dict):
    print({k: v for k, v in res.items() if k in ('batch_id', 'state', 'verdict', 'status', 'attempt_recorded', 'dispatch', 'resent', 'ledger_count', 'retry_after_s', 'verification', 'reasons', 'reason', 'code', 'error', 'counters', 'journal', 'total', 'next_step')})
    if 'preview' in res:
        p = res['preview']; lines = p if isinstance(p, list) else str(p).split('\n'); print('preview lines', len(lines)); print('\n'.join(str(x) for x in lines))
    sn = res.get('snapshot')
    if sn: print('snapshot', {k: sn.get(k) for k in ('master_alter_id', 'voucher_alter_id', 'reused')})
    it = res.get('items')
    if it: print('rows', [(x.get('name'), x.get('parent')) for x in it])
    if 'companies' in res: print([{k: c.get(k) for k in ('name', 'guid', 'identity_state')} for c in res['companies']])
else:
    print(str(sc or r)[:700])
