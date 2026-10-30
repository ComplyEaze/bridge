# Lab helper (#2017, not product code): ONE raw request by LAB-COMMON C3's line, the request's sha256 and bytes checked
# against SHA256.txt first (a difference = stop, nothing sent). Records curl's exit code, the printed status and bytes,
# CONTENT-LENGTH, the answer's size and sha256 (evidence\send-log.txt). A non-zero exit, a status other than 200, a
# size that differs from CONTENT-LENGTH or an answer above 2,000,000 B = stop. Rule 3 of the sheet: a 0-byte 200 answer,
# an empty envelope and an in-band refusal are results, recorded, not stops. Then a capture of the whole Tally window
# (every raw send is a NEW SHAPE) and, with a reference capture given, whether it equals it.
# Usage: python -I -X utf8 rawa.py <request path relative to sheet\, or a full path for a reqs-d candidate> [reference png]
import hashlib, os, re, subprocess, sys
from datetime import datetime, timezone
W = '<LAB_DIR>'; S = W + '/sheet'; E = W + '/evidence'
rel = sys.argv[1]; ref = sys.argv[2] if len(sys.argv) > 2 else None
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
def sha(b): return hashlib.sha256(b).hexdigest()
def log(s):
    with open(E + '/send-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
src = rel if ':' in rel else S + '/' + rel
rb = open(src, 'rb').read()
if ':' not in rel:
    want = {l.split()[2]: (l.split()[0], int(l.split()[1])) for l in open(S + '/SHA256.txt', encoding='utf-8') if len(l.split()) >= 3}
    if want.get(rel) != (sha(rb), len(rb)): log(f'stop: {rel} differs from SHA256.txt; nothing sent'); sys.exit(2)
label = os.path.basename(src).replace('.utf16le.xml', '').replace('.xml', '') + os.environ.get('RAWA_SUFFIX', '')  # step 12 re-sends: -r12
ans, hdr = f'{E}/{label}.answer.xml', f'{E}/{label}.headers.txt'
if os.path.exists(ans): log(f'stop: {ans} exists; nothing sent'); sys.exit(2)
t0 = now()
r = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download}', '--data-binary', '@' + src.replace('/', '\\'),
                    '-H', 'Content-Type: text/xml; charset=utf-16', '-D', hdr.replace('/', '\\'), 'http://127.0.0.1:9001', '-o', ans.replace('/', '\\')], capture_output=True, text=True)
body = open(ans, 'rb').read() if os.path.exists(ans) else b''
cl = None
if os.path.exists(hdr):
    for hl in open(hdr, encoding='latin-1'):
        if hl.upper().startswith('CONTENT-LENGTH:'): cl = int(hl.split(':', 1)[1])
pr = r.stdout.strip().split()
note = ''
if body:
    t = (body[2:] if body[:2] == b'\xff\xfe' else body).decode('utf-16-le', errors='replace')
    if re.fullmatch(r'\s*<ENVELOPE>\s*</ENVELOPE>\s*', t): note = 'empty envelope (a result)'
    m = re.search(r'<STATUS>\s*(-?\d+)\s*</STATUS>', t)
    if m and m.group(1) == '0': note = 'STATUS 0 (an in-band refusal, a result)'
else: note = '0-byte answer (a result)'
log(f'{t0} {label}: {rel} ({len(rb)} B, sha256 {sha(rb)}); answered {now()}; curl exit {r.returncode}; printed "{r.stdout.strip()}"; CONTENT-LENGTH {cl}; '
    f'bytes {len(body)}; sha256 {sha(body)}' + (f'; {note}' if note else '') + (f'; stderr "{r.stderr.strip()}"' if r.stderr.strip() else ''))
if r.returncode != 0 or not pr or pr[0] != '200' or (cl is not None and cl != len(body)): log(f'stop: {label}: the request failed'); sys.exit(2)
if len(body) > 2000000: log(f'stop: {label}: above the 2,000,000-byte bound'); sys.exit(2)
c = subprocess.run(['python', '-I', W + '/cap.py', label + '-after'], capture_output=True, text=True).stdout.strip()
same = ''
if ref:
    a = open(f'{W}/shots/{label}-after.png', 'rb').read(); b = open(ref, 'rb').read(); same = f'; equal to the reference capture: {a == b}'
log(f'  {c}{same}')
if ref and not same.endswith('True'): print('runner: the capture differs from the reference; look at it before the next send'); sys.exit(3)
