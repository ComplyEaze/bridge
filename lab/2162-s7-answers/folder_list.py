# Lab helper (2162-S7, S6's folder-list.py with its output folder changed; not product code): lists THE BOOK's data
# folder (found by the book's name in Company.1800, UTF-16LE; its number never typed or printed): every file's name,
# size and modified time. Writes evidence/<tag>-folder-list.txt.
# Usage: python -I -X utf8 folder_list.py <tag>
import datetime, os, sys
B = '<Tally data folder>'
NM = 'BRIDGE CORPUS DENSE'.encode('utf-16-le') + b'\x00\x00'
ds = [d for d in os.listdir(B) if os.path.isfile(os.path.join(B, d, 'Company.1800')) and NM in open(os.path.join(B, d, 'Company.1800'), 'rb').read()]
assert len(ds) == 1, f'{len(ds)} folders'
D = os.path.join(B, ds[0]); L = [(n, os.stat(os.path.join(D, n))) for n in sorted(os.listdir(D))]
now = datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
rows = [f'{n}  {s.st_size}  {datetime.datetime.fromtimestamp(s.st_mtime).strftime("%Y-%m-%d %H:%M:%S")}' for n, s in L]
txt = f'{now} <Tally data folder>/<company number> ({sys.argv[1]}): {len(L)} files, {sum(s.st_size for _, s in L)} B; times local\n' + '\n'.join(rows) + '\n'
open(f'<LAB_DIR>/evidence/{sys.argv[1]}-folder-list.txt', 'w', encoding='utf-8', newline='\n').write(txt)
print(txt.splitlines()[0])
