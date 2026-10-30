# Lab helper (#1713 order rows, not product code): runs the sheet's timed rows in their fixed order, one at a time.
# Per row: the pre-row checks (a capture of Tally's whole window, GET /status 200, MARKS, LEDGERS with the row's name absent,
# both by C3's curl line through c3.py); the row's create file (l2-create.xml's bytes with only the ledger name changed); the
# row's plan; the dialog watcher (watch.ps1) while order.py runs; the row's reading (the sheet's "what counts"); the
# observation reads (MARKS then LEDGERS, each plan by order.py on fresh connections) at C answered + 1, 5, 15 and 30 minutes,
# a refused read re-sent every 30 s for up to 10 minutes; then the verdict, one line in evidence/times.txt. A file
# runs/stop-ALL ends the run before the next send it reaches. Usage: python -I rows.py K0a K1a ...
import hashlib, json, os, subprocess, sys, time
from datetime import datetime, timezone
W = '<lab folder>'; E = W + '/earlier/lab/1713-abandoned-ledger-create'; R = E + '/requests'
ORDER = W + '/sheet/lab/1713-settle-order/order.py'; GUID = '5f4825c4-5d89-47fa-9872-6397cca3a591'
STOPALL = W + '/runs/stop-ALL'
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
def sha(p): return hashlib.sha256(open(p, 'rb').read()).hexdigest()
def log(row, s):
    line = f'{now()} {row} {s}'
    with open(W + '/evidence/rows-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(line + '\n')
    print(line, flush=True)
def stopped(): return os.path.exists(STOPALL)
def run(args, **kw): return subprocess.run(args, capture_output=True, text=True, **kw)
def events(outdir):
    p = outdir + '/events.jsonl'
    return [json.loads(l) for l in open(p, encoding='utf-8')] if os.path.exists(p) else []
def ev(evs, kind, name):
    return next((e for e in evs if e.get('ev') == kind and e.get('name') == name), None)
def readpy(*args):
    r = run([sys.executable, '-I', E + '/read.py', *args])
    return r.returncode, (r.stdout.strip() + ' ' + r.stderr.strip()).strip()
VARIANT = {'K0': ('new', 'new', 20), 'K1': ('K1', 'new', 20), 'K2': ('new', 'K2', 20), 'K3': ('K1', 'K2', 20), 'L1B': ('new', 'new', None)}
def plan_for(row, create):
    bconn, cconn, blimit = VARIANT['L1B' if row.startswith('L1B') else row[:2]]
    conns = {}
    for c in (bconn, cconn):
        if c != 'new': conns[c] = {'warm': R + '/marks.xml', 'sha256': sha(R + '/marks.xml')}
    steps = [{'name': 'A', 'at': 1.0, 'conn': 'new', 'file': R + '/busy-year.xml', 'sha256': sha(R + '/busy-year.xml'), 'limit': None, 'write': False, 'max_bytes': 400000000},
             {'name': 'B', 'at': 5.0, 'conn': bconn, 'file': create, 'sha256': sha(create), 'limit': blimit, 'write': True, 'max_bytes': 2000000},
             {'name': 'C', 'at': 26.0, 'conn': cconn, 'file': R + '/ledgers.xml', 'sha256': sha(R + '/ledgers.xml'), 'limit': None, 'write': False, 'max_bytes': 2000000}]
    return {'host': '127.0.0.1', 'port': 9001, 'conns': conns, 'steps': steps}
def observe(row, label):
    # one observation: MARKS then LEDGERS by order.py (fresh connections, one at a time); a refused read re-sent every 30 s, up to 10 minutes
    result = {}
    for kind, fname in (('marks', 'marks.xml'), ('ledgers', 'ledgers.xml')):
        tries = 0
        while True:
            if stopped(): log(row, f'{label} {kind}: stop-ALL present, not sent'); return None
            tries += 1; out = f'{W}/runs/{row}/{label}-{kind}-{tries}'; os.makedirs(out)
            plan = {'host': '127.0.0.1', 'port': 9001, 'conns': {}, 'steps': [{'name': kind, 'at': 0.0, 'conn': 'new', 'file': R + '/' + fname, 'sha256': sha(R + '/' + fname), 'limit': None, 'write': False, 'max_bytes': 2000000}]}
            json.dump(plan, open(out + '.plan.json', 'w'), indent=1)
            r = run([sys.executable, '-I', ORDER, out + '.plan.json', out])
            end = ev(events(out), 'end', kind)
            how = end.get('how') if end else f'no end (order.py exit {r.returncode} {r.stdout.strip()} {r.stderr.strip()})'
            if how == 'answered' and end.get('status') == 200:
                code, text = readpy(kind, out + f'/{kind}.answer', GUID, *([f'Lab Order {row}'] if kind == 'ledgers' else []))
                log(row, f'{label} {kind} try {tries}: answered {end["bytes"]} B sha256 {end["sha256"]} at {end["utc"]}; read.py exit {code}: {text}')
                result[kind] = text; break
            log(row, f'{label} {kind} try {tries}: {how} at {end.get("utc") if end else now()}')
            if how == 'refused' and tries < 21: time.sleep(30); continue
            result[kind] = f'NOT READ ({how})'; break
    return result
def row_run(row):
    log(row, 'pre-row checks')
    hwnd = run(['powershell', '-NoProfile', '-Command', "(Get-Process tally | Where-Object { $_.MainWindowTitle -like 'TallyPrime:9001*' } | Select-Object -First 1).MainWindowHandle"]).stdout.strip()
    cap = run(['powershell', '-NoProfile', '-File', W + '/capwin.ps1', '-Handle', hwnd, '-Out', f'{W}/shots/{row}-pre.png'])
    st = run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download}', '-o', f'{W}/answers/{row}-pre-status.body', 'http://127.0.0.1:9001/status'])
    log(row, f'capture {row}-pre.png: {cap.stdout.strip()} ;; GET /status exit {st.returncode} printed {st.stdout}')
    if st.returncode != 0 or not st.stdout.startswith('200'): log(row, 'stop: /status not 200'); return 'stop'
    m = run([sys.executable, '-I', W + '/c3.py', f'{row}-pre', 'marks']); log(row, f'MARKS: {m.stdout.strip()}')
    l = run([sys.executable, '-I', W + '/c3.py', f'{row}-pre', 'ledgers', f'Lab Order {row}']); log(row, f'LEDGERS: {l.stdout.strip()}')
    if m.returncode != 0 or l.returncode != 0 or 'probe=absent' not in l.stdout: log(row, 'stop: the pre-row MARKS or LEDGERS check failed (or the name is present)'); return 'stop'
    pre_mark = m.stdout.split('ALTMSTID ')[-1].split()[0] if 'ALTMSTID ' in m.stdout else '?'
    tmpl = open(R + '/l2-create.xml', 'rb').read(); old = 'Lab Probe L2'.encode('utf-16-le'); new = f'Lab Order {row}'.encode('utf-16-le')
    if tmpl.count(old) != 2: log(row, 'stop: the template does not hold the name twice'); return 'stop'
    create = f'{W}/plans/{row}-create.xml'; open(create, 'wb').write(tmpl.replace(old, new))
    plan = plan_for(row, create); planfile = f'{W}/plans/{row}.json'; json.dump(plan, open(planfile, 'w'), indent=1)
    log(row, f'create file {row}-create.xml {os.path.getsize(create)} B sha256 {sha(create)}; plan {row}.json sha256 {sha(planfile)}; B on {plan["steps"][1]["conn"]} limit {plan["steps"][1]["limit"]}, C on {plan["steps"][2]["conn"]}')
    out = f'{W}/runs/{row}/main'; os.makedirs(out); endfile = f'{W}/runs/{row}/watch.end'
    if stopped(): log(row, 'stop-ALL present: the row is not sent'); return 'stop'
    watcher = subprocess.Popen(['powershell', '-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', W + '/watch.ps1', '-Tag', row, '-OutDir', out, '-EndFile', endfile, '-Seconds', '1500'], creationflags=0x08000000)
    time.sleep(3)
    r = run([sys.executable, '-I', ORDER, planfile, out]); open(endfile, 'w').close(); watcher.wait(timeout=60)
    evs = events(out); log(row, f'order.py exit {r.returncode} {r.stdout.strip()} {r.stderr.strip()}')
    for e in evs: log(row, 'event ' + json.dumps(e))
    a_fb, b_wr, b_end, c_wr, c_fb, c_end = ev(evs, 'first_byte', 'A'), ev(evs, 'written', 'B'), ev(evs, 'end', 'B'), ev(evs, 'written', 'C'), ev(evs, 'first_byte', 'C'), ev(evs, 'end', 'C')
    a_end = ev(evs, 'end', 'A'); a_sent = ev(evs, 'send', 'A')
    d = round(a_fb['t'] - a_sent['t'], 3) if a_fb and a_sent else None
    void = []
    if os.path.exists(out + '/stop'): void.append('stop (the watcher saw a change)')
    want_b = 'answered' if row.startswith('L1B') else 'abandoned'
    if not b_wr: void.append('B not written')
    if not b_end or b_end.get('how') != want_b: void.append(f'B end {b_end.get("how") if b_end else None}, not {want_b}')
    if not (c_wr and a_fb and c_wr['t'] < a_fb['t']): void.append('C not written before A first_byte (VOID-late)')
    if not (c_fb and a_fb and c_fb['t'] > a_fb['t']): void.append('C first_byte not after A first_byte')
    if not c_end or c_end.get('how') != 'answered': void.append(f'C end {c_end.get("how") if c_end else None}')
    if d is None or d < 55: void.append(f'D {d} under 55 s')
    c_text = ''
    if c_end and c_end.get('how') == 'answered':
        code, c_text = readpy('ledgers', out + '/C.answer', GUID, f'Lab Order {row}'); log(row, f'C read: exit {code}: {c_text}')
    if a_end and a_end.get('how') == 'answered':
        code, text = readpy('busy', out + '/A.answer', GUID, '48000'); log(row, f'A read: exit {code}: {text}')
    if b_end and b_end.get('how') == 'answered':
        code, text = readpy('import', out + '/B.answer', '1'); log(row, f'B read: exit {code}: {text}')
    log(row, f'D {d} s; reading: {"VOID: " + "; ".join(void) if void else "counts"}')
    obs = {}
    if c_end:
        c_done = time.time() - (evs[-1]['t'] - c_end['t'] if evs else 0)
        for label, mins in (('obs1', 1), ('obs5', 5), ('obs15', 15), ('obs30', 30)):
            while time.time() < c_done + mins * 60: time.sleep(1)
            res = observe(row, label)
            if res is None: return 'stop'
            obs[label] = res
    present_c = 'probe=present' in c_text
    seen = ['present' if 'probe=present' in v.get('ledgers', '') else ('absent' if 'probe=absent' in v.get('ledgers', '') else 'unread') for v in obs.values()]
    marks = [v.get('marks', '').split('ALTMSTID ')[-1].split()[0] if 'ALTMSTID ' in v.get('marks', '') else '?' for v in obs.values()]
    if void: verdict = 'VOID'
    elif present_c and all(s == 'present' for s in seen): verdict = 'PASS-present'
    elif not present_c and all(s == 'absent' for s in seen) and all(mk == pre_mark for mk in marks): verdict = 'PASS-dropped'
    elif not present_c and any(s == 'present' for s in seen): verdict = 'FAIL'
    else: verdict = 'NOTE'
    line = f'{row}: D {d} s; B {b_end.get("how") if b_end else None}; C {"present" if present_c else "absent"}; observations {seen}; marks before B {pre_mark}, after {marks}; {verdict}' + (f' ({"; ".join(void)})' if void else '')
    with open(W + '/evidence/times.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(line + '\n')
    log(row, 'VERDICT ' + line)
    return verdict
for row in sys.argv[1:]:
    if stopped(): log(row, 'stop-ALL present: not started'); break
    v = row_run(row)
    if v == 'stop' or os.path.exists(f'{W}/runs/{row}/main/stop'): log(row, f'the run ends after {row} ({v})'); break
log('-', 'RUN DONE')
