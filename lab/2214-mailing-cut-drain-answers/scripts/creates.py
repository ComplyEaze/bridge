# Lab runner (#2214, not product code; 2074-L's a5.py made a create runner): the sheet's named exception 1 to C3. Sends
# create-<from> to create-<to>, one at a time, each once, in order, by LAB-COMMON C3's curl line (no client timeout; headers
# kept; status, bytes and CONTENT-LENGTH checked). Before every create: one MARKS read (exactly one COMPANY row, THE
# COMPANY's GUID, ALTVCHID 87, ALTMSTID exactly 224 + the creates sent before), the MARKS file's and the create file's sha256
# against the sheet (SHA256.txt, requests-manifest.txt). After every create: the answer must read CREATED 1 and every other
# counter 0, with no LINEERROR. Any other reading halts the runner: nothing more is sent. A create already logged as sent is
# never sent again. Captures of the whole Tally window (PrintWindow) after each create the sheet names (000 to 005, every
# 25th, the last) and on a halt; the runner then stops so the capture is looked at before anything more is sent.
# A capture byte-equal to the reference capture after create-000 (looked at: the Gateway, no dialog) lets the runner go on;
# any other capture stops it. Answers are aggregated: one copy of each answer text, the first and the last kept whole; one line per create in
# evidence/creates-log.tsv. Usage: python -I creates.py <first index> <last index>
import hashlib, os, re, subprocess, sys, time
R = '<LAB_DIR>'; S = R + '/sheet'; G = '<company guid>'
first, last = int(sys.argv[1]), int(sys.argv[2])
out = R + '/answers-creates'; os.makedirs(out, exist_ok=True); os.makedirs(R + '/evidence', exist_ok=True); os.makedirs(R + '/shots', exist_ok=True)
logp = R + '/evidence/creates-log.tsv'
def sha(b): return hashlib.sha256(b).hexdigest()
def stamp(): t = time.time(); return time.strftime('%Y-%m-%dT%H:%M:%S', time.gmtime(t)) + f'.{int(t * 1000) % 1000:03d}Z'
def log(s):
    with open(logp, 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
sheet = {}
for l in open(S + '/SHA256.txt', encoding='utf-8'):
    p = l.split()
    if len(p) >= 3: sheet[p[-1]] = (p[0], int(p[1]))
manifest = {}
for l in open(S + '/requests-manifest.txt', encoding='utf-8'):
    n, b, h = l.split(); manifest[n] = (h, int(b))
sent_before = set()
if os.path.exists(logp):
    for l in open(logp, encoding='utf-8'):
        m = re.match(r'(\d{3})\t', l)
        if m: sent_before.add(int(m.group(1)))
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
    ok = r.returncode == 0 and r.stdout.startswith('200 ') and len(body) > 0 and cl == len(body) and len(body) <= 2000000
    return ok, body, f'curl {r.returncode} "{r.stdout.strip()}" CL {cl} bytes {len(body)}'
def text(body): return (body[2:] if body[:2] == b'\xff\xfe' else body).decode('utf-16-le')
def marks():
    mp = S + '/requests/marks.utf16le.xml'; mb = open(mp, 'rb').read()
    if (sha(mb), len(mb)) != sheet['requests/marks.utf16le.xml']: return None, 'MARKS file differs from SHA256.txt; nothing sent', b''
    ok, body, info = c3(mp)
    if not ok: return None, info, body
    t = text(body); rows = re.findall(r'<COMPANY NAME="[^"]*"[^>]*>(.*?)</COMPANY>', t, re.S)
    if len(rows) != 1: return None, f'{info}; {len(rows)} COMPANY rows', body
    g = re.search(r'<GUID[^>]*>([^<]*)<', rows[0]); v = re.search(r'<ALTVCHID[^>]*>\s*([0-9]+)\s*<', rows[0]); m = re.search(r'<ALTMSTID[^>]*>\s*([0-9]+)\s*<', rows[0])
    return (g.group(1).strip() if g else None, int(v.group(1)) if v else None, int(m.group(1)) if m else None), info, body
kept = {}
for n in os.listdir(out):
    if n.endswith('.answer.xml'): kept[sha(re.sub(r'<CMPINFO>.*?</CMPINFO>', '', text(open(f'{out}/{n}', 'rb').read()), flags=re.S).encode('utf-8'))] = n
def keep(body, name):
    h = sha(re.sub(r'<CMPINFO>.*?</CMPINFO>', '', text(body), flags=re.S).encode('utf-8')) if body else 'empty'
    if h in kept: return kept[h]
    open(f'{out}/{name}', 'wb').write(body); kept[h] = name; return name
SHOTS = set(range(0, 6)) | {i for i in range(25, 201, 25)} | {200}
log(f'# creates {first:03d}-{last:03d} started {stamp()}')
t0 = time.time()
for i in range(first, last + 1):
    if i in sent_before: log(f'HALT {stamp()} create-{i:03d} is already logged as sent; never sent again'); shot(f'creates-halt-{i:03d}'); sys.exit(6)
    mk, info, body = marks()
    if mk is None: log(f'HALT {stamp()} before create-{i:03d}: MARKS read failed: {info}'); shot(f'creates-halt-{i:03d}'); sys.exit(2)
    g, v, m = mk; exp_m = 224 + i
    if g != G or v != 87 or m != exp_m:
        log(f'HALT {stamp()} before create-{i:03d}: MARKS guid {g} V {v} (expected 87) M {m} (expected {exp_m})'); shot(f'creates-halt-{i:03d}'); sys.exit(3)
    name = f'create-{i:03d}.utf16le.xml'; req = f'{R}/requests/{name}'; rb = open(req, 'rb').read()
    if (sha(rb), len(rb)) != manifest[name]: log(f'HALT {stamp()} {name} differs from requests-manifest.txt; nothing sent'); sys.exit(4)
    ts = stamp(); m0 = time.monotonic(); ok, body, info = c3(req); secs = time.monotonic() - m0
    t = text(body) if body else ''
    cnt = {k: (re.search(rf'<{k}>\s*(-?[0-9]+)\s*</{k}>', t).group(1) if re.search(rf'<{k}>\s*(-?[0-9]+)\s*</{k}>', t) else 'absent') for k in ('CREATED', 'ALTERED', 'DELETED', 'COMBINED', 'IGNORED', 'ERRORS', 'CANCELLED', 'EXCEPTIONS')}
    le = re.findall(r'<LINEERROR>(.*?)</LINEERROR>', t, re.S)
    good = ok and cnt['CREATED'] == '1' and all(cnt[k] == '0' for k in ('ALTERED', 'DELETED', 'COMBINED', 'IGNORED', 'ERRORS', 'CANCELLED', 'EXCEPTIONS')) and not le
    kn = keep(body, f'{i:03d}-create.answer.xml') if body else 'none'
    log(f'{i:03d}\t{ts}\t{name}\tM_before {m}\t{secs:.3f} s\t{info}\t{cnt}\tLINEERROR {le}\tanswer sha256 {sha(body)} kept as {kn}')
    if not good: log(f'HALT {stamp()} create-{i:03d}: answer not as expected'); keep(body, f'{i:03d}-create.halt.xml'); shot(f'creates-halt-{i:03d}'); sys.exit(5)
    if i == last: open(f'{out}/{i:03d}-last.answer.xml', 'wb').write(body)
    if i in SHOTS:
        shot(f'create-{i:03d}'); cap = open(f'{R}/shots/create-{i:03d}.png', 'rb').read(); ref = open(f'{R}/shots/create-000.png', 'rb').read()
        if cap == ref: log(f'# capture after create-{i:03d}: byte-equal to the Gateway reference (create-000.png, looked at): no dialog; sha256 {sha(cap)}')
        else: log(f'# capture after create-{i:03d} differs from the Gateway reference (sha256 {sha(cap)}); runner stops for it to be looked at'); break
mk, info, body = marks()
log(f'# segment ended {stamp()} after create-{i:03d}; elapsed {time.time() - t0:.1f} s; MARKS after: {mk} {info}')
