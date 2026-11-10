# Lab helper (row 2440-ED, not product code): counts the files in a book's data folder, found by its name and GUID (the number is never
# printed). Writes evidence/<label>-folder-count.txt (the count only) and keeps the names, sizes and times in evidence/<label>-folder-listing.txt (lab PC only).
# Usage: python -I -X utf8 fcount.py <label> <number file>
import os, sys, time
R = '<LAB_DIR>'; label, nf = sys.argv[1], sys.argv[2]
n = open(nf).read().strip(); d = f'<Tally data folder>/{n}'
t = time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()); names = sorted(os.listdir(d))
files = [x for x in names if os.path.isfile(f'{d}/{x}')]
open(f'{R}/evidence/{label}-folder-listing.txt', 'w', encoding='utf-8', newline='\n').write(''.join(f'{x} {os.path.getsize(f"{d}/{x}")} {time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(os.path.getmtime(f"{d}/{x}")))}\n' for x in files))
line = f'{t} {label}: the book\'s data folder holds files={len(files)}, other entries={len(names) - len(files)}'
open(f'{R}/evidence/{label}-folder-count.txt', 'w', encoding='utf-8', newline='\n').write(line + '\n'); print(line)
