# Lab helper (2162-S13a2, not product code): records the BK folder's files (name with the company number masked, size,
# sha256, local time) in evidence/bk.txt. Usage: python -I -X utf8 bk-record.py
import datetime, hashlib, os
R = '<LAB_DIR>'; D = R + '/backups/bk'
fs = sorted(os.listdir(D)); num = fs[0].replace('TDBK1800_', '').replace('.001', '')
t = datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
o = f'{t} <LAB_DIR>/backups/bk after the backup: {len(fs)} file(s)\n'
for f in fs:
    p = os.path.join(D, f); b = open(p, 'rb').read()
    m = datetime.datetime.fromtimestamp(os.path.getmtime(p)).strftime('%Y-%m-%d %H:%M')
    o += f'{f.replace(num, "<company number>")}  {len(b)} B  sha256 {hashlib.sha256(b).hexdigest()}  local time {m}\n'
open(R + '/evidence/bk.txt', 'w', encoding='utf-8', newline='\n').write(o); print(o, end='')
