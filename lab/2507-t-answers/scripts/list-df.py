# Lab helper (row 2507-T, new; not product code): lists BRIDGE SHAPE LAB's data folder (names, sizes, local times) to
# evidence/<label>.txt (local: the folder name is the company number) and prints the names added and gone against another listing.
# Reads only. Usage: python -I -X utf8 list-df.py <label> <listing to compare with, under evidence/>
import os, sys, time
R = '<LAB_DIR>'; label, other = sys.argv[1], sys.argv[2]
num = [n for n in os.listdir(R + '/backups/B0')][0][9:-4]
D = f'<Tally data folder>/{num}'
rows = [f'{n} {os.path.getsize(D + "/" + n)} B {time.strftime("%Y-%m-%d %H:%M:%S", time.localtime(os.path.getmtime(D + "/" + n)))}' for n in sorted(os.listdir(D))]
open(f'{R}/evidence/{label}.txt', 'w', encoding='utf-8', newline='\n').write(f'company data folder <company number>, {len(rows)} entries, {label}\n' + '\n'.join(rows) + '\n')
a = set(l.split()[0] for l in open(f'{R}/evidence/{other}', encoding='utf-8').read().split('\n')[1:] if l)
b = set(r.split()[0] for r in rows)
print(f'{label}: {len(rows)} entries; added against {other}: {sorted(b - a)}; gone: {sorted(a - b)}')
