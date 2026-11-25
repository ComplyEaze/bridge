# Lab helper (2162-PA, from 2162-SALES-Q's pg.py at lab/2162-sales-q-sheet 3bd08d02; not product code): the one post of
# the one built invoice through launch LPA's client.py, its approval box read and answered with nothing typed, and the
# two raw read-backs at about +5 s and +60 s after the post's final answer, so that no line is typed while a box is up
# or between the answer and the reads.
#   1. Starts box.ps1 (byte-equal to 2162-SALES-Q's): it logs the box's title, rectangle, foreground and every child's
#      text, takes its own capture of the screen (a GDI copy, not the screen tool), then waits for box/<tag>.answer.
#      -AnswerBy is 150 s, past the build's own 120 s limit (POST_DIALOG_LIMIT, tally/approved_import.rs:578), so a box
#      given no answer closes by the build's timeout and box.ps1 ends "box no longer visible before any click" (exit 4)
#      with nothing pressed.
#   2. A guard, in its own thread: when the box log reaches its WAITING line it reads the box's window with
#      boxstate.ps1 (byte-equal to 2569-S's) on the pid box.ps1 logged for the box, into box/<tag>-R1.txt, and passes
#      only when that read holds exactly one box line (name starting "[ComplyEaze Bridge", class #32770) with
#      iconic=False and visible=True, box.ps1's own rectangles of the box and of its Yes button lie inside the
#      screen it logged, and box.ps1 logged the box at z-order place 0 (nearest the top). Only then does it start boxcheck.py, which writes Yes when the box's whole text is the text
#      the build makes for this invoice, and writes no answer otherwise. The guard writes box/<tag>-guard.txt.
#   3. Writes the post_import call into run-pa/inbox (whole, then renamed in) and waits for its answer. On
#      approval.state pending it calls again after retry_after_s while dialog_remaining_s lasts; on approval.state
#      approved (a Yes that came too late in the call for the post to fit in it: agent_import_post.rs:611-633, its
#      next_step at :470-477) it calls again at once, as that next_step says; at most four calls in all.
#   4. When the final answer reports one voucher created (dispatch.counters.created 1) whose status is posted_verified
#      or posted_divergent (the build read it back), it sends rb.xml raw by run.py (byte-equal to 2162-RC's) at +5 s and
#      at +60 s from the moment it saw that answer whole, under the labels R5 and R60, after checking rb.xml's sha256
#      against local/rb.sha256 each time. Otherwise it sends nothing. The times go to evidence/pa-times.txt.
#   5. Waits for both helpers, prints the box log's stamped lines (never the box's own text, which holds the company
#      number), the guard's and the check's output, the times, and calls.py's reading of the last answer.
# Usage: python -I -X utf8 pa.py <tag> <build answer file name in run-pa/answers>
# Exit 0: the post is posted_verified with created 1, the guard and the check passed, and both raw reads were sent
# with run.py exit 0. Exit 1: anything else that was read (a STOP to record). Exit 2: a failed read of this helper's
# own inputs (no build answer, no rb.xml, a name already used).
import hashlib, json, os, re, subprocess, sys, threading, time
R = 'C:/bridge-lab/2162-pa'; RUN = R + '/run-pa'; URL = 'http://127.0.0.1:9001'
if len(sys.argv) != 3: print('usage: pa.py <tag> <build answer>'); sys.exit(2)
tag, build = sys.argv[1], sys.argv[2]
def utc(): return time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())
def fail(why): print('FAILED READ:', why, flush=True); sys.exit(2)
def py(*a): return subprocess.run([sys.executable, '-I', '-X', 'utf8', *a], cwd=R, capture_output=True, text=True, encoding='utf-8')
def res(f):  # the tool's result, or {} when the answer has none (a failed request: never read as a refusal)
    sc = ((json.load(open(f, encoding='utf-8')).get('response') or {}).get('result') or {}).get('structuredContent')
    return sc['result'] if isinstance(sc, dict) and isinstance(sc.get('result'), dict) else {}
try:
    b = res(f'{RUN}/answers/{build}'); guid = json.load(open(f'{RUN}/answers/{build}', encoding='utf-8'))['arguments']['company_guid']
except (OSError, ValueError, KeyError) as e: fail(f'the build answer {build}: {type(e).__name__}')
if b.get('voucher_count') != 1 or not b.get('batch_id'): fail('the build answer is not one built voucher')
bid = b['batch_id']
def rb_ok():
    try: want = open(f'{R}/local/rb.sha256', encoding='utf-8').read().split()[0]; have = hashlib.sha256(open(f'{R}/rb.xml', 'rb').read()).hexdigest()
    except (OSError, IndexError) as e: return False, f'{type(e).__name__}'
    return have == want, have
