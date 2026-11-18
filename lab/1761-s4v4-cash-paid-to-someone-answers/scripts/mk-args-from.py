# Lab helper (row 1761-S4 v4, new; not product code): writes call arguments that come from an earlier answer, by json.dump.
#   build <name> <answer file>: {company_guid, proposals_id, proposals_sha256} from a parse answer (steps 2.2 and 3.2)
#   batch <answer file>: post.json and verify.json {company_guid, batch_id} from step 3.2's build answer
# Stops if a file exists or a field is missing. Usage: python -I -X utf8 mk-args-from.py build s2-build answers/NNN-x.json
import json, os, sys
R = '<LAB_DIR>/'; G = 'b52de336-8552-4d40-9221-60aa64d3c1f1'
def res(p): return json.load(open(R + p, encoding='utf-8'))['response']['result']['structuredContent']['result']
def put(n, d):
    if os.path.exists(R + 'args/' + n): raise SystemExit(n + ' exists')
    with open(R + 'args/' + n, 'w', encoding='utf-8', newline='\n') as f: json.dump(d, f, indent=1); f.write('\n')
    print('written', n, json.dumps(d))
if sys.argv[1] == 'build':
    r = res(sys.argv[3]); pid, psha = r['proposals_id'], r['sha256']
    if not pid or not psha: raise SystemExit('proposals_id or sha256 missing')
    put(sys.argv[2] + '.json', {'company_guid': G, 'proposals_id': pid, 'proposals_sha256': psha})
elif sys.argv[1] == 'batch':
    c = res(sys.argv[2])['batch_id']
    if not c: raise SystemExit('batch_id missing')
    for n in ('post.json', 'verify.json'):
        if os.path.exists(R + 'args/' + n): raise SystemExit(n + ' exists')
    put('post.json', {'company_guid': G, 'batch_id': c}); put('verify.json', {'company_guid': G, 'batch_id': c})
else: raise SystemExit('build or batch')
