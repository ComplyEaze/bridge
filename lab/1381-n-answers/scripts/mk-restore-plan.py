# Lab helper (row 1381-N, new; not product code): S8's prep before the shut (C6). 1. checks the backup folder holds one
# file with S2's bytes and sha256. 2. writes evidence/restore-plan.txt (folder, file, bytes, sha256, the company number
# read from the file name, the Restore Data Path, the time the Restore list must show, the two prompts expected).
# 3. lists the company's data folder (names, sizes, write times UTC) to evidence/s8-data-<tag>.txt.
# Stops if a file it would write exists or a check differs. Prints no company number.
# Usage: python -I -X utf8 mk-restore-plan.py plan     (steps 1 to 3, listing tagged "before")
#        python -I -X utf8 mk-restore-plan.py after    (step 3 only, listing tagged "after")
import hashlib, os, sys
from datetime import datetime, timezone
R = os.path.dirname(os.path.abspath(__file__)).replace(os.sep, '/')
D = 'C:/lab/1381-n/backup-1'; DATA = '<Tally data folder>'
SHA = '8454721f6de95211afc8fa1d76054fad25356056a153ab084934c428cd48de7b'; SIZE = 24219
mode = sys.argv[1]
fs = os.listdir(D)
if len(fs) != 1: raise SystemExit('backup folder holds %d files' % len(fs))
f = fs[0]; num = f.split('_')[1].split('.')[0]
def listing(tag):
    out = f'{R}/evidence/s8-data-{tag}.txt'
    if os.path.exists(out): raise SystemExit(out + ' exists')
    p = f'{DATA}/{num}'; rows = []
    for n in sorted(os.listdir(p)):
        st = os.stat(f'{p}/{n}')
        t = datetime.fromtimestamp(st.st_mtime, timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
        rows.append(f'{n}\t{st.st_size}\t{t}')
    open(out, 'w', encoding='utf-8', newline='\n').write(f'data folder <company number> listed {tag} the restore, {len(rows)} entries\n' + '\n'.join(rows) + '\n')
    print('written', out, len(rows), 'entries')
if mode == 'plan':
    b = open(D + '/' + f, 'rb').read(); h = hashlib.sha256(b).hexdigest()
    if h != SHA or len(b) != SIZE: raise SystemExit('backup hash or size differs')
    print('backup checked: one file,', len(b), 'B, sha256', h[:8] + '...')
    OUT = f'{R}/evidence/restore-plan.txt'
    if os.path.exists(OUT): raise SystemExit('restore-plan.txt exists')
    lines = ['Row 1381-N S8 (C6) restore plan: S2\'s backup',
             'Source folder: C:\\lab\\1381-n\\backup-1 (one file)',
             'File: ' + f, 'Bytes: %d' % len(b), 'sha256: ' + h,
             'Backup written: 2026-11-18 13:05 local (the Restore list must show 18-Nov-26, 1:05 pm)',
             'Company: BRIDGE STOCK CREATE LAB, number ' + num,
             'Restore Data Path: <Tally data folder> (Tally data folder)',
             'Source: Specify Path = the source folder above; select BRIDGE STOCK CREATE LAB only',
             'Prompts expected, the only ones answered: "Accept ?" = Yes; "Company data folder (' + num + ') already exists ... Replace ..." = Replace',
             'EXPECT after: marks (m-r) equal to m0: M 210, ALTVCHID absent; a1-r, g-r, u-r unchanged against a1, g1, u1']
    open(OUT, 'w', encoding='utf-8', newline='\n').write('\n'.join(lines) + '\n')
    print('written', OUT)
    listing('before')
elif mode == 'after':
    listing('after')
else:
    raise SystemExit('mode is plan or after')
