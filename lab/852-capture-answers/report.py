# Lab helper (852-C1, not product code): the sheet's "What to report" rows from relay.jsonl and the client log, for each
# post. The post_import call's window is its sent time and its round trip from the client's answer file. Prints and writes
# evidence/report.txt.
import glob, hashlib, json, re
from datetime import datetime, timedelta
R = '<lab folder>/'
out = open(R + 'evidence/report.txt', 'w', encoding='utf-8', newline='\n')
def p(*a):
    s = ' '.join(str(x) for x in a); print(s); out.write(s + '\n')
def ts(u): return datetime.fromisoformat(u.replace('Z', '+00:00'))
def dec(b): return b.decode('utf-16-le', 'replace').lstrip('\ufeff') if b[1:2] == b'\0' or b[:2] == b'\xff\xfe' else b.decode('utf-8', 'replace')
for f, post in (('a', 'post-a-1'), ('b', 'post-b-1')):
    rows = [json.loads(l) for l in open(f'{R}relay/{f}/relay.jsonl', encoding='utf-8')]
    ev = [r for r in rows if 'event' in r]; reqs = [r for r in rows if 'event' not in r]
    a = json.load(open(glob.glob(f'{R}runA/answers/*-{post}.json')[0], encoding='utf-8'))
    start = None
    for l in open(R + 'runA/evidence/tools-log.txt', encoding='utf-8'):
        if f'-{post}.json' in l: start = ts(l.split()[0])
    end = start + timedelta(seconds=a['round_trip_seconds'])
    imp_req = open(f'{R}relay/{f}/import-1.request', 'rb').read(); ih = hashlib.sha256(imp_req).hexdigest()
    imp = [r for r in reqs if r.get('number') is not None]
    p(f'== batch {f}: relay events {[e["event"] for e in ev]}; requests {len(reqs)}; reply_lost/tally_closed/relay_error: {sum(1 for e in ev if e["event"] in ("reply_lost", "tally_closed", "relay_error"))}')
    p(f'   post_import call: start {start.isoformat()[:23]}Z end {end.isoformat()[:23]}Z seconds {a["round_trip_seconds"]}')
    win = [r for r in reqs if start - timedelta(seconds=1) <= ts(r['utc']) <= end + timedelta(seconds=1)]
    i = imp[0]; k = win.index(i) if i in win else None
    before = win[:k] if k is not None else []; after = win[k + 1:] if k is not None else []
    p(f'   in the call: requests before the import {len(before)}, sent bytes {sum(r["sent"] for r in before)}, answered bytes {sum(r["answered"] or 0 for r in before)}, summed answer_s {sum(r["answer_s"] or 0 for r in before):.3f}')
    p(f'   the import line: utc {i["utc"]} sent {i["sent"]} answered {i["answered"]} first_byte_s {i["first_byte_s"]} answer_s {i["answer_s"]} bridge_left_s {i["bridge_left_s"]} | import-1.request file {len(imp_req)} B (HTTP headers and body) | import lines {len(imp)}')
    ab = open(f'{R}relay/{f}/import-1.answer', 'rb').read(); SEP = bytes([13, 10, 13, 10]); t = dec(ab.split(SEP, 1)[1] if SEP in ab[:600] else ab)
    p('   Tally counters (import-1.answer):', {x: re.findall(rf'<{x}>([^<]*)', t) for x in ('CREATED', 'ALTERED', 'DELETED', 'ERRORS', 'EXCEPTIONS', 'LASTVCHID', 'LINEERROR')})
    p(f'   in the call: requests after the import {len(after)}, sent bytes {sum(r["sent"] for r in after)}, answered bytes {sum(r["answered"] or 0 for r in after)}, summed answer_s {sum(r["answer_s"] or 0 for r in after):.3f}')
    p(f'   whole log: requests {len(reqs)}, max answer_s {max((r["answer_s"] or 0) for r in reqs):.3f}, any bridge_left_s set: {[r["request"] for r in reqs if r.get("bridge_left_s") is not None]}')
    v = a['response']['result']['structuredContent']['result']; d = v.get('dispatch', {})
    p(f'   result: state {d.get("state")} created {d.get("counters", {}).get("created")} response_state {d.get("response_state")} attempt_recorded {v.get("attempt_recorded")} code {(v.get("error") or {}).get("code")}')
