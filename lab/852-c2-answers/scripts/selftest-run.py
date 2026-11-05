# Lab helper (852-C2, from 1392-E's selftest-run.py; not product code): the box check's self-test before step 4, on copies of
# 852-C1's saved 50-voucher box logs (<lab folder>\852c1\box\a, b) and 1029-R's (r50a, r50b, r25), in box/selftest/. Each
# case runs boxcheck.py through selftest-box.py (its folder moved to box/selftest) and compares the answer with the one
# expected; writer cases append a log while the check reads it (chunks with a cut inside a multibyte character and inside a
# line, WAITING last), hold it under an exclusive lock, or cut it and never finish it.
# Usage: python -I -X utf8 selftest-run.py
import os, subprocess, sys, time
R = '<LAB_DIR>'; T = R + '/box/selftest'
LOGS = {'c1a': ('<lab folder>/852c1/box/a-box.txt', 'bridge-ae5034d3-c28d-427a-9251-c9b6718d0dac'),
        'c1b': ('<lab folder>/852c1/box/b-box.txt', 'bridge-08d8a991-7c7f-4dc6-82b6-cca1867729f5')}
for k in ('r50a', 'r50b', 'r25'):
    t = open(f'<lab folder>/1029r/box/{k}-p1-box.txt', encoding='utf-8-sig').read()
    LOGS[k] = (f'<lab folder>/1029r/box/{k}-p1-box.txt', [l.split('Batch: ')[1].strip() for l in t.splitlines() if l.startswith('Batch: ')][0])
def src(k): return open(LOGS[k][0], 'rb').read()
def start(tag, bid, n):
    for x in ('-box.txt', '.answer', '-check.out', '-check.err'):
        if os.path.exists(f'{T}/{tag}{x}'): os.remove(f'{T}/{tag}{x}')
    return subprocess.Popen([sys.executable, '-I', '-X', 'utf8', R + '/selftest-box.py', tag, bid, str(n)], stdout=subprocess.DEVNULL, stderr=open(f'{T}/{tag}-check.err', 'w'))
def result(tag, p, want, note=''):
    rc = p.wait(); a = open(f'{T}/{tag}.answer').read() if os.path.exists(f'{T}/{tag}.answer') else '(none)'
    errs = sum(1 for l in open(f'{T}/{tag}-check.err', encoding='utf-8', errors='replace') if 'not whole' in l)
    line = f'{tag}: expected {want}, got {a}, exit {rc}, reads not whole {errs} {note}'.rstrip() + (' OK' if a == want else ' WRONG')
    print(line, flush=True); return line
out = []
cases = [('s1', 'c1a', None, 50, 'Yes'), ('s2', 'c1b', None, 50, 'Yes'), ('s3', 'r50a', None, 50, 'Yes'), ('s4', 'r50b', None, 50, 'Yes'),
         ('s5', 'c1a', 'c1b', 50, 'No'), ('s6', 'r25', None, 25, 'No'), ('s7', 'r25', None, 50, 'No'), ('s8', 'c1a', None, 49, 'No')]
for tag, k, other, cnt, want in cases:
    bid = LOGS[other][1] if other else LOGS[k][1]
    p = start(tag, bid, cnt); open(f'{T}/{tag}-box.txt', 'wb').write(src(k))
    out.append(result(tag, p, want, f'(saved log {k}, batch of {other or k}, count {cnt})'))
d = src('c1a'); w = d.index(b'WAITING for'); w = d.rindex(b'\n', 0, w) + 1; head, tail = d[:w], d[w:]
dash = head.index('\u2014'.encode('utf-8')) + 1
cuts = sorted({0, dash, dash + 400, len(head) // 2 + 7, len(head)}); p = start('s9', LOGS['c1a'][1], 50)
with open(f'{T}/s9-box.txt', 'ab') as f:
    for a, b in zip(cuts, cuts[1:]): f.write(head[a:b]); f.flush(); time.sleep(1.2)
    time.sleep(2); f.write(tail[:9]); f.flush(); time.sleep(1.5); f.write(tail[9:]); f.flush(); done = time.time()
r = result('s9', p, 'Yes', '(appended in chunks; the answer must come after the last write)')
out.append(r + (' answer-after-last-write' if os.path.getmtime(f'{T}/s9.answer') >= done else ' ANSWER-BEFORE-LAST-WRITE'))
p = start('s10', LOGS['c1a'][1], 50); open(f'{T}/s10-box.txt', 'wb').write(b'')
lock = subprocess.Popen(['powershell', '-NoProfile', '-Command', f"$f=[IO.File]::Open('{T}/s10-box.txt','Open','ReadWrite','None'); $b=[IO.File]::ReadAllBytes('{LOGS['c1a'][0]}'); $f.Write($b,0,$b.Length); $f.Flush(); Start-Sleep -Seconds 5; $f.Close()"])
lock.wait(); out.append(result('s10', p, 'Yes', '(held under an exclusive lock for 5 s)'))
p = start('s11', LOGS['c1a'][1], 50); open(f'{T}/s11-box.txt', 'wb').write(head + tail[:20])
out.append(result('s11', p, 'No', '(cut inside the WAITING line, never finished)'))
open(T + '/selftest-results.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
print('ALL CASES RIGHT' if all(' OK' in l and 'WRONG' not in l and 'BEFORE' not in l for l in out) else 'A CASE IS WRONG')
