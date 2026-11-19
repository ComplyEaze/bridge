# Lab helper (2074-2214 X cut, not product code): LAB-COMMON C2 and C3 by hand. Each raw send once, by C3's curl line (no
# client timeout), its headers kept, status, bytes and CONTENT-LENGTH checked, the request's sha256 checked first.
# Usage: python -I -X utf8 raw.py step0 <label> | marks <label> | status <label> | send <label> <request file> [bound]
import hashlib, os, re, subprocess, sys, time
R = '<lab folder>'; S = R + '/sheet'; G = 'a30f23c3-4819-4334-96ab-ffa6fba70976'
for d in ('reads', 'evidence', 'shots'): os.makedirs(f'{R}/{d}', exist_ok=True)
def sha(b): return hashlib.sha256(b).hexdigest()
def stamp(): t = time.time(); return time.strftime('%Y-%m-%dT%H:%M:%S', time.gmtime(t)) + f'.{int(t * 1000) % 1000:03d}Z'
def note(s):
    with open(R + '/evidence/notes.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
sheet = {}
for l in open(S + '/SHA256.txt', encoding='utf-8'):
    p = l.split()
    if len(p) >= 3: sheet[p[-1]] = (p[0], int(p[1]))
def shot(name):
    h = subprocess.run(['powershell', '-NoProfile', '-Command', '[int64](Get-Process tally | Select-Object -First 1).MainWindowHandle'], capture_output=True, text=True).stdout.strip()
    subprocess.run(['powershell', '-NoProfile', '-File', '<lab root>/rc070/tools/capwin.ps1', '-Handle', h, '-Out', f'<lab folder>\\shots\\{name}.png'], capture_output=True, text=True)
    p = f'{R}/shots/{name}.png'; return sha(open(p, 'rb').read()) if os.path.exists(p) else 'no capture'
def send(label, req, bound=2000000):
    ans = f'{R}/reads/{label}.xml'; hdr = f'{R}/reads/{label}.headers.txt'
    if os.path.exists(ans): sys.exit(f'{label} exists; never sent again')
    rb = open(req, 'rb').read(); ts = stamp(); m0 = time.monotonic()
    r = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download}', '--data-binary', '@' + req, '-H', 'Content-Type: text/xml; charset=utf-16', '-D', hdr, 'http://127.0.0.1:9001', '-o', ans], capture_output=True, text=True)
    secs = time.monotonic() - m0; cl = None
    if os.path.exists(hdr):
        for hl in open(hdr, encoding='latin-1'):
            if hl.upper().startswith('CONTENT-LENGTH:'): cl = int(hl.split(':', 1)[1])
    body = open(ans, 'rb').read() if os.path.exists(ans) else b''
    ok = r.returncode == 0 and r.stdout.startswith('200 ') and len(body) > 0 and cl == len(body) and len(body) <= bound
    note(f'{ts} {label} req {os.path.basename(req)} sha256 {sha(rb)} bytes {len(rb)} | curl {r.returncode} "{r.stdout.strip()}" {r.stderr.strip()} CL {cl} | answer {len(body)} B sha256 {sha(body)} | {secs:.3f} s | ok {ok}')
    return ok, body
def text(b): return (b[2:] if b[:2] == b'\xff\xfe' else b).decode('utf-16-le')
def marks(label):
    mp = S + '/requests/marks.utf16le.xml'; mb = open(mp, 'rb').read()
    if (sha(mb), len(mb)) != sheet['requests/marks.utf16le.xml']: sys.exit('MARKS file differs from SHA256.txt; nothing sent')
    ok, body = send(label, mp)
    rows = re.findall(r'<COMPANY NAME="[^"]*"[^>]*>(.*?)</COMPANY>', text(body), re.S) if ok else []
    if len(rows) != 1: note(f'   MARKS {label}: {len(rows)} COMPANY rows'); return None
    g = re.search(r'<GUID[^>]*>([^<]*)<', rows[0]).group(1).strip(); v = re.search(r'<ALTVCHID[^>]*>\s*([0-9]+)', rows[0]).group(1); m = re.search(r'<ALTMSTID[^>]*>\s*([0-9]+)', rows[0]).group(1)
    note(f'   MARKS {label}: GUID {g} (THE COMPANY {g == G}) ALTMSTID {m} ALTVCHID {v}'); return g, int(m), int(v)
def status(label):
    ans = f'{R}/reads/{label}.status.txt'
    if os.path.exists(ans): sys.exit(f'{label} exists')
    ts = stamp(); r = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download}', 'http://127.0.0.1:9001/status', '-o', ans], capture_output=True, text=True)
    note(f'{ts} {label} GET /status | curl {r.returncode} "{r.stdout.strip()}" {r.stderr.strip()}')
def cmd(label, line):
    r = subprocess.run('cmd.exe /c ' + line + ' 2>&1', capture_output=True, text=True)
    open(f'{R}/evidence/{label}.txt', 'w', encoding='utf-8', newline='\n').write(r.stdout); return r.returncode, r.stdout
k = sys.argv[1]; label = sys.argv[2]
if k == 'step0':
    note(f'{stamp()} {label} step 0: capture {shot(label + "-window")}')
    rc, out = cmd(label + '-tasklist', 'tasklist /FI "IMAGENAME eq tally.exe" /FO CSV /NH')
    lines = [l for l in out.splitlines() if l.strip()]; note(f'   a. tasklist exit {rc}: {len(lines)} line(s): {lines}')
    pid = lines[0].split('","')[1] if len(lines) == 1 and 'tally.exe' in lines[0] else None
    rc, out = cmd(label + '-netstat', 'netstat -ano')
    lis = [l.split() for l in out.splitlines() if 'LISTENING' in l]
    on9001 = [l for l in lis if l[1].endswith(':9001')]
    note(f'   b. netstat exit {rc}: 9001 LISTENING lines {len(on9001)}, PIDs {sorted({l[-1] for l in on9001})}, tally PID {pid}, other LISTENING lines {len(lis) - len(on9001)}')
    status(label); marks(label + '-marks')
elif k == 'marks': marks(label)
elif k == 'status': status(label)
elif k == 'send':
    req = sys.argv[3]; rel = os.path.relpath(req, S).replace(chr(92), '/')
    if rel in sheet and (sha(open(req, 'rb').read()), os.path.getsize(req)) != sheet[rel]: sys.exit(f'{rel} differs from SHA256.txt; nothing sent')
    ok, body = send(label, req, int(sys.argv[4]) if len(sys.argv) > 4 else 2000000)
    if ok:
        t = text(body); print({x: re.findall(rf'<{x}>\s*([^<]*)', t) for x in ('CREATED', 'ALTERED', 'DELETED', 'COMBINED', 'IGNORED', 'ERRORS', 'CANCELLED', 'EXCEPTIONS', 'LINEERROR', 'LASTVCHID', 'LASTMID')})
