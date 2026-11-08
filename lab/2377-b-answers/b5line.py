# Lab helper (2377-L Part B, not product code): writes one notes line for a finished B5 class, read from sendlog.txt and
# from the kit's scripts run again on the kept answers (counters, lo, present, tie), with the capture shas from shots/.
# Sends nothing and presses no key. Usage: python -I -X utf8 b5line.py <class>
import hashlib, os, subprocess, sys
R = '<LAB_DIR>'; K = R + '/kit-99c3/lab/2377-b'; c = sys.argv[1]; p = f'b5-{c}'
log = {}
for l in open(R + '/sendlog.txt', encoding='utf-8'):
    f = l.rstrip('\n').split('\t')
    if len(f) > 9: log[f[2]] = f
def run(*a):
    r = subprocess.run([sys.executable, '-I', '-X', 'utf8', *a], capture_output=True, text=True, encoding='utf-8'); return (r.stdout + r.stderr).strip()
def ans(lb): f = log[lb]; return f'{f[0][11:19]}Z {f[8].split()[1]} B {f[8].split()[3][:8]}...'
def mk(lb):
    o = run(R + '/marks.py', f'{R}/ans/{lb}.answer.xml').splitlines()
    return f'{log[lb][0][11:19]}Z ' + o[0].split(', ', 2)[2] + ('' if o[-1] == 'COMPANY rows: 1' else ' ' + o[-1])
def shot(n):
    d = open(f'{R}/shots/{n}', 'rb').read(); return f'{n} {hashlib.sha256(d).hexdigest()[:8]}...'
parts = [f'B5 {c}.']
for w in ('w1', 'w2'):
    lb = f'{p}-{w}'
    if lb not in log: parts.append(f'{w}: not sent.'); continue
    parts.append(f'{w}: marks {mk(lb + "-marks-1")}; sent {ans(lb)}; {shot(lb + "-after.png")}; counters "{run(K + "/l2377b.py", "counters", f"{R}/ans/{lb}.answer.xml")}"; marks {mk(lb + "-marks-2")}.')
if f'{p}-r-lo' in log:
    parts.append(f'r-lo-{c} {ans(p + "-r-lo")}; lo "{run(K + "/l2377.py", "lo", f"{R}/ans/{p}-r-lo.answer.xml")}".')
if f'{p}-snap' in log:
    pr = run(K + '/l2377b.py', 'present', f'{R}/ans/{p}-snap.answer.xml').splitlines()
    ex = [l.split()[0] for l in pr if l.split()[1] == 'exact']; other = [l for l in pr if l.split()[1] not in ('exact', 'absent')]
    parts.append(f'snap {ans(p + "-snap")}; present exact: {" ".join(ex)}; ' + ('other lines: ' + ' | '.join(other) if other else 'every other line absent') + ';')
    if f'{p}-r-lo' in log: parts.append(f'tie "{run(K + "/l2377b.py", "tie", c, f"{R}/ans/{p}-r-lo.answer.xml", f"{R}/ans/{p}-snap.answer.xml")}".')
line = ' '.join(parts)
open(R + '/notes.txt', 'a', encoding='utf-8', newline='\n').write(line + '\n'); print(line)
