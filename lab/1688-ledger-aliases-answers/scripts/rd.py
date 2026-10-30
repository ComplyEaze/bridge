# Lab helper (#1688, not product code): ONE raw request by LAB-COMMON C3's line, run from sheet\ (requests\...) or the
# creates folder. Before the send: the file's sha256 and bytes against SHA256.txt (a difference = stop, nothing sent); a
# create is sent once in the item's life (evidence\sent-once.txt) and only right after a MARKS read holding exactly
# THE COMPANY's row with the expected ALTMSTID (evidence\last-marks.txt, written by this helper). After it: curl exit 0,
# status 200, size = CONTENT-LENGTH, a body, the 2,000,000-byte bound; then C4: forbidden characters dropped for parsing
# only, parsed as XML. Tables (evidence\<label>.table.txt): MARKS the COMPANY rows; CAT/CV2/IDX per LEDGER row the NAME
# attribute, GUID, PARENT, ISBILLWISEON, ALTERID, the count of LANGUAGENAME.LIST blocks, and every NAME under every
# NAME.LIST of every LANGUAGENAME.LIST in document order with its length and code points; a create every counter,
# LINEERROR, LASTMID and LASTVCHID. Every GUID must begin with THE COMPANY's.
# Usage: python -I -X utf8 rd.py <label> <request rel path, e.g. requests/marks.utf16le.xml> [expected ALTMSTID]
import hashlib, json, os, re, subprocess, sys, xml.etree.ElementTree as ET
from datetime import datetime, timezone
W = '<LAB_DIR>'; S = W + '/sheet'; E = W + '/evidence'
GUID = '021bda0c-b1b1-44c8-8279-5212ce87c9c7'
label, rel = sys.argv[1], sys.argv[2]
expect_mst = sys.argv[3] if len(sys.argv) > 3 else None
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
def sha(b): return hashlib.sha256(b).hexdigest()
def log(s):
    with open(E + '/send-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
def stop(m): log('stop: ' + m); sys.exit(2)
want = {}
for l in open(S + '/SHA256.txt', encoding='utf-8'):
    p = l.split()
    if len(p) >= 3: want[p[2]] = (p[0], int(p[1]))
is_create = rel.startswith('requests/creates/')
src = (W + '/creates/' + os.path.basename(rel)) if is_create else (S + '/' + rel)
rb = open(src, 'rb').read()
if want.get(rel) != (sha(rb), len(rb)): stop(f'{label}: {rel} differs from SHA256.txt; nothing sent')
ans, hdr = f'{E}/{label}.xml', f'{E}/{label}.headers.txt'
if os.path.exists(ans): stop(f'{label}: {ans} exists; nothing sent')
if is_create:
    once = E + '/sent-once.txt'
    sent = open(once, encoding='utf-8').read().split() if os.path.exists(once) else []
    if rel in sent: stop(f'{label}: {rel} was already sent once; never again')
    lm = open(E + '/last-marks.txt', encoding='utf-8').read().split() if os.path.exists(E + '/last-marks.txt') else []
    if len(lm) != 3 or lm[0] != 'ok' or lm[1] != GUID or (expect_mst and lm[2] != expect_mst):
        stop(f'{label}: the MARKS read just before is not exactly THE COMPANY with ALTMSTID {expect_mst} ({lm}); nothing sent')
    with open(once, 'a', encoding='utf-8', newline='\n') as f: f.write(rel + '\n')
if os.path.exists(E + '/last-marks.txt'): os.remove(E + '/last-marks.txt')
t0 = now()
r = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download}', '--data-binary', '@' + src.replace('/', '\\'),
                    '-H', 'Content-Type: text/xml; charset=utf-16', '-D', hdr.replace('/', '\\'), 'http://127.0.0.1:9001', '-o', ans.replace('/', '\\')],
                   capture_output=True, text=True)
body = open(ans, 'rb').read() if os.path.exists(ans) else b''
cl = None
if os.path.exists(hdr):
    for hl in open(hdr, encoding='latin-1'):
        if hl.upper().startswith('CONTENT-LENGTH:'): cl = int(hl.split(':', 1)[1])
