# Lab helper (row 1780-W, new; not product code; written whole by an editor): LAB-COMMON C2 d's MARKS read, sent by C3's line with
# row 2161-B's request file (marks.utf16le.xml, its sha256 checked first), read by C4 (decoded, only the characters XML forbids dropped,
# parsed as XML; rows are the COMPANY children of ENVELOPE/BODY/DATA/COLLECTION). Writes evidence\<tag>-marks.response and
# .headers.txt (refuses names that exist); prints curl's line, CONTENT-LENGTH, size, sha256 and each row's GUID, ALTVCHID and ALTMSTID.
# Exit 0 only when curl exit 0, status 200, size = CONTENT-LENGTH > 0, size <= 2,000,000, and exactly one row, THE COMPANY's.
# Usage: python -I -X utf8 marks.py <tag>
import hashlib, os, re, subprocess, sys, time
import xml.etree.ElementTree as ET
R = '<LAB_DIR>'; E = R + '\\evidence'
REQ = R + '\\marks.utf16le.xml'; WANT = '539ca283638c2013df5445da0035df47eb2d7a08a4ccdae8f9858e22ed358f8d'
GUID = 'a30f23c3-4819-4334-96ab-ffa6fba70976'; EXCLUDED = 'ae1490be-'
tag = sys.argv[1]
ans = f'{E}\\{tag}-marks.response'; hdr = f'{E}\\{tag}-marks.headers.txt'
if os.path.exists(ans) or os.path.exists(hdr): sys.exit(f'stop: {tag} answer exists; nothing sent')
if hashlib.sha256(open(REQ, 'rb').read()).hexdigest() != WANT: sys.exit('stop: the request differs from its sha256; nothing sent')
t0 = time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())
p = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download}', '--data-binary', '@' + REQ, '-H',
                    'Content-Type: text/xml; charset=utf-16', '-D', hdr, 'http://127.0.0.1:9001', '-o', ans], capture_output=True, text=True)
printed = (p.stdout + p.stderr).strip()
cl = re.findall(r'CONTENT-LENGTH:\s*(\d+)', open(hdr, encoding='latin-1').read(), re.I) if os.path.exists(hdr) else []
d = open(ans, 'rb').read() if os.path.exists(ans) else b''
print(f'{tag} MARKS sent {t0} curl exit {p.returncode} printed {printed} content_length {",".join(cl)} bytes {len(d)} sha256 {hashlib.sha256(d).hexdigest()}')
ok = p.returncode == 0 and printed.startswith('200 ') and cl and int(cl[-1]) == len(d) and 0 < len(d) <= 2000000
if not ok: print('stop: the request failed'); sys.exit(2)
t = d.decode('utf-16')
t = re.sub(r'&#(x[0-9a-fA-F]+|[0-9]+);', lambda m: '' if (int(m.group(1)[1:], 16) if m.group(1)[0] == 'x' else int(m.group(1))) in set(range(0, 9)) | {11, 12} | set(range(14, 32)) else m.group(0), t)
t = re.sub('[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t)
root = ET.fromstring(t)
rows = root.findall('./BODY/DATA/COLLECTION/COMPANY')
for r in rows:
    g = (r.findtext('GUID') or '').strip()
    print(f'  row NAME={r.get("NAME")!r} GUID {g} ALTVCHID {(r.findtext("ALTVCHID") or "").strip()} ALTMSTID {(r.findtext("ALTMSTID") or "").strip()}')
    if g.startswith(EXCLUDED): print('stop: the excluded book is loaded')
good = len(rows) == 1 and (rows[0].findtext('GUID') or '').strip() == GUID
print(f'rows {len(rows)}; THE COMPANY alone: {good}')
sys.exit(0 if good else 3)
