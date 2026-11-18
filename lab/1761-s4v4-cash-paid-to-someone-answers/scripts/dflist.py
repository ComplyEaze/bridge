# Lab helper (row 1761-S4 v4, new; not product code): lists THE COMPANY's Tally data folder (the folder named by the company
# number in row 1761-S4's BK1 file name) to evidence/<name>.txt, one line per entry: bytes, modified time (local), name.
# Reads only. Stops if the output exists. Usage: python -I -X utf8 dflist.py <name>
import os, sys
from datetime import datetime
OUT = '<LAB_DIR>/evidence/' + sys.argv[1] + '.txt'
if os.path.exists(OUT): raise SystemExit(OUT + ' exists')
num = os.listdir('<ROW_1761S4_DIR>/backups/BK1')[0].split('_')[1].split('.')[0]
D = '<Tally data folder>/' + num
rows = []
for n in ['.', '..'] + sorted(os.listdir(D)):
    p = os.path.join(D, n); st = os.stat(p)
    rows.append('%d %s %s' % (0 if os.path.isdir(p) else st.st_size, datetime.fromtimestamp(st.st_mtime).strftime('%Y-%m-%dT%H:%M:%S'), n))
open(OUT, 'w', encoding='utf-8', newline='\n').write('\n'.join(rows) + '\n')
print('written', OUT, len(rows) - 2, 'entries')
