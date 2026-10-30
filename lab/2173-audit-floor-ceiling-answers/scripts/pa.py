# Lab helper (#2173-L Phase A, not product code): ONE raw request by LAB-COMMON C3's line. The file's sha256 and bytes are
# checked against the sheet's SHA256.txt first (a difference = stop, nothing sent). A file named under 'sent once' is never
# sent twice (evidence/sent-once.txt). The answer and headers are kept under answers\, curl's exit code, printed status and
# bytes, CONTENT-LENGTH, the answer's sha256 and the seconds are logged; the sheet's READ BOUND applies (marks 16,384 B,
# imports 65,536 B). The answer is parsed (UTF-16LE, &#4; removed): for marks the COMPANY rows with NAME, GUID, ALTMSTID,
# ALTVCHID, COMPANYNUMBER; for imports every counter (absent recorded as absent) and LINEERROR.
# Usage: python -I -X utf8 pa.py marks <label> | python -I -X utf8 pa.py import <label> <request file name>
import hashlib, os, re, subprocess, sys, time, xml.etree.ElementTree as ET
from datetime import datetime, timezone
W = '<LAB_DIR>'; S = W + '/sheet'
os.makedirs(W + '/answers', exist_ok=True); os.makedirs(W + '/evidence', exist_ok=True)
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
def sha(b): return hashlib.sha256(b).hexdigest()
def log(s):
    with open(W + '/evidence/phase-a-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
def stop(m): log('stop rule: ' + m); sys.exit(2)
want = {}
for l in open(S + '/SHA256.txt', encoding='utf-8'):
    p = l.split()
    if len(p) >= 3: want[p[-1]] = (p[0], int(p[1]))
kind, label = sys.argv[1], sys.argv[2]
rel = 'requests/m00-marks.utf16le.xml' if kind == 'marks' else 'requests/' + sys.argv[3]
bound = 16384 if kind == 'marks' else 65536
req = S + '/' + rel; rb = open(req, 'rb').read()
if want.get(rel) != (sha(rb), len(rb)): stop(f'{label}: {rel} differs from SHA256.txt; nothing sent')
once = W + '/evidence/sent-once.txt'
if kind == 'import':
    sent = open(once, encoding='utf-8').read().split() if os.path.exists(once) else []
    if rel in sent: stop(f'{label}: {rel} was already sent once; never again')
    with open(once, 'a', encoding='utf-8', newline='\n') as f: f.write(rel + '\n')
seq = len([f for f in os.listdir(W + '/answers') if f.endswith('.xml')])
ans = f'{W}/answers/{seq:03d}-{label}.xml'; hdr = ans[:-4] + '.headers.txt'
t0 = now(); m0 = time.monotonic()
r = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download}', '--data-binary', '@' + req, '-H', 'Content-Type: text/xml; charset=utf-16', '-D', hdr, 'http://127.0.0.1:9001', '-o', ans], capture_output=True, text=True)
secs = time.monotonic() - m0; body = open(ans, 'rb').read() if os.path.exists(ans) else b''
cl = None
if os.path.exists(hdr):
    for hl in open(hdr, encoding='latin-1'):
        if hl.upper().startswith('CONTENT-LENGTH:'): cl = int(hl.split(':', 1)[1])
log(f'{label} {rel} ({len(rb)} B, sha256 {sha(rb)}) sent {t0} answered {now()} {secs:.3f} s curl_exit {r.returncode} printed "{r.stdout.strip()}" CONTENT-LENGTH {cl} bytes {len(body)} sha256 {sha(body) if body else "none"}' + (f' stderr "{r.stderr.strip()}"' if r.stderr.strip() else ''))
if r.returncode != 0 or not r.stdout.startswith('200 ') or not body or cl != len(body): stop(f'{label}: the request failed')
if len(body) > bound: stop(f'{label}: the answer is above its {bound}-byte bound')
t = (body[2:] if body[:2] == b'\xff\xfe' else body).decode('utf-16-le'); n4 = t.count('&#4;')
root = ET.fromstring(t.replace('&#4;', ''))
if kind == 'marks':
    rows = root.findall('./BODY/DATA/COLLECTION/COMPANY')
    for c in rows:
        f = {k: (c.findtext(k) or '').strip() if c.find(k) is not None else 'absent' for k in ('GUID', 'ALTMSTID', 'ALTVCHID', 'COMPANYNUMBER')}
        log(f'{label}: COMPANY NAME="{c.get("NAME")}" ' + ' '.join(f'{k} {v}' for k, v in f.items()))
    log(f'{label}: {len(rows)} COMPANY row(s); &#4; {n4}')
else:
    cnt = {k: (root.findtext(f'.//{k}').strip() if root.find(f'.//{k}') is not None else 'absent') for k in ('CREATED', 'ALTERED', 'DELETED', 'COMBINED', 'IGNORED', 'ERRORS', 'CANCELLED', 'EXCEPTIONS', 'LASTVCHID', 'LASTMID')}
    le = [e.text for e in root.iter('LINEERROR')]
    log(f'{label}: counters {cnt}; LINEERROR {le}; &#4; {n4}')
