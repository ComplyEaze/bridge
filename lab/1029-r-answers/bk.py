# Lab helper (1029-R, not product code): step 1's record of BK. Finds THE BOOK's data folder by its name in Company.1800
# (UTF-16LE), so the folder's number is never typed or printed; lists the backup folder's files by size and sha256, writing
# the company number as <company number>. Writes evidence/s1-bk.txt. Usage: python -I -X utf8 bk.py
import datetime, hashlib, os
B = '<Tally data folder>'; K = '<BK folder>'
NM = 'BRIDGE LARGE LAB'.encode('utf-16-le') + b'\x00\x00'
ds = [d for d in os.listdir(B) if os.path.isfile(os.path.join(B, d, 'Company.1800')) and NM in open(os.path.join(B, d, 'Company.1800'), 'rb').read()]
out = [f'{datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")} folders holding THE BOOK\'s name: {len(ds)}']
ks = sorted(os.listdir(K)); out.append(f'backup folder <BK folder> holds {len(ks)} file(s)')
for n in ks:
    b = open(os.path.join(K, n), 'rb').read()
    out.append(f'{n.replace(ds[0], "<company number>")} {len(b)} B sha256 {hashlib.sha256(b).hexdigest()} (name is TDBK1800_ + this book\'s folder + .001: {n == "TDBK1800_" + ds[0] + ".001"})')
open('<LAB_DIR>/evidence/s1-bk.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
print('\n'.join(out))
