# Lab helper (row 1381-N, new; not product code): the second half of one write's loop, run only after the operator has
# looked at the screenshot taken after the write and seen no dialog (C3). 1. marks (<w>-post) and check_r1.py marks.
# 2. a1-candidate-items (a1-<w>). 3. check_r1.py diff <previous a1> <this a1>. Outputs go whole to evidence/<w>-check.txt.
# Usage: python -I -X utf8 loopb.py <w> <previous a1 answer>
import os, subprocess, sys
R = os.path.dirname(os.path.abspath(__file__)).replace(os.sep, '/'); PY = [sys.executable, '-I', '-X', 'utf8']
w, prev = sys.argv[1:3]
out = open(f'{R}/evidence/{w}-check.txt', 'a', encoding='utf-8', newline='\n')
def run(args, label):
    p = subprocess.run(args, capture_output=True, text=True, encoding='utf-8', cwd=R)
    txt = f'$ {label}\n{p.stdout}{p.stderr}exit {p.returncode}\n'
    out.write(txt); out.flush(); print(txt, end='', flush=True)
    return p
def send(label, f):
    p = run(PY + ['send.py', label, f], f'send.py {label} {f}')
    if p.returncode: raise SystemExit(f'stop at send {label}')
out.write(f'(screen after {w} checked by the operator: no dialog)\n')
send(f'{w}-post', 'requests/marks.utf16le.xml')
run(PY + ['check_r1.py', 'marks', f'answers/a-{w}-post.xml'], f'check_r1.py marks answers/a-{w}-post.xml')
send(f'a1-{w}', 'requests/a1-candidate-items.utf16le.xml')
run(PY + ['check_r1.py', 'diff', prev, f'answers/a-a1-{w}.xml'], f'check_r1.py diff {prev} answers/a-a1-{w}.xml')
out.close()
