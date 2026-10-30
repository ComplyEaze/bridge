# Lab helper (#690 R1, not product code; the R0 helper with C3's curl line and R1's names): the sheet's sends and reads. Each request file's sha256 is checked against the
# sheet's SHA256.txt first (a filled R1 file against the value fill_r1.py printed, given with --sha). Each is sent by
# curl.exe with LAB-COMMON C3's line (no client timeout; headers file; status, bytes and CONTENT-LENGTH checked), the answer kept byte for byte, and
# read with an XML parser (UTF-16LE; &#4; references removed and recorded; any other parse failure = stop).
#   python -I r0.py send <label> <request file> [--sha HEX]   a write: every counter (absent recorded as absent) and LINEERROR
#   python -I r0.py read <label> [<previous read label>]       marks, ledgers, groups; every field the sheet names; GUIDs
#                                                              added and gone against the previous read, ALTERIDs moved
# Writes answers/<seq>-<label>-<name>.xml, evidence/<label>.json and one line per request in evidence/log.txt.
import hashlib, json, os, re, subprocess, sys, time, xml.etree.ElementTree as ET
from datetime import datetime, timezone
W = '<LAB_DIR>'; S = W + '/sheet'
os.makedirs(W + '/answers', exist_ok=True); os.makedirs(W + '/evidence', exist_ok=True)
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
def sha(p): return hashlib.sha256(open(p, 'rb').read()).hexdigest()
def log(s):
    with open(W + '/evidence/log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
want = {}
for l in open(S + '/SHA256.txt', encoding='utf-8'):
    p = l.split()
    if len(p) >= 2: want[p[-1]] = p[0]
def stop(msg): log('stop: ' + msg); sys.exit(2)
def send(label, path, expect_sha):
    if sha(path) != expect_sha: stop(f'{label}: {path} sha256 differs; nothing sent')
    seq = len([f for f in os.listdir(W + '/answers') if f.endswith('.xml')])
    ans = f'{W}/answers/{seq:03d}-{label}.xml'
    t0 = now(); m0 = time.monotonic(); hdr = ans[:-4] + '.headers.txt'
    r = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download}', '--data-binary', '@' + path,
                        '-H', 'Content-Type: text/xml; charset=utf-16', '-D', hdr, 'http://127.0.0.1:9001', '-o', ans], capture_output=True, text=True)
    secs = time.monotonic() - m0; size = os.path.getsize(ans) if os.path.exists(ans) else -1
    cl = None
    if os.path.exists(hdr):
        for hl in open(hdr, encoding='latin-1'):
            if hl.upper().startswith('CONTENT-LENGTH:'): cl = int(hl.split(':', 1)[1])
    log(f'{label} {os.path.basename(path)} ({os.path.getsize(path)} B, sha256 {sha(path)}) sent {t0} answered {now()} {secs:.3f} s curl_exit {r.returncode} printed "{r.stdout.strip()}" CONTENT-LENGTH {cl} bytes {size} sha256 {sha(ans) if size > 0 else "none"}' + (f' stderr "{r.stderr.strip()}"' if r.stderr.strip() else ''))
    if r.returncode != 0 or not r.stdout.startswith('200 ') or size <= 0 or cl != size: stop(f'{label}: the request failed')
    if size > 2000000: stop(f'{label}: the answer is above the 2,000,000-byte bound')
    return ans
def parse(ans, label):
    raw = open(ans, 'rb').read(); t = raw.decode('utf-16-le').lstrip('\ufeff')
    # LAB-COMMON C4 (the run lead's word, about 01:00 UTC): only the characters XML forbids (0-8, 11, 12, 14-31) are dropped,
    # as character references and raw; every answer file is kept byte for byte. Per answer: how many, which code points.
    FORB = set(range(0, 9)) | {11, 12} | set(range(14, 32)); dropped = {}
    def ref(m):
        v = int(m.group(1)[1:], 16) if m.group(1)[0] in 'xX' else int(m.group(1))
        if v in FORB: dropped[f'ref U+{v:04X}'] = dropped.get(f'ref U+{v:04X}', 0) + 1; return ''
        return m.group(0)
    t = re.sub(r'&#(x[0-9A-Fa-f]+|[0-9]+);', ref, t)
    for ch in set(c for c in t if ord(c) in FORB): dropped[f'raw U+{ord(ch):04X}'] = t.count(ch)
    t = ''.join(c for c in t if ord(c) not in FORB)
    if dropped: log(f'{label}: C4 dropped before parsing: {dropped}')
    t = t.replace('<ENVELOPE', '<ENVELOPE xmlns:UDF="urn:x-tally-udf"', 1) if 'UDF:' in t else t
    try: return ET.fromstring(t)
    except ET.ParseError as e: stop(f'{label}: the answer does not parse ({e})')
def rows_of(root, tag):
    # rows are the children of ENVELOPE/BODY/DATA/COLLECTION (CMPINFO holds count elements with the same tag names)
    coll = root.find('./BODY/DATA/COLLECTION')
    return [] if coll is None else [r for r in coll if r.tag == tag]
def own_name(led):
    lnl = led.find('LANGUAGENAME.LIST')
    if lnl is None: return None
    nl = lnl.find('NAME.LIST')
    return None if nl is None or nl.find('NAME') is None else nl.find('NAME').text
