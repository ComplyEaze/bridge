# Lab helper (row 703-G, not product code): step 8's row check. For each gst_status call, result.counts and the items
# (each as JSON, sorted) equal across two run folders, with the first difference; the same for vouchers' result items.
# Prints counts and hashes only, no row content.
import hashlib, json, sys
A, B = sys.argv[1:3]
def res(run, n):
    j = json.load(open(f'<row folder>/{run}/answers/{n}', encoding='utf-8'))
    return json.loads(j['response']['result']['content'][0]['text'])['result']
names = ['003-vouchers.json'] + [f'{i:03d}-gst{i-3}.json' for i in range(4, 12)]
bad = 0
for n in names:
    ra, rb = res(A, n), res(B, n)
    ia = sorted(json.dumps(x, sort_keys=True) for x in ra.get('items', [])); ib = sorted(json.dumps(x, sort_keys=True) for x in rb.get('items', []))
    ce = ra.get('counts') == rb.get('counts'); ie = ia == ib
    first = next((k for k, (x, y) in enumerate(zip(ia, ib)) if x != y), None) if not ie else None
    h = hashlib.sha256('\n'.join(ia).encode()).hexdigest()[:16]
    print(f'{n}: items {len(ia)} and {len(ib)}, items equal {ie}, counts equal {ce}, sorted-items sha256 {h}' + ('' if ie else f', first differing sorted position {first}'))
    bad += (not ie) or (not ce)
print('differing calls:', bad)
