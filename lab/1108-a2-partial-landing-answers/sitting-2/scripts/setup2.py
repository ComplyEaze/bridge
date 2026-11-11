# Lab helper (1108-A2 sitting 2, not product code): makes the second sitting's folder from the first's. Copies the sheet
# export (lab/1108-a2, the sheet's own batch files) whole, and the first sitting's helpers with the row folder 1108a2 turned
# into 1108a2s2 and nothing else; 1392-E's stdio client and box watcher byte for byte; THE COMPANY's GUID and number records
# (lab PC only). p1108.py, p1108b.py and restore-prep.py are not copied (p1108s2.py and restore-prep2.py stand for them).
# Makes the sitting's empty folders. Prints each copied file's sha256 and whether it still names the first sitting's folder.
# Usage: python -I -X utf8 setup2.py
import hashlib, os, shutil
S, R = '<lab folder>/1108a2/', '<LAB_DIR>/'
for d in ('evidence', 'answers', 'shots', 'box', 'backups/bk', 'agent-run/inbox', 'agent-run/answers', 'agent-run/step0', 'agent-run/evidence', 'agent', 'calls'):
    os.makedirs(R + d, exist_ok=True)
for d in ('backups/bk', 'agent', 'agent-run/inbox', 'agent-run/answers', 'box', 'answers'):
    if os.listdir(R + d): raise SystemExit(f'stop: {d} is not empty')
if os.path.exists(R + 'lab'): raise SystemExit('stop: lab/ exists')
shutil.copytree(S + 'lab', R + 'lab')
def sha(p): return hashlib.sha256(open(p, 'rb').read()).hexdigest()
for n in ('send.py', 'chk.py', 'bk-record.py', 'c2.py', 'cap.py', 'capwin2.ps1', 'front.ps1', 'restore-prep2.py', 'boxcheck.py', 'args1108.py', 'call.py', 'show.py'):
    t = open(S + n, encoding='utf-8').read()
    if '1108a2s2' in t: raise SystemExit(f'stop: {n} already names 1108a2s2')
    k = t.count('1108a2'); t = t.replace('1108a2', '1108a2s2')
    open(R + n, 'w', encoding='utf-8', newline='\n').write(t)
    print(f'{n}: {k} folder name(s) changed, sha256 {sha(R + n)}')
c = open('<lab folder>/1392e/agent-run/client.py', 'rb').read(); assert hashlib.sha256(c).hexdigest().startswith('0ed0bb03'); open(R + 'agent-run/client.py', 'wb').write(c)
shutil.copyfile(S + 'box.ps1', R + 'box.ps1'); assert sha(R + 'box.ps1') == sha('<lab folder>/1392e/box.ps1')
for n in ('guid.txt', 'company-number.txt'): shutil.copyfile(S + n, R + n)
print('client.py', sha(R + 'agent-run/client.py')); print('box.ps1', sha(R + 'box.ps1'))
for n in sorted(os.listdir(R)):
    if n.endswith(('.py', '.ps1')) and n != 'setup2.py':
        t = open(R + n, encoding='utf-8').read()
        print(n, 'NAMES THE FIRST SITTING' if ('1108a2/' in t or "1108a2'" in t or '1108a2\\' in t) else 'ok')