pr = r.stdout.strip().split()
head = (f'{t0} {label}: {rel} ({len(rb)} B, sha256 {sha(rb)}); answered {now()}; curl exit {r.returncode}; printed "{r.stdout.strip()}"; '
        f'CONTENT-LENGTH {cl}; bytes {len(body)}; sha256 {sha(body) if body else "none"}' + (f'; stderr "{r.stderr.strip()}"' if r.stderr.strip() else ''))
if r.returncode != 0 or not pr or pr[0] != '200' or not body or cl != len(body): log(head); stop(f'{label}: the request failed')
if len(body) > 2000000: log(head); stop(f'{label}: above its 2,000,000-byte bound; kept, not used')
t = (body[2:] if body[:2] == b'\xff\xfe' else body).decode('utf-16-le')
t = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t)
t = re.sub(r'&#(x[0-9a-fA-F]+|[0-9]+);', lambda m: '' if (int(m.group(1)[1:], 16) if m.group(1)[0] == 'x' else int(m.group(1))) in set(range(0, 9)) | {11, 12} | set(range(14, 32)) else m.group(0), t)
try: root = ET.fromstring(t)
except ET.ParseError as e: log(head); stop(f'{label}: the answer does not parse ({e})')
out = [head]; summary = {}
le = [x.text for x in root.iter('LINEERROR')]
coll = root.find('./BODY/DATA/COLLECTION'); rows = list(coll) if coll is not None else []
def cp(s): return ' '.join(f'U+{ord(ch):04X}' for ch in s)
def tx(e, k):
    c = e.find(k); return 'absent' if c is None else (c.text or '')
bad_guid = []
if is_create:
    keys = ('CREATED', 'ALTERED', 'DELETED', 'COMBINED', 'IGNORED', 'ERRORS', 'CANCELLED', 'EXCEPTIONS', 'LASTVCHID', 'LASTMID')
    cnt = {k: (root.findtext(f'.//{k}').strip() if root.find(f'.//{k}') is not None else 'absent') for k in keys}
    summary = {'counters': cnt, 'lineerror': le}
    out.append('counters ' + json.dumps(cnt) + f'; LINEERROR {le}')
else:
    for x in rows:
        if x.tag == 'COMPANY':
            out.append(f'COMPANY NAME={x.get("NAME")!r} ' + ' '.join(f'{c.tag} {(c.text or "").strip()}' for c in x))
            continue
        g = tx(x, 'GUID').strip()
        if not g.startswith(GUID): bad_guid.append(g)
        lists = x.findall('LANGUAGENAME.LIST')
        names = [n.text or '' for ll in lists for nl in ll.findall('NAME.LIST') for n in nl.findall('NAME')]
        out.append(f'{x.tag} NAME={x.get("NAME")!r} GUID {g} PARENT {tx(x, "PARENT")!r} ISBILLWISEON {tx(x, "ISBILLWISEON").strip()} ALTERID {tx(x, "ALTERID").strip()} '
                   f'LANGUAGENAME.LIST blocks {len(lists)}')
        for i, n in enumerate(names, 1): out.append(f'    name {i}: {n!r} (len {len(n)}) {cp(n)}')
    counts = {}
    for x in rows: counts[x.tag] = counts.get(x.tag, 0) + 1
    summary = {'rows': counts, 'lineerror': le}
    out.append(f'rows per tag {json.dumps(counts)}; LINEERROR {le}')
co = [x for x in rows if x.tag == 'COMPANY']
if 'marks' in rel:
    m = [(tx(x, 'GUID').strip(), tx(x, 'ALTMSTID').strip(), tx(x, 'ALTVCHID').strip()) for x in co]
    summary['marks'] = m
    if len(co) == 1 and m[0][0] == GUID: open(E + '/last-marks.txt', 'w').write(f'ok {GUID} {m[0][1]}')
    else: log(head + ' ' + json.dumps(summary)); stop(f'{label}: the MARKS answer does not hold exactly THE COMPANY\'s row')
open(f'{E}/{label}.table.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
log(head + '; ' + json.dumps(summary, ensure_ascii=False))
if bad_guid: stop(f'{label}: GUID(s) outside THE COMPANY: {bad_guid}')
if expect_mst and 'marks' in rel and summary['marks'][0][1] != expect_mst: stop(f'{label}: ALTMSTID {summary["marks"][0][1]}, expected {expect_mst}')
