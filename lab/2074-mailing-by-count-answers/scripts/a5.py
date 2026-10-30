# Lab runner (#2074-L A5 and Part L, not product code): the sheet's named exception to C3. Sends alter-a and alter-b
# alternately, one at a time, by LAB-COMMON C3's curl line (no client timeout; headers kept; status, bytes and
# CONTENT-LENGTH checked). Before every alteration: one MARKS read (exactly one COMPANY row, THE COMPANY's GUID, ALTVCHID
# unchanged, ALTMSTID exactly the one expected) and the alter file's sha256 against the sheet's SHA256.txt. After every
# alteration: the answer must read CREATED 0, ALTERED 1, ERRORS 0, EXCEPTIONS 0 (every other counter 0) with no LINEERROR.
# Any other reading halts the runner: nothing more is sent. Captures of the whole Tally window (PrintWindow) at the 1st,
# the 10th, every 500th and the last alteration, and on a halt. Answers are aggregated: the first and the last kept whole,
# and one copy of every answer whose text, CMPINFO removed, differs from those already kept; every alteration is one line
# of the log with its answer's sha256.
# Usage: python -I a5.py <label> <start alteration index> (--count N | --until M)
import hashlib, os, re, subprocess, sys, time
R = '<LAB_DIR>'; S = R + '/sheet'; G = 'a30f23c3-4819-4334-96ab-ffa6fba70976'
label, start = sys.argv[1], int(sys.argv[2]); mode, target = sys.argv[3], int(sys.argv[4])
out = f'{R}/answers/{label}'; os.makedirs(out, exist_ok=True); logp = f'{R}/evidence/{label}-log.tsv'
want = {}
for l in open(S + '/SHA256.txt', encoding='utf-8'):
    p = l.split()
    if len(p) >= 3: want[p[2]] = (p[0], int(p[1]))
def sha(b): return hashlib.sha256(b).hexdigest()
def stamp(): t = time.time(); return time.strftime('%Y-%m-%dT%H:%M:%S', time.gmtime(t)) + f'.{int(t * 1000) % 1000:03d}Z'
def log(s):
    with open(logp, 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
def shot(name):
    h = subprocess.run(['powershell', '-NoProfile', '-Command', '[int64](Get-Process tally | Select-Object -First 1).MainWindowHandle'], capture_output=True, text=True).stdout.strip()
    subprocess.run(['powershell', '-NoProfile', '-File', '<lab tools>/capwin.ps1', '-Handle', h, '-Out', f'<LAB_DIR>\\shots\\{name}.png'], capture_output=True, text=True)
def c3(reqpath):
    tmp_a, tmp_h = f'{out}/_last.answer', f'{out}/_last.headers'
    for p in (tmp_a, tmp_h):
        if os.path.exists(p): os.remove(p)
    r = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download}', '--data-binary', '@' + reqpath, '-H', 'Content-Type: text/xml; charset=utf-16', '-D', tmp_h, 'http://127.0.0.1:9001', '-o', tmp_a], capture_output=True, text=True)
    cl = None
    if os.path.exists(tmp_h):
        for hl in open(tmp_h, encoding='latin-1'):
            if hl.upper().startswith('CONTENT-LENGTH:'): cl = int(hl.split(':', 1)[1])
    body = open(tmp_a, 'rb').read() if os.path.exists(tmp_a) else b''
    ok = r.returncode == 0 and r.stdout.startswith('200 ') and len(body) > 0 and cl == len(body)
    return ok, body, f'curl {r.returncode} "{r.stdout.strip()}" CL {cl} bytes {len(body)}'
def text(body): return (body[2:] if body[:2] == b'\xff\xfe' else body).decode('utf-16-le')
def marks():
    ok, body, info = c3(R + '/raw/marks-batch.xml')
    if not ok: return None, info, body
    t = text(body); rows = re.findall(r'<COMPANY NAME="[^"]*"[^>]*>(.*?)</COMPANY>', t, re.S)
    if len(rows) != 1: return None, f'{info}; {len(rows)} COMPANY rows', body
    g = re.search(r'<GUID[^>]*>([^<]*)<', rows[0]); v = re.search(r'<ALTVCHID[^>]*>\s*([0-9]+)\s*<', rows[0]); m = re.search(r'<ALTMSTID[^>]*>\s*([0-9]+)\s*<', rows[0])
    return (g.group(1).strip() if g else None, int(v.group(1)) if v else None, int(m.group(1)) if m else None), info, body
