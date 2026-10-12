# Compare two GSTR-1 JSON exports by parsed structure (leaf paths). Usage: python -I jcmp.py <a> <b>
import json, sys
def flat(o, p=''):
    if isinstance(o, dict):
        for k, v in o.items(): yield from flat(v, p + '/' + k)
    elif isinstance(o, list):
        for i, v in enumerate(o): yield from flat(v, '%s[%d]' % (p, i))
    else: yield p, o
a = dict(flat(json.load(open(sys.argv[1], encoding='utf-8'))))
b = dict(flat(json.load(open(sys.argv[2], encoding='utf-8'))))
print('leaves', len(a), len(b))
for k in sorted(set(a) | set(b)):
    if a.get(k) != b.get(k): print(k, a.get(k, '<absent>'), '->', b.get(k, '<absent>'))
