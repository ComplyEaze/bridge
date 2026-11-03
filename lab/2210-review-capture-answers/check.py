# Lab helper (#2210, not product code): the sheet's checks from out/relay.jsonl and out/bodies by parsing: open, close and error lines; each verify_import's requests by kind against the sheet's string; each census request's $AlterID bounds against the mark that call read; each repeat's answer_sha256; the import verification VOUCHER rows. Writes evidence/check.txt.
import json, re
from datetime import datetime
import xml.etree.ElementTree as ET
L = '<lab folder>/'; O = L + 'out/'
out = open(L + 'evidence/check.txt', 'w', encoding='utf-8', newline='\n')
def p(*a):
    s = ' '.join(str(x) for x in a); print(s); out.write(s + '\n')
rows = [json.loads(l) for l in open(O + 'relay.jsonl', encoding='utf-8')]
ev = [r for r in rows if 'event' in r]; reqs = [r for r in rows if 'event' not in r]
p('lines', len(rows), 'open', sum(1 for r in ev if r['event'] == 'open'), 'close', sum(1 for r in ev if r['event'] == 'close'), 'tally_closed or relay_error', sum(1 for r in ev if r['event'] in ('tally_closed', 'relay_error')), 'requests', len(reqs))
def body(h): return open(O + 'bodies/' + h, 'rb').read()
def dec(b): return b.decode('utf-16-le', 'replace').lstrip(chr(0xfeff)) if b[1:2] == b'\0' or b[:2] == b'\xff\xfe' else b.decode('utf-8', 'replace')
KIND = {'BridgeCompanyExtent': 'E', 'Bridge Agent Company High Water': 'H', 'Bridge Agent Voucher Census': 'C', 'Bridge Agent Import Verification': 'V'}
def kind(r):
    if r['sent'] == 0: return 'S'
    s = dec(body(r['sent_sha256'])); i = (re.findall(r'<ID>([^<]*)</ID>', s) or ['?'])[0]
    return KIND.get(i, '?' + i)
calls = {}
for line in open(L + 'run/client-log.txt', encoding='utf-8'):
    m = re.match(r'(\S+Z) (\d{3}-\S+)\.json tool=(\S+) secs=([\d.]+)', line)
    if m and m.group(3) == 'verify_import':
        end = datetime.fromisoformat(m.group(1).replace('Z', '+00:00')); calls[m.group(2)] = (end.timestamp() - float(m.group(4)) - 1, end.timestamp() + 1)
WANT = 'SEESESEHSHSEECSCSEEVSVSEEVSVSE'
def ts(r): return datetime.fromisoformat(r['utc'].replace('Z', '+00:00')).timestamp()
marks = {}
for name, (a, b) in calls.items():
    seq = [r for r in reqs if a <= ts(r) <= b]
    k = ''.join(kind(r) for r in seq)
    p(name, 'requests', len(seq), 'kinds', k, '| equals the sheet', k == WANT)
    hw = [r for r in seq if kind(r) == 'H']
    m = sorted({int(re.search(r'<ALTVCHID[^>]*>\s*(\d+)', dec(body(r['answer_sha256']))).group(1)) for r in hw})
    p('   voucher mark read by this call', m)
    for r in [r for r in seq if kind(r) == 'C']:
        s = dec(body(r['sent_sha256'])); f = re.findall(r'<SYSTEM[^>]*>([^<]*)</SYSTEM>', s)
        spans = [x for x in f if 'AlterID' in x or 'ALTERID' in x.upper()]
        p('   census', r['conn'], r['request'], 'formulae', len(f), '|', ' || '.join(x.replace('&gt;', '>').replace('&lt;', '<') for x in f)[:300])
    for kk in sorted(set(k)):
        same = {}
        for r in seq:
            if kind(r) == kk: same.setdefault(r['sent_sha256'], set()).add(r['answer_sha256'])
        p('   kind', kk, 'distinct requests', len(same), 'each repeat one answer', all(len(v) == 1 for v in same.values()))
    for r in [r for r in seq if kind(r) == 'V'][:1]:
        a = dec(body(r['answer_sha256'])); a = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', a); a = re.sub(r'&#([0-9]);', '', a)
        t = ET.fromstring(a); n = sum(1 for v in t.iter('VOUCHER') if v.attrib or len(v)); p('   import verification VOUCHER rows', n, 'answer bytes', r['answered'])
