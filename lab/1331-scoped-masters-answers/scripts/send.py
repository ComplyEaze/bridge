# Lab helper (#1331 scoped masters, not product code): ONE raw request under LAB-COMMON C3 with the sheet's -w format
# ("%{http_code} %{size_download} %{time_total}"), run from sheet\. Before the send: the file's sha256 and bytes against
# SHA256.txt (a difference = stop, nothing sent); an import is sent once in the item's life (evidence/sent-once.txt) and
# only right after a marks read holding exactly THE COMPANY's row (evidence/last-marks.txt, written by this helper).
# After it: curl exit 0, status 200, size = CONTENT-LENGTH, a body, the read bound; then C4: forbidden characters dropped
# for parsing only, parsed as XML, rows = children of ENVELOPE/BODY/DATA/COLLECTION. The table (evidence/<label>.table.txt)
# lists each row's tag, NAME (repr, with its length), GUID, PARENT and ISBILLWISEON, and the count per tag; marks: the
# COMPANY rows; an import: every counter and LINEERROR. A line per send goes to evidence/send-log.txt.
# Usage: python -I -X utf8 send.py <label> <request path under requests/> <bound> [capture-after]
import hashlib, json, os, re, subprocess, sys, xml.etree.ElementTree as ET
from datetime import datetime, timezone
W = '<LAB_DIR>'; S = W + '/sheet'; E = W + '/evidence'
GUID = '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
label, rel, bound = sys.argv[1], 'requests/' + sys.argv[2], int(sys.argv[3])
capture = len(sys.argv) > 4 and sys.argv[4] == 'capture-after'
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
def sha(b): return hashlib.sha256(b).hexdigest()
def log(s):
    with open(E + '/send-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
def stop(m): log('stop: ' + m); sys.exit(2)
want = {}
for l in open(S + '/SHA256.txt', encoding='utf-8'):
    p = l.split(); m = re.search(r'\((\d+) bytes\)', l)
    if len(p) >= 2: want[p[1]] = (p[0], int(m.group(1)) if m else None)
rb = open(S + '/' + rel, 'rb').read()
w = want.get(rel)
if not w or w[0] != sha(rb) or (w[1] is not None and w[1] != len(rb)): stop(f'{label}: {rel} differs from SHA256.txt; nothing sent')
is_import = rel.startswith('requests/imports/')
ans, hdr = f'{E}/{label}.xml', f'{E}/{label}.headers.txt'
if os.path.exists(ans): stop(f'{label}: {ans} exists; nothing sent')
if is_import:
    once = E + '/sent-once.txt'
    sent = open(once, encoding='utf-8').read().split() if os.path.exists(once) else []
    if rel in sent: stop(f'{label}: {rel} was already sent once; never again')
    lm = open(E + '/last-marks.txt', encoding='utf-8').read().split() if os.path.exists(E + '/last-marks.txt') else []
    if lm != ['ok', GUID]: stop(f'{label}: the marks read just before is not exactly THE COMPANY\'s row ({lm}); nothing sent')
    with open(once, 'a', encoding='utf-8', newline='\n') as f: f.write(rel + '\n')
if os.path.exists(E + '/last-marks.txt'): os.remove(E + '/last-marks.txt')
t0 = now()
r = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download} %{time_total}', '--data-binary', '@' + rel.replace('/', '\\'),
                    '-H', 'Content-Type: text/xml; charset=utf-16', '-D', hdr.replace('/', '\\'), 'http://127.0.0.1:9001', '-o', ans.replace('/', '\\')],
                   cwd=S, capture_output=True, text=True)
body = open(ans, 'rb').read() if os.path.exists(ans) else b''
cl = None
if os.path.exists(hdr):
    for hl in open(hdr, encoding='latin-1'):
        if hl.upper().startswith('CONTENT-LENGTH:'): cl = int(hl.split(':', 1)[1])
