# Lab helper (1392-E, not product code): the box check's self-test on copies of saved box logs, in box/selftest/, before
# B3's third post. Each case runs boxcheck.py through selftest-box.py (its folder moved to box/selftest) and compares the
# answer with the one expected. Writer cases append a saved log while the check reads it: in chunks with a multibyte
# character and a line cut across writes (WAITING last), held under an exclusive lock, or cut and never finished.
# Usage: python -I -X utf8 selftest-run.py
import os, subprocess, sys, time
R = '<LAB_DIR>'; T = R + '/box/selftest'; B3 = 'bridge-519a6289-40d3-4980-99ca-f7ed2e44c063'
F = 'bridge-e4abb1d3-6ef6-4211-8cb4-185d35013a4a'; B1 = 'bridge-66b8afb1-e6ed-4d20-bd45-598113b0e544'
def src(n): return open(f'{R}/box/{n}-box.txt', 'rb').read()
def start(tag, bid, n):
    for x in ('-box.txt', '.answer', '-check.out', '-check.err'):
        if os.path.exists(f'{T}/{tag}{x}'): os.remove(f'{T}/{tag}{x}')
    return subprocess.Popen([sys.executable, '-I', '-X', 'utf8', R + '/selftest-box.py', tag, bid, str(n)], stdout=subprocess.DEVNULL, stderr=open(f'{T}/{tag}-check.err', 'w'))
def result(tag, p, want, note=''):
    rc = p.wait(); a = open(f'{T}/{tag}.answer').read() if os.path.exists(f'{T}/{tag}.answer') else '(none)'
    errs = sum(1 for l in open(f'{T}/{tag}-check.err', encoding='utf-8', errors='replace') if 'not whole' in l)
    line = f'{tag}: expected {want}, got {a}, exit {rc}, reads not whole {errs} {note}'.rstrip() + (' OK' if a == want else ' WRONG')
    print(line); return line
out = []
for tag, n_, bid, cnt, want in (('t11', 'b3-p1', B3, 1, 'Yes'), ('t12', 'b3-p2', B3, 1, 'Yes'), ('t13', 'b3-p2', B3, 2, 'No'),
                                ('t14', 'f-p1', F, 3, 'Yes'), ('t15', 'f-p1', F, 1, 'No'), ('t16', 'b1-p1', B1, 2, 'Yes'), ('t16b', 'b1-p1', B1, 3, 'No')):
    p = start(tag, bid, cnt); open(f'{T}/{tag}-box.txt', 'wb').write(src(n_)); out.append(result(tag, p, want, f'(saved log {n_}, count {cnt})'))
# t17: appended in chunks while read; a cut inside a multibyte character and inside a line; WAITING written last
d = src('b3-p2'); w = d.index(b'WAITING for'); w = d.rindex(b'\n', 0, w) + 1; head, tail = d[:w], d[w:]
dash = head.index('\u2014'.encode('utf-8')) + 1
cuts = sorted({0, dash, dash + 400, len(head) // 2 + 7, len(head)}); p = start('t17', B3, 1)
with open(f'{T}/t17-box.txt', 'ab') as f:
    for a, b in zip(cuts, cuts[1:]): f.write(head[a:b]); f.flush(); time.sleep(1.2)
    time.sleep(2); f.write(tail[:9]); f.flush(); time.sleep(1.5); f.write(tail[9:]); f.flush(); done = time.time()
r = result('t17', p, 'Yes', '(appended in chunks; the answer must come after the last write)')
out.append(r + (' answer-after-last-write' if os.path.getmtime(f'{T}/t17.answer') >= done else ' ANSWER-BEFORE-LAST-WRITE'))
# t18: the whole log written, then held under an exclusive lock for 5 s while the check polls
p = start('t18', B3, 1); open(f'{T}/t18-box.txt', 'wb').write(b'')
lock = subprocess.Popen(['powershell', '-NoProfile', '-Command', f"$f=[IO.File]::Open('{T}/t18-box.txt','Open','ReadWrite','None'); $b=[IO.File]::ReadAllBytes('{R}/box/b3-p2-box.txt'); $f.Write($b,0,$b.Length); $f.Flush(); Start-Sleep -Seconds 5; $f.Close()"])
lock.wait(); out.append(result('t18', p, 'Yes', '(held under an exclusive lock for 5 s)'))
# t19: cut inside the WAITING line and never finished: must not say Yes (No after 30 s of reads not whole)
p = start('t19', B3, 1); open(f'{T}/t19-box.txt', 'wb').write(head + tail[:20])
out.append(result('t19', p, 'No', '(cut inside the WAITING line, never finished)'))
open(T + '/selftest-results.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
print('ALL CASES RIGHT' if all(l.endswith(('OK', 'answer-after-last-write')) and 'WRONG' not in l and 'BEFORE' not in l for l in out) else 'A CASE IS WRONG')