def mode_send(label, path, given):
    rel = os.path.relpath(path, S).replace(os.sep, '/') if os.path.abspath(path).startswith(os.path.abspath(S)) else None
    exp = given or (want.get(rel) if rel else None)
    if not exp: stop(f'{label}: no sha256 to check {path} against')
    root = parse(send(label, path, exp), label)
    counters = {k: (root.findtext('.//' + k).strip() if root.find('.//' + k) is not None and root.findtext('.//' + k) is not None else 'absent')
                for k in ('CREATED', 'ALTERED', 'DELETED', 'COMBINED', 'IGNORED', 'ERRORS', 'CANCELLED', 'EXCEPTIONS', 'LASTMID', 'LASTVCHID')}
    errs = [e.text for e in root.iter('LINEERROR')]
    out = {'label': label, 'file': os.path.basename(path), 'counters': counters, 'lineerror': errs, 'root': root.tag}
    json.dump(out, open(f'{W}/evidence/{label}.json', 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
    log(f'{label} counters {json.dumps(counters)} LINEERROR {json.dumps(errs, ensure_ascii=False)}')
def mode_read(label, prev):
    marks = parse(send(f'{label}-marks', S + '/requests/marks.utf16le.xml', want['requests/marks.utf16le.xml']), label)
    comps = rows_of(marks, 'COMPANY')
    if len(comps) != 1: stop(f'{label}: {len(comps)} COMPANY rows')
    c = comps[0]; company = {'name': c.get('NAME'), 'guid': (c.findtext('GUID') or '').strip(), 'altmstid': (c.findtext('ALTMSTID') or '').strip(), 'altvchid': (c.findtext('ALTVCHID') or '').strip()}
    leds = parse(send(f'{label}-ledgers', S + '/requests/ledgers.utf16le.xml', want['requests/ledgers.utf16le.xml']), label)
    ledgers = {}
    for led in rows_of(leds, 'LEDGER'):
        lists = [{'languageid': (lnl.findtext('LANGUAGEID') or '').strip(), 'names': [n.text for nl in lnl.findall('NAME.LIST') for n in nl.findall('NAME')]} for lnl in led.findall('LANGUAGENAME.LIST')]
        g = (led.findtext('GUID') or '').strip()
        ledgers[g] = {'row_name': led.get('NAME'), 'reservedname': led.get('RESERVEDNAME'), 'own_name': own_name(led), 'lists': lists,
                      'alterid': (led.findtext('ALTERID') or '').strip(), 'parent': led.findtext('PARENT'), 'isbillwiseon': led.findtext('ISBILLWISEON')}
    grps = parse(send(f'{label}-groups', S + '/requests/groups.utf16le.xml', want['requests/groups.utf16le.xml']), label)
    groups = {(g.findtext('GUID') or '').strip(): {'name': g.get('NAME'), 'alterid': (g.findtext('ALTERID') or '').strip()} for g in rows_of(grps, 'GROUP')}
    out = {'label': label, 'company': company, 'ledgers': ledgers, 'groups': groups}
    json.dump(out, open(f'{W}/evidence/{label}.json', 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
    r0 = {n.lower().replace(' ', '') for n in ('R1 Guard X', 'R1 Guard X2', 'R1 Guard Y', 'R1 Guard Y2', 'R0 Missing Ledger', 'R0 Missing Renamed')}
    held = sorted({n for l in ledgers.values() for n in [l['row_name'], l['own_name']] + [x for li in l['lists'] for x in li['names']] if n and n.lower().replace(' ', '') in r0} | {g['name'] for g in groups.values() if g['name'] and g['name'].lower().replace(' ', '') in r0})
    log(f'{label} read: company {company}; ledgers {len(ledgers)}, groups {len(groups)}; S1 names held: {held}')
    for g, l in sorted(ledgers.items(), key=lambda kv: int(kv[1]['alterid'] or 0)):
        if any(n and n.startswith(('R0', 'R1')) for n in [l['row_name'], l['own_name']] + [x for li in l['lists'] for x in li['names']]):
            log(f'   ledger {g[-8:]} row NAME {l["row_name"]!r} RESERVEDNAME {l["reservedname"]!r} own {l["own_name"]!r} lists {json.dumps(l["lists"], ensure_ascii=False)} ALTERID {l["alterid"]} PARENT {l["parent"]!r} ISBILLWISEON {l["isbillwiseon"]!r}')
    if prev:
        p = json.load(open(f'{W}/evidence/{prev}.json', encoding='utf-8'))
        for kind in ('ledgers', 'groups'):
            a, b = p[kind], out[kind]
            added = sorted(set(b) - set(a)); gone = sorted(set(a) - set(b)); moved = sorted((g, a[g]['alterid'], b[g]['alterid']) for g in set(a) & set(b) if a[g]['alterid'] != b[g]['alterid'])
            log(f'   against {prev}: {kind} added {[(g[-8:], b[g].get("own_name") or b[g].get("name")) for g in added]}; gone {[(g[-8:], a[g].get("own_name") or a[g].get("name")) for g in gone]}; ALTERID moved {[(g[-8:], x, y) for g, x, y in moved]}')
        log(f'   against {prev}: ALTMSTID {p["company"]["altmstid"]} -> {company["altmstid"]}; ALTVCHID {p["company"]["altvchid"]!r} -> {company["altvchid"]!r}')
if sys.argv[1] == 'send': mode_send(sys.argv[2], sys.argv[3], sys.argv[5] if len(sys.argv) > 5 and sys.argv[4] == '--sha' else None)
elif sys.argv[1] == 'read': mode_read(sys.argv[2], sys.argv[3] if len(sys.argv) > 3 else None)
