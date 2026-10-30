# Lab helper (#1738-COUNTS, not product code): reads out\copy-<label>.txt and checks, for each file copied, that the
# source's sha256 before, the source's after and the copy's are equal (and the sizes); prints one line per file and
# writes out\copy-check.txt. Usage: python -I -X utf8 verify.py
import os, re
W = 'C:/bridge-lab/1738c'
out = []
for lab in ['Q', 'R'] + [f'L{i}' for i in range(1, 28)]:
    t = open(f'{W}/out/copy-{lab}.txt', encoding='ascii', errors='replace').read()
    for name in ('agent-import-ledger.jsonl', 'agent-import-statement-skips.jsonl'):
        if f'== {name} absent' in t: out.append(f'{lab} {name}: absent'); continue
        parts = {}
        for tag in ('source before', 'source after', 'copy'):
            m = re.search(rf'== {re.escape(name)} {tag}\s+size (\d+)\s+SHA256 hash of .*?:\s+([0-9a-f]{{64}})', t, re.S)
            parts[tag] = (m.group(1), m.group(2)) if m else None
        ok = all(parts.values()) and len({v for v in parts.values()}) == 1
        out.append(f'{lab} {name}: ' + ('copy stands' if ok else 'DIFFERS or unread') + f'; size {parts["copy"][0] if parts["copy"] else "?"} sha256 {parts["copy"][1] if parts["copy"] else "?"}')
open(f'{W}/out/copy-check.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
print('\n'.join(out))