if not rb_ok()[0]: fail('rb.xml missing, or its sha256 differs from local/rb.sha256')
for p in [f'{R}/box/{tag}{x}' for x in ('-box.txt', '.answer', '.answer.tmp', '-check.out', '-R1.txt', '-guard.txt')] + \
         [f'{R}/a-R5-rb.xml', f'{R}/a-R60-rb.xml', f'{R}/evidence/pa-times.txt']:
    if os.path.exists(p): fail(f'{p} exists; a name is used once')
os.makedirs(f'{R}/box', exist_ok=True)
box = subprocess.Popen(['powershell', '-NoProfile', '-File', R + '/box.ps1', '-RowDir', 'C:\\bridge-lab\\2162-pa', '-Tag', tag,
                        '-SeeWithin', '120', '-AnswerBy', '150'], stdout=open(f'{R}/box/{tag}-ps.out', 'w'), stderr=subprocess.STDOUT)
time.sleep(2)  # as pg.py: box.ps1 starts its watch before the call
guard = {'ok': False, 'check': None, 'lines': []}
def inside(rect, w, h): return rect is not None and 0 <= rect[0] < rect[2] <= w and 0 <= rect[1] < rect[3] <= h
def run_guard():
    g = guard['lines']; log = f'{R}/box/{tag}-box.txt'; t = ''
    end = time.monotonic() + 150
    while time.monotonic() < end:
        try: t = open(log, 'rb').read().decode('utf-8-sig')
        except (OSError, UnicodeDecodeError): t = ''
        if re.search(r'^\S+Z (WAITING for \S.*|NO BOX seen.*)\r?\n', t, re.M): break
        if box.poll() is not None: break
        time.sleep(0.25)
    if not re.search(r'^\S+Z WAITING for \S', t, re.M):
        g.append(f'{utc()} no WAITING line in the box log (box.ps1 exit {box.poll()}): no read, no check, no answer'); return
    seen = re.search(r'^\S+Z BOX SEEN 0x[0-9a-f]+ pid (\d+) ', t, re.M)
    scr = re.search(r'^\S+Z rect (-?\d+),(-?\d+),(-?\d+),(-?\d+) \(screen (\d+)x(\d+)\).*z-order place box (-?\d+) ', t, re.M)
    yes = [l for l in t.splitlines() if re.match(r'^\S+Z child Button rect -?\d+,-?\d+,-?\d+,-?\d+ text: &?Yes\s*$', l)]
    if not seen or not scr or len(yes) != 1:
        g.append(f'{utc()} the box log lacks the BOX SEEN pid, the rect line or exactly one Yes button: no check, no answer'); return
    pid = seen.group(1); w, h = int(scr.group(5)), int(scr.group(6))
    rect = tuple(int(scr.group(i)) for i in range(1, 5))
    yr = tuple(int(x) for x in re.search(r'rect (-?\d+),(-?\d+),(-?\d+),(-?\d+)', yes[0]).groups())
    out = f'{R}/box/{tag}-R1.txt'
    p = subprocess.run(['powershell', '-NoProfile', '-File', R + '/boxstate.ps1', '-ProcId', pid, '-Out', out], capture_output=True, text=True, encoding='utf-8', errors='replace')
    try: read = open(out, encoding='utf-8-sig').read()
    except OSError: read = ''
    lines = [l for l in read.splitlines() if l.startswith('WINDOW ') and ' name=[ComplyEaze Bridge' in l and ' class=#32770 ' in l]
    g.append(f'{utc()} R1 boxstate.ps1 on pid {pid}: exit {p.returncode}; box lines {len(lines)}')
    state = lines[0] if len(lines) == 1 else ''
    checks = [('boxstate exit 0', p.returncode == 0), ('exactly one box line', len(lines) == 1),
              ('iconic=False', ' iconic=False ' in state), ('visible=True', ' visible=True ' in state),
              (f'the box rect {rect} inside the screen {w}x{h}', inside(rect, w, h)),
              (f'the Yes button rect {yr} inside the screen', inside(yr, w, h)),
              ('the box nearest the top of the z-order (place 0), so the click lands on it', scr.group(7) == '0')]
    g += [('PASS ' if ok else 'FAIL ') + name for name, ok in checks]
    if not all(ok for _, ok in checks):
        g.append(f'{utc()} GUARD NOT PASSED: no check, no answer file (the box is left to the build\'s own timeout)'); return
    g.append(f'{utc()} GUARD PASSED: boxcheck.py started')
    guard['ok'] = True
    c = subprocess.run([sys.executable, '-I', '-X', 'utf8', 'boxcheck.py', tag, bid, '1'], cwd=R,
                       stdout=subprocess.DEVNULL, stderr=open(f'{R}/box/{tag}-check.err', 'w'))
    guard['check'] = c.returncode
    g.append(f'{utc()} boxcheck.py exit {c.returncode}')
