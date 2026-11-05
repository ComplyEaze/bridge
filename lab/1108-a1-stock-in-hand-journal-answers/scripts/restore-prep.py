# Lab helper (1108-A1, from 1392-E, from 1029-P's restore-prep.py with the folder and backup changed, and the book's folder found by its name
# in Company.1800 as bk.py does, never typed; not product code): C6's checks before the restore of BK-1108. Checks BK-1108's one file
# against the recorded bytes and sha256 (any difference: stop), writes restore-plan.txt on the lab PC (with the company number)
# and a masked copy evidence/restore-plan.txt, and keeps a listing (names, sizes, times) of THE COMPANY's data folder in
# company-folder-<when>.txt (lab PC only) with a count line in evidence/company-folder.txt.
# Usage: python -I -X utf8 restore-prep.py before|after
import datetime, hashlib, os, sys
R = '<LAB_DIR>'; BK = '<lab folder>/1108a/backups/bk'; DATA = '<Tally data folder>'
when = sys.argv[1]
NM = 'BRIDGE STOCK CREATE LAB'.encode('utf-16-le') + b'\x00\x00'
ds = [d for d in os.listdir(DATA) if os.path.isfile(os.path.join(DATA, d, 'Company.1800')) and NM in open(os.path.join(DATA, d, 'Company.1800'), 'rb').read()]
if len(ds) != 1: raise SystemExit(f'stop: {len(ds)} data folders hold THE BOOK\'s name')
num = ds[0]
SHA, SIZE = '3aba50fe98516e61059fbd471cfabb0d6f0f9cf7ed7b73465de15c007efbd465', 24219
def now(): return datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
if when == 'before':
    fs = os.listdir(BK)
    b = open(os.path.join(BK, fs[0]), 'rb').read() if len(fs) == 1 else b''
    ok = len(fs) == 1 and fs[0] == f'TDBK1800_{num}.001' and len(b) == SIZE and hashlib.sha256(b).hexdigest() == SHA
    print(f'{now()} BK-1108 folder: {len(fs)} file(s); name, bytes and sha256 equal to the record: {ok}')
    if not ok: raise SystemExit('stop: BK-1108 differs from its record')
    plan = (f'restore plan ({now()})\nfolder: <lab folder>/1108a/backups/bk\nfile: TDBK1800_{num}.001\nbytes: {SIZE}\nsha256: {SHA}\n'
            f'company number: {num}\nRestore Data Path: {DATA}\nlist must show: BRIDGE STOCK CREATE LAB ({num}), backup of 2026-11-05 15:09 local\n'
            f'prompts expected, the only ones answered: "Accept ?" = Yes; "Company data folder ({num}) already exists ... Replace ..." = Replace\n')
    open(f'{R}/restore-plan.txt', 'w', encoding='utf-8', newline='\n').write(plan)
    open(f'{R}/evidence/restore-plan.txt', 'w', encoding='utf-8', newline='\n').write(plan.replace(num, '<company number>').replace(DATA, "Tally's data folder"))
d = f'{DATA}/{num}'; rows = []
for f in sorted(os.listdir(d)):
    st = os.stat(os.path.join(d, f)); rows.append(f'{f}  {st.st_size}  {datetime.datetime.fromtimestamp(st.st_mtime):%Y-%m-%d %H:%M:%S}')
open(f'{R}/company-folder-{when}.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(rows) + '\n')
line = f'{now()} {when}: THE COMPANY\'s data folder holds {len(rows)} files, {sum(int(r.split("  ")[1]) for r in rows)} bytes in all (listing on the lab PC)'
open(f'{R}/evidence/company-folder.txt', 'a', encoding='utf-8', newline='\n').write(line + '\n'); print(line)
