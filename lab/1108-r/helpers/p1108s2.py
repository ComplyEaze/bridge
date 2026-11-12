# Lab helper (1108-R, from 1108-A2 sitting 2's p1108s2.py with its row folder and client folder changed, not product code; from the first sitting's p1108.py, itself from 1392-E's p.py; written whole
# by an editor, no shell heredoc): the sheet's step 6, items 2 to 6, for one built batch. box.ps1's row folder is
# os.path.normpath(R), computed, never typed; box_argv() builds box.ps1's whole argument list, and the selftest mode runs
# box.ps1 with that same list (a see-within of 3 s), prints the list's repr and the log's watch start line.
# The post: mN-before marks (send.py, chk.py: one COMPANY row, THE COMPANY's); box.ps1 started, and post_import is sent only once
# box.ps1's log holds its watch start line (else a halt before the post); boxcheck.py (Yes only on the run lead's corrected
# Difference 3) beside it; post_import once; an answer with approval.state pending is called again after its retry_after_s
# with the box left as it is, an approved answer with no attempt at once (Difference 6); never again once an attempt is
# recorded. Then a capture of the Tally window, mN-after marks, vouchers and its diff against the previous vouchers, and
# verify_import once. Halts (exit 1) after the post if the box check did not pass. Records only; the EXPECT check is the lab's.
# Each call's tag is <pN>-p<k>; a tag whose box files exist is never used again.
# Usage: python -I -X utf8 p1108s2.py selftest <tag>
#        python -I -X utf8 p1108s2.py <pN> <batch_id> <count 1|3> <total 10|66|94> <previous vouchers answer> [<k, default 1>]
import json, os, subprocess, sys, time
from datetime import datetime, timezone
R = 'C:/bridge-lab/1108r'
G = open(R + '/guid.txt').read().strip()
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
def box_argv(tag, see, answer_by):
    return ['powershell', '-NoProfile', '-File', os.path.normpath(R + '/box.ps1'), '-RowDir', os.path.normpath(R), '-Tag', tag,
            '-SeeWithin', str(see), '-AnswerBy', str(answer_by)]
def used(tag): return any(os.path.exists(f'{R}/box/{tag}{s}') for s in ('.answer', '-box.txt', '-check.out', '-ps.out'))
def watch_start(tag, proc, limit):
    wl = f'{R}/box/{tag}-box.txt'; t0 = time.monotonic()
    while True:
        if os.path.exists(wl):
            t = open(wl, encoding='utf-8-sig', errors='replace').read()
            if 'watch start' in t: return t.strip().splitlines()[0]
        if time.monotonic() - t0 > limit or proc.poll() is not None: return None
        time.sleep(0.25)
if sys.argv[1] == 'selftest':
    tag = sys.argv[2]
    if used(tag): print(f'HALT: tag {tag} already used'); sys.exit(1)
    argv = box_argv(tag, 3, 5)
    print('box.ps1 argv:', repr(argv))
    print('row folder exists:', os.path.isdir(argv[5]), '| box folder exists:', os.path.isdir(os.path.join(argv[5], 'box')),
          '| characters below 32 in the argv:', sum(ord(c) < 32 for a in argv for c in a))
    p = subprocess.Popen(argv, stdout=open(f'{R}/box/{tag}-ps.out', 'w'), stderr=subprocess.STDOUT)
    line = watch_start(tag, p, 15); p.wait()
    print('watch start line:', line); print('box.ps1 exit:', p.returncode, '| its console output bytes:', os.path.getsize(f'{R}/box/{tag}-ps.out'))
    print(open(f'{R}/box/{tag}-box.txt', encoding='utf-8-sig').read().strip())
    sys.exit(0 if (line and p.returncode == 3 and os.path.getsize(f'{R}/box/{tag}-ps.out') == 0) else 1)
