# Lab helper (rows 1381-R1 and 2162-L2b, from 1381-M's send.py; not product code): LAB-COMMON C3 for one request at a
# time, to 127.0.0.1:9001. The kit is this script's own folder. The request's sha256 and size must equal its line in the
# kit's SHA256.txt, or, for a file the row generates on the lab PC (make_b.py's b1/b2 creates), its line in
# generated-sha.txt, written when it was made. Any difference: stop, nothing sent. A write (a file named in WRITES below)
# is sent once in the row's life: if an answer to it exists, stop. Sends with C3's curl line (no client timeout, headers
# kept) and records curl's first-byte and total seconds. Saves the answer as answers/a-<label>.xml and its headers beside
# it. Appends one line to evidence/send-log.txt. Any C3 miss (curl exit, status, CONTENT-LENGTH, empty answer): stop.
# Usage: python -I -X utf8 send.py <label> <request file, relative to the kit>
import hashlib, os, re, subprocess, sys
from datetime import datetime, timezone
R = os.path.dirname(os.path.abspath(__file__)).replace(os.sep, '/'); A = R + '/answers'; URL = 'http://127.0.0.1:9001'
WRITES = ('requests/b1-create.utf16le.xml', 'requests/b2-create.utf16le.xml', 'c2-alter.xml')
label, f = sys.argv[1], sys.argv[2].replace(os.sep, '/')
os.makedirs(A, exist_ok=True); os.makedirs(R + '/evidence', exist_ok=True)
def now(): t = datetime.now(timezone.utc); return t.strftime('%Y-%m-%dT%H:%M:%S.') + f'{t.microsecond // 1000:03d}Z'
listed = {}
for name in ('SHA256.txt', 'generated-sha.txt'):
    p = f'{R}/{name}'
    if os.path.exists(p):
        for l in open(p, encoding='utf-8'):
            x = l.split()
            if len(x) >= 3: listed[x[2].lstrip('*')] = (x[0], int(x[1]))
d = open(f'{R}/{f}', 'rb').read(); hq = hashlib.sha256(d).hexdigest()
if listed.get(f) != (hq, len(d)): raise SystemExit(f'stop: {f} differs from its listed sha256 and size, or is not listed; nothing sent')
ans = f'{A}/a-{label}.xml'; hdr = ans + '.headers.txt'
if os.path.exists(ans): raise SystemExit(f'stop: {ans} already exists; nothing sent')
if f in WRITES:
    sentlog = open(R + '/evidence/send-log.txt', encoding='utf-8').read() if os.path.exists(R + '/evidence/send-log.txt') else ''
    if f' {f} ' in sentlog: raise SystemExit(f'stop: {f} is a write and has been sent already; nothing sent')
t0 = now()
r = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download} %{time_starttransfer} %{time_total}', '--data-binary', '@' + f'{R}/{f}',
                    '-H', 'Content-Type: text/xml; charset=utf-16', '-D', hdr, URL, '-o', ans], capture_output=True, text=True)
status, size, tfb, ttot = (r.stdout.strip().split() + ['', '', '', ''])[:4]
cl = re.findall(r'(?im)^content-length:\s*(\d+)', open(hdr, encoding='latin-1').read()) if os.path.exists(hdr) else []
b = open(ans, 'rb').read() if os.path.exists(ans) else b''
line = (f'{t0} {label} {f} request {len(d)} B sha256 {hq} | curl exit {r.returncode} status {status} CONTENT-LENGTH {cl[-1] if cl else "none"} '
        f'answer {len(b)} B sha256 {hashlib.sha256(b).hexdigest()} | first byte {tfb} s total {ttot} s')
open(R + '/evidence/send-log.txt', 'a', encoding='utf-8', newline='\n').write(line + '\n'); print(line)
if r.returncode or status != '200' or not cl or int(cl[-1]) != len(b) or str(len(b)) != size or not b:
    raise SystemExit(f'stop: the request failed ({r.stderr.strip()[:200]})')
