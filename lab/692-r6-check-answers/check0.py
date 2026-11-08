# Lab helper (692-R6 step 0, not product code): checks each kept file the sheet names against its line in
# lab/692-opening-stock-answers/SHA256.txt at 09287f70 (bytes and sha256). Prints one verdict line per file.
# Usage: python -I -X utf8 check0.py <sha256-list> <answers folder>
import hashlib, os, sys
want = {}
for l in open(sys.argv[1], encoding='utf-8-sig'):
    p = l.lstrip('# ').split()
    if len(p) == 3: want[p[0]] = (int(p[1]), p[2])
names = [f'answers/{r}-{k}.utf16le.xml' for r in ('r1-w1', 'r1-w2', 'r1-w3', 'r1-w4', 'r5-w5', 'r3-w2') for k in ('stock-summary', 'negative-stock') if not (r in ('r1-w1', 'r1-w3') and k == 'negative-stock')]
bad = 0
for n in names:
    p = os.path.join(sys.argv[2], os.path.basename(n))
    if not os.path.isfile(p): print(f'{n}: MISSING'); bad += 1; continue
    d = open(p, 'rb').read(); got = (len(d), hashlib.sha256(d).hexdigest())
    ok = want.get(n) == got; bad += not ok
    print(f'{n}: {got[0]} B {got[1]}  listed {want.get(n)}  equal: {"yes" if ok else "NO"}')
print(f'{len(names)} files, {len(names) - bad} equal, {bad} not')
sys.exit(1 if bad else 0)
