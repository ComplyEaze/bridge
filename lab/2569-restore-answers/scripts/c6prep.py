# Lab helper (2569-R step 12, from 2440-CP sitting 2's c6prep.py; not product code): LAB-COMMON C6's checks before a restore,
# and its listing after. "before": checks backups/BK1 holds one file and that its bytes and sha256 are as recorded at Part A
# step 2, writes evidence/restore-plan.txt, and lists THE COMPANY's own data folder (names, sizes, UTC times) to
# evidence/c6-folder-before.txt. "after": the same listing to evidence/c6-folder-after.txt and the differences from before.
# The company number is taken from the backup's file name and written only as <company number>. Nothing is changed in
# Tally's folders.
# Usage: python -I -X utf8 c6prep.py before|after
import hashlib, os, sys, time
R = '<lab folder>'; BK = '<lab folder>/backups/BK1'; DATA = '<Tally data folder>'
WANT = ('6092110e7cdf985fd2ae5c14061aea9297fab55a7bb0f81151692cc29d238b0b', 32101)
names = os.listdir(BK)
if len(names) != 1: raise SystemExit(f'stop: bk holds {len(names)} entries')
n = names[0]
if not (n.startswith('TDBK1800_') and n.endswith('.001')): raise SystemExit('stop: unexpected backup name')
num = n[len('TDBK1800_'):-len('.001')]
b = open(f'{BK}/{n}', 'rb').read(); h = hashlib.sha256(b).hexdigest()
print('backup TDBK1800_<company number>.001', len(b), h, 'as recorded' if (h, len(b)) == WANT else 'DIFFERS')
if (h, len(b)) != WANT: raise SystemExit('stop: backup differs from Part A step 2')
def listing():
    out = []
    for root, dirs, files in os.walk(f'{DATA}/{num}'):
        for f in sorted(files):
            p = os.path.join(root, f); st = os.stat(p)
            rel = os.path.relpath(p, f'{DATA}/{num}').replace('\\', '/')
            out.append(f"{rel}\t{st.st_size}\t{time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime(st.st_mtime))}")
    return out
mode = sys.argv[1]
out_path = f'{R}/evidence/c6-folder-{mode}.txt'
if os.path.exists(out_path): raise SystemExit(f'stop: exists {out_path}')
L = listing()
open(out_path, 'w', encoding='utf-8', newline='\n').write(
    f'# THE COMPANY data folder <Tally data folder>/<company number>, {mode} the restore, read {time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())}\n' + '\n'.join(L) + '\n')
print(mode, 'listing files', len(L))
if mode == 'before':
    plan = (f'restore plan, 2569-R step 12, written {time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())}\n'
            f'folder: <lab folder>/backups/BK1 (one file)\nfile: TDBK1800_<company number>.001\nbytes: {len(b)}\nsha256: {h}\n'
            'company: BRIDGE EINVOICE LAB, number <company number> (its own data folder name)\n'
            'backup taken: 2026-11-20, 22:37 local (17:07:37Z); the Restore list should show 20-Nov-26 at that time\n'
            'Restore Data Path: Tally data folder <Tally data folder>\n'
            'source: Specify Path, <lab folder>/backups/BK1\n'
            'prompts expected: "Accept ?" = Yes; "Company data folder (<company number>) already exists ... Replace will overwrite the existing folder." = Replace\n'
            'before it: Company > Shut BRIDGE EINVOICE LAB\n')
    open(f'{R}/evidence/restore-plan.txt', 'x', encoding='utf-8', newline='\n').write(plan); print(plan)
else:
    B = {l.split('\t')[0]: l for l in open(f'{R}/evidence/c6-folder-before.txt', encoding='utf-8').read().splitlines()[1:] if l}
    A = {l.split('\t')[0]: l for l in L}
    for k in sorted(set(B) | set(A)):
        if B.get(k) != A.get(k): print('before:', B.get(k), '| after:', A.get(k))
