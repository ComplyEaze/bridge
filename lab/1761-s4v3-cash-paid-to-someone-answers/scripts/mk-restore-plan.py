# Lab helper (row 1761-S4 v3, new; not product code): writes restore-plan.txt for step 6 (C6) from this row's BK1 folder:
# the folder, the one file's name, bytes and sha256 (checked against the recorded ones), the company number read from the
# file name, the Restore Data Path and the two prompts C6 expects. Stops if the plan exists or a check differs.
# Usage: python -I -X utf8 mk-restore-plan.py
import hashlib, os
D = '<LAB_DIR>/backups/BK1'; OUT = '<LAB_DIR>/restore-plan.txt'
SHA = 'dde2e47ffa5d7431d296791ce97429482d8b3e19964c38c39053128cfd8bd15f'; SIZE = 25676
if os.path.exists(OUT): raise SystemExit('restore-plan.txt exists')
fs = os.listdir(D)
if len(fs) != 1: raise SystemExit('folder holds %d files' % len(fs))
f = fs[0]; b = open(D + '/' + f, 'rb').read(); h = hashlib.sha256(b).hexdigest()
if h != SHA or len(b) != SIZE: raise SystemExit('hash or size differs')
num = f.split('_')[1].split('.')[0]
lines = ['Row 1761-S4 v3 step 6 (C6) restore plan',
         'Source folder: <LAB_DIR>\\backups\\BK1 (one file)',
         'File: ' + f, 'Bytes: %d' % len(b), 'sha256: ' + h,
         'Backup written: 2026-11-12 11:39 local (the Restore list must show this date and time)',
         'Company: BRIDGE CASHPAY LAB, number ' + num,
         'Restore Data Path: <Tally data folder> (Tally data folder)',
         'Source: Specify Path = the source folder above; select BRIDGE CASHPAY LAB only',
         'Prompts expected, the only ones answered: "Accept ?" = Yes; "Company data folder (' + num + ') already exists ... Replace ..." = Replace',
         'EXPECT after: M 214, V 1 (M0b, V0b); then list_companies; vouchers 2025-04-01..2025-04-01 exactly the seed; vouchers 2025-04-08..2025-04-09 no voucher']
open(OUT, 'w', encoding='utf-8', newline='\n').write('\n'.join(lines) + '\n')
print('written', OUT)
