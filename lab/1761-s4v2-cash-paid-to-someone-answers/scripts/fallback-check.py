# Lab helper (row 1761-S4 v2, new; not product code): step 0.6, the fallback checked. Lists row 1761-S4's BK1 folder,
# checks it holds one file named TDBK1800_<number>.001, of 24241 B, with the sha256 that row's RESULTS.md records; writes
# evidence/fallback-check.txt with the number written as <company number>, and one notes line. Reads only.
# Usage: python -I -X utf8 fallback-check.py
import hashlib, os, re, time
from datetime import datetime, timezone
R = '<LAB_DIR>'; B = '<ROW_1761S4_DIR>/backups/BK1'
SHA = 'bce8c06cc0dcf344bc4408e34e0c8374b9920518387552e916e44e0fc4c23986'; SIZE = 24241
names = os.listdir(B)
one = len(names) == 1 and re.fullmatch(r'TDBK1800_\d+\.001', names[0]) is not None
d = open(f'{B}/{names[0]}', 'rb').read() if one else b''
h = hashlib.sha256(d).hexdigest(); mt = time.strftime('%Y-%m-%d %H:%M:%S', time.localtime(os.path.getmtime(f'{B}/{names[0]}'))) if one else '-'
ok = one and h == SHA and len(d) == SIZE
txt = (f"fallback: row 1761-S4's BK1 folder, {len(names)} entries\n"
       f'TDBK1800_<company number>.001 {len(d)} B sha256 {h} written {mt} local\n'
       f'equal to that row\'s record ({SIZE} B, sha256 {SHA}): {ok}\n')
open(R + '/evidence/fallback-check.txt', 'w', encoding='utf-8', newline='\n').write(txt)
line = datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ') + f' step 0.6 the fallback checked: one file, {len(d)} B, sha256 {h}, written {mt} local; equal to row 1761-S4\'s record: {ok} (evidence/fallback-check.txt)\n'
open(R + '/evidence/notes.txt', 'a', encoding='utf-8', newline='\n').write(line); print(txt + line, end='')
