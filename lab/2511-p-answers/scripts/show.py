# Lab helper (row 1761-S4, new; not product code): prints every leaf of one answer's structured result as path = value (lists show
# their length and every item), so a step's EXPECT can be checked by eye against the answer file. Reads only.
# Usage: python -I -X utf8 show.py <answer file> [max items per list]
import json, sys
a = json.load(open(sys.argv[1], encoding='utf-8'))
N = int(sys.argv[2]) if len(sys.argv) > 2 else 50
r = a['response']['result']
sc = r.get('structuredContent')
if sc is None:
    sc = json.loads(r['content'][0]['text'])
print('isError', r.get('isError'))
def walk(o, p=''):
    if isinstance(o, dict):
        for k, v in o.items(): walk(v, p + '.' + k)
    elif isinstance(o, list):
        print(p, '= list', len(o))
        for i, v in enumerate(o[:N]): walk(v, p + f'[{i}]')
    else:
        print(p, '=', repr(o)[:200])
walk(sc)
