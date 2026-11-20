# Lab helper (row 2440-CP, from the #1713 Row S call.py; not product code): one tool call through the running client:
# writes inbox\<seq>-<label>.json, waits for the answer (no timeout; a printed line at 170 s), logs the round trip,
# prints the structured result (cut at 4000 characters; the saved answer is whole).
# Usage: python -I -X utf8 call.py <label> <tool> <args json or file>
import json, os, sys, time
from datetime import datetime, timezone
R = '<lab folder>/sitting-2/run'; label, tool, a = sys.argv[1:4]
args = json.load(open(a, encoding='utf-8')) if os.path.exists(a) else json.loads(a)
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
r = j['response'].get('result') or {}; sc = r.get('structuredContent')
print(f'{fn}: {tool} sent {t0} secs {j.get("round_trip_seconds")} bytes {os.path.getsize(out)} isError {r.get("isError")} rpc {bool(j["response"].get("error"))}', flush=True)
with open(R + '/tools-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(f'{t0} {fn} {tool} secs {j.get("round_trip_seconds")} bytes {os.path.getsize(out)} isError {r.get("isError")}\n')
if tool == 'list': print(len(r.get('tools', [])), [t['name'] for t in r.get('tools', [])]); sys.exit(0)
body = sc if sc is not None else (r.get('content') or j['response'].get('error'))
print(json.dumps(body, ensure_ascii=False)[:4000])
