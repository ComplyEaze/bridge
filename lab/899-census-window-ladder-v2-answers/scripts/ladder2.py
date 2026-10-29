# Lab helper (#899-P v2, not product code; v1's ladder.py with the v2 first span). Runs the sheet's steps 2 to 9 for one part (D or H) in order, one request at a
# time, with C3's line plus the sheet's two named differences (the extended -w, --max-filesize 33554432), no --max-time, a 600 s
# watch (a held gateway = stop, curl left running, nothing more sent), a capture of Tally's window and GET /status after each
# send, census_read.py on each census answer, and the ladder's rules (a census over 20 s ends the ladder; curl exit 63 ends the
# census sends). Writes evidence/<part>-times-local.txt and prints each line. Usage: python -I ladder2.py <D|H> <W1from> <W1to> <W2from> <W2to> <W3from> <W3to>
import hashlib, os, re, subprocess, sys, time
from datetime import datetime, timezone
PART = sys.argv[1]; P = "<lab folder>"; D = P + "/" + PART; S = P + "/sheet/lab/899-census-window-ladder"
W = {'w1': sys.argv[2:4], 'w2': sys.argv[4:6], 'w3': sys.argv[6:8]}
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
def log(s):
    with open(P + '/evidence/' + PART + '-times-local.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
def sha(p): return hashlib.sha256(open(p, 'rb').read()).hexdigest()
def gen(args):
    r = subprocess.run([sys.executable, '-I', S + '/gen_census.py'] + args, capture_output=True, text=True)
    if r.returncode: log(f'stop: gen_census {args[0]} failed: {r.stdout.strip()} {r.stderr.strip()}'); sys.exit(3)
    return r.stdout.strip()
hwnd = subprocess.run(['powershell', '-NoProfile', '-Command', "(Get-Process tally | Where-Object { $_.MainWindowTitle -like 'TallyPrime:9001*' } | Select-Object -First 1).MainWindowHandle"], capture_output=True, text=True).stdout.strip()
recorded = {}
def send(name, req):
    if sha(req) != recorded[req]: log(f'stop: {name} request sha256 differs from the one recorded'); sys.exit(3)
    ans = f'{D}/answers/{name}.response'; hdr = f'{D}/answers/{name}.headers.txt'
    if os.path.exists(ans): log(f'stop: {name} answer exists (never resent)'); sys.exit(3)
    t0 = now(); st = time.monotonic()
    p = subprocess.Popen(['curl.exe', '-sS', '-w', '%{http_code} %{size_download} %{size_upload} %{time_connect} %{time_starttransfer} %{time_total}',
                          '--max-filesize', '33554432', '--data-binary', '@' + req, '-H', 'Content-Type: text/xml; charset=utf-16', '-D', hdr,
                          'http://127.0.0.1:9001', '-o', ans], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    while p.poll() is None:
        if time.monotonic() - st > 600: log(f'{now()} stop: {name} has no answer at 600 s: gateway held; curl pid {p.pid} left running; nothing more sent'); sys.exit(7)
        time.sleep(0.1)
    out, err = p.communicate(); t1 = now()
    cl = ''.join(re.findall(r'CONTENT-LENGTH:\s*(\d+)', open(hdr, encoding='latin-1').read(), re.I)) if os.path.exists(hdr) else ''
    size = os.path.getsize(ans) if os.path.exists(ans) else -1
    f = out.split()
    line = f'{name} request_bytes={os.path.getsize(req)} start={t0} end={t1} curl_exit={p.returncode} printed="{out}" content_length={cl} bytes={size} sha256={sha(ans) if size >= 0 else "none"}'
    log(line + (f' stderr="{err.strip()}"' if err.strip() else ''))
    subprocess.run(['powershell', '-NoProfile', '-File', P + '/capwin.ps1', '-Handle', hwnd, '-Out', f'{P}/shots/{PART.lower()}-{name}-after.png'], capture_output=True)
    s0 = time.monotonic(); r = subprocess.run(['curl.exe', '-sS', '-m', '60', '-w', '%{http_code} %{size_download}', '-o', f'{D}/answers/{name}-status.body', 'http://127.0.0.1:9001/status'], capture_output=True, text=True)
    log(f'{name} GET /status {now()} exit {r.returncode} printed {r.stdout} in {time.monotonic() - s0:.3f} s')
    if r.returncode == 28 or time.monotonic() - s0 >= 60: log('stop: /status had no answer in 60 s: gateway held'); sys.exit(7)
    if p.returncode == 63: return {'exit63': True, 'cl': cl}
    if p.returncode != 0 or not out.startswith('200 ') or str(size) != cl or size <= 0: log(f'stop: {name} request failed (C3)'); sys.exit(2)
    return {'exit63': False, 'total': float(f[5]), 'start': float(f[4]), 'bytes': size}
def read(name, lo, hi, frm, to):
    r = subprocess.run([sys.executable, '-I', S + '/census_read.py', f'{D}/answers/{name}.response', str(lo), str(hi), frm, to], capture_output=True, text=True)
    log(f'{name} census_read exit {r.returncode}: {r.stdout.strip()}')
    m = dict(re.findall(r'(\w+) (\d+)', r.stdout))
    if r.returncode != 0 or any(m.get(k, '1') != '0' for k in ('rows_missing_a_field', 'rows_outside_span', 'rows_outside_dates', 'repeated_guids')):
        log(f'stop: {name} census_read failure or a count other than its EXPECT'); sys.exit(4)
    return int(m.get('rows', '0'))
def marks(name):
    req = f'{D}/requests/marks.utf16le.xml'; recorded[req] = sha(req)
    send(name, req)
    t = open(f'{D}/answers/{name}.response', 'rb').read().decode('utf-16-le').lstrip('\ufeff')
    rows = re.findall(r'<COMPANY NAME="[^"]*"[^>]*>(.*?)</COMPANY>', t, re.S)
    if len(rows) != 1: log(f'stop: {name} holds {len(rows)} COMPANY rows'); sys.exit(5)
    v = int(re.search(r'<ALTVCHID[^>]*>\s*(\d+)', rows[0]).group(1)); m = int(re.search(r'<ALTMSTID[^>]*>\s*(\d+)', rows[0]).group(1))
    log(f'{name}: one COMPANY row; V={v} M={m}'); return v, m
# step 2
log(f'marks request: {gen(["marks", "--company-file", D + "/company.txt", "--out", D + "/requests/marks.utf16le.xml"])}; sha256 {sha(D + "/requests/marks.utf16le.xml")}')
v0, m0 = marks('m0')
log(f'V0 above 2097152: {"yes" if v0 > 2097152 else "no"}; V0 at most 8192: {"yes" if v0 <= 8192 else "no"}')
results = {}
if v0 > 8192:
    lo, hi = map(int, gen(['aligned', '--mark', str(v0), '--last']).split()); kind1 = 'last'
    if hi - lo < 1024: lo, hi = map(int, gen(['aligned', '--mark', str(v0), '--newest']).split()); kind1 = 'newest full'
    log(f'first span ({kind1}): lo {lo} hi {hi} width {hi - lo}')
    widest = None; stop_census = False
    for w in ('w1', 'w2', 'w3'):
        frm, to = W[w]
        req = f'{D}/requests/{w}.utf16le.xml'
        log(f'{w} window {frm}..{to}: {gen(["census", "--company-file", D + "/company.txt", "--from", frm, "--to", to, "--lo", str(lo), "--hi", str(hi), "--out", req])}; sha256 {sha(req)}'); recorded[req] = sha(req)
        r = send(w, req)
        if r['exit63']: log(f'{w}: curl exit 63 (answer over 33554432 B; CONTENT-LENGTH {r["cl"]}): rule 5, no further census'); stop_census = True; break
        rows = read(w, lo, hi, frm, to); results[w] = (r, rows)
        log(f'{w}: rows {rows}; bytes per row {r["bytes"] / rows if rows else "n/a"}; time_starttransfer {r["start"]}; time_total {r["total"]}')
        if r['total'] <= 20: widest = w
        if r['total'] > 20: log(f'{w}: time_total above 20 s: the ladder ends'); break
    if not stop_census and widest:
        if lo >= 8192: lo2, hi2 = map(int, gen(['aligned', '--mark', str(v0), '--before', str(lo)]).split()); kind = 'full (before the first)'
        else: lo2, hi2 = map(int, gen(['aligned', '--mark', str(v0), '--after', str(hi)]).split()); kind = 'after the first, ' + ('full' if hi2 - lo2 == 8192 else 'narrower')
        frm, to = W[widest]; req = f'{D}/requests/s2.utf16le.xml'
        log(f's2 second span lo {lo2} hi {hi2} ({kind}); WIDEST {widest} {frm}..{to}: {gen(["census", "--company-file", D + "/company.txt", "--from", frm, "--to", to, "--lo", str(lo2), "--hi", str(hi2), "--out", req])}; sha256 {sha(req)}'); recorded[req] = sha(req)
        r = send('s2', req)
        if r['exit63']: log('s2: curl exit 63: rule 5')
        else:
            rows = read('s2', lo2, hi2, frm, to); log(f's2: rows {rows}; bytes per row {r["bytes"] / rows if rows else "n/a"}; time_starttransfer {r["start"]}; time_total {r["total"]}')
            if 'w1' in results:
                r1 = send('r1', f'{D}/requests/w1.utf16le.xml')
                if not r1['exit63']: rows = read('r1', lo, hi, *W['w1']); log(f'r1: rows {rows}; time_starttransfer {r1["start"]}; time_total {r1["total"]}')
else:
    log('V0 at most 8192: the product sends no span; no census sent')
v1, m1 = marks('m1')
log(f'marks: V1 {"equal" if v1 == v0 else "NOT equal"} to V0, M1 {"equal" if m1 == m0 else "NOT equal"} to M0')
log('LADDER DONE')
