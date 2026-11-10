# Lab helper (row 2440-ED, from row 2399-D's mkresults): writes evidence/answers-sha.txt and RESULTS.md (RESULTS-head.md, then every output whole).
import os, hashlib
os.chdir('<LAB_DIR>')
NL = chr(10)
def sha(p): d = open(p, 'rb').read(); return len(d), hashlib.sha256(d).hexdigest()
lines = ["$ size and sha256 of every answer as saved on the lab PC (Tally's answers, as timed.py wrote them)"]
for n in sorted(os.listdir('.')):
    if n.startswith('a-') and n.endswith('.xml'): b, h = sha(n); lines.append(f'{n} {b} B {h}')
open('evidence/answers-sha.txt', 'w', encoding='utf-8', newline=NL).write(NL.join(lines) + NL)
ev = sorted(os.listdir('evidence'))
def block(title, f):
    return [f'### {title}', '```', open('evidence/' + f, encoding='utf-8').read().rstrip(NL), '```', '']
out = [open('RESULTS-head.md', encoding='utf-8').read().rstrip(NL), '', '## Outputs, whole', '']
items = [('bookcheck-einv.txt', "step 0: BRIDGE EINVOICE LAB's data folder and the excluded GUID"), ('bookcheck-dense.txt', "step 0: BRIDGE CORPUS DENSE's data folder and the excluded GUID"),
         ('dense-shut-before-folder-count.txt', "step 0: the dense book's folder, shut"), ('einv-shut-before-folder-count.txt', "step 0: the e-invoice book's folder, shut")]
for s, after in (('m1', 'm1'), ('s1', 's1'), ('s2', None), ('m2', 'm2')):
    items.append((f'wall-{s}.txt', f'{s}: the request check and the wall clock'))
    if after: items.append((f'count-after-{after}.txt', f'{s}: count-ed.py after it' + (' (s2 and m2 are in the output after m2)' if s == 'm2' else '')))
items += [('einv-after-shut-folder-count.txt', "C10 of the e-invoice book: its folder after the shut"), ('dense-shut-folder-count.txt', "step 2: the dense book's folder, shut"),
          ('dense-loaded-folder-count.txt', "step 2: the dense book's folder, loaded")]
for s in ('m3', 's3', 's4', 'm4'):
    items += [(f'wall-{s}.txt', f'{s}: the request check and the wall clock'), (f'count-after-{s}.txt', f'{s}: count-ed.py after it')]
items += [('dense-after-shut-folder-count.txt', "C10 of the dense book: its folder after the shut"), ('tally-exe.txt', 'the program file of the Tally that answered'),
          ('answers-sha.txt', 'size and sha256 of every answer')]
used = set()
for f, title in items:
    out += block(title, f); used.add(f)
local = {f for f in ev if f.endswith(('-folder-listing.txt', '-netstat.txt', '-tasklist.txt'))} | {'notes.txt', 'c10-after-quit.txt', 'c10-after-quit-first-command-failed.txt'}
assert set(ev) == used | local, set(ev) ^ (used | local)
out += ["## timed.py's logs (each send: UTC start, UTC end, request, answer, curl exit, then HTTP code, seconds to the first byte, seconds in all, bytes)", '']
for s in ('m1', 's1', 's2', 'm2', 'm3', 's3', 's4', 'm4'):
    out += [f'### {s}', '```', open(f'log-{s}.txt', encoding='utf-8').read().rstrip(NL), '```', '']
open('RESULTS.md', 'w', encoding='utf-8', newline=NL).write(NL.join(out) + NL)
print(sum(1 for _ in open('RESULTS.md', encoding='utf-8')), 'lines;', len(lines) - 1, 'answers hashed;', len(used), 'evidence blocks')
