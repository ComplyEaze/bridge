# Lab helper (738-A, not product code): B7 and B8's record of what BUILD-L kept under <data dir>\lab\. Lists every file
# under it recursively (relative name, bytes, sha256) into evidence/<tag>-lab-listing.txt; then checks the sheet's shape:
# the folders under audit-reads\ (each named by a 64-hex sha256, holding manifest.json and a parts\ folder of plain files
# and nothing else), manifest.json's sha256 against its folder's name, and lab-manifest.jsonl's lines (company_guid and
# manifest_sha256). With two folders (B8) it also compares them: manifest.json parsed, keys that differ; each part file
# against its namesake by sha256; and the first folder's files against the B7 listing. Copies nothing out of the folders.
# Usage: python -I -X utf8 keptread.py <tag> <expected folder count> [<B7 listing file>]
import hashlib, json, os, re, sys
tag, want = sys.argv[1], int(sys.argv[2]); b7 = sys.argv[3] if len(sys.argv) > 3 else None
L = '<LAB_DIR>/agent-l/lab'; G = open('<LAB_DIR>/guid.txt', encoding='utf-8').read().strip()
def sha(p): return hashlib.sha256(open(p, 'rb').read()).hexdigest()
rows = []
for root, dirs, names in os.walk(L):
    dirs.sort()
    for n in sorted(names):
        p = os.path.join(root, n); rows.append((os.path.relpath(p, L).replace(os.sep, '/'), os.path.getsize(p), sha(p)))
out = [f'{tag}: <data dir>/lab holds {len(rows)} files'] + [f'{r}  {n}  {h}' for r, n, h in rows]
A = os.path.join(L, 'audit-reads'); fs = sorted(os.listdir(A)) if os.path.isdir(A) else []
chk = [(f'{want} folder(s) under audit-reads', len(fs) == want)]
for f in fs:
    d = os.path.join(A, f); items = sorted(os.listdir(d))
    parts = os.path.join(d, 'parts')
    plain = os.path.isdir(parts) and all(os.path.isfile(os.path.join(parts, x)) for x in os.listdir(parts))
    m = os.path.join(d, 'manifest.json')
    chk += [(f'{f[:12]}.. named by a 64-hex sha256', re.fullmatch('[0-9a-f]{64}', f) is not None),
            (f'{f[:12]}.. holds manifest.json and parts/ only', items == ['manifest.json', 'parts']),
            (f'{f[:12]}.. parts/ holds plain files only ({len(os.listdir(parts)) if os.path.isdir(parts) else 0})', plain),
            (f'{f[:12]}.. manifest.json sha256 equals the folder name', os.path.isfile(m) and sha(m) == f)]
lm = os.path.join(L, 'lab-manifest.jsonl')
lines = [json.loads(x) for x in open(lm, encoding='utf-8') if x.strip()] if os.path.isfile(lm) else []
chk += [(f'lab-manifest.jsonl has {want} line(s)', len(lines) == want),
        ('each line names THE COMPANY\'s company_guid', bool(lines) and all(x.get('company_guid') == G for x in lines)),
        ('each line\'s manifest_sha256 is one of the folders', bool(lines) and all(x.get('manifest_sha256') in fs for x in lines))]
top = sorted(os.listdir(L)); out.append(f'top level of <data dir>/lab: {top}')
if want == 2 and len(fs) == 2:
    order = [x.get('manifest_sha256') for x in lines]
    a, b = [os.path.join(A, x) for x in order]
    ma, mb = [json.load(open(os.path.join(x, 'manifest.json'), encoding='utf-8')) for x in (a, b)]
    diff = sorted(k for k in set(ma) | set(mb) if ma.get(k) != mb.get(k))
    chk.append((f'manifest.json files differ only in created_at and read_at (they differ in: {diff})', set(diff) <= {'created_at', 'read_at'}))
    pa, pb = sorted(os.listdir(os.path.join(a, 'parts'))), sorted(os.listdir(os.path.join(b, 'parts')))
    chk.append((f'the two parts/ folders hold the same names ({len(pa)} and {len(pb)})', pa == pb))
    neq = [x for x in pa if x in pb and sha(os.path.join(a, 'parts', x)) != sha(os.path.join(b, 'parts', x))]
    chk.append((f'each part byte-equal to its namesake (differing: {neq})', not neq))
    if b7:
        old = {l.split('  ')[0]: l.split('  ')[2].strip() for l in open(b7, encoding='utf-8').read().splitlines()[1:] if l.count('  ') == 2}
        now = {r: h for r, n, h in rows}
        changed = [r for r in old if r.startswith('audit-reads/') and now.get(r) != old[r]]
        chk.append((f'the first folder\'s files unchanged since B7 (changed or gone: {changed})', not changed))
out += [('PASS ' if ok else 'FAIL ') + k for k, ok in chk]
open(f'<LAB_DIR>/evidence/{tag}-lab-listing.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
print('\n'.join(out[-(len(chk) + 1):])); print(f'files listed: {len(rows)}')
