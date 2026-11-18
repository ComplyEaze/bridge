# Lab helper (row 1761-S4 v4, new; not product code): step 4.2's post and its ONE timed follow-up, through call.py.
# 1. post_import with args/post.json (label s4-post1). 2. If that answer's approval.state is "pending": the arrival time is
# the answer file's modified time; the follow-up goes once retry_after_s seconds have passed since it (plus 1 s, never
# earlier), or at once if dialog_remaining_s is null or not more than retry_after_s. Just before sending, if the file
# followup.cancel exists in the row folder (a closure the owner reported before the follow-up), nothing is sent. Exactly one
# follow-up (label s4-post2), never a third call. Any first answer other than "pending" ends the script with no call: the
# lab decides by the sheet. Every time goes to evidence/notes.txt by note.py. Usage: python -I -X utf8 post4.py
import json, os, subprocess, sys, time
from datetime import datetime, timezone
R = '<LAB_DIR>'; PY = [sys.executable, '-I', '-X', 'utf8']
def iso(t): return datetime.fromtimestamp(t, timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
def note(t): subprocess.run(PY + [R + '/note.py', t], check=True)
def call(label):
    seq = len([f for f in os.listdir(R + '/inbox') if f.endswith('.json')]); fn = f'{seq:03d}-{label}.json'
    subprocess.run(PY + [R + '/call.py', R, label, 'post_import', R + '/args/post.json'], check=True)
    p = f'{R}/answers/{fn}'; j = json.load(open(p, encoding='utf-8'))
    return fn, os.path.getmtime(p), j['response']['result']['structuredContent']['result']
if os.path.exists(R + '/followup.cancel'): raise SystemExit('followup.cancel exists before the post: nothing sent')
fn1, t1, r1 = call('s4-post1'); ap = r1.get('approval') or {}
note(f'Step 4.2: {fn1} answer arrived {iso(t1)} (answer file time); approval {json.dumps(ap)}; attempt_recorded {r1.get("attempt_recorded")}; dispatch {json.dumps(r1.get("dispatch"))[:300]}')
if ap.get('state') != 'pending':
    note(f'Step 4.2: first answer approval.state {ap.get("state")!r}, not pending: post4.py makes no further call; the lab decides by the sheet.')
    sys.exit(0)
ra, dr = ap.get('retry_after_s'), ap.get('dialog_remaining_s')
at_once = dr is None or ra is None or dr <= ra
target = t1 if at_once else t1 + ra + 1.0
note(f'Step 4.2: pending; retry_after_s {ra}, dialog_remaining_s {dr}; follow-up due {iso(target)} ({"at once" if at_once else "retry_after_s + 1 s after the answer"}); window closes by about {iso(t1 + dr) if dr is not None else "unknown"}')
while time.time() < target:
    if os.path.exists(R + '/followup.cancel'): break
    time.sleep(0.1)
if os.path.exists(R + '/followup.cancel'):
    note('Step 4.2: followup.cancel found before the follow-up was sent (a closure reported first): the follow-up is not sent (step 4.3).')
    sys.exit(3)
note(f'Step 4.2: sending the one follow-up post_import at {iso(time.time())}')
fn2, t2, r2 = call('s4-post2'); ap2 = r2.get('approval') or {}
note(f'Step 4.2: {fn2} answer arrived {iso(t2)}; approval {json.dumps(ap2)}; attempt_recorded {r2.get("attempt_recorded")}; dispatch {json.dumps(r2.get("dispatch"))[:300]}. No further post_import is made.')
