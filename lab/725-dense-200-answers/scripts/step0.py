# Lab helper (725-D1, not product code): step 0's file checks. Finds THE BOOK's data folder by its name in Company.1800
# (UTF-16LE), so the folder's number is never typed or printed; counts its files and bytes; hashes BK0; prints the PC's
# date and time. Writes evidence/s0-folder.txt. Usage: python -I -X utf8 step0.py
import datetime, hashlib, os
B = '<Tally data folder>'
NM = 'BRIDGE CORPUS DENSE'.encode('utf-16-le') + b'\x00\x00'
ds = [d for d in os.listdir(B) if os.path.isfile(os.path.join(B, d, 'Company.1800')) and NM in open(os.path.join(B, d, 'Company.1800'), 'rb').read()]
out = [f'{datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")} PC local date/time {datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S")} (IST)',
       f'folders holding THE BOOK\'s name: {len(ds)}']
D = os.path.join(B, ds[0]); L = [(n, os.stat(os.path.join(D, n))) for n in sorted(os.listdir(D))]
tot = sum(s.st_size for _, s in L)
out.append(f'<Tally data folder>/<company number>: {len(L)} files, {tot} B ({tot / 2**20:.1f} MiB); newest modified {datetime.datetime.fromtimestamp(max(s.st_mtime for _, s in L)).strftime("%Y-%m-%d %H:%M:%S")} local')
K = '<BK0 folder>'; ks = os.listdir(K)
for n in ks:
    b = open(os.path.join(K, n), 'rb').read()
    out.append(f'BK0 folder holds {len(ks)} file(s): TDBK1800_<company number>.001 {len(b)} B sha256 {hashlib.sha256(b).hexdigest()} (name matches the folder: {n == "TDBK1800_" + ds[0] + ".001"})')
open('<LAB_DIR>/evidence/s0-folder.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
print('\n'.join(out))
