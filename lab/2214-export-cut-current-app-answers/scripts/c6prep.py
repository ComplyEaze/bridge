# Lab helper (2214-cut Close, 19 Nov, from the 2074-2214 W row's; not product code): LAB-COMMON C6's checks before a restore, and its listing
# after. "before": checks bk\ holds one file and that its bytes and sha256 are as recorded at S0, writes restore-plan.txt,
# and lists THE COMPANY's own data folder (names, sizes, UTC times) to evidence/c6-folder-before.txt. "after": the same
# listing to evidence/c6-folder-after.txt and the differences from before. The company number is taken from the backup's
# file name and written only as <company number>. Nothing is changed in Tally's folders.
# Usage: python -I -X utf8 c6prep.py before|after
import hashlib, os, sys, time
R = '<lab folder>'; DATA = '<Tally data folder>'
WANT = ('7cec76828a020ee8496ad4d842ae163a794e34723f575c3ab8fa20a9ada592cb', 57180)
names = os.listdir(R + '/bk')
if len(names) != 1: raise SystemExit(f'stop: bk holds {len(names)} entries')
n = names[0]
if not (n.startswith('TDBK1800_') and n.endswith('.001')): raise SystemExit('stop: unexpected backup name')
num = n[len('TDBK1800_'):-len('.001')]
b = open(f'{R}/bk/{n}', 'rb').read(); h = hashlib.sha256(b).hexdigest()
print('backup TDBK1800_<company number>.001', len(b), h, 'as recorded' if (h, len(b)) == WANT else 'DIFFERS')
if (h, len(b)) != WANT: raise SystemExit('stop: backup differs from S0')
def listing():
    out = []
    for root, dirs, files in os.walk(f'{DATA}/{num}'):
        for f in sorted(files):
            p = os.path.join(root, f); st = os.stat(p)
            rel = os.path.relpath(p, f'{DATA}/{num}').replace('\\', '/')
            out.append(f"{rel}\t{st.st_size}\t{time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime(st.st_mtime))}")
    return out
mode = sys.argv[1]
L = listing()
open(f'{R}/evidence/c6-folder-{mode}.txt', 'w', encoding='utf-8', newline='\n').write(
    f'# THE COMPANY data folder <Tally data folder>/<company number>, {mode} the restore, read {time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())}\n' + '\n'.join(L) + '\n')
print(mode, 'listing files', len(L))
if mode == 'before':
    plan = (f'restore plan, 2214-cut Close, written {time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())}\n'
            f'folder: <lab folder>\\bk (one file)\nfile: TDBK1800_<company number>.001\nbytes: {len(b)}\nsha256: {h}\n'
            'company: BRIDGE BATCH LAB, number <company number> (its own data folder name)\n'
            'backup taken: 2026-11-19 17:28 local (11:58:12Z); the Restore list should show 19-Nov-26 at that time\n'
            'Restore Data Path: Tally data folder <Tally data folder> (<Tally data folder>)\n'
            'source: Specify Path, <lab folder>\\bk\n'
            'prompts expected: "Accept ?" = Yes; "Company data folder (<company number>) already exists ... Replace will overwrite the existing folder." = Replace\n'
            'before it: Company > Shut THE COMPANY\n')
    open(f'{R}/evidence/restore-plan.txt', 'w', encoding='utf-8', newline='\n').write(plan); print(plan)
else:
    B = {l.split('\t')[0]: l for l in open(f'{R}/evidence/c6-folder-before.txt', encoding='utf-8').read().splitlines()[1:] if l}
    A = {l.split('\t')[0]: l for l in L}
    for k in sorted(set(B) | set(A)):
        if B.get(k) != A.get(k): print('before:', B.get(k), '| after:', A.get(k))
