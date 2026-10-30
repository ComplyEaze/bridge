# Lab runner (#2173-L Phase B, not product code): one Phase B step, after its company is loaded alone, C2 step 0 and the
# capture. 1. Start the relay in its own console (cmd /k, the sheet's command line, new recording folder) and read its
# first line from that console. 2. Start the step's build through client.py with the sheet's environment (a new data
# folder). 3. list_companies, tally_status, then ONE audit_test_run (or, for B2pre, ONE vouchers call and every page),
# each answer kept whole, UTC times recorded. 4. Close the session (its stop file). 5. Difference 7: the relay's own status
# (curl, no --max-time, up to 15 minutes), then GET /status straight to 9001 (60 s). 6. Ctrl+C to the relay's console
# (ctrlc.py) and its exit read. 7. summarise.py on the relay folder. Everything goes to evidence/<step>-log.txt.
# The relay is started from PowerShell's Start-Process before this runner (a child of a background shell inherits 'ignore
# Ctrl+C'); its cmd pid comes in RELAY_CMD_PID. Usage: python -I -X utf8 pb.py <step> A|B <company guid> <from> <to>
import json, os, subprocess, sys, time, hashlib
from datetime import datetime, timezone
W = '<LAB_DIR>'; SHEET = '<LAB_DIR>\\sheet'
step, build, guid, frm, to = sys.argv[1:6]
EXE = {'A': ('<BUILD-A folder>\\bin\\x86_64-pc-windows-msvc\\bridge_mcp.exe', '576cc0c5aab4e72c43c05799116d76a29429c595a86dd54834d841c551f50d7c'),
       'B': ('<BUILD-B folder>\\bin\\x86_64-pc-windows-msvc\\bridge_mcp.exe', 'bf5a210616291ab89856dddd4f2c2cc8b785eb23449a74320f7bc2702579a446')}[build]
