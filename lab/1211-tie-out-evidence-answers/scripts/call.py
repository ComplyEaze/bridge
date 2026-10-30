# Lab helper (#1211, not product code): ONE tool call through the running client.py session: writes inbox\<seq>-<label>.json,
# waits for answers\<seq>-<label>.json (no timeout of its own; a stop line after 10 minutes, the call left running), logs
# the round trip and the answer's main fields to evidence/tools-log.txt. Usage: python -I -X utf8 call.py <label> <tool> <json args>
import json, os, sys, time, hashlib
from datetime import datetime, timezone
W = '<LAB_DIR>'; SD = W + '/srv'
label, tool, args = sys.argv[1], sys.argv[2], json.loads(sys.argv[3])
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
def log(s):
    with open(W + '/evidence/tools-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
seq = len([f for f in os.listdir(SD + '/inbox') if f.endswith('.json')]); fn = f'{seq:03d}-{label}.json'
t0 = now(); m0 = time.monotonic()
with open(f'{SD}/inbox/{fn}', 'w', encoding='utf-8', newline='\n') as f: json.dump({'tool': tool, 'arguments': args}, f)
out = f'{SD}/answers/{fn}'; warned = False
while not os.path.exists(out):
    time.sleep(0.2)
    if not warned and time.monotonic() - m0 > 600: log(f'stop: {fn} has no answer in 10 minutes; left running'); warned = True
time.sleep(0.3)
while True:
    try: j = json.load(open(out, encoding='utf-8')); break
    except ValueError: time.sleep(0.2)
d = open(out, 'rb').read()
r = j['response'].get('result') or {}; sc = r.get('structuredContent')
log(f'{t0} {fn}: {tool} {json.dumps(args)}; answered {now()}; round trip {j.get("round_trip_seconds")} s; isError {r.get("isError")}; '
    f'rpc error {j["response"].get("error")}; saved {len(d)} B sha256 {hashlib.sha256(d).hexdigest()}')
print(json.dumps(sc if sc is not None else r, ensure_ascii=False)[:6000])
