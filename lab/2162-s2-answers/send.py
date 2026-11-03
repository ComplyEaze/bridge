# Lab helper (2162-S2, S1's sender with this row's folder, not product code): one raw request by curl on 9001, the file as it is, content type text/xml; charset=utf-16 as LAB-COMMON C3 (bare text/xml drew "Unknown Request" on the first marks read); the answer saved whole and never resent. Usage: python -I send.py <label> <file under the row folder>
import hashlib, os, subprocess, sys
from datetime import datetime, timezone
L = '<lab folder>/'; label, f = sys.argv[1], sys.argv[2]
req = L + f; ans = L + f'reads/{label}.xml'
if os.path.exists(ans): sys.exit(f'{label} exists; never resent')
b = open(req, 'rb').read(); t0 = datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
r = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download} %{time_total}', '--data-binary', '@' + req, '-H', 'Content-Type: text/xml; charset=utf-16', '-D', ans[:-4] + '.headers.txt', 'http://127.0.0.1:9001', '-o', ans], capture_output=True, text=True)
a = open(ans, 'rb').read() if os.path.exists(ans) else b''
line = f'{t0} {label} file {f} req {hashlib.sha256(b).hexdigest()} exit {r.returncode} printed {r.stdout} {r.stderr.strip()} bytes {len(a)} sha256 {hashlib.sha256(a).hexdigest()}'
open(L + 'evidence/send-log.txt', 'a', encoding='utf-8', newline='\n').write(line + '\n'); print(line)
