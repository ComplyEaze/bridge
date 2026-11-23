# Lab helper (1392-E34, changed from 1392-E's restore-prep.py, lab original 2959 B sha256 70738af8c3163b5a7784a3e54df9cf08d164d7d49e4cb7dbde1a84f5477e85fa;
# not product code). Changes: the row folder, the backup folder bke2, and the backup's bytes, sha256 and Restore-list time
# taken as arguments from step 1's record (BK-E2 is taken by this row, so its hash is not known when the sheet is written).
# C6's checks before the restore of BK-E2: checks its one file against the given bytes and sha256 (any difference: stop),
# writes restore-plan.txt on the lab PC (with the company number) and a masked copy evidence/restore-plan.txt, and keeps a
# listing (names, sizes, times) of THE COMPANY's data folder in company-folder-<when>.txt (lab PC only) with a count line
# in evidence/company-folder.txt. <Tally data folder> is replaced in the lab's own copy (the sheet's Files).
# Usage: python -I -X utf8 restore-prep.py before <bytes> <sha256> "<Restore list time>" | after
import datetime, hashlib, os, sys
R = 'C:/bridge-lab/1392e34'; BK = R + '/backups/bke2'; DATA = '<Tally data folder>'
when = sys.argv[1]
NM = 'BRIDGE TRANSFER LAB'.encode('utf-16-le') + b'\x00\x00'
ds = [d for d in os.listdir(DATA) if os.path.isfile(os.path.join(DATA, d, 'Company.1800')) and NM in open(os.path.join(DATA, d, 'Company.1800'), 'rb').read()]
if len(ds) != 1: raise SystemExit(f'stop: {len(ds)} data folders hold THE BOOK\'s name')
num = ds[0]
def now(): return datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
if when == 'before':
    SIZE, SHA, SHOWN = int(sys.argv[2]), sys.argv[3].lower(), sys.argv[4]
    fs = os.listdir(BK)
    b = open(os.path.join(BK, fs[0]), 'rb').read() if len(fs) == 1 else b''
    ok = len(fs) == 1 and fs[0] == f'TDBK1800_{num}.001' and len(b) == SIZE and hashlib.sha256(b).hexdigest() == SHA
    print(f'{now()} BK-E2 folder: {len(fs)} file(s); name, bytes and sha256 equal to the record: {ok}')
    if not ok: raise SystemExit('stop: BK-E2 differs from its record')
    plan = (f'restore plan ({now()})\nfolder: {BK}\nfile: TDBK1800_{num}.001\nbytes: {SIZE}\nsha256: {SHA}\n'
            f'company number: {num}\nRestore Data Path: {DATA}\nlist must show: BRIDGE TRANSFER LAB ({num}), backup of {SHOWN}\n'
            f'prompts expected, the only ones answered: "Accept ?" = Yes; "Company data folder ({num}) already exists ... Replace ..." = Replace\n')
    open(f'{R}/restore-plan.txt', 'w', encoding='utf-8', newline='\n').write(plan)
    open(f'{R}/evidence/restore-plan.txt', 'w', encoding='utf-8', newline='\n').write(plan.replace(num, '<company number>').replace(DATA, "Tally's data folder"))
d = f'{DATA}/{num}'; rows = []
for f in sorted(os.listdir(d)):
    st = os.stat(os.path.join(d, f)); rows.append(f'{f}  {st.st_size}  {datetime.datetime.fromtimestamp(st.st_mtime):%Y-%m-%d %H:%M:%S}')
open(f'{R}/company-folder-{when}.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(rows) + '\n')
line = f'{now()} {when}: THE COMPANY\'s data folder holds {len(rows)} files, {sum(int(r.split("  ")[1]) for r in rows)} bytes in all (listing on the lab PC)'
open(f'{R}/evidence/company-folder.txt', 'a', encoding='utf-8', newline='\n').write(line + '\n'); print(line)
