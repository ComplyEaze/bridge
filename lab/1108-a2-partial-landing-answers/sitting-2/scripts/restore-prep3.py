# Lab helper (1108-A2 sitting 2, not product code; written whole by an editor): restore-prep2.py as copied into this sitting's
# folder, with this sitting's BK-A2 record (bytes, sha256, backup time) in place of the first sitting's; nothing else changed.
# C6's checks before the restore of BK-A2: checks BK-A2's one file against the recorded bytes and sha256 (any difference: stop),
# writes restore-plan.txt on the lab PC (with the company number) and a masked copy evidence/restore-plan.txt, and keeps a
# listing (names, sizes, times) of THE COMPANY's data folder in company-folder-<when>.txt (lab PC only) with a count line in
# evidence/company-folder.txt. The book's folder is found by its name in Company.1800, never typed.
# Usage: python -I -X utf8 restore-prep3.py before|after|after-load
import datetime, hashlib, os, sys
R = '<LAB_DIR>'; BK = '<LAB_DIR>/backups/bk'; DATA = '<Tally data folder>'
when = sys.argv[1]
NM = 'BRIDGE STOCK CREATE LAB'.encode('utf-16-le') + b'\x00\x00'
ds = [d for d in os.listdir(DATA) if os.path.isfile(os.path.join(DATA, d, 'Company.1800')) and NM in open(os.path.join(DATA, d, 'Company.1800'), 'rb').read()]
if len(ds) != 1: raise SystemExit(f'stop: {len(ds)} data folders hold THE BOOK\'s name')
num = ds[0]
SHA, SIZE = '8365e521e52c2f48d26cbc43a4f181a88686a91affaf3d236fca6ea8d45770f3', 24219
def now(): return datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
if when == 'before':
    fs = os.listdir(BK)
    b = open(os.path.join(BK, fs[0]), 'rb').read() if len(fs) == 1 else b''
    ok = len(fs) == 1 and fs[0] == f'TDBK1800_{num}.001' and len(b) == SIZE and hashlib.sha256(b).hexdigest() == SHA
    print(f'{now()} BK-A2 folder: {len(fs)} file(s); name, bytes and sha256 equal to the record: {ok}')
    if not ok: raise SystemExit('stop: BK-A2 differs from its record')
    plan = (f'restore plan ({now()})\nfolder: <lab folder>/1108a2s2/backups/bk\nfile: TDBK1800_{num}.001\nbytes: {SIZE}\nsha256: {SHA}\n'
            f'company number: {num}\nRestore Data Path: {DATA}\nlist must show: BRIDGE STOCK CREATE LAB ({num}), backup of 2026-11-11 12:31 local\n'
            f'prompts expected, the only ones answered: "Accept ?" = Yes; "Company data folder ({num}) already exists ... Replace ..." = Replace\n')
    open(f'{R}/restore-plan.txt', 'w', encoding='utf-8', newline='\n').write(plan)
    open(f'{R}/evidence/restore-plan.txt', 'w', encoding='utf-8', newline='\n').write(plan.replace(num, '<company number>').replace(DATA, "Tally's data folder"))
d = f'{DATA}/{num}'; rows = []
for f in sorted(os.listdir(d)):
    st = os.stat(os.path.join(d, f)); rows.append(f'{f}  {st.st_size}  {datetime.datetime.fromtimestamp(st.st_mtime):%Y-%m-%d %H:%M:%S}')
open(f'{R}/company-folder-{when}.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(rows) + '\n')
line = f'{now()} {when}: THE COMPANY\'s data folder holds {len(rows)} files, {sum(int(r.split("  ")[1]) for r in rows)} bytes in all (listing on the lab PC)'
open(f'{R}/evidence/company-folder.txt', 'a', encoding='utf-8', newline='\n').write(line + '\n'); print(line)
