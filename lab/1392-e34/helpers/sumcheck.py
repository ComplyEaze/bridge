# Lab helper (1392-E34, new; not product code): checks every line of a sha256sum-form list ("<sha256>  <name>") against the
# files under one folder, read as bytes. Prints and writes one line per file and a count line; a missing file is told apart
# from a different one. Exit 0 only when every listed file is present and equal. Writes nothing but <out file>.
# Usage: python -I -X utf8 sumcheck.py <list file> <folder> <out file>
import hashlib, os, sys
lst, folder, out = sys.argv[1:4]
rows = [l.rstrip('\r\n') for l in open(lst, encoding='utf-8') if l.strip()]
lines, bad = [], 0
for l in rows:
    want, name = l[:64].lower(), l[64:].lstrip(' *')
    p = os.path.join(folder, *name.split('/'))
    if not os.path.isfile(p): lines.append(f'MISSING {name}'); bad += 1; continue
    b = open(p, 'rb').read(); got = hashlib.sha256(b).hexdigest()
    lines.append(f'{"EQUAL" if got == want else "DIFFERENT"} {name} {len(b)} B sha256 {got}'); bad += got != want
lines.append(f'{len(rows) - bad} of {len(rows)} equal')
open(out, 'w', encoding='utf-8', newline='\n').write('\n'.join(lines) + '\n'); print('\n'.join(lines))
sys.exit(0 if bad == 0 and rows else 1)
