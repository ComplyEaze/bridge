# Lab helper (#1713 order rows, not product code): one read by LAB-COMMON C3's curl line (the request's sha256 checked
# against the row note's first), its answer written with -o, then read by the earlier sheet's read.py by structure.
# Usage: python -I c3.py <label> marks|ledgers [<probe name>]
import hashlib, os, re, subprocess, sys
from datetime import datetime, timezone
W = '<lab folder>'; E = W + '/earlier/lab/1713-abandoned-ledger-create'; GUID = '5f4825c4-5d89-47fa-9872-6397cca3a591'
REQ = {'marks': ('marks.xml', '0464f624a2f7205a4f03668b2f6da7703589fe7bd590b4d22f2ec50c3a32542b'),
       'ledgers': ('ledgers.xml', '3eb31c35d3b911421742005af98e5e87453fbfc7dbc0d812ac1856761a3f280b')}
label, kind = sys.argv[1], sys.argv[2]; probe = sys.argv[3] if len(sys.argv) > 3 else None
req = f'{E}/requests/{REQ[kind][0]}'
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
def log(s):
    with open(W + '/evidence/c3-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
if hashlib.sha256(open(req, 'rb').read()).hexdigest() != REQ[kind][1]: log(f'stop: {label} {kind} request sha256 differs; nothing sent'); sys.exit(3)
os.makedirs(W + '/answers', exist_ok=True)
ans = f'{W}/answers/{label}-{kind}.xml'; hdr = f'{W}/answers/{label}-{kind}.headers.txt'
if os.path.exists(ans): log(f'stop: {ans} exists'); sys.exit(3)
t0 = now()
r = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download}', '--max-filesize', '2000000', '--data-binary', '@' + req,
                    '-H', 'Content-Type: text/xml; charset=utf-16', '-D', hdr, 'http://127.0.0.1:9001', '-o', ans], capture_output=True, text=True)
cl = ''.join(re.findall(r'CONTENT-LENGTH:\s*(\d+)', open(hdr, encoding='latin-1').read(), re.I)) if os.path.exists(hdr) else ''
size = os.path.getsize(ans) if os.path.exists(ans) else -1
line = f'{label} {kind} sent {t0} answered {now()} curl_exit {r.returncode} printed "{r.stdout}" content_length {cl} bytes {size} sha256 {hashlib.sha256(open(ans, "rb").read()).hexdigest() if size >= 0 else "none"}'
if r.returncode != 0 or not r.stdout.startswith('200 ') or str(size) != cl or size <= 0: log(line + f' stderr "{r.stderr.strip()}"'); log(f'stop: {label} {kind} failed (C3)'); sys.exit(2)
args = [sys.executable, '-I', E + '/read.py', kind, ans, GUID] + ([probe] if probe else [])
rd = subprocess.run(args, capture_output=True, text=True)
log(line + f' ;; read.py exit {rd.returncode}: {rd.stdout.strip()} {rd.stderr.strip()}')
sys.exit(0 if rd.returncode == 0 else 4)