th = threading.Thread(target=run_guard); th.start()
def send(k):
    name = f'{tag}-post-{k}.json'; tmp = f'{RUN}/{name}.tmp'
    if os.path.exists(f'{RUN}/inbox/{name}') or os.path.exists(f'{RUN}/answers/{name}'): print(f'STOP: {name} exists; never sent twice'); return None, {}
    with open(tmp, 'w', encoding='utf-8', newline='\n') as f: json.dump({'tool': 'post_import', 'arguments': {'company_guid': guid, 'batch_id': bid}}, f)
    os.replace(tmp, f'{RUN}/inbox/{name}'); print(f'{utc()} sent {name}', flush=True)
    out = f'{RUN}/answers/{name}'
    while True:
        if os.path.exists(out):
            try: return out, res(out)
            except ValueError: pass  # not yet written whole
        time.sleep(0.25)
k, deadline = 1, None
last, r = send(1)
while last and k < 4:
    a = r.get('approval') or {}
    if a.get('state') not in ('pending', 'approved') or r.get('attempt_recorded'): break
    if a.get('state') == 'approved':  # answered Yes too late in the call to post in it: call again now
        print(f'approved, not yet posted (expires_in_s {a.get("expires_in_s")}); calling again now', flush=True)
        k += 1; last, r = send(k); continue
    wait = a.get('retry_after_s') or 15
    if deadline is None: deadline = time.monotonic() + (a.get('dialog_remaining_s') or 0)
    if time.monotonic() + wait > deadline: print('pending, but dialog_remaining_s would run out before retry_after_s: no further call'); break
    print(f'pending: retry_after_s {wait}, dialog_remaining_s {a.get("dialog_remaining_s")}; calling again', flush=True)
    time.sleep(wait); k += 1; last, r = send(k)
t0, t0u = time.monotonic(), utc()
d = r.get('dispatch') or {}; vs = r.get('vouchers') if isinstance(r.get('vouchers'), list) else []
status = vs[0].get('status') if len(vs) == 1 and isinstance(vs[0], dict) else None
created = (d.get('counters') or {}).get('created')
times = [f'post\t{os.path.basename(last) if last else "(none)"}\tanswer seen whole at {t0u}\t+0.000s']
sent = {}
if created == 1 and status in ('posted_verified', 'posted_divergent'):
    for label, at in (('R5', 5), ('R60', 60)):
        time.sleep(max(0.0, t0 + at - time.monotonic()))
        ok, have = rb_ok()
        if not ok:
            times.append(f'rb.xml\ta-{label}-rb.xml\tnot sent: sha256 {have} differs from local/rb.sha256'); break
        s = time.monotonic() - t0
        p = subprocess.run([sys.executable, '-I', 'run.py', URL, label, 'rb.xml'], cwd=R, capture_output=True, text=True, encoding='utf-8', errors='replace')
        e = time.monotonic() - t0; sent[label] = p.returncode
        times.append(f'rb.xml\ta-{label}-rb.xml\tsent +{s:.3f}s\treturned +{e:.3f}s\trun.py {p.returncode}\t{(p.stdout + p.stderr).strip()[:200]}')
        if p.returncode != 0: break
else:
    times.append(f'no raw read sent: created {created!r}, voucher status {status!r}')
with open(f'{R}/evidence/pa-times.txt', 'x', encoding='utf-8', newline='\n') as f: f.write('\n'.join(times) + '\n')
th.join(); box.wait()
open(f'{R}/box/{tag}-guard.txt', 'x', encoding='utf-8', newline='\n').write('\n'.join(guard['lines']) + '\n')
blog = open(f'{R}/box/{tag}-box.txt', encoding='utf-8-sig').read().splitlines() if os.path.exists(f'{R}/box/{tag}-box.txt') else []
stamped = [l[:220] for l in blog if re.match(r'^\d{4}-\d\d-\d\dT\S+Z ', l) and ' child ' not in l and 'ompany number' not in l]
print(f'--- box log {tag} (stamped lines only; box.ps1 exit {box.returncode})'); print('\n'.join(stamped))
print('--- guard'); print('\n'.join(guard['lines']))
if guard['check'] is not None: print(f'--- check (exit {guard["check"]})'); print(open(f'{R}/box/{tag}-check.out', encoding='utf-8').read().strip())
print('--- times'); print('\n'.join(times))
print(f'--- last answer ({k} call(s))'); print(py('calls.py', 'show', last).stdout.strip()[:4000] if last else '(none)')
ok = (d.get('state') == 'posted_verified' and created == 1 and guard['ok'] and guard['check'] == 0
      and sent.get('R5') == 0 and sent.get('R60') == 0)
print('POST', 'posted_verified, created 1, the box guarded and checked, both raw reads sent' if ok else
      f'NOT as expected (dispatch {d.get("state")!r}, created {created!r}, voucher {status!r}, guard {guard["ok"]}, check {guard["check"]}, reads {sent})')
sys.exit(0 if ok else 1)
