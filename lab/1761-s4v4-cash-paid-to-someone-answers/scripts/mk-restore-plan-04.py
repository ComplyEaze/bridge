# Lab helper (row 1761-S4 v4, new; not product code): writes restore-plan-04.txt for step 0.4 (C6) from row 1761-S4's BK1
# folder, the fallback: the folder, the one file's name, bytes and sha256 (checked against the recorded ones), the company
# number read from the file name, the Restore Data Path and the two prompts C6 expects. Stops if the plan exists or a check
# differs. Usage: python -I -X utf8 mk-restore-plan-04.py
import hashlib, os
D = '<ROW_1761S4_DIR>/backups/BK1'; OUT = '<LAB_DIR>/restore-plan-04.txt'
SHA = 'bce8c06cc0dcf344bc4408e34e0c8374b9920518387552e916e44e0fc4c23986'; SIZE = 24241
if os.path.exists(OUT): raise SystemExit('restore-plan-04.txt exists')
fs = os.listdir(D)
if len(fs) != 1: raise SystemExit('folder holds %d files' % len(fs))
f = fs[0]; b = open(D + '/' + f, 'rb').read(); h = hashlib.sha256(b).hexdigest()
if h != SHA or len(b) != SIZE: raise SystemExit('hash or size differs')
num = f.split('_')[1].split('.')[0]
lines = ['Row 1761-S4 v4 step 0.4 (C6) restore plan: the starting book, row 1761-S4\'s masters-only BK1',
         'Source folder: <ROW_1761S4_DIR>\\backups\\BK1 (one file)',
         'File: ' + f, 'Bytes: %d' % len(b), 'sha256: ' + h,
         'Backup written: 2026-11-11 15:05 local (the Restore list must show this date and time)',
         'Company: BRIDGE CASHPAY LAB, number ' + num,
         'Restore Data Path: <Tally data folder> (Tally data folder)',
         'Source: Specify Path = the source folder above; select BRIDGE CASHPAY LAB only',
         'Prompts expected, the only ones answered: "Accept ?" = Yes; "Company data folder (' + num + ') already exists ... Replace ..." = Replace',
         'EXPECT after: MARKS first: M0 214, V0 null, reused false (row 1761-S4\'s M0b and V0b)']
open(OUT, 'w', encoding='utf-8', newline='\n').write('\n'.join(lines) + '\n')
print('written', OUT)
