import re, hashlib, subprocess, os, sys
os.chdir('<LAB_DIR>')
rows = re.findall(r'^\| `(a-s[^`]+)` \| ([\d,]+) \| `([0-9a-f]{64})` \| (committed|lab PC) \|$', open('ROW.md', encoding='utf-8').read(), re.M)
assert len(rows) == 19, len(rows)
out = []; bad = 0
for name, size, sha, where in rows:
    if where == 'committed':
        d = subprocess.run(['git', '-C', 'D:/dev/bridge', 'show', f'afe4fbadd9a6ddafe82a1dede40e22bef4835913:lab/2464-rb-answers/answers/{name}'], capture_output=True, check=True).stdout
    else:
        d = open(f'<lab folder>/2464-rb/{name}', 'rb').read()
    ok = len(d) == int(size.replace(',', '')) and hashlib.sha256(d).hexdigest() == sha
    if not ok: bad += 1
    else: open(f'a-rb-{name}', 'xb').write(d)
    out.append(f'a-rb-{name} {len(d)} B {hashlib.sha256(d).hexdigest()} from {where}: {"equal to the sheet" if ok else "DIFFERS, not copied"}')
out.append(f'copies 19, equal {19 - bad}, differ {bad}')
open('evidence/rb-copies.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
print('\n'.join(out[-3:]))
