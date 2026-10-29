# Lab helper (#1780-F R1, not product code): the side-by-side table of the reader's fields for the box's process (C1's child
# against every cell's child), the same for its parent, and each CHAIN line read with the reader's one-step name shift undone
# (ctx-reader.ps1 writes each ancestor's pid beside the image name of the process one step below it). Reads only.
import os, re
EV = r'<lab folder>\evidence'
CELLS = ['c1', 'f0', 'f1', 'f2', 'f3', 'f4', 'g1', 'g2', 'g3', 't']
FIELDS = ['in_job', 'integrity_rid', 'appcontainer', 'session', 'wow64', 'console_handle', 'std_in', 'std_out', 'std_err',
          'startf', 'show_window', 'desktop', 'title_chars']
KV = re.compile(r'(\w+)=(\S+)')
ctx, chains = {}, {}
for c in CELLS:
    lines = open(os.path.join(EV, f'ctx-{c}.txt'), encoding='utf-8-sig').read().splitlines()
    ctx[c] = [dict(KV.findall(l.split(' cmd=')[0])) for l in lines if l.startswith('CTX ')]
    chains[c] = [l for l in lines if l.startswith('CHAIN ')]
def unshift(line):
    parts = line.split(' <- ')
    pid = parts[0].split('=')[1]
    pairs = [p.split(':', 1) for p in parts[1:]]
    out = [f'{pid} {pairs[0][1]}'] + [f'{pairs[i][0]} {pairs[i + 1][1]}' for i in range(len(pairs) - 1)] + [f'{pairs[-1][0]} (image not read)']
    return ' <- '.join(out)
out = ['The box\'s process (C1: the server\'s child; F/T: the dialog child; G: the control box), from each ctx file\'s CTX line for it:', '',
       '| field | ' + ' | '.join(c.upper() for c in CELLS) + ' |', '|---' * (len(CELLS) + 1) + '|']
child = {c: (ctx[c][0] if c == 'c1' else ctx[c][1]) for c in CELLS}
parent = {c: (ctx[c][1] if c == 'c1' else ctx[c][0]) for c in CELLS}
for f in ['pid'] + FIELDS:
    vals = [child[c].get(f, '?') for c in CELLS]
    mark = '' if f in ('pid', 'console_handle', 'std_in', 'std_out', 'std_err') or vals[0] in vals[1:] and all(v == vals[0] for v in vals) else ''
    out.append(f'| {f} | ' + ' | '.join(vals) + ' |')
out += ['', 'Its parent (C1: the server; F/G/T: spawn-parent.ps1\'s PowerShell):', '',
        '| field | ' + ' | '.join(c.upper() for c in CELLS) + ' |', '|---' * (len(CELLS) + 1) + '|']
for f in ['pid'] + FIELDS:
    out.append(f'| {f} | ' + ' | '.join(parent[c].get(f, '?') for c in CELLS) + ' |')
out += ['', 'Ancestry, read with the one-step name shift undone (pid image <- parent pid image ...):', '']
for c in CELLS:
    for l in chains[c]:
        out.append(f'- {c.upper()}: {unshift(l)}')
same = [f for f in FIELDS if len({child[c].get(f) for c in CELLS}) == 1]
diff_c1 = [f for f in FIELDS if f not in ('console_handle', 'std_in', 'std_out', 'std_err') and
           all(child[c].get(f) != child['c1'].get(f) for c in CELLS if c != 'c1')]
out += ['', f'Fields equal in all ten: {", ".join(same)}.',
        f'Fields where C1\'s child differs from every cell that reads 1 (handle values aside): {", ".join(diff_c1) or "none"}.']
open(os.path.join(EV, 'r1-table.txt'), 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
print('\n'.join(out))
