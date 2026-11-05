# Lab helper (1392-E, not product code): writes the row's argument files under calls/: the fixed ones, one per
# statement from make_statements.py --calls (printed to calls/calls-printed.txt), the F build from batches/f.json, and on
# request a build from a parse answer (proposals_id and its sha256) or a post/verify for a batch id.
# Usage: python -I -X utf8 args.py fixed | build-from <parse answer> <name> | batch <name> <batch_id>
import hashlib, json, sys
C = '<LAB_DIR>/calls/'; G = 'd6e4c24f-8c7a-4797-93ec-e44bdff60bc9'
def put(n, o): json.dump(o, open(C + n, 'w', encoding='utf-8', newline='\n'), ensure_ascii=False); print(n, hashlib.sha256(open(C + n, 'rb').read()).hexdigest()[:16])
if sys.argv[1] == 'fixed':
    put('args-empty.json', {}); put('args-guid.json', {'company_guid': G}); put('args-vt.json', {'company_guid': G, 'kind': 'voucher_types'})
    put('args-egress.json', {'limit': 256})
    for k, v in json.load(open(C + 'calls-printed.txt', encoding='utf-8-sig')).items(): put(f'args-parse-{k}.json', v)
    put('args-build-f.json', {'company_guid': G, 'vouchers': json.load(open('<LAB_DIR>/run/batches/f.json', encoding='utf-8'))})
elif sys.argv[1] == 'build-from':
    r = json.load(open(sys.argv[2], encoding='utf-8'))['response']['result']['structuredContent']['result']
    put(f'args-build-{sys.argv[3]}.json', {'company_guid': G, 'proposals_id': r['proposals_id'], 'proposals_sha256': r['sha256']})
elif sys.argv[1] == 'batch':
    put(f'args-post-{sys.argv[2]}.json', {'company_guid': G, 'batch_id': sys.argv[3]}); put(f'args-verify-{sys.argv[2]}.json', {'company_guid': G, 'batch_id': sys.argv[3]})
