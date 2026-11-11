# Lab helper (row 2507-T, new; from row 2346-N134's prep-c6-2.sh, in Python; not product code): C6's prep before a restore of B0.
# Re-hashes B0 and checks it equals the step 0 record, checks its folder holds only that file, writes restore-plan.txt (and a copy
# restore-plan-<part>.txt; both name the company number, so they stay on the lab PC), lists the company's data folder to
# evidence/<part>-df-before-restore.txt (local) and appends one line to evidence/notes.txt. Sends nothing to 9001 and presses no key.
# Usage: python -I -X utf8 prep-c6.py <part>
import hashlib, os, re, sys, time
from datetime import datetime, timezone
R = '<LAB_DIR>'; B = R + '/backups/B0'; P = sys.argv[1]
B0_SHA = 'fbe17805840636daf30aa7af199da1ac3526f904ae48cae2b565310fcde79410'; B0_SIZE = 115721
names = os.listdir(B)
if len(names) != 1: raise SystemExit(f'B0 holds {len(names)} entries: stop')
f = names[0]; num = re.fullmatch(r'TDBK1800_(\d+)\.001', f).group(1)
d = open(f'{B}/{f}', 'rb').read()
if hashlib.sha256(d).hexdigest() != B0_SHA or len(d) != B0_SIZE: raise SystemExit('B0 differs from its step 0 record: stop')
D = f'<Tally data folder>/{num}'
rows = [f'{n} {os.path.getsize(D + "/" + n)} B {time.strftime("%Y-%m-%d %H:%M:%S", time.localtime(os.path.getmtime(D + "/" + n)))}' for n in sorted(os.listdir(D))]
open(f'{R}/evidence/{P}-df-before-restore.txt', 'w', encoding='utf-8', newline='\n').write(f'company data folder <company number>, {len(rows)} entries, before the restore after {P}\n' + '\n'.join(rows) + '\n')
t = datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
plan = (f'Written {t}, before the restore after {P} (C6).\n'
        f'folder <LAB_DIR>\\backups\\B0 (holds this one file), file {f}, {len(d)} B, sha256 {B0_SHA} (equal to the step 0 record)\n'
        f'company number {num}, BRIDGE SHAPE LAB, GUID 3a6bd6e1-b835-4bff-89dd-8a6af138c346\n'
        'Restore Data Path <Tally data folder>; source: Specify Path, the B0 folder\n'
        'the Restore list must show BRIDGE SHAPE LAB, its number, and B0\'s time (11-Nov-26, about 3:51 pm)\n'
        f'two prompts expected, the only ones answered: "Accept ?" = y; "Company data folder ({num}) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." = r (Replace)\n'
        f'data folder listing before the restore: evidence/{P}-df-before-restore.txt ({len(rows)} entries)\n')
open(R + '/restore-plan.txt', 'w', encoding='utf-8', newline='\n').write(plan)
open(f'{R}/restore-plan-{P}.txt', 'w', encoding='utf-8', newline='\n').write(plan)
line = (datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z' +
        f' {P} C6 prep: B0 re-hashed equal to the step 0 record ({len(d)} B), its folder holds only that file; restore-plan.txt written (copy restore-plan-{P}.txt, local); the data folder listed (evidence/{P}-df-before-restore.txt, {len(rows)} entries, local)\n')
open(R + '/evidence/notes.txt', 'a', encoding='utf-8', newline='\n').write(line)
print(plan.replace(num, '<company number>'))