SD = f'{W}/{step}'; RELAY = f'<LAB_DIR>\\{step}\\relay'
os.makedirs(SD + '/inbox', exist_ok=True); os.makedirs(SD + '/answers', exist_ok=True); os.makedirs(W + '/evidence', exist_ok=True)
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
def log(s):
    with open(f'{W}/evidence/{step}-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
def sha(p): return hashlib.sha256(open(p, 'rb').read()).hexdigest()
if os.path.exists(SD + '/data'): log('stop rule: the step folder already holds a data folder; nothing started'); sys.exit(2)
env = {'BRIDGE_TERMS_ACCEPTED': 'true', 'BRIDGE_TALLY_HOST': '127.0.0.1', 'BRIDGE_TALLY_PORT': '9012', 'BRIDGE_AGENT_ENABLE_TAX_AUDIT': 'true',
       'BRIDGE_AGENT_DATA_DIR': f'<LAB_DIR>\\{step}\\data'}
open(SD + '/launch-env.json', 'w', encoding='utf-8', newline='\n').write(json.dumps(env, indent=2) + '\n')
log(f'# {step} build {build} exe {EXE[0]} bytes {os.path.getsize(EXE[0])} sha256 {sha(EXE[0])} (expected {EXE[1]}); environment {json.dumps(env)}')
# 1. the relay
cmdline = f'python -I read_relay.py --listen-port 9012 --record {RELAY}'
class _P: pass
relay_cmd = _P(); relay_cmd.pid = int(os.environ['RELAY_CMD_PID'])
log(f'{now()} relay console started before this runner from PowerShell (Start-Process cmd.exe, working directory {SHEET}, /k {cmdline}); cmd pid {relay_cmd.pid}')
rpid = None
for _ in range(60):
    time.sleep(0.5)
    out = subprocess.run(['powershell', '-NoProfile', '-Command', f"(Get-CimInstance Win32_Process -Filter 'ParentProcessId={relay_cmd.pid}' | Where-Object Name -eq 'python.exe').ProcessId"], capture_output=True, text=True).stdout.strip()
    if out and os.path.exists(RELAY + '\\manifest.tsv') or (out and os.path.isdir(RELAY)): rpid = int(out.split()[0]); break
time.sleep(1)
con = subprocess.run(['python', '-I', f'{W}/readcon.py', str(relay_cmd.pid), '12', f'{SD}/relay-console-start.txt'], capture_output=True, text=True)
log(f'{now()} relay python pid {rpid}; its console reads:\n{con.stdout.strip()}')
first = 'relay: 127.0.0.1:9012 -> 127.0.0.1:9001, reads only, recording into ' + RELAY + '; Ctrl+C to stop'
if first not in con.stdout: log('stop rule: the relay did not print its expected first line; nothing sent'); sys.exit(3)
# 2. the build
client = subprocess.Popen(['python', '-I', '-X', 'utf8', f'{W}/client.py', SD.replace('/', '\\'), EXE[0], EXE[1]])
time.sleep(4); log(f'{now()} client started pid {client.pid}; ' + open(SD + '/client-log.txt', encoding='utf-8').read().strip().replace('\n', ' | '))
def call(name, tool, args):
    seq = len([f for f in os.listdir(SD + '/inbox') if f.endswith('.json')]); fn = f'{seq:03d}-{name}.json'; t0 = now(); m0 = time.monotonic()
    with open(f'{SD}/inbox/{fn}', 'w', encoding='utf-8', newline='\n') as f: json.dump({'tool': tool, 'arguments': args}, f)
    out = f'{SD}/answers/{fn}'
    while not os.path.exists(out):
        time.sleep(0.2)
        if time.monotonic() - m0 > 900: log(f'stop rule: {fn} has no answer in 15 minutes; left running, nothing more sent'); os._exit(4)
    time.sleep(0.2)
    while True:
        try: j = json.load(open(out, encoding='utf-8')); break
        except ValueError: time.sleep(0.1)
    r = j['response'].get('result') or {}; sc = r.get('structuredContent') or {}
    log(f'{fn}: called {t0} answered {now()} wall {time.monotonic() - m0:.3f} s round trip {j.get("round_trip_seconds")} s isError {r.get("isError")} size {os.path.getsize(out)} sha256 {sha(out)}')
    return j, sc
j, sc = call('list_companies', 'list_companies', {}); log('  ' + json.dumps(sc.get('result'), ensure_ascii=False)[:1200])
j, sc = call('tally_status', 'tally_status', {}); log('  ' + json.dumps(sc.get('result'), ensure_ascii=False)[:1200])
if step == 'B2pre':
    off = None; n = 0; page = 0
    while True:
        a = {'company_guid': guid, 'from': frm, 'to': to}
        if off: a['offset'] = off
        j, sc = call(f'vouchers-p{page}', 'vouchers', a); v = sc.get('result') or {}
        items = v.get('items') or []; n += len(items)
        log(f'  page {page}: rows {len(items)} total {v.get("total")} next_offset {v.get("next_offset")} error {json.dumps(v.get("error"))[:600] if v.get("error") else None}')
        off = v.get('next_offset'); page += 1
        if not off or j['response'].get('result', {}).get('isError'): break
    log(f'  B2pre: vouchers on {frm}: {n}')
else:
    args = {'company_guid': guid, 'from': frm, 'to': to, 'test': 'cash_44ab', 'cash_groups': ['Cash-in-Hand'], 'bank_groups': ['Bank Accounts']}
    log(f'{now()} audit_test_run called with {json.dumps(args)}')
    j, sc = call('audit_test_run', 'audit_test_run', args); v = sc.get('result') or {}
    err = v.get('error') if isinstance(v, dict) else None
    log(f'  audit answer: error code {(err or {}).get("code")}; result keys {sorted(v.keys()) if isinstance(v, dict) else type(v)}; ' + json.dumps(v, ensure_ascii=False)[:2500])
# 4. close the session
open(SD + '/inbox/stop', 'w').close()
try: rc = client.wait(timeout=60)
except subprocess.TimeoutExpired: rc = 'still running'
log(f'{now()} session closed: client exit {rc}; ' + open(SD + '/client-log.txt', encoding='utf-8').read().strip().split('\n')[-1])
# 5. Difference 7
m0 = time.monotonic(); r = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code}', 'http://127.0.0.1:9012/status', '-o', f'{SD}/relay-status.body'], capture_output=True, text=True, timeout=900)
log(f'{now()} relay status through the relay: printed "{r.stdout.strip()}" exit {r.returncode} in {time.monotonic() - m0:.2f} s')
if r.stdout.strip() != '200': log('stop rule: the relay status did not answer 200'); sys.exit(5)
m0 = time.monotonic(); r = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download}', 'http://127.0.0.1:9001/status', '-o', f'{SD}/tally-status.body'], capture_output=True, text=True, timeout=70)
log(f'{now()} GET /status straight to 9001: printed "{r.stdout.strip()}" exit {r.returncode} in {time.monotonic() - m0:.2f} s')
# 6. stop the relay
r = subprocess.run(['python', '-I', f'{W}/ctrlc.py', str(rpid)], capture_output=True, text=True)
time.sleep(2)
con = subprocess.run(['python', '-I', f'{W}/readcon.py', str(relay_cmd.pid), '40', f'{SD}/relay-console-end.txt'], capture_output=True, text=True)
alive = subprocess.run(['powershell', '-NoProfile', '-Command', f'Get-Process -Id {rpid} -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Id'], capture_output=True, text=True).stdout.strip()
log(f'{now()} Ctrl+C sent to the relay console (ctrlc exit {r.returncode}); relay python still running: {bool(alive)}; console tail:\n{con.stdout.strip()[-1500:]}')
# 7. the summary
s = subprocess.run(['python', '-I', '<LAB_DIR>/sheet/summarise.py', RELAY], capture_output=True, text=True)
open(SD + '/summary.txt', 'w', encoding='utf-8', newline='\n').write(s.stdout)
log(f'summarise.py exit {s.returncode}:\n{s.stdout}{s.stderr}')
log(f'# {step} exe sha256 after: {sha(EXE[0])}')
