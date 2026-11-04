# Lab helper (2162-S13a2, copied from 2162-S13's with its folder changed; not product code): one of the sheet's step lines. Checks each request file's sha256 and size
# against the sheet's SHA256.txt (any difference: stop, nothing sent), then runs the sheet's own run.py with the
# gateway URL, the label and the files, then the sheet's count.py on each answer, in the order sent. Both scripts run
# from run/, a byte-equal copy of the sheet. count.py's whole output is kept as evidence/count-<label>.txt.
# Usage: python -I -X utf8 step.py <label> <file>...
import hashlib, pathlib, subprocess, sys
from datetime import datetime, timezone
R = pathlib.Path('<LAB_DIR>'); RUN = R / 'run'; URL = 'http://127.0.0.1:9001'
label, files = sys.argv[1], sys.argv[2:]
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
listed = {l.split('  ')[2]: (l.split('  ')[0], int(l.split('  ')[1])) for l in (R / 'sheet/SHA256.txt').read_text().splitlines() if l}
for f in files:
    d = (RUN / f).read_bytes()
    if listed.get(f) != (hashlib.sha256(d).hexdigest(), len(d)): raise SystemExit(f'stop: {f} differs from SHA256.txt; nothing sent')
print(f'{now()} {label}: {len(files)} request file(s) equal to SHA256.txt by sha256 and size')
r = subprocess.run([sys.executable, '-I', 'run.py', URL, label] + files, cwd=RUN)
print(f'{now()} run.py exit {r.returncode}')
if r.returncode: raise SystemExit('stop: run.py stopped')
answers = [f'a-{label}-{f}' for f in files]
for a in answers:
    d = (RUN / a).read_bytes(); print(f'  {a} {len(d)} B sha256 {hashlib.sha256(d).hexdigest()}')
c = subprocess.run([sys.executable, '-I', 'count.py'] + answers, cwd=RUN, capture_output=True, text=True, encoding='utf-8')
out = c.stdout + (c.stderr or '')
(R / f'evidence/count-{label}.txt').write_text(f'python -I count.py {" ".join(answers)}\nexit {c.returncode}\n{out}', encoding='utf-8', newline='\n')
print(out, end=''); print(f'count.py exit {c.returncode}')
if c.returncode: raise SystemExit('stop: count.py stopped')
