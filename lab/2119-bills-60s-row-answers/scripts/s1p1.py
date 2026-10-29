# Lab helper (#2119 B row steps 2 and 3, not product code). Queues S1 (outstandings, as_of 20260331) in process A's inbox and,
# while it runs: a capture of Tally's whole window at about 8 s (P1's), process B started at about 8.5 s and its one tally_status
# queued at 10.0 s after S1 by this timer, B stopped by its stop file once it has answered; captures at 20 s, 40 s and every 30 s
# after until S1 ends; then S2 (ledger_masters basic) queued through A at once, nothing between. Prints times, counts and outcomes.
import json, os, subprocess, sys, time
from datetime import datetime, timezone
R = '<lab folder>'; B = R + '/b'
GUID = 'd45bc1b0-e5e3-4261-b3b2-cce3915f42d3'
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
def log(s):
    line = f'{now()} {s}'
    with open(R + '/evidence/s1p1-timeline.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(line + '\n')
    print(line, flush=True)
def queue(root, name, tool, args):
    seq = '{:03d}'.format(len([f for f in os.listdir(root + '/inbox') if f.endswith('.json')]))
    fn = f'{seq}-{name}.json'
    with open(root + '/inbox/' + fn, 'w', encoding='utf-8', newline='\n') as f: json.dump({'tool': tool, 'arguments': args}, f)
    return fn
hwnd = subprocess.run(['powershell', '-NoProfile', '-Command', "(Get-Process tally | Where-Object { $_.MainWindowTitle -like 'TallyPrime:9001*' } | Select-Object -First 1).MainWindowHandle"], capture_output=True, text=True).stdout.strip()
def shot(tag):
    r = subprocess.run(['powershell', '-NoProfile', '-File', R + '/capwin.ps1', '-Handle', hwnd, '-Out', f'{R}/shots/{tag}.png'], capture_output=True, text=True)
    log(f'capture {tag} exit {r.returncode}')
fa = queue(R, 's1-outstandings', 'outstandings', {'company_guid': GUID, 'as_of': '20260331'})
t0 = time.monotonic(); log(f'S1 queued {fa} (outstandings company_guid, as_of 20260331)')
def wait_until(sec):
    while time.monotonic() - t0 < sec: time.sleep(0.05)
wait_until(8); shot('p1-t8')
wait_until(8.5)
bp = subprocess.Popen([sys.executable, '-I', B + '/client.py'], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
log(f'B client started pid {bp.pid}')
wait_until(10); fb = queue(B, 'p1-tally-status', 'tally_status', {}); log(f'B tally_status queued {fb} at S1+{time.monotonic() - t0:.2f} s')
marks = [20, 40] + list(range(70, 700, 30))
b_done = False; a_done = False; mi = 0
while not a_done:
    t = time.monotonic() - t0
    if not b_done and os.path.exists(B + '/answers/' + fb):
        time.sleep(0.3); w = json.load(open(B + '/answers/' + fb, encoding='utf-8'))
        res = w['response'].get('result') or {}
        txt = next((c['text'] for c in res.get('content', []) if c.get('type') == 'text'), '')
        try: body = json.loads(txt)
        except ValueError: body = {}
        err = (body.get('result') or {}).get('error') if isinstance(body, dict) else None
        log(f'P1 B answered: round_trip {w["round_trip_seconds"]} s, sent {w["call_sent_utc"]}, isError {res.get("isError")}, error {json.dumps(err)}')
        open(B + '/inbox/stop', 'w').close(); b_done = True
        try: bp.wait(timeout=60); log(f'B client exited {bp.returncode}')
        except subprocess.TimeoutExpired: log('B client has not exited 60 s after its stop file: stop')
    if os.path.exists(R + '/answers/' + fa):
        time.sleep(0.2); a_done = True; break
    if mi < len(marks) and t >= marks[mi]: shot(f's1-t{marks[mi]}'); mi += 1
    if t > 600: log('S1 has no answer after 600 s: stop (send nothing)'); sys.exit(3)
    time.sleep(0.1)
fs2 = queue(R, 's2-marks', 'ledger_masters', {'company_guid': GUID, 'fields': 'basic'})
log(f'S1 answer file seen at S1+{time.monotonic() - t0:.2f} s; S2 queued {fs2} at once')
w = json.load(open(R + '/answers/' + fa, encoding='utf-8'))
res = w['response'].get('result') or {}; sc = res.get('structuredContent') or {}
r = sc.get('result') or {}
log(f'S1: round_trip {w["round_trip_seconds"]} s, sent {w["call_sent_utc"]}, isError {res.get("isError")}, truncated {sc.get("truncated")}, error {json.dumps(r.get("error"))}')
for k in ('open_bills_total', 'receivable', 'payable', 'as_of', 'state'):
    log(f'S1 result.{k} = {json.dumps(r.get(k))[:200]}')
log('S1 result keys: ' + ', '.join(sorted(r.keys()))[:600])
while not os.path.exists(R + '/answers/' + fs2): time.sleep(0.1)
time.sleep(0.3); w2 = json.load(open(R + '/answers/' + fs2, encoding='utf-8'))
res2 = w2['response'].get('result') or {}; r2 = (res2.get('structuredContent') or {}).get('result') or {}
log(f'S2: round_trip {w2["round_trip_seconds"]} s, isError {res2.get("isError")}, error {json.dumps(r2.get("error"))}, snapshot {json.dumps({k: (r2.get("snapshot") or {}).get(k) for k in ("master_alter_id", "voucher_alter_id", "reused")})}')
if not b_done: log('B never answered while S1 ran')
