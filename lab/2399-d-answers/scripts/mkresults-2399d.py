# Lab helper (row 2399-D, from row 2399-R's mkresults): writes evidence/answers-sha.txt and RESULTS.md (RESULTS-head.md, then every output whole).
import os, hashlib
os.chdir('<LAB_DIR>')
NL = chr(10)
def sha(p): d = open(p, 'rb').read(); return len(d), hashlib.sha256(d).hexdigest()
lines = ['$ size and sha256 of every answer as saved on the lab PC (the client files; the extracted rpc files; the egress file, and its copy taken after the close)']
for d in ('run/answers', 'run/step0', 'C:/lab/2399-d/rpc'):
    for n in sorted(os.listdir(d)):
        if n.endswith('.json'):
            p = f'{d}/{n}'; b, h = sha(p); lines.append(f'{p} {b} B {h}')
for p in ('C:/lab/2399-d/data/agent-egress.jsonl', 'egress-copy-after-close.jsonl'):
    b, h = sha(p); lines.append(f'{p} {b} B {h}')
open('evidence/answers-sha.txt', 'w', encoding='utf-8', newline=NL).write(NL.join(lines) + NL)
ev = sorted(os.listdir('evidence'))
def block(title, f):
    return [f'### {title}', '```', open('evidence/' + f, encoding='utf-8').read().rstrip(NL), '```', '']
out = [open('RESULTS-head.md', encoding='utf-8').read().rstrip(NL), '', '## Outputs, whole', '']
items = [('build.txt', 'step 0: the build'), ('exe-before.txt', 'step 0: the exe before the launch'), ('client.txt', 'step 0: the client'),
         ('bookcheck.txt', "step 0: the book's data folder and the excluded GUID"), ('data-before-launch.txt', 'step 1: the data folder before the launch'),
         ('trail-c0-python3-not-found.txt', 'c0: trail.py as the sheet spells it (python3)'), ('trail-c0.txt', 'c0: trail.py --expect list_companies 4'),
         ('trail-m0.txt', 'm0: trail.py'),
         ('trail-c1.txt', 'D1 c1: trail.py'), ('rpc-c1.txt', 'D1 c1: rpc.py'), ('answer-c1.txt', 'D1 c1: answer.py'),
         ('trail-c2.txt', 'D2 c2: trail.py'), ('rpc-c2.txt', 'D2 c2: rpc.py'), ('answer-c2.txt', 'D2 c2: answer.py'),
         ('same-items-c1-c2.txt', 'D2: answer.py --same-items c1 c2'), ('total-c1-c2.txt', 'D1 and D2: result.total and the census'),
         ('trail-m1.txt', 'm1: trail.py'), ('marks-m0-m1.txt', 'm0 and m1: the marks'),
         ('exe-after.txt', 'step 4: the exe after the close'), ('tally-exe.txt', 'the program file of the Tally that answered'),
         ('answers-sha.txt', 'size and sha256 of every answer')]
used = set()
for f, title in items:
    out += block(title, f); used.add(f)
local = {'notes.txt', 's0-netstat.txt', 's0-tasklist.txt', 'c10-after-quit.txt', 'c10-tasklist.txt', 'c10-netstat.txt'}
assert set(ev) == used | local, set(ev) ^ (used | local)
out += ['## The client log', '', '### run/client-log.txt', '```', open('run/client-log.txt', encoding='utf-8').read().rstrip(NL), '```', '']
open('RESULTS.md', 'w', encoding='utf-8', newline=NL).write(NL.join(out) + NL)
print(sum(1 for _ in open('RESULTS.md', encoding='utf-8')), 'lines;', len(lines) - 1, 'answers hashed;', len(used), 'evidence blocks')
