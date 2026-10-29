# Lab helper (#1233 stock census slice, not product code). Sends the sheet's request files one at a time, each exactly as it is
# (sha256 checked against SHA256.txt first), by curl under C3 with the sheet's read bound as --max-filesize, the answer and its
# headers written with -o and -D; logs UTC times, curl's fields, CONTENT-LENGTH, bytes and sha256; then a capture of Tally's whole
# window and GET /status. Stops at the first failure. Usage: python -I send.py <label>=<request stem>:<bound> ...
import hashlib, os, re, subprocess, sys
from datetime import datetime, timezone
W = '<lab folder>'; S = W + '/sheet/lab/1233-stock-census-slice'
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
def sha(p): return hashlib.sha256(open(p, 'rb').read()).hexdigest()
want = {}
for l in open(S + '/SHA256.txt', encoding='utf-8'):
    p = l.split()
    if len(p) >= 3 and not l.startswith('#'): want[p[2]] = p[0]
hwnd = subprocess.run(['powershell', '-NoProfile', '-Command', "(Get-Process tally | Where-Object { $_.MainWindowTitle -like 'TallyPrime:9001*' } | Select-Object -First 1).MainWindowHandle"], capture_output=True, text=True).stdout.strip()
def log(s):
    with open(W + '/evidence/send-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
for arg in sys.argv[1:]:
    label, rest = arg.split('='); stem, bound = rest.split(':')
    rel = f'requests/{stem}.utf16le.xml'; req = S + '/' + rel
    if sha(req) != want.get(rel): log(f'stop: {rel} sha256 differs from SHA256.txt; nothing sent'); sys.exit(3)
    ans = f'{W}/answers/{label}.xml'; hdr = f'{W}/answers/{label}.headers.txt'
    if os.path.exists(ans): log(f'stop: {label} answer exists'); sys.exit(3)
    t0 = now()
    r = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download} %{time_total}', '--max-filesize', bound, '--data-binary', '@' + req,
                        '-H', 'Content-Type: text/xml; charset=utf-16', '-D', hdr, 'http://127.0.0.1:9001', '-o', ans], capture_output=True, text=True)
    t1 = now(); cl = ''.join(re.findall(r'CONTENT-LENGTH:\s*(\d+)', open(hdr, encoding='latin-1').read(), re.I)) if os.path.exists(hdr) else ''
    size = os.path.getsize(ans) if os.path.exists(ans) else -1
    log(f'{label} ({stem}) bound {bound} sent {t0} answered {t1} curl_exit {r.returncode} printed "{r.stdout}" content_length {cl} bytes {size} sha256 {sha(ans) if size >= 0 else "none"}' + (f' stderr "{r.stderr.strip()}"' if r.stderr.strip() else ''))
    subprocess.run(['powershell', '-NoProfile', '-File', W + '/capwin.ps1', '-Handle', hwnd, '-Out', f'{W}/shots/{label}-after.png'], capture_output=True)
    st = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download}', '-o', f'{W}/answers/{label}-status.body', 'http://127.0.0.1:9001/status'], capture_output=True, text=True)
    log(f'{label} capture shots/{label}-after.png; GET /status exit {st.returncode} printed {st.stdout}')
    if r.returncode != 0 or not r.stdout.startswith('200 ') or str(size) != cl or size <= 0: log(f'stop: {label} failed (C3)'); sys.exit(2)
