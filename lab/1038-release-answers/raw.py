# Lab helper (1038-R1, not product code): the sheet's marks.utf16le.xml sent raw to 9001 with charset=utf-16, each label
# once, answer and headers kept whole; prints the COMPANY rows' GUID, ALTVCHID and ALTMSTID.
# Usage: python -I -X utf8 raw.py <label>
import hashlib, os, re, subprocess, sys
from datetime import datetime, timezone
L = '<lab folder>/'; label = sys.argv[1]
for d in ('reads', 'evidence'): os.makedirs(L + d, exist_ok=True)
req = L + 'sheet/marks.utf16le.xml'; ans = f'{L}reads/{label}.xml'
if os.path.exists(ans): sys.exit(f'{label} exists; never resent')
rb = open(req, 'rb').read(); t0 = datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
r = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download} %{time_total}', '--data-binary', '@' + req, '-H', 'Content-Type: text/xml; charset=utf-16', '-D', ans[:-4] + '.headers.txt', 'http://127.0.0.1:9001', '-o', ans], capture_output=True, text=True)
a = open(ans, 'rb').read() if os.path.exists(ans) else b''
line = f'{t0} {label} req {hashlib.sha256(rb).hexdigest()} exit {r.returncode} printed {r.stdout} {r.stderr.strip()} bytes {len(a)} sha256 {hashlib.sha256(a).hexdigest()}'
open(L + 'evidence/send-log.txt', 'a', encoding='utf-8', newline='\n').write(line + '\n'); print(line)
t = a.decode('utf-16-le', 'replace').lstrip('\ufeff') if a[1:2] == b'\0' or a[:2] == b'\xff\xfe' else a.decode('utf-8', 'replace')
rows = re.findall(r'<COMPANY NAME="([^"]*)"[^>]*>(.*?)</COMPANY>', t, re.S)
print(len(rows), [(n, re.findall(r'<GUID[^>]*>\s*([^<]*)', b), 'V', re.findall(r'<ALTVCHID[^>]*>\s*([^<]*)', b), 'M', re.findall(r'<ALTMSTID[^>]*>\s*([^<]*)', b)) for n, b in rows])