pr = r.stdout.strip().split()
secs = pr[2] if len(pr) > 2 else None
head = (f'{t0} {label}: {rel} ({len(rb)} B, sha256 {sha(rb)}); curl exit {r.returncode}; printed "{r.stdout.strip()}"; CONTENT-LENGTH {cl}; '
        f'bytes {len(body)}; sha256 {sha(body) if body else "none"}; seconds {secs}; bound {bound}' + (f'; stderr "{r.stderr.strip()}"' if r.stderr.strip() else ''))
if r.returncode != 0 or not pr or pr[0] != '200' or not body or cl != len(body): log(head); stop(f'{label}: the request failed')
if len(body) > bound: log(head); stop(f'{label}: {len(body)} B is above its {bound}-byte bound; kept, not used')
t = (body[2:] if body[:2] == b'\xff\xfe' else body).decode('utf-16-le')
t = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t)
t = re.sub(r'&#(x[0-9a-fA-F]+|[0-9]+);', lambda m: '' if (int(m.group(1)[1:], 16) if m.group(1)[0] == 'x' else int(m.group(1))) in set(range(0, 9)) | {11, 12} | set(range(14, 32)) else m.group(0), t)
try: root = ET.fromstring(t)
except ET.ParseError as e: log(head); stop(f'{label}: the answer does not parse ({e})')
out = [head]
le = [x.text for x in root.iter('LINEERROR')]
coll = root.find('./BODY/DATA/COLLECTION')
rows = list(coll) if coll is not None else []
counts = {}
for x in rows: counts[x.tag] = counts.get(x.tag, 0) + 1
def txt(e, k):
    c = e.find(k)
    return 'absent' if c is None else (c.text or '')
summary = {'rows': counts, 'lineerror': le, 'collection': coll is not None}
if is_import:
    keys = ('CREATED', 'ALTERED', 'DELETED', 'COMBINED', 'IGNORED', 'ERRORS', 'CANCELLED', 'EXCEPTIONS', 'LASTVCHID', 'LASTMID')
    cnt = {k: (root.findtext(f'.//{k}').strip() if root.find(f'.//{k}') is not None else 'absent') for k in keys}
    summary['counters'] = cnt
    out.append('counters ' + json.dumps(cnt))
    ok = cnt['CREATED'] == '500' and all(cnt[k] == '0' for k in keys[1:8]) and not le
for x in rows:
    nm = x.get('NAME') if x.get('NAME') is not None else txt(x, 'NAME')
    if x.tag == 'COMPANY':
        out.append(f'COMPANY NAME={nm!r} ' + ' '.join(f'{c.tag} {(c.text or "").strip()}' for c in x))
    else:
        out.append(f'{x.tag} NAME={nm!r} (len {len(nm) if nm not in (None, "absent") else "-"}) GUID {txt(x, "GUID").strip()} PARENT {txt(x, "PARENT")!r} ISBILLWISEON {txt(x, "ISBILLWISEON").strip()}')
out.append(f'rows per tag {json.dumps(counts)}; LINEERROR {le}')
open(f'{E}/{label}.table.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
co = [x for x in rows if x.tag == 'COMPANY']
if 'marks' in rel:
    summary['marks'] = [(txt(x, 'GUID').strip(), txt(x, 'ALTMSTID').strip(), txt(x, 'ALTVCHID').strip()) for x in co]
    if len(co) == 1 and txt(co[0], 'GUID').strip() == GUID: open(E + '/last-marks.txt', 'w').write('ok ' + GUID)
    else: log(head + ' ' + json.dumps(summary)); stop(f'{label}: the marks answer does not hold exactly THE COMPANY\'s row')
log(head + '; ' + json.dumps(summary, ensure_ascii=False))
if capture:
    c = subprocess.run(['python', '-I', W + '/cap.py', label], capture_output=True, text=True); log('  ' + c.stdout.strip())
if le: stop(f'{label}: LINEERROR {le}')
if is_import and not ok: stop(f'{label}: import counters are not CREATED 500 with every other counter 0')
