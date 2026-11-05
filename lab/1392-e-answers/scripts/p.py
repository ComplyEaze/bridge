# Lab helper (1392-E, from 1029-R's step3.sh and batch.sh; not product code): the sheet's P for one built batch. Starts
# box.ps1 (watching for Bridge's box) and boxcheck.py (writes Yes only when the box names this company, GUID, batch and
# count), sends post_import once and waits for its answer, then stops (exit 1) unless dispatch is posted_verified with
# created = count; otherwise captures the Tally window, sends verify_import, takes marks and checks V + count, M 211.
# A pending or approved-without-post answer is left to the lab (the sheet's call again). Usage:
# python -I -X utf8 p.py <name> <batch_id> <count> <V before>
import json, subprocess, sys, time
R = '<LAB_DIR>'; name, bid, n, v0 = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4]); tag = name + '-p' + (sys.argv[5] if len(sys.argv) > 5 else '1')  # 5th: the call's number, a new tag
def py(*a): return subprocess.run([sys.executable, '-I', '-X', 'utf8', *a], cwd=R, capture_output=True, text=True, encoding='utf-8')
def res(f): return json.load(open(f, encoding='utf-8'))['response']['result']['structuredContent']['result']
def last(label):
    import os; fs = sorted(f for f in os.listdir(R + '/agent-run/answers') if f.endswith(f'-{label}.json')); return f'{R}/agent-run/answers/{fs[-1]}'
subprocess.run([sys.executable, '-I', '-X', 'utf8', 'args.py', 'batch', name, bid], cwd=R)
box = subprocess.Popen(['powershell', '-NoProfile', '-File', R + '/box.ps1', '-RowDir', '<LAB_DIR>', '-Tag', tag, '-SeeWithin', '120', '-AnswerBy', '120'],
                       stdout=open(f'{R}/box/{tag}-ps.out', 'w'), stderr=subprocess.STDOUT)
time.sleep(2)
chk = subprocess.Popen([sys.executable, '-I', '-X', 'utf8', 'boxcheck.py', tag, bid, str(n)], cwd=R, stdout=subprocess.DEVNULL, stderr=open(f'{R}/box/{tag}-check.err', 'w'))
o = py('call.py', 'agent-run', tag, 'post_import', f'calls/args-post-{name}.json'); print(o.stdout.strip()[:600])
# 1392-E, on the run lead's word: an answer with approval.state pending is called again after its retry_after_s while the box
# is up, at most until its dialog_remaining_s runs out (the sheet's P); each call is its own answer, <tag>-r<k>.
k, lab, deadline = 0, tag, None
while k < 8:
    r = res(last(lab)); a = r.get('approval') or {}
    if a.get('state') != 'pending' or r.get('attempt_recorded'): break
    wait = a.get('retry_after_s') or 15
    if deadline is None: deadline = time.monotonic() + (a.get('dialog_remaining_s') or 0)
    if time.monotonic() + wait > deadline: print('pending, but dialog_remaining_s would run out before retry_after_s: no further call'); break
    k += 1; lab = f'{tag}-r{k}'
    print(f'pending: retry_after_s {wait}, dialog_remaining_s {a.get("dialog_remaining_s")}; calling again as {lab}', flush=True); time.sleep(wait)
    o = py('call.py', 'agent-run', lab, 'post_import', f'calls/args-post-{name}.json'); print(o.stdout.strip()[:600])
box.wait(); chk.wait()
print('--- box log'); print('\n'.join(l[:220] for l in open(f'{R}/box/{tag}-box.txt', encoding='utf-8-sig').read().splitlines() if 'child' not in l))
print('--- check'); print(open(f'{R}/box/{tag}-check.out', encoding='utf-8').read().strip())
pf = last(lab); print('--- post answer'); print(py('show.py', pf).stdout.strip()[:4000])
d = res(pf).get('dispatch') or {}
if not (d.get('state') == 'posted_verified' and (d.get('counters') or {}).get('created') == n): print('HALT after post: not the plain path'); sys.exit(1)
print(py('cap.py', name + '-after').stdout.strip())
o = py('call.py', 'agent-run', name + '-verify', 'verify_import', f'calls/args-verify-{name}.json'); print(o.stdout.strip()[:300])
vf = last(name + '-verify'); print('--- verify answer'); print(py('show.py', vf).stdout.strip()[:6000])
m = py('marks.py', name + '-marks').stdout.strip(); print(m)
v = res(vf); c = v.get('counts') or {}
ok_v = v.get('verification_status') == 'posted_verified' and c.get('posted_verified') == n and sum(x for x in c.values() if isinstance(x, int)) == n
ok_m = f'V {v0 + n} M 211' in m
print('VERIFY', 'posted_verified n of n' if ok_v else 'NOT posted_verified n of n', '| MARKS', f'V {v0 + n} M 211' if ok_m else 'NOT as expected')
sys.exit(0 if ok_v and ok_m else 1)
