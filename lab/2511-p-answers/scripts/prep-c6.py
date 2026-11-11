# Lab helper (row 2511-P; row 2507-T's prep-c6.py adapted to this row's BK, book and folders; not product code): C6's prep
# before the restore of bk-2511p, or (with "after") the data folder listing after it. Before: re-hashes the BK and checks it
# equals evidence/bk.txt, checks its folder holds only that file, writes restore-plan.txt (it names the company number, so it
# stays on the lab PC), lists the company's data folder to evidence/8-df-before-restore.txt (local) and appends one line to
# evidence/notes.txt. Sends nothing to 9001 and presses no key. Usage: python -I -X utf8 prep-c6.py before|after
import hashlib, os, re, sys, time
from datetime import datetime, timezone
R = '<LAB_DIR>'; B = 'C:/lab/bk-2511p'; W = sys.argv[1]
SHA = '645d3ef3dba9e7eb7e417b1122ee37ec23b237ded2e7db05ef49867cb6618d3e'; SIZE = 37618
names = os.listdir(B)
if len(names) != 1: raise SystemExit(f'bk-2511p holds {len(names)} entries: stop')
f = names[0]; num = re.fullmatch(r'TDBK1800_(\d+)\.001', f).group(1)
d = open(f'{B}/{f}', 'rb').read()
if hashlib.sha256(d).hexdigest() != SHA or len(d) != SIZE: raise SystemExit('the BK differs from its record: stop')
D = f'<Tally data folder>/{num}'
rows = [f'{n} {os.path.getsize(D + "/" + n)} B {time.strftime("%Y-%m-%d %H:%M:%S", time.localtime(os.path.getmtime(D + "/" + n)))}' for n in sorted(os.listdir(D))]
open(f'{R}/evidence/8-df-{W}-restore.txt', 'w', encoding='utf-8', newline='\n').write(f'company data folder <company number>, {len(rows)} entries, {W} the restore\n' + '\n'.join(rows) + '\n')
t = datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
if W == 'before':
    plan = (f'Written {t}, before the restore of step 8 (C6).\n'
            f'folder C:\\lab\\bk-2511p (holds this one file), file {f}, {len(d)} B, sha256 {SHA} (equal to evidence/bk.txt)\n'
            f'company number {num}, BRIDGE FOREXGROUP LAB, GUID 1bb48be5-baba-45b2-90d8-7593244915a9\n'
            'Restore Data Path <Tally data folder>; source: Specify Path, C:\\lab\\bk-2511p\n'
            "the Restore list must show BRIDGE FOREXGROUP LAB, its number, and the BK's time (11-Nov-26, about 7:04 pm)\n"
            f'two prompts expected, the only ones answered: "Accept ?" = y; "Company data folder ({num}) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." = r (Replace)\n'
            f'data folder listing before the restore: evidence/8-df-before-restore.txt ({len(rows)} entries)\n')
    open(R + '/restore-plan.txt', 'w', encoding='utf-8', newline='\n').write(plan)
line = (datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S') + 'Z' +
        (f' step 8 C6 prep: the BK re-hashed equal to evidence/bk.txt ({len(d)} B), its folder holds only that file; restore-plan.txt written (local); the data folder listed (evidence/8-df-before-restore.txt, {len(rows)} entries, local)\n'
         if W == 'before' else f' step 8 after the restore: the BK still equal to its record; the data folder listed (evidence/8-df-after-restore.txt, {len(rows)} entries, local)\n'))
open(R + '/evidence/notes.txt', 'a', encoding='utf-8', newline='\n').write(line); print(line, end='')
