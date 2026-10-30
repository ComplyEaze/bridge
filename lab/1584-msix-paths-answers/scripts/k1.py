# Lab helper (#1584 K1, not product code): reads the three K1 probe files and prints the readings the sheet's
# classification uses, in its order: out (K1b family none, N2-out in X and not XQ, K1c sees X\N2-out), then in (K1a's
# family, where N1-in is seen from out), the KA instrument, decision 2 (final path forms, known folders).
import json, sys
E = 'C:/bridge-lab/1584/evidence'
def load(n): return json.load(open(f'{E}/{n}.json', encoding='utf-8-sig'))
a, b, c = load('K1a-in'), load('K1b-out'), load('K1c-in')
n1, n2 = sys.argv[1], sys.argv[2]
def names(rec, listing): return {x['name']: x['info'] for x in (rec['listings'][listing].get('shown') or [])}
def info(s): return s if not s.startswith('final=') else dict(p.split('=', 1) for p in s.split('|'))
out = []
out.append(f"package_family: K1a(in) {a['package_family']}; K1b(out) {b['package_family']}; K1c(in) {c['package_family']}")
out.append(f"pids: K1a {a['pid']} parent {a['parent_pid']}; K1b {b['pid']} parent {b['parent_pid']}; K1c {c['pid']} parent {c['parent_pid']}")
out.append(f"written: K1a {a['written']}; K1b {b['written']}")
bx, bxq = names(b, 'X'), names(b, 'XQ')
cx, cxq = names(c, 'X'), names(c, 'XQ')
o1 = b['package_family'] == 'none'; o2 = f'{n2}-out.txt' in bx and f'{n2}-out.txt' not in bxq; o3 = f'{n2}-out.txt' in cx
out.append(f"OUT test: K1b family none {o1}; N2-out in X (K1b) {f'{n2}-out.txt' in bx}, not in XQ (K1b) {f'{n2}-out.txt' not in bxq}; K1c sees X\\N2-out {o3}  => out decided {o1 and o2 and o3}")
out.append(f"IN reading: K1a family {a['package_family']}; from out (K1b): X\\N1-in {'present' if f'{n1}-in.txt' in bx else 'absent'}, XQ\\N1-in {'present' if f'{n1}-in.txt' in bxq else 'absent'}")
out.append(f"   from in (K1c): X\\N1-in {'present' if f'{n1}-in.txt' in cx else 'absent'}, XQ listing: {c['listings']['XQ'].get('error') or sorted(cxq)}")
ka_b, ka_c = info(b['paths']['KA']['info']), info(c['paths']['KA']['info'])
inst = isinstance(ka_b, dict) and isinstance(ka_c, dict) and ka_b['vol'] == ka_c['vol'] and ka_b['id'] == ka_c['id'] and ka_b['final'] == ka_c['final'] == '\\\\?\\C:\\bridge-lab\\1584\\ka.txt'
out.append(f"KA instrument: out {ka_b if isinstance(ka_b, str) else (ka_b['final'], ka_b['vol'], ka_b['id'])}; in {ka_c if isinstance(ka_c, str) else (ka_c['final'], ka_c['vol'], ka_c['id'])} => {'passes' if inst else 'FAILS'}")
for rec, nm in ((c, 'K1c in'), (b, 'K1b out')):
    for lst in ('X', 'XQ'):
        for k, v in names(rec, lst).items():
            if k.startswith(n1):
                out.append(f"decision 2, {nm} {lst}\\{k}: {v}")
for rec, nm in ((a, 'K1a in'), (b, 'K1b out')):
    kf = rec['known_folders']
    out.append(f"known folders {nm}: " + '; '.join(f'{k}={v}' for k, v in kf.items()))
    out.append(f"   LocalAppData 0x44000 differs from 0x4000: {kf['LocalAppData-0x44000'] != kf['LocalAppData-0x4000']}; Profile-0x4000 == env_USERPROFILE: {kf['Profile-0x4000'] == rec['env_USERPROFILE']}")
for rec, nm in ((b, 'K1b out (before)'),):
    for k in ('R', 'Q', 'P', 'R-journal', 'Q-journal', 'R-K.lock', 'R-K.snapshot.lock', 'R-K.wire.lock', 'Q-K.lock', 'Q-K.snapshot.lock', 'Q-K.wire.lock', 'P-K.lock', 'P-K.snapshot.lock', 'P-K.wire.lock'):
        p = rec['paths'][k]; out.append(f"{nm} {k}: {p['info']} bytes {p['bytes']}")
    out.append(f"{nm} lease_counts: {json.dumps(rec['lease_counts'])}")
    out.append(f"{nm} extra (python): " + json.dumps(rec['extra']))
open(f'{E}/K1-classification.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n'); print('\n'.join(out))
