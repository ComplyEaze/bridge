# Lab helper (#1780-G, not product code; row 1780-F's trace-sum.py with the folder changed): summarises each box trace for RESULTS.md: FIRST and the first block's stamp, the PROC
# lines, the box's own window (hwnd, class, thread, owner, exstyle), every topmost change with the values either side, the
# windows of its process at the first and last block, and every fg_pid change. Reads only.
import os, re, sys
EV = r'<lab folder>\evidence'
cells = sys.argv[1:] or ['b3b', 'f0', 'f1', 'f2', 'f3', 'f4', 'g1', 'g2', 'g3', 't', 'c1']
KV = re.compile(r'(\w+)=(\S+)')
out = []
for c in cells:
    lines = open(os.path.join(EV, f'trace-{c}.txt'), encoding='utf-8-sig').read().splitlines()
    first = next((l for l in lines if l.startswith('FIRST')), None)
    procs = [l for l in lines if l.startswith('PROC')]
    blocks = []  # (stamp, fg_pid, [W lines])
    for l in lines:
        if l.startswith('T '):
            d = dict(KV.findall(l)); blocks.append([l.split()[1], d.get('fg_pid'), []])
        elif l.startswith('W ') and blocks and l.split()[1] == blocks[-1][0]:
            blocks[-1][2].append(l)
    out.append(f'## trace-{c}.txt')
    out.append(f'- {first}; first block {blocks[0][0] if blocks else "none"}; blocks {len(blocks)}')
    out += ['- ' + p for p in procs]
    box = None
    for b in blocks:
        for w in b[2]:
            if 'visible=1' in w and ('ComplyEaze Bridge' in w or 'Lab topmost control' in w or 'Tally probe control' in w):
                d = dict(KV.findall(w)); box = box or d
    if box:
        out.append(f"- box hwnd={box['hwnd']} class={box['class']} thread={box['thread']} owner={box['owner']} exstyle={box['exstyle']}")
    prev = {}
    for b in blocks:
        for w in b[2]:
            d = dict(KV.findall(w)); t = w.split('title=', 1)[-1]
            if 'visible=1' not in w: continue
            key = d['hwnd']
            if key in prev and prev[key][0] != d['topmost']:
                out.append(f"- topmost change {key} ({t}): {prev[key][0]} at {prev[key][1]} -> {d['topmost']} at {b[0]} (exstyle {prev[key][2]} -> {d['exstyle']})")
            prev[key] = (d['topmost'], b[0], d['exstyle'])
    if blocks:
        for label, b in (('first', blocks[0]), ('last', blocks[-1])):
            out.append(f'- windows of its process at the {label} block ({b[0]}): ' + ' | '.join(
                f"{dict(KV.findall(w))['hwnd']} {dict(KV.findall(w))['class']} vis={dict(KV.findall(w))['visible']} top={dict(KV.findall(w))['topmost']}" for w in b[2]))
        fg = None
        for b in blocks:
            if b[1] != fg:
                out.append(f'- fg_pid {fg} -> {b[1]} at {b[0]}'); fg = b[1]
    tops = {dict(KV.findall(w))['topmost'] for b in blocks for w in b[2] if 'visible=1' in w and box and dict(KV.findall(w))['hwnd'] == box['hwnd']}
    out.append(f"- box topmost values seen: {sorted(tops)}; reading {'0' if '0' in tops else '1'}")
    out.append('')
open(os.path.join(EV, 'trace-summary.txt'), 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
print('\n'.join(out))
