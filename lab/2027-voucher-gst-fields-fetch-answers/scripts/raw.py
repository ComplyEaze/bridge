# Lab helper (#2027 step 2, not product code): ONE raw read by LAB-COMMON C3's curl line. The request file's sha256 and
# bytes are checked against the sheet's SHA256.txt first (a difference = stop, nothing sent). The answer and its headers are
# kept byte for byte under raw\; curl's exit code, printed status and bytes, CONTENT-LENGTH, the answer's sha256 and the
# time taken are logged. A control (or the repeat) above 2,000,000 bytes = stop; a candidate above it is kept, not parsed
# (sheet rule 8). Then the VOUCHER rows that are direct children of ENVELOPE/BODY/DATA/COLLECTION are counted with their
# DATE, VOUCHERTYPENAME and VOUCHERNUMBER (an XML parser; &#4; references removed and counted).
# Usage: python -I -X utf8 raw.py <label> <request path relative to the sheet> control|candidate
import hashlib, os, re, subprocess, sys, time, xml.etree.ElementTree as ET
from datetime import datetime, timezone
W = '<LAB_DIR>'; S = W + '/sheet'; BOUND = 2000000
label, rel, kind = sys.argv[1], sys.argv[2], sys.argv[3]
os.makedirs(W + '/raw', exist_ok=True); os.makedirs(W + '/evidence', exist_ok=True)
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
def sha(b): return hashlib.sha256(b).hexdigest()
def log(s):
    with open(W + '/evidence/send-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
want = {}
for l in open(S + '/SHA256.txt', encoding='utf-8'):
    p = l.split()
    if len(p) >= 3: want[p[2]] = (p[0], int(p[1]))
req = f'{S}/{rel}'; rb = open(req, 'rb').read()
if want.get(rel) != (sha(rb), len(rb)): log(f'stop: {label}: {rel} differs from SHA256.txt; nothing sent'); sys.exit(2)
seq = len([f for f in os.listdir(W + '/raw') if f.endswith('.xml')])
ans = f'{W}/raw/{seq:03d}-{label}.xml'; hdr = f'{W}/raw/{seq:03d}-{label}.headers.txt'
t0 = now(); m0 = time.monotonic()
r = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download}', '--data-binary', '@' + req,
                    '-H', 'Content-Type: text/xml; charset=utf-16', '-D', hdr, 'http://127.0.0.1:9001', '-o', ans], capture_output=True, text=True)
secs = time.monotonic() - m0
body = open(ans, 'rb').read() if os.path.exists(ans) else b''
cl = None
if os.path.exists(hdr):
    for hl in open(hdr, encoding='latin-1'):
        if hl.upper().startswith('CONTENT-LENGTH:'): cl = int(hl.split(':', 1)[1])
log(f'{label} {rel} ({len(rb)} B, sha256 {sha(rb)}) sent {t0} answered {now()} {secs:.3f} s curl_exit {r.returncode} printed "{r.stdout.strip()}" CONTENT-LENGTH {cl} bytes {len(body)} sha256 {sha(body) if body else "none"}' + (f' stderr "{r.stderr.strip()}"' if r.stderr.strip() else ''))
if r.returncode != 0 or not r.stdout.startswith('200 ') or not body or cl != len(body): log(f'stop: {label}: the request failed'); sys.exit(2)
if len(body) > BOUND:
    if kind == 'candidate': log(f'{label}: above the bound ({len(body)} B): kept, not parsed (rule 8)'); sys.exit(0)
    log(f'stop: {label}: the answer is above the 2,000,000-byte bound'); sys.exit(2)
bom = body[:2] == b'\xff\xfe'; t = (body[2:] if bom else body).decode('utf-16-le')
n4 = t.count('&#4;'); root = ET.fromstring(t.replace('&#4;', ''))
rows = root.findall('./BODY/DATA/COLLECTION/VOUCHER')
crlf = t.count('\r\n'); lf = t.count('\n')
desc = '; '.join(f'{(v.findtext("DATE") or "").strip()} {(v.findtext("VOUCHERTYPENAME") or "").strip()} {(v.findtext("VOUCHERNUMBER") or "").strip()}' for v in rows)
status = root.findtext('./HEADER/STATUS'); le = [e.text for e in root.iter('LINEERROR')]
log(f'{label}: BOM {bom}; CRLF {crlf} of {lf} line ends; &#4; {n4}; HEADER/STATUS {status}; LINEERROR {le}; VOUCHER rows {len(rows)}: {desc}')
