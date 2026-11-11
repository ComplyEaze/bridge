# Lab helper (row 2480-SV, new; not product code; written whole by an editor): writes RESULTS.md as RESULTS-head.md (written by hand)
# followed by every reader output of evidence/, whole, each under its file name in a fenced block, in the order the labels were sent
# (evidence/sends-utc.txt): after each label the outputs named for it (count-, nm-, wx-, cv-, names-, bw-, import-, alt-, altstate-,
# pair-, same-, landed-<label>.txt); after a pair's second label its --same line (same-<label less its last letter>.txt); after the last
# send of an id of step 8 its JUDGE2 lines; after s2, s6kb and s9 the file-work outputs of those steps; then the rest of the reader outputs.
# The notes, the send log, the run logs, the Gateway list, the timings and the folder, process and port listings are left out (they are
# files of evidence/ in their own right, or stay on the lab PC). Reads only; RESULTS.md must not exist yet. Usage: python -I -X utf8 mk-results-sv.py
import os, re
R = '<LAB_DIR>/'; E = R + 'evidence/'
if os.path.exists(R + 'RESULTS.md'): raise SystemExit('RESULTS.md exists')
LEAVE = re.compile(r'^(run-|er-|send-)|^(notes|sends-utc|durations|gateway-refs|bk)\.txt$|(netstat|tasklist)\.txt$|data-(before-start|at-load|before-restore|after-restore)\.txt$')
left = {n for n in os.listdir(E) if n.endswith('.txt') and not LEAVE.search(n)}
PRE = ('count', 'nm', 'wx', 'cv', 'names', 'bw', 'import', 'alt', 'altstate', 'pair', 'same', 'landed')
EXTRA = {'s2': ['st-before-s2.txt', 'st-names-s2.txt', 's2-sha256.txt'], 's6kb': ['same-s6s.txt', 'same-s6b.txt', 'same-s6g.txt', 'same-s6k.txt', 'st-names-s6.txt'], 's9': ['s9-against-s2.txt']}
labels = []
for l in open(E + 'sends-utc.txt', encoding='utf-8'):
    m = re.match(r'\S+ (\S+) .*\(hashes equal to the list\)$', l.rstrip('\n'))
    if m: labels.append(m.group(1))
order = []
def take(n):
    if n in left: left.discard(n); order.append(n)
for i, L in enumerate(labels):
    for p in PRE: take(f'{p}-{L}.txt')
    if L.endswith('b'): take(f'same-{L[:-1]}.txt')
    for n in EXTRA.get(L, []): take(n)
    m = re.fullmatch(r'(r\d)[a-z]+', L)
    if m and not (i + 1 < len(labels) and labels[i + 1].startswith(m.group(1))):
        for x in 'cqwl': take(f'judge2-{m.group(1)}{x}.txt')
order += sorted(left)
with open(R + 'RESULTS.md', 'w', encoding='utf-8', newline='\n') as f:
    f.write(open(R + 'RESULTS-head.md', encoding='utf-8').read().rstrip('\n') + '\n')
    for n in order:
        t = open(E + n, encoding='utf-8').read().rstrip('\n')
        f.write(f'\n### evidence/{n}\n```\n{t}\n```\n')
print(len(order), 'outputs;', 'the last ten:', order[-10:])
