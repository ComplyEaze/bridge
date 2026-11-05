# Lab helper (852-C2, not product code): copies 1392-E's helpers with the folder, book and GUID changed and makes the row's
# folders. Usage: python -I -X utf8 setup.py
import hashlib, os, shutil
S, R = '<lab folder>/1392e/', '<LAB_DIR>/'
for d in ('evidence', 'reads', 'shots', 'box/selftest', 'calls', 'agent', 'backups/bkc2', 'agent-run/inbox', 'agent-run/answers', 'agent-run/step0', 'agent-run/evidence'):
    os.makedirs(R + d, exist_ok=True)
G_OLD, G_NEW = 'd6e4c24f-8c7a-4797-93ec-e44bdff60bc9', '2ba6df28-d1e3-4e81-b6d2-eb50f0a693ab'
for n in ('marks.py', 'c2.py', 'cap.py', 'capwin2.ps1', 'front.ps1', 'box.ps1', 'boxcheck.py', 'call.py', 'bk.py', 'show.py', 'selftest-box.py'):
    t = open(S + n, encoding='utf-8').read()
    t = t.replace('<lab folder>/1392e/backups/bke', R + 'backups/bkc2').replace('<lab folder>/1392e', R.rstrip('/')).replace('<lab folder>\\\\1392e\\\\', '<LAB_DIR>\\\\')
    t = t.replace(G_OLD, G_NEW).replace('BRIDGE TRANSFER LAB', 'BRIDGE LARGE LAB').replace("L + 'run/", "L + 'lab/852-c2/").replace('(1392-E, from', '(852-C2, from 1392-E, from').replace('(1392-E, not', '(852-C2, from 1392-E, not')
    open(R + n, 'w', encoding='utf-8', newline='\n').write(t)
c = open('<lab folder>/1029r/run/client.py', 'rb').read(); assert hashlib.sha256(c).hexdigest().startswith('0ed0bb03'); open(R + 'agent-run/client.py', 'wb').write(c)
for n in sorted(os.listdir(R)):
    if n.endswith(('.py', '.ps1')) and n != 'setup.py': print(n, os.path.getsize(R + n), 'still names 1392e' if '1392e' in open(R + n, encoding='utf-8').read() else 'ok')
