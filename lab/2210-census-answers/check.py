# Lab helper (#2210 census, not product code): the sheet's checks from relay.jsonl and bodies/: relay events, each call's
# request order by kind (by the client log's times), every census and import-verification request with its date window and
# $AlterID bounds, the answers counted by parsing VOUCHER elements, and whether each repeat of a request has the same answer.
# Writes evidence/check.txt.
import collections, glob, json, re, xml.etree.ElementTree as ET
from datetime import datetime
L = '<lab folder>/'; O = L + 'out/'
out = open(L + 'evidence/check.txt', 'w', encoding='utf-8', newline='\n')
def p(*a):
    s = ' '.join(str(x) for x in a); print(s); out.write(s + '\n')
def dec(b): return b.decode('utf-16-le', 'replace').lstrip(chr(0xfeff)) if b[1:2] == b'\0' or b[:2] == b'\xff\xfe' else b.decode('utf-8', 'replace')
def body(h): return dec(open(O + 'bodies/' + h, 'rb').read())
K = {'BridgeCompanyExtent': 'E', 'Bridge Agent Company High Water': 'H', 'Bridge Agent Voucher Census': 'C', 'Bridge Agent Import Verification': 'V',
     'Import Data': 'I', 'List of Ledgers': 'L', 'List of VoucherTypes': 'T', 'BridgeCompanyCurrencies': 'U', 'List of Groups': 'G'}
rows = [json.loads(l) for l in open(O + 'relay.jsonl', encoding='utf-8')]
reqs = [r for r in rows if 'event' not in r]
ev = collections.Counter(r['event'] for r in rows if 'event' in r)
p('relay events', dict(ev), 'requests', len(reqs), '| relay_error or tally_closed:', ev.get('relay_error', 0) + ev.get('tally_closed', 0))
def kind(r):
    if not r['sent']: return 'S'
    s = body(r['sent_sha256']); k = (re.findall(r'<ID>([^<]*)</ID>', s) or re.findall(r'<TALLYREQUEST>([^<]*)', s) or ['?'])[0]
    return K.get(k, '?')
def ts(r): return datetime.fromisoformat(r['utc'].replace('Z', '+00:00')).timestamp()
def count(h):
    s = re.sub(r'&#([0-9]+);', lambda m: '' if int(m.group(1)) < 32 and m.group(1) not in ('9', '10', '13') else m.group(0), body(h))
    vs = [v for v in ET.fromstring(s).iter('VOUCHER') if (v.findtext('DATE') or '').strip()]
    return len(vs), dict(collections.Counter(v.findtext('DATE').strip() for v in vs))
calls = []
for line in open(L + 'run/client-log.txt', encoding='utf-8'):
    m = re.match(r'(\S+Z) (\d{3}-\S+)\.json tool=(\S+) secs=([\d.]+)', line)
    if m:
        end = datetime.fromisoformat(m.group(1).replace('Z', '+00:00')).timestamp(); calls.append((m.group(2), m.group(3), end - float(m.group(4)) - 1, end + 1))
p('== per call: request order (S status, E extent, H marks, C census, V import verification, I import, L ledgers, T voucher types, U currencies, G groups)')
for n, tool, a, b in calls:
    rs = [r for r in reqs if a <= ts(r) <= b]
    p(n, tool, ''.join(kind(r) for r in rs))
    for r in rs:
        k = kind(r)
        if k in 'CVHI':
            s = body(r['sent_sha256']) if r['sent'] else ''
            f = re.findall(r'NAME="(?:BridgeAgentCensus|BridgeImportWindow)">([^<]*)', s)
            mark = re.findall(r'<ALTVCHID[^>]*>\s*([^<]*)', body(r['answer_sha256'])) if k == 'H' else ''
            extra = f[0].replace('&gt;', '>').replace('&lt;', '<') if f else (f'mark {mark}' if mark else '')
            rc = count(r['answer_sha256']) if k in 'CV' else ''
            p(f"   {r['utc'][11:19]} conn {r['conn']} req {r['request']} {k} sent {r['sent_sha256'][:12]} answer {r['answered']} {r['answer_sha256'][:12]} {extra} {rc}")
p('== repeats: each request sha with more than one answer sha')
by = collections.defaultdict(set)
for r in reqs:
    if r['sent']: by[r['sent_sha256']].add(r['answer_sha256'])
pre = [r for r in reqs if r['sent']]
for h, ans in by.items():
    if len(ans) > 1:
        seq = []
        for r in pre:
            if r['sent_sha256'] == h and (not seq or seq[-1][1] != r['answer_sha256']): seq.append((r['conn'], r['request'], r['answer_sha256'][:12]))
        p(kind(next(r for r in pre if r['sent_sha256'] == h)), h[:12], 'answers in order', seq)
p('== consecutive repeats (same request sent twice in a row, S between): differing answers')
last = None; diff = 0
for r in pre:
    if last and last['sent_sha256'] == r['sent_sha256'] and last['answer_sha256'] != r['answer_sha256']:
        diff += 1; p('   differs', last['conn'], last['request'], r['request'])
    last = r
p('   pairs differing:', diff)
