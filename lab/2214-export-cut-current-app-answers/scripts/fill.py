# Lab helper (2214-cut, not product code): copies the kit's helpers to the lab folder's root, args\ to args\ and
# scripts\ to scripts\, sets the sheet's placeholders in the copies only (never in sheet\), and prints per file the
# count of each replacement and the copy's sha256. Python files take forward-slash paths (a backslash in a string
# literal is an escape), .cmd files backslash paths, JSON escaped backslashes. <lab root name> is left as it is.
# Refuses if a copy exists. Usage: python -I -X utf8 fill.py
import hashlib, os
BS = chr(92)
R = '<lab folder>'; S = R + '/sheet'; USER = os.environ['USERNAME']
WIN_LAB = BS.join(['C:', '<lab root name>', '2214-cut'])
WIN_LAD = BS.join(['C:', 'Users', USER, '<app data>', 'Local'])
VAL = {'py':   {'<lab folder>': '<lab folder>', '<Tally data folder>': '<Tally data folder>', '<lab root>': '<lab root>'},
       'cmd':  {'<lab folder>': WIN_LAB, '<local app data>': WIN_LAD},
       'json': {'<lab folder>': WIN_LAB.replace(BS, BS + BS)}}
jobs = [(f'helpers/{n}', n) for n in sorted(os.listdir(S + '/helpers'))] + [(f'args/{n}', f'args/{n}') for n in sorted(os.listdir(S + '/args'))] + \
       [('scripts/cutlog.py', 'scripts/cutlog.py'), ('scripts/listdirs.cmd', 'scripts/listdirs.cmd'), ('scripts/make_front.py', 'make_front.py'), ('proxy.py', 'proxy.py')]
for src, dst in jobs:
    out = f'{R}/{dst}'
    if os.path.exists(out): raise SystemExit(f'exists: {dst}')
    os.makedirs(os.path.dirname(out), exist_ok=True)
    t = open(f'{S}/{src}', 'rb').read().decode('utf-8'); kind = src.rsplit('.', 1)[1]; done = []
    for ph, v in VAL.get(kind, {}).items():
        c = t.count(ph)
        if c: t = t.replace(ph, v); done.append(f'{ph} x{c}')
    left = [p for p in ('<lab folder>', '<local app data>', '<Tally data folder>', '<lab root>') if p in t]
    b = t.encode('utf-8'); open(out, 'wb').write(b)
    print(f'{dst}: {", ".join(done) or "no placeholder"}{"; LEFT " + str(left) if left else ""}; sha256 {hashlib.sha256(b).hexdigest()} {len(b)} B'.replace(USER, '<user>'))
