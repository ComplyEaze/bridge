# Lab helper (683-R1, not product code): groups the relay's exchanges by tool call, using each answer's
# call_sent_utc and round_trip_seconds against the relay's utc times, and counts each call's requests by collection
# ID (GET /status counted apart), with the first and last exchange number. Output: evidence/per-call.txt.
# Usage: python -I -X utf8 percall.py
import datetime as dt, glob, json, os, re
R = '<lab folder>/'
def t(s): return dt.datetime.strptime(s.rstrip('Z'), '%Y-%m-%dT%H:%M:%S.%f' if '.' in s else '%Y-%m-%dT%H:%M:%S').replace(tzinfo=dt.timezone.utc)
calls = []
for p in sorted(glob.glob(R + 'run/answers/*.json')):
    a = json.load(open(p, encoding='utf-8')); s = t(a['call_sent_utc'])
    calls.append((os.path.basename(p), s, s + dt.timedelta(seconds=a['round_trip_seconds'] + 1.5)))
rows = {c[0]: [] for c in calls}; other = []
for line in open(R + 'relay/relay.jsonl', encoding='utf-8'):
    e = json.loads(line)
    if 'event' in e: continue
    rq = open(f"{R}relay/exchange-{e['number']}.request", 'rb').read(); b = rq[rq.find(b'\r\n\r\n') + 4:]
    tx = b[2:].decode('utf-16-le') if b[:2] == b'\xff\xfe' else b.decode('utf-16-le') if b[1:2] == b'\0' else b.decode('utf-8', 'replace')
    cid = (re.findall(r'<ID>([^<]*)</ID>', tx) or ['GET /status' if e['line'].startswith('GET') else '?'])[0]
    when = t(e['utc']); hit = [c for c in calls if c[1] - dt.timedelta(seconds=1) <= when <= c[2]]
    (rows[hit[0][0]] if hit else other).append((e['number'], cid, e['sent_sha256'][:12], e['answered']))
out = []
for name, _, _ in calls:
    r = rows[name]
    if not r: out.append(f'{name}: no exchange'); continue
    cnt = {}
    for _, cid, sha, _ in r: cnt[(cid, sha)] = cnt.get((cid, sha), 0) + 1
    out.append(f'{name}: exchanges {len(r)} (numbers {r[0][0]} to {r[-1][0]})')
    for (cid, sha), n in cnt.items(): out.append(f'   {cid} request {sha}: {n}')
out.append(f'outside any call: {len(other)}')
open(R + 'evidence/per-call.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n'); print('\n'.join(out))
