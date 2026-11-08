# Lab helper (2377-L Part B, not product code): B8. Copies every answer of B4 to B7 (labels b4-, b5-, b6-, b7-) with its
# headers file into evidence/b8-answers, then checks each copied answer's sha256 against its sendlog.txt line. Sends nothing.
# Usage: python -I -X utf8 b8copy.py
import hashlib, os, shutil
R = '<LAB_DIR>'; D = R + '/evidence/b8-answers'
if os.path.exists(D): raise SystemExit('evidence/b8-answers exists already: nothing copied')
os.makedirs(D)
log = [l.rstrip('\n').split('\t') for l in open(R + '/sendlog.txt', encoding='utf-8') if l.count('\t') >= 9]
rows = [f for f in log if f[2].split('-')[0] in ('b4', 'b5', 'b6', 'b7')]
ok = bad = 0; out = []
for f in rows:
    lb = f[2]; want = f[8].split()[3]
    for suf in ('.answer.xml', '.headers.txt'): shutil.copy2(f'{R}/ans/{lb}{suf}', f'{D}/{lb}{suf}')
    got = hashlib.sha256(open(f'{D}/{lb}.answer.xml', 'rb').read()).hexdigest()
    v = 'OK' if got == want else 'DIFFERS'; ok += v == 'OK'; bad += v != 'OK'
    out.append(f'{lb}.answer.xml {os.path.getsize(f"{D}/{lb}.answer.xml")} B {got} {v}')
open(R + '/evidence/b8-check.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + f'\nanswers {len(rows)}: OK {ok}, differs {bad}\n')
print(f'answers {len(rows)}: OK {ok}, differs {bad}; files copied {2 * len(rows)}')
