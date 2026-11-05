# Lab helper (1029-P, from 1381-M's bk-record.py with the folder changed, not product code): records BK-P's folder (name with the
# company number masked, size, sha256, local time) in evidence/bkp.txt; the unmasked record goes to bkp-local.txt (lab PC only).
# Usage: python -I -X utf8 bk-record.py
import datetime, hashlib, os
R = '<LAB_DIR>'; D = R + '/backups/bkp'
fs = sorted(os.listdir(D)); num = open(R + '/company-number.txt').read().strip()
t = datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
o = f'{t} <LAB_DIR>/backups/bkp after the backup: {len(fs)} file(s)\n'; loc = o
for f in fs:
    p = os.path.join(D, f); b = open(p, 'rb').read()
    m = datetime.datetime.fromtimestamp(os.path.getmtime(p)).strftime('%Y-%m-%d %H:%M')
    line = f'  {len(b)} B  sha256 {hashlib.sha256(b).hexdigest()}  local time {m}\n'
    o += f.replace(num, '<company number>') + line; loc += f + line
open(R + '/evidence/bkp.txt', 'w', encoding='utf-8', newline='\n').write(o)
open(R + '/bkp-local.txt', 'w', encoding='utf-8', newline='\n').write(loc); print(o, end='')
