# Lab helper (852-C1, the 2210 hand-import row's sender for this book; not product code): raw sends on 9001, each once, saved whole, with the census row's curl line.
# Each file's content type comes from its own BOM or encoding declaration (UTF-16LE BOM: charset=utf-16; UTF-8 declaration:
# charset=utf-8), checked before the send; the first batch send went with charset=utf-16 by hand and drew Unknown Request.
# Usage: python -I -X utf8 raw.py marks <label> | send <label> <file>
import hashlib, os, re, subprocess, sys
from datetime import datetime, timezone
L = '<lab folder>/'; C = 'BRIDGE LARGE LAB'
for d in ('reqs', 'reads', 'evidence'): os.makedirs(L + d, exist_ok=True)
def send(label, reqbytes, ctype='text/xml; charset=utf-16'):
    req = f'{L}reqs/{label}.xml'; ans = f'{L}reads/{label}.xml'
    if os.path.exists(ans): sys.exit(f'{label} exists; never resent')
    open(req, 'wb').write(reqbytes)
    t0 = datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
    r = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download} %{time_total}', '--data-binary', '@' + req, '-H', 'Content-Type: ' + ctype, '-D', ans[:-4] + '.headers.txt', 'http://127.0.0.1:9001', '-o', ans], capture_output=True, text=True)
    a = open(ans, 'rb').read() if os.path.exists(ans) else b''
    line = f'{t0} {label} req {hashlib.sha256(reqbytes).hexdigest()} bytes {len(reqbytes)} content-type "{ctype}" exit {r.returncode} printed {r.stdout} {r.stderr.strip()} sha256 {hashlib.sha256(a).hexdigest()}'
    open(L + 'evidence/send-log.txt', 'a', encoding='utf-8', newline='\n').write(line + '\n'); print(line)
    return a.decode('utf-16-le', 'replace') if a[1:2] == b'\0' or a[:2] == b'\xff\xfe' else a.decode('utf-8', 'replace')
kind = sys.argv[1]
if kind == 'marks':
    t = open('<lab root>/1331/sheet/requests/m-marks.xml', 'rb').read().decode('utf-16-le').lstrip(chr(0xfeff)).replace('BRIDGE PILOT LAB', C)
    s = send(sys.argv[2], b'\xff\xfe' + t.encode('utf-16-le'))
    rows = re.findall(r'<COMPANY NAME="([^"]*)"[^>]*>(.*?)</COMPANY>', s, re.S)
    print(len(rows), [(n, re.findall(r'<GUID[^>]*>\s*([^<]*)', b), 'V', re.findall(r'<ALTVCHID[^>]*>\s*([^<]*)', b), 'M', re.findall(r'<ALTMSTID[^>]*>\s*([^<]*)', b)) for n, b in rows])
elif kind == 'send':
    b = open(sys.argv[3], 'rb').read()
    ct = 'text/xml; charset=utf-16' if b[:2] == b'\xff\xfe' else ('text/xml; charset=utf-8' if b.lstrip()[:60].lower().startswith(b'<?xml') and b'encoding="utf-8"' in b[:80].lower() else None)
    if ct is None: sys.exit('no BOM and no UTF-8 declaration: nothing sent')
    print('encoding check:', 'UTF-16LE BOM' if b[:2] == b'\xff\xfe' else 'UTF-8 declaration', '->', ct)
    s = send(sys.argv[2], b, ct)
    print({k: re.findall(rf'<{k}>([^<]*)', s) for k in ('CREATED', 'ALTERED', 'DELETED', 'COMBINED', 'IGNORED', 'ERRORS', 'CANCELLED', 'EXCEPTIONS', 'LINEERROR', 'LASTVCHID')})