kept = {}
def keep(body, name):
    # answers that differ only in CMPINFO (Tally's session object counts) are one answer: keyed by the text without CMPINFO
    h = sha(re.sub(r'<CMPINFO>.*?</CMPINFO>', '', text(body), flags=re.S).encode('utf-8')) if body else 'empty'
    if h in kept: return kept[h]
    open(f'{out}/{name}', 'wb').write(body); kept[h] = name; return name
first = None; m0 = None; v0 = None; i = start; sent = 0; t0 = time.time()
log(f'# {label} started {stamp()} start index {start} {mode} {target}')
while True:
    mk, info, body = marks()
    if mk is None: log(f'HALT {stamp()} before alteration {i}: MARKS read failed: {info}'); keep(body, f'halt-marks.xml'); shot(f'{label}-halt'); sys.exit(2)
    g, v, m = mk
    if m0 is None: m0, v0 = m, v
    exp_m = m0 + sent
    if g != G or v != v0 or m != exp_m:
        log(f'HALT {stamp()} before alteration {i}: MARKS guid {g} V {v} (expected {v0}) M {m} (expected {exp_m})'); shot(f'{label}-halt'); sys.exit(3)
    if (mode == '--count' and sent >= target) or (mode == '--until' and m >= target):
        log(f'END {stamp()} sent {sent}; last MARKS V {v} M {m}; elapsed {time.time() - t0:.1f} s'); print(f'END sent {sent} M {m} V {v} elapsed {time.time() - t0:.1f} s'); break
    name = 'alter-a' if i % 2 == 1 else 'alter-b'; rel = f'requests/{name}.utf16le.xml'; req = f'{S}/{rel}'
    rb = open(req, 'rb').read()
    if (sha(rb), len(rb)) != want[rel]: log(f'HALT {stamp()} alteration {i}: {rel} differs from SHA256.txt; nothing sent'); sys.exit(4)
    ts = stamp(); ok, body, info = c3(req)
    t = text(body) if body else ''
    cnt = {k: (re.search(rf'<{k}>\s*(-?[0-9]+)\s*</{k}>', t).group(1) if re.search(rf'<{k}>\s*(-?[0-9]+)\s*</{k}>', t) else 'absent') for k in ('CREATED', 'ALTERED', 'DELETED', 'COMBINED', 'IGNORED', 'ERRORS', 'CANCELLED', 'EXCEPTIONS')}
    le = re.findall(r'<LINEERROR>(.*?)</LINEERROR>', t, re.S)
    good = ok and cnt['ALTERED'] == '1' and all(cnt[k] == '0' for k in ('CREATED', 'DELETED', 'COMBINED', 'IGNORED', 'ERRORS', 'CANCELLED', 'EXCEPTIONS')) and not le
    kn = keep(body, f'{i:05d}-{name}.answer.xml') if body else 'none'
    sent += 1
    log(f'{i}\t{ts}\t{name}\tM_before {m}\t{info}\t{cnt}\tLINEERROR {le}\tanswer sha256 {sha(body)} kept as {kn}')
    if first is None: first = kn
    if not good: log(f'HALT {stamp()} alteration {i}: answer not as expected'); keep(body, f'{i:05d}-{name}.halt.xml'); shot(f'{label}-halt'); sys.exit(5)
    if sent in (1, 10) or sent % 500 == 0: shot(f'{label}-alt{sent:05d}')
    if sent % 250 == 0: print(f'{stamp()} sent {sent} M before {m}', flush=True)
    i += 1
open(f'{out}/{i - 1:05d}-last.answer.xml', 'wb').write(body)
shot(f'{label}-last'); log(f'# kept answers: {sorted(kept.values())}; next index {i}')
print('kept', sorted(kept.values()), 'next index', i)
