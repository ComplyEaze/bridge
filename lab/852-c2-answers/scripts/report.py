# Lab helper (852-C2, from 852-C1's report.py; not product code): the sheet's "What to report" rows from relay e's
# relay.jsonl (both posts went through relay e, on the run lead's word: e's import is import-1, f's import-2) and the client
# log, for each post. The post_import call's window is its sent time and its round trip from the client's answer file; the
# screen work (C2a) from box/e-p1-keys.txt. Prints and writes evidence/report.txt.
import glob, hashlib, json, re
from datetime import datetime, timedelta
R = '<LAB_DIR>/'
out = open(R + 'evidence/report.txt', 'w', encoding='utf-8', newline='\n')
def p(*a):
    s = ' '.join(str(x) for x in a); print(s); out.write(s + '\n')
def ts(u): return datetime.fromisoformat(u.replace('Z', '+00:00'))
def dec(b): return b.decode('utf-16-le', 'replace').lstrip('\ufeff') if b[1:2] == b'\0' or b[:2] == b'\xff\xfe' else b.decode('utf-8', 'replace')
rows = [json.loads(l) for l in open(f'{R}e/relay.jsonl', encoding='utf-8')]
ev = [r for r in rows if 'event' in r]; reqs = [r for r in rows if 'event' not in r]
p(f'relay e (both posts): events {sorted({e["event"] for e in ev})}; connections opened {sum(1 for e in ev if e["event"] == "open")}; requests {len(reqs)}; reply_lost/tally_closed/relay_error: {sum(1 for e in ev if e["event"] in ("reply_lost", "tally_closed", "relay_error"))}')
p(f'   whole log: max answer_s {max((r["answer_s"] or 0) for r in reqs):.3f}, any bridge_left_s set: {[r["request"] for r in reqs if r.get("bridge_left_s") is not None]}')
for f, post, n in (('e', 'e-p1', 1), ('f', 'f-p1', 2)):
    a = json.load(open(glob.glob(f'{R}agent-run/answers/*-{post}.json')[0], encoding='utf-8'))
    start = None
    for l in open(R + 'agent-run/evidence/tools-log.txt', encoding='utf-8'):
        if f'-{post}.json' in l: start = ts(l.split()[0])
    end = start + timedelta(seconds=a['round_trip_seconds'])
    imp_req = open(f'{R}e/import-{n}.request', 'rb').read()
    imp = [r for r in reqs if r.get('number') == n and r.get('answered') is not None]
    p(f'== batch {f} (import-{n}): post_import call start {start.isoformat()[:23]}Z end {end.isoformat()[:23]}Z seconds {a["round_trip_seconds"]}')
    win = [r for r in reqs if start - timedelta(seconds=1) <= ts(r['utc']) <= end + timedelta(seconds=1)]
    i = imp[0]; k = win.index(i) if i in win else None
    before = win[:k] if k is not None else []; after = win[k + 1:] if k is not None else []
    p(f'   relay lines of this call: utc {win[0]["utc"]} to {win[-1]["utc"]}, connection(s) {sorted({r["conn"] for r in win})}, requests {len(win)}')
    p(f'   in the call: requests before the import {len(before)}, sent bytes {sum(r["sent"] for r in before)}, answered bytes {sum(r["answered"] or 0 for r in before)}, summed answer_s {sum(r["answer_s"] or 0 for r in before):.3f}')
    sent = [e for e in ev if e['event'] == 'sent' and e.get('number') == n][0]
    p(f'   the import line: sent at {sent["utc"]}, answered line utc {i["utc"]}; sent {i["sent"]} answered {i["answered"]} first_byte_s {i["first_byte_s"]} answer_s {i["answer_s"]} bridge_left_s {i["bridge_left_s"]} | import-{n}.request file {len(imp_req)} B (HTTP headers and body) sha256 {hashlib.sha256(imp_req).hexdigest()}')
    ab = open(f'{R}e/import-{n}.answer', 'rb').read(); SEP = bytes([13, 10, 13, 10]); t = dec(ab.split(SEP, 1)[1] if SEP in ab[:600] else ab)
    p(f'   Tally counters (import-{n}.answer):', {x: re.findall(rf'<{x}>([^<]*)', t) for x in ('CREATED', 'ALTERED', 'DELETED', 'ERRORS', 'EXCEPTIONS', 'LASTVCHID', 'LINEERROR')})
    p(f'   in the call: requests after the import {len(after)}, sent bytes {sum(r["sent"] for r in after)}, answered bytes {sum(r["answered"] or 0 for r in after)}, summed answer_s {sum(r["answer_s"] or 0 for r in after):.3f}')
    v = a['response']['result']['structuredContent']['result']; d = v.get('dispatch', {})
    p(f'   result: verification_status {v.get("verification_status")} counts.posted_verified {(v.get("counts") or {}).get("posted_verified")}; dispatch state {d.get("state")} created {d.get("counters", {}).get("created")} response_state {d.get("response_state")} resent {d.get("resent")}; attempt_recorded {v.get("attempt_recorded")}; error {(v.get("error") or {}).get("code")}; next_step {v.get("next_step")}')
    box = open(f'{R}box/{post}-box.txt', encoding='utf-8-sig').read()
    seen = re.search(r'^(\S+) BOX SEEN', box, re.M).group(1); yes = re.search(r'CLICKED Yes at \S+ \(physical\) (\S+);', box).group(1)
    p(f'   box shown at {seen}; Yes at {yes}')
    if f == 'e':
        kl = open(f'{R}box/e-p1-keys.txt', encoding='utf-8-sig').read().splitlines()
        keys = [l for l in kl if ' KEY ' in l]; skips = [l for l in kl if ' SKIP' in l]
        p(f'   screen work: Day Book open from 4a through the call; key presses {len(keys)}, skipped {len(skips)} (Tally not in front), first skip {skips[0][:24] if skips else "-"}, last skip {skips[-1][:24] if skips else "-"}; the import line {sent["utc"]} + {i["answer_s"]} s lies inside the skip window: {bool(skips) and skips[0][:24] <= sent["utc"] <= skips[-1][:24]}')
    else:
        p(f'   screen work: the Journal half entered (5a) left on screen untouched from before the call to after it (5a {"09:25"} capture, 5c capture); the import line inside the call: {start <= ts(sent["utc"]) <= end}')
