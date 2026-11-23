# Lab helper (1392-E34, new; from 1392-E's selftest-run.py and selftest-box.py; not product code): the box check's self-test
# before the first post of this row. It runs THIS row's boxcheck.py on copies of 1392-E's saved box logs (5 Nov 2026,
# kept on the lab PC in that row's folder) and compares each answer with the one 1392-E's own self-test gave. Nothing
# is written outside <new row folder>/box/selftest/; the saved logs and build answers are only read.
# boxcheck.py runs as written except two strings: its box folder becomes box/selftest, and the folder whose build answers
# it reads for a one-voucher box becomes 1392-E's agent-run/answers (where B3's build answer of 5 Nov is).
# Usage: python -I -X utf8 selftest34.py <1392-E row folder> <this row folder>     (both with forward slashes)
import os, subprocess, sys, time
B3 = 'bridge-519a6289-40d3-4980-99ca-f7ed2e44c063'; F = 'bridge-e4abb1d3-6ef6-4211-8cb4-185d35013a4a'
B1 = 'bridge-66b8afb1-e6ed-4d20-bd45-598113b0e544'
CASES = (('t11', 'b3-p1', B3, 1, 'Yes'), ('t12', 'b3-p2', B3, 1, 'Yes'), ('t13', 'b3-p2', B3, 2, 'No'),
         ('t14', 'f-p1', F, 3, 'Yes'), ('t15', 'f-p1', F, 1, 'No'), ('t16', 'b1-p1', B1, 2, 'Yes'), ('t16b', 'b1-p1', B1, 3, 'No'))
BOX_LINE = "D = 'C:/bridge-lab/1392e34/box'"
GLOB_ROOT = "'C:/bridge-lab/1392e34/agent-run/answers/"
if sys.argv[1] == 'one':  # one case, in its own process, as boxcheck.py runs: one <old> <new> <tag> <batch> <count>
    old, new = sys.argv[2], sys.argv[3]
    src = open(f'{new}/boxcheck.py', encoding='utf-8').read()
    if src.count(BOX_LINE) != 1 or src.count(GLOB_ROOT) != 1: sys.exit('HALT: boxcheck.py does not hold the two strings once each')
    src = src.replace(BOX_LINE, f"D = '{new}/box/selftest'").replace(GLOB_ROOT, f"'{old}/agent-run/answers/")
    sys.argv = ['boxcheck.py'] + sys.argv[4:]
    exec(compile(src, 'boxcheck.py', 'exec'))
old, new = sys.argv[1], sys.argv[2]; T = f'{new}/box/selftest'
os.makedirs(T, exist_ok=True)
saved = [f'{old}/box/{n}-box.txt' for n in sorted({c[1] for c in CASES})]
missing = [p for p in saved if not os.path.isfile(p)]
if missing: sys.exit(f'HALT: {len(missing)} saved box log(s) missing: ' + ', '.join(os.path.basename(p) for p in missing))
def start(tag, bid, n):
    for x in ('-box.txt', '.answer', '-check.out', '-check.err'):
        if os.path.exists(f'{T}/{tag}{x}'): os.remove(f'{T}/{tag}{x}')
    return subprocess.Popen([sys.executable, '-I', '-X', 'utf8', os.path.abspath(__file__), 'one', old, new, tag, bid, str(n)],
                            stdout=subprocess.DEVNULL, stderr=open(f'{T}/{tag}-check.err', 'w'))
def result(tag, p, want, note):
    rc = p.wait(); a = open(f'{T}/{tag}.answer').read() if os.path.exists(f'{T}/{tag}.answer') else '(none)'
    line = f'{tag}: expected {want}, got {a}, exit {rc} {note}' + (' OK' if a == want else ' WRONG')
    print(line, flush=True); return line
out = []
for tag, name, bid, cnt, want in CASES:
    p = start(tag, bid, cnt); open(f'{T}/{tag}-box.txt', 'wb').write(open(f'{old}/box/{name}-box.txt', 'rb').read())
    out.append(result(tag, p, want, f'(saved log {name}, count {cnt})'))
# t19: a log cut inside its WAITING line and never finished must not give Yes (No after 30 s of reads not whole, exit 2)
d = open(f'{old}/box/b3-p2-box.txt', 'rb').read(); w = d.index(b'WAITING for'); w = d.rindex(b'\n', 0, w) + 1
p = start('t19', B3, 1); open(f'{T}/t19-box.txt', 'wb').write(d[:w] + d[w:w + 20])
out.append(result('t19', p, 'No', '(cut inside the WAITING line, never finished)'))
open(T + '/selftest34-results.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
print('ALL CASES RIGHT' if all(l.endswith(' OK') for l in out) else 'A CASE IS WRONG')
sys.exit(0 if all(l.endswith(' OK') for l in out) else 1)
