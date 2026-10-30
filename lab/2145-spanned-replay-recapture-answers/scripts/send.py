# Lab helper (#2145-R, not product code): sends ONE request of the sheet by LAB-COMMON C3's line and records it.
# Checks the request file's sha256 and bytes against the sheet's SHA256.txt first (a difference: exit 3, nothing sent);
# sends it with curl.exe exactly as C3 writes (no client timeout); records curl's exit, the printed status and bytes, and
# CONTENT-LENGTH from the headers file; checks the answer against its READ BOUND; then sends GET /status (C3's line,
# without the data) and records it. Exit 0 = all as C3 wants; 1 = a C3 stop (non-zero exit, non-200, size differing
# from CONTENT-LENGTH, empty body, above the bound, or a status read that failed). Log: <out>/<name>.send.txt
# Usage: python -I send.py <name> <out folder> <read bound>
import hashlib, os, subprocess, sys, time
SHEET = '<LAB_DIR>/sheet'
name, out, bound = sys.argv[1], sys.argv[2].rstrip('/\\'), int(sys.argv[3])
req = f'{SHEET}/requests/{name}.request.utf16le.xml'
def stamp(): return time.strftime('%Y-%m-%dT%H:%M:%S', time.gmtime()) + f'.{int(time.time() * 1000) % 1000:03d}Z'
lines = []
def log(s): lines.append(s); print(s)
want = None
for l in open(f'{SHEET}/SHA256.txt', encoding='utf-8'):
    p = l.split()
    if len(p) >= 3 and p[2] == f'requests/{name}.request.utf16le.xml': want = (p[0], int(p[1]))
d = open(req, 'rb').read(); have = (hashlib.sha256(d).hexdigest(), len(d))
log(f'request requests/{name}.request.utf16le.xml {have[1]} B sha256 {have[0]}; SHA256.txt {want}')
if want != have:
    log('stop: the request file differs from SHA256.txt; nothing sent'); open(f'{out}/{name}.send.txt', 'w', encoding='utf-8').write('\n'.join(lines) + '\n'); sys.exit(3)
ans, hdr = f'{out}/{name}.answer.xml', f'{out}/{name}.headers.txt'
if os.path.exists(ans): log('stop: an answer file exists already; never re-sent'); sys.exit(3)
cmd = ['curl.exe', '-sS', '-w', '%{http_code} %{size_download}', '--data-binary', '@' + req, '-H', 'Content-Type: text/xml; charset=utf-16', '-D', hdr, 'http://127.0.0.1:9001', '-o', ans]
t0 = stamp(); r = subprocess.run(cmd, capture_output=True, text=True); t1 = stamp()
log(f'sent {t0} answered {t1}; curl exit {r.returncode}; printed "{r.stdout.strip()}"; stderr "{r.stderr.strip()}"')
cl = None
if os.path.exists(hdr):
    for l in open(hdr, encoding='latin-1'):
        if l.upper().startswith('CONTENT-LENGTH:'): cl = int(l.split(':', 1)[1])
size = os.path.getsize(ans) if os.path.exists(ans) else 0
sha = hashlib.sha256(open(ans, 'rb').read()).hexdigest() if size else ''
log(f'CONTENT-LENGTH {cl}; answer {size} B sha256 {sha}; READ BOUND {bound}')
bad = r.returncode != 0 or not r.stdout.startswith('200 ') or size == 0 or cl != size or size > bound
s0 = stamp(); s = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download}', 'http://127.0.0.1:9001/status', '-o', f'{out}/{name}.status.body'], capture_output=True, text=True, timeout=60)
log(f'GET /status sent {s0} answered {stamp()}; exit {s.returncode}; printed "{s.stdout.strip()}"')
bad = bad or s.returncode != 0 or not s.stdout.startswith('200 ')
log('C3 stop: see above' if bad else 'C3 ok')
open(f'{out}/{name}.send.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(lines) + '\n')
sys.exit(1 if bad else 0)
