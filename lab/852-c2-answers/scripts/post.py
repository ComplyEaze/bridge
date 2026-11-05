# Lab helper (852-C2, from 1392-E's p.py; not product code): steps 4b and 5b, the post of one built batch. Checks the batch
# file's sha256 against the sheet's SHA256.txt again; starts box.ps1 (watching for Bridge's box), boxcheck.py (writes Yes
# only when the box names this company, GUID, batch and 50 vouchers; stderr to box/<tag>-check.err) and, with --keys, keys.ps1
# (step 4b's Down presses after Yes); sends post_import once and waits for its answer. An answer with approval.state pending
# is called again after its retry_after_s, at most until dialog_remaining_s runs out (the keys stay paused: keys.ps1 stops at
# the first answer). It stops there: the screenshots, the screen's way back, verify_import, egress_log and marks are the next
# steps. Usage: python -I -X utf8 post.py <name: e|f> <batch_id> [--keys]
import hashlib, json, os, subprocess, sys, time
R = '<LAB_DIR>'; name, bid = sys.argv[1], sys.argv[2]; keys = '--keys' in sys.argv; tag = name + '-p1'
G = '2ba6df28-d1e3-4e81-b6d2-eb50f0a693ab'
def py(*a): return subprocess.run([sys.executable, '-I', '-X', 'utf8', *a], cwd=R, capture_output=True, text=True, encoding='utf-8')
def res(f): return json.load(open(f, encoding='utf-8'))['response']['result'].get('structuredContent', {}).get('result') or {}
def last(label):
    fs = sorted(f for f in os.listdir(R + '/agent-run/answers') if f.endswith(f'-{label}.json')); return f'{R}/agent-run/answers/{fs[-1]}'
bf = f'{R}/lab/852-c2/batches/c50{name}.json'; h = hashlib.sha256(open(bf, 'rb').read()).hexdigest()
w = [l.split()[0] for l in open(f'{R}/lab/852-c2/SHA256.txt', encoding='utf-8') if l.rstrip().endswith(f'batches/c50{name}.json')]
print(f'c50{name}.json sha256 {h} = SHA256.txt: {w == [h]}')
if w != [h]: sys.exit('HALT: batch file differs from SHA256.txt')
for p in (f'{R}/box/{tag}-box.txt', f'{R}/box/{tag}.answer'):
    if os.path.exists(p): sys.exit(f'HALT: {p} exists (a new tag is needed)')
json.dump({'company_guid': G, 'batch_id': bid}, open(f'{R}/calls/args-post-{name}.json', 'w', encoding='utf-8', newline='\n'))
box = subprocess.Popen(['powershell', '-NoProfile', '-File', R + '/box.ps1', '-RowDir', '<LAB_DIR>', '-Tag', tag, '-SeeWithin', '120', '-AnswerBy', '120'],
                       stdout=open(f'{R}/box/{tag}-ps.out', 'w'), stderr=subprocess.STDOUT)
kp = subprocess.Popen(['powershell', '-NoProfile', '-File', R + '/keys.ps1', '-RowDir', '<LAB_DIR>', '-Tag', tag],
                      stdout=open(f'{R}/box/{tag}-keys-ps.out', 'w'), stderr=subprocess.STDOUT) if keys else None
time.sleep(2)
chk = subprocess.Popen([sys.executable, '-I', '-X', 'utf8', 'boxcheck.py', tag, bid, '50'], cwd=R, stdout=subprocess.DEVNULL, stderr=open(f'{R}/box/{tag}-check.err', 'w'))
o = py('call.py', 'agent-run', tag, 'post_import', f'calls/args-post-{name}.json'); print(o.stdout.strip()[:400], flush=True)
k, lab, deadline = 0, tag, None
while k < 8:
    r = res(last(lab)); a = r.get('approval') or {}
    if a.get('state') != 'pending' or r.get('attempt_recorded'): break
    wait = a.get('retry_after_s') or 15
    if deadline is None: deadline = time.monotonic() + (a.get('dialog_remaining_s') or 0)
    if time.monotonic() + wait > deadline: print('pending, but dialog_remaining_s would run out before retry_after_s: no further call'); break
    k += 1; lab = f'{tag}-r{k}'
    print(f'pending: retry_after_s {wait}, dialog_remaining_s {a.get("dialog_remaining_s")}; calling again as {lab}', flush=True); time.sleep(wait)
    o = py('call.py', 'agent-run', lab, 'post_import', f'calls/args-post-{name}.json'); print(o.stdout.strip()[:400], flush=True)
box.wait(); chk.wait()
if kp: kp.wait()
print('--- box log'); print('\n'.join(l[:200] for l in open(f'{R}/box/{tag}-box.txt', encoding='utf-8-sig').read().splitlines() if 'child' not in l and 'Company number' not in l))
print('--- check'); print(open(f'{R}/box/{tag}-check.out', encoding='utf-8').read().strip() if os.path.exists(f'{R}/box/{tag}-check.out') else '(no check output)')
if keys and os.path.exists(f'{R}/box/{tag}-keys.txt'):
    kl = open(f'{R}/box/{tag}-keys.txt', encoding='utf-8-sig').read().splitlines(); ks = [l for l in kl if ' KEY ' in l]
    print('--- keys', len(ks), 'presses; first', ks[0][:24] if ks else '-', 'last', ks[-1][:24] if ks else '-'); print('\n'.join(l for l in kl if ' KEY ' not in l))
pf = last(lab); r = res(pf); d = r.get('dispatch') or {}
print('--- post answer', os.path.basename(pf)); print(py('show.py', pf).stdout.strip()[:3000])
ok = d.get('state') == 'posted_verified' and (d.get('counters') or {}).get('created') == 50
print('PLAIN PATH' if ok else 'HALT after post: not the plain path'); sys.exit(0 if ok else 1)
