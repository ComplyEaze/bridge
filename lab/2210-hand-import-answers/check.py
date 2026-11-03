# Lab helper (#2210 hand-import, not product code): the sheet's checks from relay.jsonl and bodies/: relay events, each call's
# request order by kind (by the client log's times), the census and import-verification requests with their date and
# $AlterID bounds, the verify's census and V answers counted by parsing (rows: VOUCHER elements carrying a DATE), the V rows'
# narrations, tags and REMOTEID against GUID and against the saved file, and equal answers for requests sent twice in a row.
import collections, json, re, xml.etree.ElementTree as ET
from datetime import datetime
L = '<lab folder>/'; O = L + 'out/'; BATCH = 'bridge-c3160d0e-9fc0-48e5-8e5c-48ba0d502f94'
out = open(L + 'evidence/check.txt', 'w', encoding='utf-8', newline='\n')
def p(*a):
    s = ' '.join(str(x) for x in a); print(s); out.write(s + '\n')
def dec(b): return b.decode('utf-16-le', 'replace').lstrip(chr(0xfeff)) if b[1:2] == b'\0' or b[:2] == b'\xff\xfe' else b.decode('utf-8', 'replace')
def body(h): return dec(open(O + 'bodies/' + h, 'rb').read())
BAD = re.compile(r'[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:[0-8]|1[124-9]|2[0-9]|3[01]);')
K = {'BridgeCompanyExtent': 'E', 'Bridge Agent Company High Water': 'H', 'Bridge Agent Voucher Census': 'C', 'Bridge Agent Import Verification': 'V',
     'Import Data': 'I', 'List of Ledgers': 'L', 'List of VoucherTypes': 'T', 'BridgeCompanyCurrencies': 'U', 'List of Groups': 'G', 'BridgeCompanyBookExtentV2': 'B'}
rows = [json.loads(l) for l in open(O + 'relay.jsonl', encoding='utf-8')]
reqs = [r for r in rows if 'event' not in r]
ev = collections.Counter(r['event'] for r in rows if 'event' in r)
p('relay events', dict(ev), 'requests', len(reqs), '| relay_error or tally_closed:', ev.get('relay_error', 0) + ev.get('tally_closed', 0))
def kind(r):
    if not r['sent']: return 'S'
    s = body(r['sent_sha256']); k = (re.findall(r'<ID>([^<]*)</ID>', s) or re.findall(r'<TALLYREQUEST>([^<]*)', s) or ['?'])[0]
    return K.get(k, '?' + k)
def ts(r): return datetime.fromisoformat(r['utc'].replace('Z', '+00:00')).timestamp()
def vrows(h):
    r = ET.fromstring(BAD.sub('', body(h))); return [v for v in r.iter('VOUCHER') if (v.findtext('DATE') or '').strip()]
calls = []
for line in open(L + 'run/client-log.txt', encoding='utf-8'):
    m = re.match(r'(\S+Z) (\d{3}-\S+)\.json tool=(\S+) secs=([\d.]+)', line)
    if m:
        end = datetime.fromisoformat(m.group(1).replace('Z', '+00:00')).timestamp(); calls.append((m.group(2), m.group(3), end - float(m.group(4)) - 1, end + 1))
p('== per call: request order (S status, E extent, B book extent, H marks, C census, V import verification, L ledgers, T voucher types, U currencies, G groups)')
for n, tool, a, b in calls:
    rs = [r for r in reqs if a <= ts(r) <= b]
    seq = ''.join(kind(r) for r in rs); p(n, tool, len(rs), seq)
    for r in rs:
        k = kind(r)
        if k in 'CVH':
            s = body(r['sent_sha256'])
            f = re.findall(r'NAME="(?:BridgeAgentCensus|BridgeImportWindow)">([^<]*)', s)
            extra = f[0].replace('&gt;', '>').replace('&lt;', '<') if f else 'marks ' + str(re.findall(r'<ALTVCHID[^>]*>\s*([^<]*)', body(r['answer_sha256'])))
            vr = vrows(r['answer_sha256']) if k in 'CV' else []
            p(f"   {r['utc'][11:19]} conn {r['conn']} req {r['request']} {k} sent {r['sent_sha256'][:12]} answer {r['answered']} {r['answer_sha256'][:12]} | {extra} | rows {len(vr)} dates {dict(collections.Counter(v.findtext('DATE').strip() for v in vr))}")
p('== requests sent twice in a row with differing answers')
pre = [r for r in reqs if r['sent']]; diff = 0
for x, y in zip(pre, pre[1:]):
    if x['sent_sha256'] == y['sent_sha256'] and x['answer_sha256'] != y['answer_sha256']: diff += 1; p('   differs', x['conn'], x['request'], y['request'])
p('   pairs differing:', diff)
vcall = [c for c in calls if c[1] == 'verify_import'][0]
vreqs = [r for r in reqs if vcall[2] <= ts(r) <= vcall[3] and r['sent'] and kind(r) == 'V']
h = vreqs[0]['answer_sha256']; vs = vrows(h)
saved = open(L + f'agent/imports/{BATCH}.xml', 'rb').read().decode('utf-8')
saved_rid = re.findall(r'<VOUCHER[^>]*REMOTEID="([^"]*)"', saved)
p('== the verify V answer', h[:12], 'rows', len(vs), '| saved file REMOTEIDs', len(saved_rid))
tagged = 0
for v in vs:
    nar = v.findtext('NARRATION') or ''; tag = re.search(r'\[BRIDGE:[^\]]*\]\s*$', nar); tagged += bool(tag)
    rid = v.get('REMOTEID'); guid = (v.findtext('GUID') or '').strip()
    p(f"   {v.findtext('DATE').strip()} {v.findtext('VOUCHERTYPENAME')} no {v.findtext('VOUCHERNUMBER')} ALTERID {(v.findtext('ALTERID') or '').strip()} | narration {nar[:40]!r} tag {bool(tag)} | REMOTEID {rid} | equals GUID {rid == guid} | in saved file {rid in saved_rid}")
p('   tagged', tagged, 'untagged', len(vs) - tagged, '| untagged narrations', [v.findtext('NARRATION') for v in vs if not re.search(r'\[BRIDGE:[^\]]*\]\s*$', v.findtext('NARRATION') or '')])
