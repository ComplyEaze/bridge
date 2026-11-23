# Lab helper (2162-SALES-Q, from 1392-E34's p.py at lab/1392-e34-sheet 0e2fc769; not product code): one post of one built
# invoice through launch LG's client.py, so that nothing is typed while a box is up. It starts box.ps1 (which watches for
# Bridge's box), then, for the real post only, boxcheck.py (which writes Yes only when the box names this company, GUID,
# batch and count and the built invoice's type, date, number, customer, lines and totals); then it writes the post_import
# call into run-g\inbox (written whole, then renamed in) and waits for its answer. On approval.state pending it calls again
# after retry_after_s while dialog_remaining_s lasts, at most four calls in all (1392-E34's p.py). Then it waits for both
# helpers to end and prints the box log's stamped lines (never the box's own text, which holds the company number), the
# check's output and calls.py's reading of the last answer.
#   pg.py smoke <tag> <build answer>   the smoke box: no boxcheck, box.ps1 clicks No at 20 s while the call waits (the
#                                      shape of 1392-E34's K3); one call only. Exit 0 only if the answer is refused
#                                      import_approval_declined and the log shows CLICKED No and then box visible False.
#   pg.py post <tag> <build answer>    the real post. Exit 0 only if dispatch is posted_verified with created 1.
# <build answer> is a file name in run-g\answers. Usage: python -I -X utf8 pg.py post g06 g05-build-0072.json
import json, os, re, subprocess, sys, time
R = 'C:/bridge-lab/2162-sales-q'; RUN = R + '/run-g'
mode, tag, build = sys.argv[1], sys.argv[2], sys.argv[3]
if mode not in ('smoke', 'post'): sys.exit('STOP: mode is smoke or post')
def py(*a): return subprocess.run([sys.executable, '-I', '-X', 'utf8', *a], cwd=R, capture_output=True, text=True, encoding='utf-8')
def res(f):  # the tool's result, or {} when the answer has none (a failed request: never read as a refusal)
    sc = ((json.load(open(f, encoding='utf-8')).get('response') or {}).get('result') or {}).get('structuredContent')
    return sc['result'] if isinstance(sc, dict) and isinstance(sc.get('result'), dict) else {}
b = res(f'{RUN}/answers/{build}')
if b.get('voucher_count') != 1 or not b.get('batch_id'): sys.exit('STOP: the build answer is not one built voucher')
bid = b['batch_id']; guid = json.load(open(f'{RUN}/answers/{build}', encoding='utf-8'))['arguments']['company_guid']
for x in ('-box.txt', '.answer', '-check.out'):
    if os.path.exists(f'{R}/box/{tag}{x}'): sys.exit(f'STOP: box/{tag}{x} exists; a tag is used once')
box = subprocess.Popen(['powershell', '-NoProfile', '-File', R + '/box.ps1', '-RowDir', 'C:\\bridge-lab\\2162-sales-q', '-Tag', tag,
                        '-SeeWithin', '120', '-AnswerBy', '20' if mode == 'smoke' else '70'],
                       stdout=open(f'{R}/box/{tag}-ps.out', 'w'), stderr=subprocess.STDOUT)
time.sleep(2)
chk = None if mode == 'smoke' else subprocess.Popen([sys.executable, '-I', '-X', 'utf8', 'boxcheck.py', tag, bid, '1'], cwd=R,
                                                   stdout=subprocess.DEVNULL, stderr=open(f'{R}/box/{tag}-check.err', 'w'))
def send(k):
    name = f'{tag}-post-{k}.json'; tmp = f'{RUN}/{name}.tmp'
    if os.path.exists(f'{RUN}/inbox/{name}') or os.path.exists(f'{RUN}/answers/{name}'): sys.exit(f'STOP: {name} exists; never sent twice')
    with open(tmp, 'w', encoding='utf-8', newline='\n') as f: json.dump({'tool': 'post_import', 'arguments': {'company_guid': guid, 'batch_id': bid}}, f)
    os.replace(tmp, f'{RUN}/inbox/{name}'); print(f'{time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())} sent {name}', flush=True)
    out = f'{RUN}/answers/{name}'
    while True:
        if os.path.exists(out):
            try: return out, res(out)
            except ValueError: pass  # not yet written whole
        time.sleep(0.25)
k, deadline = 1, None
last, r = send(1)
while mode == 'post' and k < 4:
    a = r.get('approval') or {}
    if a.get('state') != 'pending' or r.get('attempt_recorded'): break
    wait = a.get('retry_after_s') or 15
    if deadline is None: deadline = time.monotonic() + (a.get('dialog_remaining_s') or 0)
    if time.monotonic() + wait > deadline: print('pending, but dialog_remaining_s would run out before retry_after_s: no further call'); break
    print(f'pending: retry_after_s {wait}, dialog_remaining_s {a.get("dialog_remaining_s")}; calling again', flush=True)
    time.sleep(wait); k += 1; last, r = send(k)
box.wait()
if chk: chk.wait()
blog = open(f'{R}/box/{tag}-box.txt', encoding='utf-8-sig').read().splitlines() if os.path.exists(f'{R}/box/{tag}-box.txt') else []
stamped = [l[:220] for l in blog if re.match(r'^\d{4}-\d\d-\d\dT\S+Z ', l) and ' child ' not in l and 'ompany number' not in l]
print(f'--- box log {tag} (stamped lines only; box.ps1 exit {box.returncode})'); print('\n'.join(stamped))
if chk: print(f'--- check (exit {chk.returncode})'); print(open(f'{R}/box/{tag}-check.out', encoding='utf-8').read().strip())
print(f'--- last answer ({k} call(s))'); print(py('calls.py', 'show', last).stdout.strip()[:4000])
clicked_no = any(' CLICKED No ' in l for l in stamped) and any('after click: box visible False' in l for l in stamped)
if mode == 'smoke':
    code = ((r.get('error') or {}).get('code'))
    ok = code == 'import_approval_declined' and clicked_no and box.returncode == 0
    print('SMOKE', 'declined, CLICKED No, box gone' if ok else f'NOT as expected (code {code}, CLICKED No and gone {clicked_no}, box.ps1 exit {box.returncode})')
else:
    d = r.get('dispatch') or {}
    ok = d.get('state') == 'posted_verified' and (d.get('counters') or {}).get('created') == 1
    print('POST', 'posted_verified, created 1' if ok else 'NOT posted_verified with created 1')
sys.exit(0 if ok else 1)
