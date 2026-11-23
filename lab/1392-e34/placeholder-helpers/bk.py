# Lab helper (1392-E34, changed from 1392-E's bk.py, lab original 1296 B sha256 3e1903933db225877ac0339ea50c4eda1d040c6dc40061dcfee5501d382a95b8;
# not product code). Changes: the row folder and the backup folder bke2 only. Step 1's record of BK-E2. Finds THE BOOK's
# data folder by its name in Company.1800 (UTF-16LE), so the folder's number is never typed or printed; lists the backup
# folder's files by size and sha256, writing the company number as <company number>. Writes evidence/s1-bk.txt.
# <Tally data folder> is replaced in the lab's own copy (the sheet's Files). Usage: python -I -X utf8 bk.py
import datetime, hashlib, os
B = '<Tally data folder>'; K = 'C:/bridge-lab/1392e34/backups/bke2'
NM = 'BRIDGE TRANSFER LAB'.encode('utf-16-le') + b'\x00\x00'
ds = [d for d in os.listdir(B) if os.path.isfile(os.path.join(B, d, 'Company.1800')) and NM in open(os.path.join(B, d, 'Company.1800'), 'rb').read()]
out = [f'{datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")} folders holding THE BOOK\'s name: {len(ds)}']
if len(ds) != 1: raise SystemExit('\n'.join(out) + '\nstop: not exactly one data folder holds THE BOOK\'s name')
ks = sorted(os.listdir(K)); out.append(f'backup folder <BK folder> holds {len(ks)} file(s)')
for n in ks:
    b = open(os.path.join(K, n), 'rb').read()
    out.append(f'{n.replace(ds[0], "<company number>")} {len(b)} B sha256 {hashlib.sha256(b).hexdigest()} (name is TDBK1800_ + this book\'s folder + .001: {n == "TDBK1800_" + ds[0] + ".001"})')
open('C:/bridge-lab/1392e34/evidence/s1-bk.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
print('\n'.join(out))
