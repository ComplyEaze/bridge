# Lab helper (#690 L0b Part A, not product code): compares the parsed summaries of two sittings (A0 against A2 or A5).
# Usage: python -I a0a2.py <evidence dir> <first label> <second label>
import glob, json, sys
E, a, b = sys.argv[1:4]
def load(label, kind):
    p = glob.glob(f'{E}/*-{label}-{kind}.json')
    if len(p) != 1: sys.exit(f'{label} {kind}: {len(p)} summaries')
    return {x['guid']: x for x in json.loads(open(p[0], encoding='utf-8-sig').read())[kind.upper()[:-1] if kind != 'marks' else 'COMPANY']}
for kind in ('ledgers', 'groups'):
    try: x, y = load(a, kind), load(b, kind)
    except SystemExit as e: print(e); continue
    changed = [(x[g]['name'], x[g]['alterid'], y[g]['alterid']) for g in x if g in y and x[g]['alterid'] != y[g]['alterid']]
    gone = [x[g]['name'] for g in x if g not in y]
    new = sorted((int(y[g]['alterid']), y[g]['name'], g) for g in y if g not in x)
    print(f'{kind}: {a} {len(x)}, {b} {len(y)}; of {a} unchanged {len(x) - len(changed) - len(gone)}, another ALTERID {changed}, missing {gone}')
    print(f'   new in {b}: {[(n, aid) for aid, n, _ in new]}')
