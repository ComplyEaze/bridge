# Lab helper (699-C16, copied from 738-A with its folder and request changed; not product code): one LAB-COMMON v3 C3 send through c3-send.ps1 (the exact curl line; it checks the
# request's sha256 against the sheet's SHA256.txt first, never overwrites an answer and enforces the read bound), then C4's
# reading of the answer: decoded as UTF-16LE, only the characters XML forbids dropped (raw or as references; 9, 10 and 13
# kept), parsed as XML. marks: the COMPANY rows under ENVELOPE/BODY/DATA/COLLECTION with GUID, ALTVCHID, ALTMSTID.
# import: every counter, any LINEERROR, LASTVCHID and LASTMID. Appends one line to evidence/send-log.txt (c3-send's) and
# evidence/reads.txt (this reading). Usage: python -I -X utf8 send699.py marks <request file name> <label>
import os, re, subprocess, sys, xml.etree.ElementTree as ET
from datetime import datetime, timezone
L = '<LAB_DIR>/'; kind, req, label = sys.argv[1:4]
bound = 16384 if kind == 'marks' else 65536
row = [l.rstrip('\n').split('  ', 2) for l in open(L + 'sheet/SHA256.txt', encoding='utf-8')]
sha = [h for h, n, p in row if p == 'requests/' + req]
if len(sha) != 1: sys.exit(f'HALT: {req} not in SHA256.txt')
ans = f'{label}.xml'
r = subprocess.run(['powershell', '-NoProfile', '-File', L + 'c3-send.ps1', '-Req', L + 'sheet/requests/' + req, '-Sha', sha[0],
                    '-AnsDir', L + 'reads', '-Ans', ans, '-Bound', str(bound), '-Log', L + 'evidence/send-log.txt'], capture_output=True, text=True)
print(r.stdout.strip(), r.stderr.strip())
if r.returncode != 0: sys.exit(f'stop: c3-send exit {r.returncode}')
a = open(L + 'reads/' + ans, 'rb').read()
t = (a[2:] if a[:2] == b'\xff\xfe' else a).decode('utf-16-le')
BAD = set(range(0, 9)) | {11, 12} | set(range(14, 32))
def ref(m):
    v = m.group(1); n = int(v[1:], 16) if v[0] in 'xX' else int(v)
    return '' if n in BAD else m.group(0)
t = re.sub(r'&#([xX][0-9a-fA-F]+|[0-9]+);', ref, t)
t = ''.join(ch for ch in t if ord(ch) not in BAD)
try: root = ET.fromstring(t)
except ET.ParseError as e: sys.exit(f'stop: the answer does not parse ({e})')
now = datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
if kind == 'marks':
    rows = root.findall('./BODY/DATA/COLLECTION/COMPANY')
    g = lambda c, tag: (c.findtext(tag) or '').strip()
    line = f'{now} {label}: COMPANY rows {len(rows)}: ' + '; '.join(f'GUID {g(c, "GUID")} ALTVCHID {g(c, "ALTVCHID")} ALTMSTID {g(c, "ALTMSTID")}' for c in rows)
else:
    names = ['CREATED', 'ALTERED', 'DELETED', 'COMBINED', 'IGNORED', 'ERRORS', 'CANCELLED', 'EXCEPTIONS', 'LASTVCHID', 'LASTMID']
    found = {n: [(e.text or '').strip() for e in root.iter(n)] for n in names}
    le = [(e.text or '').strip() for e in root.iter('LINEERROR')]
    line = f'{now} {label}: ' + ' '.join(f'{n}={",".join(v) if v else "ABSENT"}' for n, v in found.items()) + f' LINEERROR x{len(le)}' + (f' {le}' if le else '')
open(L + 'evidence/reads.txt', 'a', encoding='utf-8', newline='\n').write(line + '\n')
print(line)
