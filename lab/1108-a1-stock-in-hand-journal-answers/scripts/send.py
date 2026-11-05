# Lab helper (1108-A1, from 1381-M, from 692's send.py with the folder, bound and write rule changed; not product code): LAB-COMMON C3 for one
# request at a time. Checks the request's sha256 and size against the sheet's SHA256.txt (r2 against r2-sha.txt, kept on the lab
# PC; any difference: stop, nothing sent). Sends with C3's curl line and saves the answer as answers/<label>-<request file>; a write
# file (r*.utf16le.xml) is sent once in the row's life: if any answer to it exists, stop. Appends a line to evidence/send-log.txt.
# Bound: 200,000 bytes for every answer (the sheet's Difference 3). Any C3 miss, or an answer that does not parse (C4): stop.
# Usage: python -I -X utf8 send.py <label> <request file>
import glob, hashlib, os, re, subprocess, sys, xml.etree.ElementTree as ET
from datetime import datetime, timezone
R = '<LAB_DIR>'; Q = R + '/lab/1108-a1/requests'; A = R + '/answers'; URL = 'http://127.0.0.1:9001'; BOUND = 200000
label, f = sys.argv[1], sys.argv[2]
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
listed = {l.split('  ')[2].strip(): (l.split('  ')[0], int(l.split('  ')[1])) for l in open(R + '/lab/1108-a1/SHA256.txt', encoding='utf-8') if l.strip() and not l.startswith('#')}
if os.path.exists(R + '/r2-sha.txt'):
    h, s = open(R + '/r2-sha.txt').read().split()[:2]; listed['requests/r2-copied.utf16le.xml'] = (h, int(s))
q = f'{Q}/{f}'; d = open(q, 'rb').read(); hq = hashlib.sha256(d).hexdigest()
if listed.get('requests/' + f) != (hq, len(d)): raise SystemExit(f'stop: {f} differs from its listed sha256 and size; nothing sent')
if re.match(r'(w1|w2|a1|b)-', f) and glob.glob(f'{A}/*-{f}'): raise SystemExit(f'stop: {f} is a write and has been sent already; nothing sent')
ans = f'{A}/{label}-{f}'; hdr = f'{A}/{label}-{f}.headers.txt'
if os.path.exists(ans): raise SystemExit(f'stop: {ans} already exists; nothing sent')
t0 = now()
r = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download}', '--data-binary', '@' + q, '-H', 'Content-Type: text/xml; charset=utf-16', '-D', hdr, URL, '-o', ans], capture_output=True, text=True)
out = r.stdout.strip(); status, size = (out.split() + ['', ''])[:2]
cl = re.findall(r'(?im)^content-length:\s*(\d+)', open(hdr, encoding='latin-1').read()) if os.path.exists(hdr) else []
b = open(ans, 'rb').read() if os.path.exists(ans) else b''
line = f'{t0} {label}-{f} request sha256 {hq} curl exit {r.returncode} status {status} CONTENT-LENGTH {cl[-1] if cl else "none"} answer {len(b)} B sha256 {hashlib.sha256(b).hexdigest()}'
open(R + '/evidence/send-log.txt', 'a', encoding='utf-8', newline='\n').write(line + '\n'); print(line)
if r.returncode or status != '200' or not cl or int(cl[-1]) != len(b) or str(len(b)) != size or not b:
    raise SystemExit(f'stop: the request failed ({r.stderr.strip()[:200]})')
if len(b) > BOUND: raise SystemExit(f'stop: answer {len(b)} B above its bound {BOUND}; kept, not used')
t = b.decode('utf-16-le').lstrip('\ufeff')
t = re.sub(r'&#(?:x0*([0-8bcBCeEfF]|1[0-9a-fA-F])|0*([0-8]|1[124-9]|2[0-9]|3[01]));', '', t)
t = ''.join(ch for ch in t if ord(ch) >= 32 or ch in '\t\n\r')
try: ET.fromstring(t.strip().replace('\r', '&#13;'))
except ET.ParseError as e: raise SystemExit(f'stop: answer does not parse as XML ({e})')
print('  parses as XML (C4)')
