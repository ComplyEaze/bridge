# Lab helper (rows 1108-R0 to R2, new; not product code): C6 prep before a shut, written whole by an editor. Re-hashes the
# backup folder (exactly one file, its bytes and sha256 equal to those recorded at the backup) and writes
# <LAB_DIR>/restore-plan-<tag>.txt (folder, file, bytes, sha256, company number, the Restore Data Path, the two
# prompts C6 answers and the reads after) and evidence/c6-datafolder-<tag>-before.txt, the listing of THE COMPANY's data
# folder (bytes, modified time, name). Stops if a file exists or a check differs. Reads Tally's files only; changes none.
# Usage: python -I -X utf8 c6prep.py <tag> <bk0|bkr> <sha256> <bytes>
import hashlib, os, sys
from datetime import datetime
tag, bk, want_sha, want_size = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4])
R = '<LAB_DIR>'; D = f'{R}/backups/{bk}'
num = open(R + '/company-number.txt').read().strip()
plan = f'{R}/restore-plan-{tag}.txt'; lst = f'{R}/evidence/c6-datafolder-{tag}-before.txt'
for p in (plan, lst):
    if os.path.exists(p): raise SystemExit(p + ' exists')
fs = os.listdir(D)
if len(fs) != 1: raise SystemExit(f'{D} holds {len(fs)} files')
b = open(f'{D}/{fs[0]}', 'rb').read(); h = hashlib.sha256(b).hexdigest()
if h != want_sha or len(b) != want_size: raise SystemExit(f'differs: {len(b)} B {h}')
mt = datetime.fromtimestamp(os.path.getmtime(f'{D}/{fs[0]}')).strftime('%Y-%m-%d %H:%M')
lines = [f'Rows 1108-R C6 restore plan ({tag}): {bk}', f'Source folder: <LAB_DIR>\\backups\\{bk} (one file)',
         'File: ' + fs[0], f'Bytes: {len(b)}', 'sha256: ' + h, f'Backup written: {mt} local (the Restore list must show this date and time)',
         'Company: BRIDGE STOCK CREATE LAB, number ' + num,
         'Restore Data Path: <Tally data folder> (Tally data folder)',
         'Source: Specify Path = the source folder above; select BRIDGE STOCK CREATE LAB only; End of List',
         'Prompts expected, the only ones answered: "Accept ?" = y; "Company data folder (' + num + ') already exists ... Replace ..." = Replace',
         'After: load THE COMPANY alone (filter, one row, Enter); then marks, ledgers and vouchers as the step EXPECTs']
open(plan, 'w', encoding='utf-8', newline='\n').write('\n'.join(lines) + '\n')
F = '<Tally data folder>/' + num; rows = []
for n in ['.', '..'] + sorted(os.listdir(F)):
    p = os.path.join(F, n); st = os.stat(p)
    rows.append('%d %s %s' % (0 if os.path.isdir(p) else st.st_size, datetime.fromtimestamp(st.st_mtime).strftime('%Y-%m-%dT%H:%M:%S'), n))
open(lst, 'w', encoding='utf-8', newline='\n').write('\n'.join(rows) + '\n')
print(f'{bk}: one file, {len(b)} B, sha256 {h} (equal to recorded), written {mt} local; plan {plan}; data folder listing {len(rows) - 2} entries')
