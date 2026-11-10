# Lab helper (row 2399-R, from row 2480-NM's mkresults): writes evidence/answers-sha.txt and RESULTS.md (RESULTS-head.md, then every output whole).
import os, hashlib
os.chdir('<LAB_DIR>')
NL = chr(10)
def sha(p): d = open(p, 'rb').read(); return len(d), hashlib.sha256(d).hexdigest()
lines = ['$ size and sha256 of every answer as saved on the lab PC (raw Tally answers; the client files; the extracted rpc files; the egress files)']
for d, pre in (('.', 'a-'), ('run/answers', ''), ('run2/answers', ''), ('rpc', '')):
    for n in sorted(os.listdir(d)):
        if n.startswith(pre) and (n.endswith('.xml') or n.endswith('.json')):
            p = f'{d}/{n}'.lstrip('./') if d == '.' else f'{d}/{n}'; b, h = sha(p); lines.append(f'{p} {b} B {h}')
for p in ('C:/lab/2399-r/data/agent-egress.jsonl', 'C:/lab/2399-r/data-2/agent-egress.jsonl'):
    b, h = sha(p); lines.append(f'{p} {b} B {h}')
open('evidence/answers-sha.txt', 'w', encoding='utf-8', newline=NL).write(NL.join(lines) + NL)
ev = sorted(os.listdir('evidence'))
def block(title, f):
    return [f'### {title}', '```', open('evidence/' + f, encoding='utf-8').read().rstrip(NL), '```', '']
out = [open('RESULTS-head.md', encoding='utf-8').read().rstrip(NL), '', '## Outputs, whole', '']
used = set()
SENDS = ['s1', 'se', 's2', 's4m', 's4', 's4r', 's5m', 's5', 's5r', 's8']
CALLS = ['c0', 'c1', 'c2', 'c3', 'c4', 'c4b', 'c5', 'c6', 'c7', 'c5b', 'c8a', 'c8b']
items = [('build.txt', 'step 0: the build'), ('client.txt', 'step 0: the client'), ('bookcheck.txt', "step 1: the book's data folder and the excluded GUID"),
         ('expect-s1.txt', 's1: marks against the sheet (--expect)'), ('data-before-launch.txt', 'step 3: the first data folder before the launch')]
for s in SENDS:
    items.append((f'lab-{s}.txt', f'{s}: lab.py'))
    if f'expect-{s}.txt' in ev and s != 's1': items.append((f'expect-{s}.txt', f'{s}: marks against the sheet (--expect)'))
items.append(('egress-find-c0.txt', 'c0: the first data folder after c0'))
for c in CALLS:
    for k, t in (('times', 'start and end, UTC'), ('trail', 'trail.py'), ('answer', 'answer.py')):
        if f'{k}-{c}.txt' in ev: items.append((f'{k}-{c}.txt', f'{c}: {t}'))
    if c == 'c5b': items.append(('egress-c5b.txt', 'c5b: the egress file after it'))
    if c == 'c8a': items.append(('s8-hashes.txt', 's8: every answer against the earlier answers to the same request; the egress files'))
    if c == 'c8b': items += [('data-2-before-launch.txt', 'c8b: the second data folder before the launch'), ('egress-find-c8b.txt', 'c8b: the second data folder after it'), ('close-2.txt', 'c8b: the second client closed')]
items += [('ex.txt', 'every call: the client file and the extracted file, and their JSON-equality'), ('exe-after.txt', 'the exe after the run'),
          ('tally-exe.txt', 'the program file of the Tally that answered'), ('answers-sha.txt', 'size and sha256 of every answer'), ('sends-utc.txt', "each raw send's start and answer, UTC")]
for f, title in items:
    out += block(title, f); used.add(f)
local = {'notes.txt', 'bk.txt', 'bk-before-listing.txt', 'bk-after-listing.txt', '8-data-before-restore.txt', '8-data-after-restore.txt', 's0-netstat.txt', 's0-tasklist.txt', 'c10-after-quit.txt', 'c10-tasklist.txt', 'c10-netstat.txt'}
assert set(ev) == used | local, set(ev) ^ (used | local)
out += ['## run.py logs (each raw send: local IST time to the second, request, answer file, curl exit, bytes)', '']
for s in SENDS:
    out += [f'### {s}', '```', open(f'log-{s}.txt', encoding='utf-8').read().rstrip(NL), '```', '']
out += ['## The client logs', '']
for p in ('run/client-log.txt', 'run2/client-log.txt', 'run/evidence/tools-log.txt', 'run2/evidence/tools-log.txt'):
    out += [f'### {p}', '```', open(p, encoding='utf-8').read().rstrip(NL), '```', '']
open('RESULTS.md', 'w', encoding='utf-8', newline=NL).write(NL.join(out) + NL)
print(sum(1 for _ in open('RESULTS.md', encoding='utf-8')), 'lines;', len(lines) - 1, 'answers hashed;', len(used), 'evidence blocks')
