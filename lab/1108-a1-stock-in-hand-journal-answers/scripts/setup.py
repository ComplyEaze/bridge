# Lab helper (1108-A1, not product code): copies 1381-M's helpers with the folder, sheet and checker changed, makes the row's
# folders and copies 1381-M's records of THE COMPANY's GUID and number (lab PC only). Usage: python -I -X utf8 setup.py
import os, shutil
S, R = '<lab folder>/1381sc/', '<LAB_DIR>/'
for d in ('evidence', 'answers', 'shots'): os.makedirs(R + d, exist_ok=True)
os.makedirs('<lab folder>/1108a/backups/bk', exist_ok=True)
for n in ('send.py', 'chk.py', 'bk-record.py', 'c2.py', 'cap.py', 'capwin2.ps1', 'front.ps1'):
    t = open(S + n, encoding='utf-8').read()
    t = t.replace('(1381-M, ', '(1108-A1, from 1381-M, ').replace('(1381-M,', '(1108-A1, from 1381-M,')
    t = t.replace("Q = R + '/run/requests'", "Q = R + '/lab/1108-a1/requests'").replace("R + '/run/SHA256.txt'", "R + '/lab/1108-a1/SHA256.txt'")
    t = t.replace("re.match(r'r\\d', f)", "re.match(r'(w1|w2|a1|b)-', f)")
    t = t.replace('check_1381.py', 'check_1108.py').replace("cwd=R + '/run'", "cwd=R + '/lab/1108-a1'")
    t = t.replace("D = R + '/backups/bk0'", "D = '<lab folder>/1108a/backups/bk'").replace('<LAB_DIR>/backups/bk0', '<lab folder>/1108a/backups/bk')
    t = t.replace("'/evidence/bk0.txt'", "'/evidence/bk.txt'").replace("'/bk0-local.txt'", "'/bk-local.txt'")
    t = t.replace('<lab folder>/1381sc', '<LAB_DIR>').replace('<lab folder>\\\\1381sc\\\\', '<LAB_DIR>\\\\')
    open(R + n, 'w', encoding='utf-8', newline='\n').write(t)
for n in ('guid.txt', 'company-number.txt'): shutil.copyfile(S + n, R + n)
for n in sorted(os.listdir(R)):
    if n.endswith(('.py', '.ps1')) and n != 'setup.py':
        t = open(R + n, encoding='utf-8').read(); print(n, 'names 1381sc' if '1381sc' in t else 'ok', '| r2 rule left' if 'r2-sha' in t else '')
