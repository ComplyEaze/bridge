# Lab helper (row 2480-SV, new; not product code; written whole by an editor): writes evidence/durations.txt from evidence/sends-utc.txt, in
# row 2480-SC's form: each label, its files and the wall seconds from send.sh's send line to its "answered" line (one run.py call per label). A
# label is marked [capture] when shots/<label>-after.png exists: send.sh takes that capture after run.py returns and before its answered line,
# so the read itself took less. Reads only; writes evidence/durations.txt, which must not exist yet. Usage: python -I -X utf8 durations-sv.py
import datetime, os, re
R = '<LAB_DIR>/'
OUT = R + 'evidence/durations.txt'
if os.path.exists(OUT): raise SystemExit('evidence/durations.txt exists')
def t(s): return datetime.datetime.strptime(s, '%Y-%m-%dT%H:%M:%S.%fZ')
start, rows = {}, []
for l in open(R + 'evidence/sends-utc.txt', encoding='utf-8'):
    m = re.match(r'(\S+) (\S+) (.*) \(hashes equal to the list\)$', l.rstrip('\n'))
    if m: start[m.group(2)] = (t(m.group(1)), m.group(3)); continue
    m = re.match(r'(\S+) (\S+) answered$', l.rstrip('\n'))
    if m:
        s, files = start.pop(m.group(2)); d = (t(m.group(1)) - s).total_seconds()
        cap = ' [capture]' if os.path.exists(f'{R}shots/{m.group(2)}-after.png') else ''
        rows.append(f"{m.group(2)}\t{files}\t{d:.3f} s, from the send line to run.py's return (answered){cap}")
if start: raise SystemExit(f'labels with no answered line: {sorted(start)}')
with open(OUT, 'w', encoding='utf-8', newline='\n') as f:
    f.write("Row 2480-SV. label, files, wall seconds from send.sh's send line to its end line (one run.py call per label; a label of several files is their sum). For a label marked [capture], the window capture after run.py's return is inside the figure, so the read itself took less; labels without one show the read alone, with python's start.\n")
    f.write('\n'.join(rows) + '\n')
print(len(rows), 'labels')
