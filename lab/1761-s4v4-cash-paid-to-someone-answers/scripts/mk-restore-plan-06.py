# Lab helper (row 1761-S4 v4, new; not product code): writes restore-plan-06.txt for step 6.1 (C6) from this row's BK1
# folder: the one file's name, bytes and sha256 (checked against the ones step 0.10 recorded), the company number read from
# the file name, the Restore Data Path and the two prompts C6 expects. Stops if the plan exists or a check differs.
# Usage: python -I -X utf8 mk-restore-plan-06.py
import hashlib, os
D = '<LAB_DIR>/backups/BK1'; OUT = '<LAB_DIR>/restore-plan-06.txt'
SHA = '79be45dd05c25c1eeb16b94f4a9dc76de4e1dbc78f03a48b48909f16db26a9c0'; SIZE = 25675
if os.path.exists(OUT): raise SystemExit('restore-plan-06.txt exists')
fs = os.listdir(D)
if len(fs) != 1: raise SystemExit('folder holds %d files' % len(fs))
f = fs[0]; b = open(D + '/' + f, 'rb').read(); h = hashlib.sha256(b).hexdigest()
if h != SHA or len(b) != SIZE: raise SystemExit('hash or size differs')
num = f.split('_')[1].split('.')[0]
lines = ['Row 1761-S4 v4 step 6.1 (C6) restore plan: this row\'s BK1 (the masters and the seed)',
         'Source folder: <LAB_DIR>\\backups\\BK1 (one file)',
         'File: ' + f, 'Bytes: %d' % len(b), 'sha256: ' + h,
         'Backup written: 2026-11-18 12:48 local (the Restore list must show this date and time)',
         'Company: BRIDGE CASHPAY LAB, number ' + num,
         'Restore Data Path: <Tally data folder> (Tally data folder)',
         'Source: Specify Path = the source folder above; select BRIDGE CASHPAY LAB only',
         'Prompts expected, the only ones answered: "Accept ?" = Yes; "Company data folder (' + num + ') already exists ... Replace ..." = Replace',
         'EXPECT after: MARKS first: M 214, V 1, reused false (M0b and V0b)']
open(OUT, 'w', encoding='utf-8', newline='\n').write('\n'.join(lines) + '\n')
print('written', OUT)
