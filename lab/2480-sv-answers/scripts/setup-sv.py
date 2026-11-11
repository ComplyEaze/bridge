# Lab helper (row 2480-SV, not product code; written whole by an editor): makes the row's lab side from row 2480-SC's. Checks
# every file of SHA256.txt (sha256 and size) first, and stops on any difference. Copies SC's lab helpers with the folder
# name 2480-sc turned into 2480-sv and nothing else (each file's count of changes and new sha256 printed), and SC's
# evidence/gateway-refs.txt (the Gateway captures SC looked at) byte for byte as this row's starting list. Makes evidence/,
# shots/ and the new backup folder C:/lab/2480-sv/backup-1, each of which must not exist or be empty. Removes nothing.
# Usage: python -I -X utf8 setup-sv.py
import hashlib, os
S, R = '<lab folder>/2480-sc/', '<LAB_DIR>/'
def sha(p): return hashlib.sha256(open(p, 'rb').read()).hexdigest()
bad = n = 0
for l in open(R + 'SHA256.txt', encoding='utf-8'):
    if not l.strip() or l.startswith('#'): continue
    h, size, path = l.rstrip('\n').split('  ', 2); n += 1
    p = R + path
    if not os.path.isfile(p) or os.path.getsize(p) != int(size) or sha(p) != h: bad += 1; print('DIFFERS', path)
print(f'SHA256.txt: {n} files, {bad} differ')
if bad: raise SystemExit('stop: the sheet folder differs from its list')
for d in ('evidence', 'shots', 'C:/lab/2480-sv/backup-1'):
    p = d if d.startswith('C:') else R + d
    os.makedirs(p, exist_ok=True)
    if os.listdir(p): raise SystemExit(f'stop: {p} is not empty')
HELPERS = ['bg.sh', 'wd.sh', 'send.sh', 'shot.sh', 'w.sh', 'ctl.sh', 'pair.sh', 's2chk.sh', 's6.sh', 'side.sh', 'prep-c6.sh', 'er.sh', 'step0.py']
for h in HELPERS:
    if os.path.exists(R + h): raise SystemExit(f'stop: {h} exists here')
    t = open(S + h, encoding='utf-8').read()
    if '2480-sv' in t: raise SystemExit(f'stop: {h} already names 2480-sv')
    k = t.count('2480-sc'); t = t.replace('2480-sc', '2480-sv')
    open(R + h, 'w', encoding='utf-8', newline='\n').write(t)
    print(f'{h}: {k} folder name(s) changed, sha256 {sha(R + h)}')
open(R + 'evidence/gateway-refs.txt', 'wb').write(open(S + 'evidence/gateway-refs.txt', 'rb').read())
print('evidence/gateway-refs.txt from SC, sha256', sha(R + 'evidence/gateway-refs.txt'))
for h in HELPERS:
    t = open(R + h, encoding='utf-8').read()
    print(h, 'NAMES SC' if '2480-sc' in t.lower() and 'row 2480-sc' not in t.lower() else 'ok')
