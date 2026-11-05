# Lab helper (1392-E, from 1029-P's restore-prep.py with the folder and backup changed, and the book's folder found by its name
# in Company.1800 as bk.py does, never typed; not product code): C6's checks before the restore of BK-E. Checks BK-E's one file
# against the recorded bytes and sha256 (any difference: stop), writes restore-plan.txt on the lab PC (with the company number)
# and a masked copy evidence/restore-plan.txt, and keeps a listing (names, sizes, times) of THE COMPANY's data folder in
# company-folder-<when>.txt (lab PC only) with a count line in evidence/company-folder.txt.
# Usage: python -I -X utf8 restore-prep.py before|after
import datetime, hashlib, os, sys
R = '<LAB_DIR>'; BK = R + '/backups/bke'; DATA = '<Tally data folder>'
when = sys.argv[1]
NM = 'BRIDGE TRANSFER LAB'.encode('utf-16-le') + b'\x00\x00'
ds = [d for d in os.listdir(DATA) if os.path.isfile(os.path.join(DATA, d, 'Company.1800')) and NM in open(os.path.join(DATA, d, 'Company.1800'), 'rb').read()]
if len(ds) != 1: raise SystemExit(f'stop: {len(ds)} data folders hold THE BOOK\'s name')
num = ds[0]
SHA, SIZE = '29c2de73fe7d383f5c388d92ff193e39ea9f65b59a57c8f812891dabf457cbb5', 32428
def now(): return datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
if when == 'before':
    fs = os.listdir(BK)
    b = open(os.path.join(BK, fs[0]), 'rb').read() if len(fs) == 1 else b''
    ok = len(fs) == 1 and fs[0] == f'TDBK1800_{num}.001' and len(b) == SIZE and hashlib.sha256(b).hexdigest() == SHA
    print(f'{now()} BK-E folder: {len(fs)} file(s); name, bytes and sha256 equal to the record: {ok}')
    if not ok: raise SystemExit('stop: BK-E differs from its record')
    plan = (f'restore plan ({now()})\nfolder: <LAB_DIR>/backups/bke\nfile: TDBK1800_{num}.001\nbytes: {SIZE}\nsha256: {SHA}\n'
            f'company number: {num}\nRestore Data Path: {DATA}\nlist must show: BRIDGE TRANSFER LAB ({num}), backup of 2026-11-05 13:55 local\n'
            f'prompts expected, the only ones answered: "Accept ?" = Yes; "Company data folder ({num}) already exists ... Replace ..." = Replace\n')
    open(f'{R}/restore-plan.txt', 'w', encoding='utf-8', newline='\n').write(plan)
    open(f'{R}/evidence/restore-plan.txt', 'w', encoding='utf-8', newline='\n').write(plan.replace(num, '<company number>').replace(DATA, "Tally's data folder"))
d = f'{DATA}/{num}'; rows = []
for f in sorted(os.listdir(d)):
    st = os.stat(os.path.join(d, f)); rows.append(f'{f}  {st.st_size}  {datetime.datetime.fromtimestamp(st.st_mtime):%Y-%m-%d %H:%M:%S}')
open(f'{R}/company-folder-{when}.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(rows) + '\n')
line = f'{now()} {when}: THE COMPANY\'s data folder holds {len(rows)} files, {sum(int(r.split("  ")[1]) for r in rows)} bytes in all (listing on the lab PC)'
open(f'{R}/evidence/company-folder.txt', 'a', encoding='utf-8', newline='\n').write(line + '\n'); print(line)
