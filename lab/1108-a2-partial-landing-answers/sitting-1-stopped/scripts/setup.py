# Lab helper (1108-A2, not product code): copies 1108-A1's helpers with the folder, sheet, backup folder and write files changed,
# and 1392-E's stdio client and box watcher unchanged; makes the row's folders; copies A1's records of THE COMPANY's GUID and
# number (lab PC only). Usage: python -I -X utf8 setup.py
import hashlib, os, shutil
S, R = '<lab folder>/1108a1/', '<LAB_DIR>/'
for d in ('evidence', 'answers', 'shots', 'box', 'backups/bk', 'agent-run/inbox', 'agent-run/answers', 'agent-run/step0', 'agent'):
    os.makedirs(R + d, exist_ok=True)
for n in ('send.py', 'chk.py', 'bk-record.py', 'c2.py', 'cap.py', 'capwin2.ps1', 'front.ps1', 'restore-prep.py'):
    t = open(S + n, encoding='utf-8').read()
    t = t.replace('(1108-A1, ', '(1108-A2, from 1108-A1, ').replace('(1108-A1,', '(1108-A2, from 1108-A1,')
    t = t.replace('/lab/1108-a1/', '/lab/1108-a2/').replace("cwd=R + '/lab/1108-a1'", "cwd=R + '/lab/1108-a2'")
    t = t.replace("re.match(r'(w1|w2|a1|b)-', f)", "re.match(r'(w1|w2|w3|s0)-', f)")
    t = t.replace('<lab folder>/1108a/backups/bk', '<LAB_DIR>/backups/bk').replace('<lab folder>/1108a/backups/bk', '<lab folder>/1108a2/backups/bk')
    t = t.replace('<lab folder>/1108a1', '<LAB_DIR>').replace('<lab folder>\\\\1108a1\\\\', '<LAB_DIR>\\\\')
    open(R + n, 'w', encoding='utf-8', newline='\n').write(t)
c = open('<lab folder>/1392e/agent-run/client.py', 'rb').read(); assert hashlib.sha256(c).hexdigest().startswith('0ed0bb03'); open(R + 'agent-run/client.py', 'wb').write(c)
shutil.copyfile('<lab folder>/1392e/box.ps1', R + 'box.ps1')
for n in ('guid.txt', 'company-number.txt'): shutil.copyfile(S + n, R + n)
for n in sorted(os.listdir(R)):
    if n.endswith(('.py', '.ps1')) and n != 'setup.py':
        t = open(R + n, encoding='utf-8').read(); print(n, 'names 1108a1 or 1108a/' if ('1108a1' in t or '1108a/' in t or '1108-a1' in t) else 'ok', '| w-rule' if 'w3|s0' in t else '')
