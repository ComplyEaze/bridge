# Lab helper (1029-T, copied from 1029-R with its folder changed; not product code): the sheet's MARKS: sends the committed marks.utf16le.xml (checked against the
# sheet's SHA256.txt first) raw to 9001, never through the relay, with Content-Type text/xml; charset=utf-16; saves the
# answer whole as reads/<label>.xml (never resent: an existing answer is a stop) with its headers; logs the send to
# evidence/send-log.txt; reads the COMPANY rows by structure and prints GUID, V (ALTVCHID) and M (ALTMSTID); appends a
# line to evidence/marks.txt. Usage: python -I -X utf8 marks.py <label>
import hashlib, os, re, subprocess, sys, xml.etree.ElementTree as ET
from datetime import datetime, timezone
L = '<LAB_DIR>/'; label = sys.argv[1]
req = L + 'sheet/marks.utf16le.xml'; rb = open(req, 'rb').read(); rs = hashlib.sha256(rb).hexdigest()
row = [l for l in open(L + 'sheet/SHA256.txt', encoding='utf-8') if l.rstrip().endswith('marks.utf16le.xml')]
if len(row) != 1 or row[0].split()[0] != rs: sys.exit('HALT: marks.utf16le.xml does not match SHA256.txt')
ans = f'{L}reads/{label}.xml'
if os.path.exists(ans): sys.exit(f'HALT: {label} exists; never resent')
t0 = datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
r = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download} %{time_total}', '--data-binary', '@' + req,
                    '-H', 'Content-Type: text/xml; charset=utf-16', '-D', ans[:-4] + '.headers.txt', 'http://127.0.0.1:9001', '-o', ans], capture_output=True, text=True)
a = open(ans, 'rb').read() if os.path.exists(ans) else b''
sha = hashlib.sha256(a).hexdigest()
t = (a[2:] if a[:2] == b'\xff\xfe' else a).decode('utf-16-le', 'replace')
t = re.sub('[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t)
rows = []
try:
    root = ET.fromstring(t.encode('utf-8'))
    for c in root.findall('./BODY/DATA/COLLECTION/COMPANY'):
        g = lambda tag: (c.findtext('.//' + tag) or '').strip()
        rows.append((g('GUID'), g('ALTVCHID'), g('ALTMSTID')))
    parsed = 'parsed'
except ET.ParseError as e:
    parsed = f'NOT PARSED ({e})'
line = f'{t0} {label}: req sha256 {rs[:16]}... curl exit {r.returncode} printed "{r.stdout}" {r.stderr.strip()} answer {len(a)} B sha256 {sha}; {parsed}; COMPANY rows {len(rows)}: ' + '; '.join(f'GUID {g} V {v} M {m}' for g, v, m in rows)
for f in ('send-log.txt', 'marks.txt'):
    open(L + 'evidence/' + f, 'a', encoding='utf-8', newline='\n').write(line + '\n')
print(line)
