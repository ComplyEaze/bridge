# Lab helper (row 2511-P, new; not product code): writes run8.py from the kit's run.py with one exact replacement, the
# request header charset=utf-16 -> charset=utf-8 (the run lead's decision before p1w: the import files Bridge saves are UTF-8
# and declare it). Checks the old text occurs once; stops if run8.py exists; prints both hashes and the line diff.
# Usage: python -I -X utf8 mk-run8.py
import difflib, hashlib, os
R = '<LAB_DIR>'
if os.path.exists(R + '/run8.py'): raise SystemExit('run8.py exists')
s = open(R + '/run.py', encoding='utf-8', newline='').read()
a = '"Content-Type: text/xml; charset=utf-16"'; b = '"Content-Type: text/xml; charset=utf-8"'
if s.count(a) != 1: raise SystemExit('not found once')
t = s.replace(a, b)
open(R + '/run8.py', 'w', encoding='utf-8', newline='').write(t)
for f in ('run.py', 'run8.py'): print(f, hashlib.sha256(open(R + '/' + f, 'rb').read()).hexdigest(), os.path.getsize(R + '/' + f), 'B')
d = list(difflib.unified_diff(s.splitlines(), t.splitlines(), 'run.py', 'run8.py', lineterm='', n=0))
print('\n'.join(d)); print('changed lines:', sum(1 for x in d if x.startswith(('-', '+')) and not x.startswith(('---', '+++'))))
