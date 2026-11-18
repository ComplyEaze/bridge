# Lab helper (row 1381-N, new; not product code): one write of S5 to S7 by the sheet's loop. 1. marks (<w>-pre) and
# check_r1.py marks: EXPECT one COMPANY row, the book's 8 (bac2b5e9) and ALTMSTID equal to the expected M given; else stop,
# nothing sent. 2. send the write once (<w>). 3. capture Tally's window to shots/<w>-after.png at once. 4. check_r1.py import.
# 5. marks (<w>-post). 6. a1-candidate-items (a1-<w>). 7. check_r1.py diff <previous a1> <this a1>. Every check_r1.py output
# goes whole to evidence/<w>-check.txt and to the screen. Stops at the first send or read that fails.
# Usage: python -I -X utf8 loop.py <w> <request file> <expected M before> <previous a1 answer>
import os, subprocess, sys
R = os.path.dirname(os.path.abspath(__file__)).replace(os.sep, '/'); PY = [sys.executable, '-I', '-X', 'utf8']
w, req, m_exp, prev = sys.argv[1:5]
out = open(f'{R}/evidence/{w}-check.txt', 'a', encoding='utf-8', newline='\n')
def run(args, label):
    p = subprocess.run(args, capture_output=True, text=True, encoding='utf-8', cwd=R)
    txt = f'$ {label}\n{p.stdout}{p.stderr}exit {p.returncode}\n'
    out.write(txt); out.flush(); print(txt, end='', flush=True)
    return p
def send(label, f):
    p = run(PY + ['send.py', label, f], f'send.py {label} {f}')
    if p.returncode: raise SystemExit(f'stop at send {label}')
def chk(*a):
    return run(PY + ['check_r1.py'] + list(a), 'check_r1.py ' + ' '.join(a))
send(f'{w}-pre', 'requests/marks.utf16le.xml')
p = chk('marks', f'answers/a-{w}-pre.xml')
lines = [l for l in p.stdout.splitlines() if 'GUID ' in l]
if p.returncode or 'COMPANY rows: 1' not in p.stdout or len(lines) != 1 or 'GUID bac2b5e9...' not in lines[0] or not lines[0].rstrip().endswith(f'ALTMSTID {m_exp}'):
    raise SystemExit(f'stop before {w}: marks before not as expected (M {m_exp}, the book\'s 8, one row); nothing sent')
send(w, req)
s = subprocess.run(['powershell', '-NoProfile', '-Command', '[int64](Get-Process tally | Select-Object -First 1).MainWindowHandle'], capture_output=True, text=True).stdout.strip()
c = subprocess.run(['powershell', '-NoProfile', '-File', '<lab folder>/rc070/tools/capwin.ps1', '-Handle', s, '-Out', R.replace('/', '\\') + f'\\shots\\{w}-after.png'], capture_output=True, text=True)
print('capture', c.stdout.strip().splitlines()[-1:] if c.stdout.strip() else c.stderr.strip()[:200], flush=True)
chk('import', f'answers/a-{w}.xml')
if len(sys.argv) > 5 and sys.argv[5] == 'screen-checked':
    send(f'{w}-post', 'requests/marks.utf16le.xml')
    chk('marks', f'answers/a-{w}-post.xml')
    send(f'a1-{w}', 'requests/a1-candidate-items.utf16le.xml')
    chk('diff', prev, f'answers/a-a1-{w}.xml')
else:
    print('HALF DONE: the screen is checked by the operator before the reads; then run loopb.py', flush=True)
out.close()
