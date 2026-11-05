# Lab helper (1029-P, from 1381-M's send.py with the folder, bound and marks reading changed; not product code): one raw marks read
# by LAB-COMMON C3 and C4. Checks marks.utf16le.xml's sha256 against the sheet's SHA256.txt (a difference: stop, nothing sent), sends
# it with C3's curl line, saves the answer as answers/<label>-marks.utf16le.xml, appends to evidence/send-log.txt, and reads it as C4
# says: the COMPANY rows under ENVELOPE/BODY/DATA/COLLECTION; exactly one, with THE COMPANY's GUID, else stop. Bound 20,000 bytes (the
# sheet's Difference 2). Prints GUID shape, V (ALTVCHID) and M (ALTMSTID); with a second label, says whether the answer is
# byte-equal to that one's. Usage: python -I -X utf8 marks.py <label> [<compare-to label>]
import hashlib, os, re, subprocess, sys, xml.etree.ElementTree as ET
from datetime import datetime, timezone
R = '<LAB_DIR>'; Q = R + '/run/marks.utf16le.xml'; URL = 'http://127.0.0.1:9001'; BOUND = 20000
GUID = 'd45bc1b0-e5e3-4261-b3b2-cce3915f42d3'
label = sys.argv[1]; cmp = sys.argv[2] if len(sys.argv) > 2 else None
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
listed = {l.split('  ')[1].strip(): l.split('  ')[0] for l in open(R + '/run/SHA256.txt', encoding='utf-8') if l.strip() and not l.startswith('#')}
d = open(Q, 'rb').read(); hq = hashlib.sha256(d).hexdigest()
if listed.get('marks.utf16le.xml') != hq: raise SystemExit('stop: marks.utf16le.xml differs from SHA256.txt; nothing sent')
ans = f'{R}/answers/{label}-marks.utf16le.xml'; hdr = ans + '.headers.txt'
if os.path.exists(ans): raise SystemExit(f'stop: {ans} exists; nothing sent')
t0 = now()
r = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download}', '--data-binary', '@' + Q, '-H', 'Content-Type: text/xml; charset=utf-16', '-D', hdr, URL, '-o', ans], capture_output=True, text=True)
status, size = (r.stdout.strip().split() + ['', ''])[:2]
cl = re.findall(r'(?im)^content-length:\s*(\d+)', open(hdr, encoding='latin-1').read()) if os.path.exists(hdr) else []
b = open(ans, 'rb').read() if os.path.exists(ans) else b''
line = f'{t0} {label}-marks.utf16le.xml request sha256 {hq} curl exit {r.returncode} status {status} CONTENT-LENGTH {cl[-1] if cl else "none"} answer {len(b)} B sha256 {hashlib.sha256(b).hexdigest()}'
open(R + '/evidence/send-log.txt', 'a', encoding='utf-8', newline='\n').write(line + '\n'); print(line)
if r.returncode or status != '200' or not cl or int(cl[-1]) != len(b) or str(len(b)) != size or not b: raise SystemExit('stop: the request failed')
if len(b) > BOUND: raise SystemExit(f'stop: answer {len(b)} B above its bound {BOUND}; kept, not used')
t = b.decode('utf-16-le').lstrip('\ufeff')
t = re.sub(r'&#(?:x0*([0-8bcBCeEfF]|1[0-9a-fA-F])|0*([0-8]|1[124-9]|2[0-9]|3[01]));', '', t)
t = ''.join(ch for ch in t if ord(ch) >= 32 or ch in '\t\n\r')
try: root = ET.fromstring(t.strip().replace('\r', '&#13;'))
except ET.ParseError as e: raise SystemExit(f'stop: answer does not parse ({e})')
rows = root.findall('./BODY/DATA/COLLECTION/COMPANY')
mine = [c for c in rows if (c.findtext('GUID') or '').strip() == GUID]
if len(rows) != 1 or len(mine) != 1: raise SystemExit(f'stop: {len(rows)} COMPANY row(s), {len(mine)} with THE COMPANY\'s GUID')
v, m = (mine[0].findtext('ALTVCHID') or '').strip(), (mine[0].findtext('ALTMSTID') or '').strip()
eq = ''
if cmp: eq = f'; byte-equal to {cmp}: {open(f"{R}/answers/{cmp}-marks.utf16le.xml", "rb").read() == b}'
out = f'{t0} {label}: one COMPANY row, GUID {GUID[:8]}... (THE COMPANY\'s), V {v}, M {m}, {len(b)} B{eq}'
open(R + '/evidence/marks.txt', 'a', encoding='utf-8', newline='\n').write(out + '\n'); print(out)
