# Lab helper (2377-L Part A, not product code): one C3 send. Checks the request file's sha256 against the one given
# (the one l2377.py printed) before sending, refuses if the answer file already exists (a request is never sent again),
# runs LAB-COMMON C3's curl line exactly, and appends one line to sendlog.txt: UTC start and end, label, request,
# its bytes and sha256, curl exit, status, bytes, CONTENT-LENGTH, the answer's sha256, and a verdict.
# Usage: python -I send2377.py <label> <request file> <sha256>
import datetime, hashlib, os, re, subprocess, sys
R = '<LAB_DIR>'; ANS = R + '/ans'
label, req, want = sys.argv[1], sys.argv[2], sys.argv[3]
def now(): return datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%fZ')[:-4] + 'Z'
def log(s): open(R + '/sendlog.txt', 'a', encoding='utf-8', newline='\n').write(s + '\n'); print(s)
d = open(req, 'rb').read(); h = hashlib.sha256(d).hexdigest()
if h != want: log(f'{now()} {label} {os.path.basename(req)} {len(d)} B {h}: NOT the printed sha256 {want}: stop, nothing sent'); sys.exit(3)
out = f'{ANS}/{label}.answer.xml'; hdr = f'{ANS}/{label}.headers.txt'
if os.path.exists(out) or os.path.exists(hdr): log(f'{now()} {label}: answer file exists: never sent again'); sys.exit(4)
t0 = now()
p = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download}', '--data-binary', '@' + req, '-H', 'Content-Type: text/xml; charset=utf-16',
                    '-D', hdr, 'http://127.0.0.1:9001', '-o', out], capture_output=True, text=True)
t1 = now()
st = p.stdout.strip().split(); status, size = (st + ['?', '?'])[:2]
cl = None
if os.path.exists(hdr):
    m = re.search(r'(?im)^content-length:\s*(\d+)', open(hdr, encoding='latin-1').read()); cl = m.group(1) if m else None
a = open(out, 'rb').read() if os.path.exists(out) else b''
ok = p.returncode == 0 and status == '200' and size == str(len(a)) and cl == str(len(a)) and len(a) > 0 and len(a) <= 2_000_000
v = 'ok' if ok else ('the request failed' if len(a) <= 2_000_000 else 'above the 2,000,000 B bound: stop before it is used')
log(f'{t0}\t{t1}\t{label}\t{os.path.basename(req)} {len(d)} B {h}\tcurl {p.returncode}\tstatus {status}\tbytes {size}\tCONTENT-LENGTH {cl}\tanswer {len(a)} B {hashlib.sha256(a).hexdigest()}\t{v}' + (f'\tstderr {p.stderr.strip()}' if p.stderr.strip() else ''))
sys.exit(0 if ok else 2)
