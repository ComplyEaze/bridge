# Lab helper (#1584, not product code): from a pair of probe files (<step>-out.json, <step>-in.json) prints, per side,
# each -ExtraFile path's open result (final path, vol+id, sha256), the journal sizes (R and Q) against K1b, the nine lock
# files' presence and vol+id, and the leases folders' counts. Usage: python -I -X utf8 rowread.py <step> [K1b]
import json, os, sys
E = 'C:/bridge-lab/1584/evidence'; step = sys.argv[1]; base = sys.argv[2] if len(sys.argv) > 2 else 'K1b'
def load(n): return json.load(open(f'{E}/{n}.json', encoding='utf-8-sig'))
def short(info):
    if not info.startswith('final='): return info
    d = dict(p.split('=', 1) for p in info.split('|')); return f"final={d['final']} vol={d['vol']} id={d['id']}"
out = []; user = os.environ.get('USERNAME', '?')
for side in ('out', 'in'):
    try: r = load(f'{step}-{side}')
    except FileNotFoundError: continue
    out.append(f'{step}-{side}: family {r["package_family"]}; pid {r["pid"]} parent {r["parent_pid"]}')
    for x in r['extra']: out.append(f'  extra {x["path"]}: {short(x["info"])} sha256 {x["sha256"]}')
    for k in ('R-journal', 'Q-journal'): out.append(f'  {k}: {short(r["paths"][k]["info"])} bytes {r["paths"][k]["bytes"]}')
    for pre in ('R', 'Q', 'P'):
        for suf in ('lock', 'snapshot.lock', 'wire.lock'):
            k = f'{pre}-K.{suf}'; out.append(f'  {k}: {short(r["paths"][k]["info"])}')
    out.append(f'  lease_counts: {json.dumps(r["lease_counts"])}')
b = load(base)
out.append(f'{base} (before): R-journal {b["paths"]["R-journal"]["info"][:6]} bytes {b["paths"]["R-journal"]["bytes"]}; Q-journal bytes {b["paths"]["Q-journal"]["bytes"]}')
text = '\n'.join(out).replace(user, '<u>')
open(f'{E}/{step}-read.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n'); print(text)
