# Lab helper (2162-S13a2, copied from 2162-S13's with its folder changed; not product code): runs the sheet's count.py with the given arguments (--before-write,
# --same-row or --diff) from run/ and keeps its whole output as evidence/<name>.txt. A non-zero exit is a stop.
# Usage: python -I -X utf8 chk.py <name> <count.py argument>...
import pathlib, subprocess, sys
from datetime import datetime, timezone
R = pathlib.Path('<LAB_DIR>')
name, args = sys.argv[1], sys.argv[2:]
t = datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
c = subprocess.run([sys.executable, '-I', 'count.py'] + args, cwd=R / 'run', capture_output=True, text=True, encoding='utf-8')
out = c.stdout + (c.stderr or '')
(R / f'evidence/{name}.txt').write_text(f'{t} python -I count.py {" ".join(args)}\nexit {c.returncode}\n{out}', encoding='utf-8', newline='\n')
print(f'{t}\n{out}', end=''); print(f'count.py exit {c.returncode}')
if c.returncode: raise SystemExit('stop: count.py stopped')
