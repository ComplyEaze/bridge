# Lab helper (#2016, not product code): the sheet's checks from the tool answers and the relay's bytes: the step 3 FETCH and COMPUTEs as sent, rows against step 2, every row's reserved name and class, lines per row, step 4 against step 3, which Tally requests each call sent (from relay.jsonl by the call's time), sizes. Writes evidence/check.txt.
import glob, json, re
from datetime import datetime
L = '<lab folder>/'; O = L + 'out/'
out = open(L + 'evidence/check.txt', 'w', encoding='utf-8', newline='\n')
def p(*a):
    s = ' '.join(str(x) for x in a); print(s); out.write(s + '\n')
def ans(n): return json.load(open(glob.glob(f'{L}run/answers/*-{n}.json')[0], encoding='utf-8'))
def res(n): return ans(n)['response']['result']['structuredContent']
def dec(b): return b.decode('utf-16-le', 'replace').lstrip(chr(0xfeff)) if b[1:2] == b'\0' or b[:2] == b'\xff\xfe' else b.decode('utf-8', 'replace')
rows = [json.loads(l) for l in open(O + 'relay.jsonl', encoding='utf-8')]
reqs = [r for r in rows if 'event' not in r]
p('relay', 'open', sum(1 for r in rows if r.get('event') == 'open'), 'errors', sum(1 for r in rows if r.get('event') in ('tally_closed', 'relay_error')), 'requests', len(reqs))
calls = []
for line in open(L + 'run/client-log.txt', encoding='utf-8'):
    m = re.match(r'(\S+Z) (\d{3}-\S+)\.json tool=(\S+) secs=([\d.]+)', line)
    if m:
        end = datetime.fromisoformat(m.group(1).replace('Z', '+00:00')).timestamp(); calls.append((m.group(2), m.group(3), end - float(m.group(4)) - 1, end + 1))
def ts(r): return datetime.fromisoformat(r['utc'].replace('Z', '+00:00')).timestamp()
def kind(r):
    if r['sent'] == 0: return 'status'
    s = dec(open(O + 'bodies/' + r['sent_sha256'], 'rb').read()); return (re.findall(r'<ID>([^<]*)</ID>', s) or ['?'])[0]
p('== Tally requests per call (by time window; a call that starts within a second of the last may share)')
for n, tool, a, b in calls:
    ks = [kind(r) for r in reqs if a <= ts(r) <= b]
    c = {}
    for k in ks: c[k] = c.get(k, 0) + 1
    p(n, tool, len(ks), c)
win = [r for r in reqs if r['sent'] and 'Bridge Agent Vouchers' in dec(open(O + 'bodies/' + r['sent_sha256'], 'rb').read())]
shapes = {}
for r in win: shapes.setdefault(r['sent_sha256'], []).append(r)
WANT = 'DATE,VOUCHERNUMBER,VOUCHERTYPENAME,PARTYLEDGERNAME,NARRATION,GUID,ALTERID,MASTERID,ISCANCELLED,ISOPTIONAL,ISPOSTDATED,ALLINVENTORYENTRIES.*,REFERENCE,ISINVOICE,PARTYGSTIN,REFERENCEDATE'
p('== window requests')
for h, rs in shapes.items():
    s = dec(open(O + 'bodies/' + h, 'rb').read()); f = re.findall(r'<FETCH>([^<]*)</FETCH>', s)
    p(h[:16], 'sends', len(rs), 'bytes', rs[0]['sent'], '| FETCH', f, '| equals the sheet', WANT in f, '| COMPUTE', re.findall(r'<COMPUTE>([^<]*)</COMPUTE>', s), '| answers', sorted({r['answered'] for r in rs}), sorted({r['answer_sha256'][:12] for r in rs}))
s2 = res('s2')['result']; s3 = res('s3')['result']; s4 = res('s4')['result']
key = lambda i: (i.get('date'), i.get('voucher_number'), i.get('voucher_type'), i.get('guid'))
p('== rows: step 2', len(s2['items']), 'step 3', len(s3['items']), '| same date, number, type, GUID in order', [key(i) for i in s2['items']] == [key(i) for i in s3['items']])
p('step 2 carries any stock key:', any(k.startswith('stock') or k == 'kinds_not_read' for i in s2['items'] for k in i) or any(k.startswith('stock') or k == 'kinds_not_read' for k in s2))
for i in s3['items']:
    st = 'lines %d' % len(i['stock_lines']) if 'stock_lines' in i else ('not_read' if 'stock_lines_not_read' in i else 'unreadable ' + str(i.get('stock_lines_unreadable')))
    p(' ', i.get('voucher_type'), i.get('voucher_number'), 'alter', i.get('alter_id'), '| reserved', repr(i.get('voucher_type_reserved_name')), '| class', repr(i.get('voucher_class')), '| cancelled', i.get('cancelled'), 'optional', i.get('optional'), '|', st)
    for ln in i.get('stock_lines', []):
        def g(d, k):
            v = d.get(k); return v.get('value', v.get('state')) if isinstance(v, dict) else v
        p('      ', g(ln, 'direction'), g(ln, 'stock_item'), g(ln, 'actual_quantity'), g(ln, 'billed_quantity'), 'rate', g(ln, 'rate'), 'amount', g(ln, 'amount'), '| batches', [(g(b, 'godown'), g(b, 'batch'), g(b, 'actual_quantity'), g(b, 'amount')) for b in ln.get('batches', [])])
p('kinds_not_read', json.dumps(s3.get('kinds_not_read')), '| tie', s3.get('stock_summary_tie'))
T = ('read_at', 'duration_ms', 'id', 'ms')
def strip(x):
    if isinstance(x, dict): return {k: strip(v) for k, v in x.items() if k not in T}
    if isinstance(x, list): return [strip(v) for v in x]
    return x
p('step 4 equals step 3 apart from read time, timings and snapshot id:', strip(res('s4')) == strip(res('s3')))
def walk(x, y, path=''):
    if isinstance(x, dict) and isinstance(y, dict):
        for k in sorted(set(x) | set(y)): walk(x.get(k), y.get(k), path + '/' + k)
    elif isinstance(x, list) and isinstance(y, list) and len(x) == len(y):
        for n, (m, q) in enumerate(zip(x, y)): walk(m, q, f'{path}[{n}]')
    elif x != y: p('   differs', path, str(x)[:50], '|', str(y)[:50])
walk(res('s3'), res('s4'))