name, bid, n, total, prevv = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4], sys.argv[5]
tag = name + '-p' + (sys.argv[6] if len(sys.argv) > 6 else '1')
if used(tag): print(f'HALT: tag {tag} already used'); sys.exit(1)
def py(*a): return subprocess.run([sys.executable, '-I', '-X', 'utf8', *a], cwd=R, capture_output=True, text=True, encoding='utf-8')
def run(*a):
    r = py(*a); print(r.stdout.strip())
    if r.stderr.strip(): print(r.stderr.strip())
    if r.returncode: print(f'HALT: {a[0]} exit {r.returncode}'); sys.exit(1)
    return r.stdout
def res(f): return json.load(open(f, encoding='utf-8'))['response']['result']['structuredContent']['result']
CL = 'agent-run-' + name  # the row's own client folder (1108-R): no row's client reads an earlier row's inbox
def last(label):
    fs = sorted(f for f in os.listdir(f'{R}/{CL}/answers') if f.endswith(f'-{label}.json')); return f'{R}/{CL}/answers/{fs[-1]}'
run('args1108.py', 'batch', name, bid)
print('--- mN-before'); run('send.py', name + 'pre', 'marks.utf16le.xml'); run('chk.py', name + 'pre-marks', 'marks', name + 'pre-marks.utf16le.xml')
argv = box_argv(tag, 120, 120); print('box.ps1 argv:', repr(argv))
box = subprocess.Popen(argv, stdout=open(f'{R}/box/{tag}-ps.out', 'w'), stderr=subprocess.STDOUT)
line = watch_start(tag, box, 15)
if not line: print('HALT before post_import: box.ps1 wrote no watch start line'); sys.exit(1)
print('box.ps1 watching:', line)
chk = subprocess.Popen([sys.executable, '-I', '-X', 'utf8', 'boxcheck.py', tag, bid, n, total], cwd=R, stdout=subprocess.DEVNULL, stderr=open(f'{R}/box/{tag}-check.err', 'w'))
print(f'--- post_import start {now()}'); o = py('call.py', CL, tag, 'post_import', f'calls/args-post-{name}.json'); print(o.stdout.strip()[:800]); print(f'post_import end {now()}')
k, lab = 0, tag
while k < 8:
    r = res(last(lab)); a = r.get('approval') or {}
    if r.get('attempt_recorded'): break
    if a.get('state') == 'pending': wait = a.get('retry_after_s') or 15
    elif a.get('state') == 'approved': wait = 0
    else: break
    k += 1; lab = f'{tag}-r{k}'
    print(f'{now()} approval {a.get("state")}: retry_after_s {a.get("retry_after_s")}; calling again as {lab} after {wait} s', flush=True); time.sleep(wait)
    o = py('call.py', CL, lab, 'post_import', f'calls/args-post-{name}.json'); print(o.stdout.strip()[:800]); print(f'call end {now()}')
box.wait(); chk.wait()
print('--- box log'); print('\n'.join(l[:240] for l in open(f'{R}/box/{tag}-box.txt', encoding='utf-8-sig').read().splitlines() if 'child' not in l).replace(G, '<company guid>'))
print('--- check'); co = open(f'{R}/box/{tag}-check.out', encoding='utf-8').read().strip(); print(co.replace(G, '<company guid>'))
pf = last(lab); print('--- post answer', os.path.basename(pf)); print(py('show.py', pf).stdout.strip()[:6000])
if co.splitlines()[-2] != 'ALL PASS': print('HALT: the box check did not pass'); sys.exit(1)
print('--- capture'); run('cap.py', name + '-after')
print('--- mN-after'); run('send.py', name + 'post', 'marks.utf16le.xml'); run('chk.py', name + 'post-marks', 'marks', name + 'post-marks.utf16le.xml')
run('send.py', name + 'post', 'vouchers.utf16le.xml'); py('chk.py', name + 'post-vouchers', 'rows', name + 'post-vouchers.utf16le.xml')
run('chk.py', name + '-vdiff', 'diff', prevv, name + 'post-vouchers.utf16le.xml')
print(f'--- verify_import {now()}'); o = py('call.py', CL, name + '-verify', 'verify_import', f'calls/args-verify-{name}.json'); print(o.stdout.strip()[:400])
vf = last(name + '-verify'); print(py('show.py', vf).stdout.strip()[:6000])
