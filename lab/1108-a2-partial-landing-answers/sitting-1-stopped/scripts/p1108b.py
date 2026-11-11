# Lab helper (1108-A2, p1108.py with box.ps1's row folder given as os.path.normpath(R) and a gate before the post (box.ps1's watch start line seen); from 1392-E's p.py, not product code): the sheet's step 6, items 2 to 6, for one built batch. mN-before
# marks (send.py, chk.py: one COMPANY row, THE COMPANY's), then box.ps1 (watching for Bridge's box) and boxcheck.py (Yes only
# on the run lead's corrected Difference 3), post_import once; an answer with approval.state pending is called again after its
# retry_after_s with the box left as it is, and an approved answer with no attempt at once (Difference 6); never again once an
# attempt is recorded. Then a capture of the Tally window, mN-after marks, vouchers and its diff against the previous vouchers,
# and verify_import once. Stops (exit 1) after the post if the box check did not pass. Records only; the EXPECT check is the lab's.
# Usage: python -I -X utf8 p1108b.py <pN> <batch_id> <count 1|3> <total 10|66|94> <previous vouchers answer> [<call number, default 1>]
import json, os, subprocess, sys, time
from datetime import datetime, timezone
R = '<LAB_DIR>'; name, bid, n, total, prevv = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4], sys.argv[5]; tag = name + '-p' + (sys.argv[6] if len(sys.argv) > 6 else '1')  # 6th: the call's number, a new tag
if any(os.path.exists(f'{R}/box/{tag}{s}') for s in ('.answer', '-box.txt', '-check.out', '-ps.out')): print(f'HALT: tag {tag} already used'); sys.exit(1)
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
def py(*a):
    r = subprocess.run([sys.executable, '-I', '-X', 'utf8', *a], cwd=R, capture_output=True, text=True, encoding='utf-8'); return r
def run(*a):
    r = py(*a); print(r.stdout.strip()); print(r.stderr.strip()) if r.stderr.strip() else None
    if r.returncode: print(f'HALT: {a[0]} exit {r.returncode}'); sys.exit(1)
    return r.stdout
def res(f): return json.load(open(f, encoding='utf-8'))['response']['result']['structuredContent']['result']
def last(label):
    fs = sorted(f for f in os.listdir(R + '/agent-run/answers') if f.endswith(f'-{label}.json')); return f'{R}/agent-run/answers/{fs[-1]}'
run('args1108.py', 'batch', name, bid)
print('--- mN-before'); run('send.py', name + 'pre', 'marks.utf16le.xml'); run('chk.py', name + 'pre-marks', 'marks', name + 'pre-marks.utf16le.xml')
box = subprocess.Popen(['powershell', '-NoProfile', '-File', R + '/box.ps1', '-RowDir', os.path.normpath(R), '-Tag', tag, '-SeeWithin', '120', '-AnswerBy', '120'],
                       stdout=open(f'{R}/box/{tag}-ps.out', 'w'), stderr=subprocess.STDOUT)
time.sleep(2)
wl = f'{R}/box/{tag}-box.txt'; t_g = time.monotonic()
while not (os.path.exists(wl) and 'watch start' in open(wl, encoding='utf-8-sig', errors='replace').read()):
    if time.monotonic() - t_g > 15 or box.poll() is not None: print('HALT before post_import: box.ps1 wrote no watch start line'); sys.exit(1)
    time.sleep(0.25)
print('box.ps1 watching:', open(wl, encoding='utf-8-sig').read().strip().splitlines()[0][:160])
chk = subprocess.Popen([sys.executable, '-I', '-X', 'utf8', 'boxcheck.py', tag, bid, n, total], cwd=R, stdout=subprocess.DEVNULL, stderr=open(f'{R}/box/{tag}-check.err', 'w'))
print(f'--- post_import start {now()}'); o = py('call.py', 'agent-run', tag, 'post_import', f'calls/args-post-{name}.json'); print(o.stdout.strip()[:800]); print(f'post_import end {now()}')
k, lab = 0, tag
while k < 8:
    r = res(last(lab)); a = r.get('approval') or {}
    if r.get('attempt_recorded'): break
    if a.get('state') == 'pending': wait = a.get('retry_after_s') or 15
    elif a.get('state') == 'approved': wait = 0
    else: break
    k += 1; lab = f'{tag}-r{k}'
    print(f'{now()} approval {a.get("state")}: retry_after_s {a.get("retry_after_s")}; calling again as {lab} after {wait} s', flush=True); time.sleep(wait)
    o = py('call.py', 'agent-run', lab, 'post_import', f'calls/args-post-{name}.json'); print(o.stdout.strip()[:800]); print(f'call end {now()}')
box.wait(); chk.wait()
print('--- box log'); print('\n'.join(l[:240] for l in open(f'{R}/box/{tag}-box.txt', encoding='utf-8-sig').read().splitlines() if 'child' not in l).replace(open(R + '/guid.txt').read().strip(), '<company guid>'))
print('--- check'); co = open(f'{R}/box/{tag}-check.out', encoding='utf-8').read().strip(); print(co.replace(open(R + '/guid.txt').read().strip(), '<company guid>'))
pf = last(lab); print('--- post answer', os.path.basename(pf)); print(py('show.py', pf).stdout.strip()[:6000])
if co.splitlines()[-2] != 'ALL PASS': print('HALT: the box check did not pass'); sys.exit(1)
print('--- capture'); run('cap.py', name + '-after')
print('--- mN-after'); run('send.py', name + 'post', 'marks.utf16le.xml'); run('chk.py', name + 'post-marks', 'marks', name + 'post-marks.utf16le.xml')
run('send.py', name + 'post', 'vouchers.utf16le.xml'); py('chk.py', name + 'post-vouchers', 'rows', name + 'post-vouchers.utf16le.xml')
run('chk.py', name + '-vdiff', 'diff', prevv, name + 'post-vouchers.utf16le.xml')
print(f'--- verify_import {now()}'); o = py('call.py', 'agent-run', name + '-verify', 'verify_import', f'calls/args-verify-{name}.json'); print(o.stdout.strip()[:400])
vf = last(name + '-verify'); print(py('show.py', vf).stdout.strip()[:6000])
