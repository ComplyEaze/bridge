# Lab helper (1108-A2, from 1392-E's args.py, not product code): writes the row's argument files under calls/: the fixed ones,
# a build from batches/<pN>.json, and a post/verify for a batch id. THE COMPANY's GUID from guid.txt.
# Usage: python -I -X utf8 args1108.py fixed | build <pN> | batch <pN> <batch_id>
import hashlib, json, sys
R = '<LAB_DIR>/'; C = R + 'calls/'; G = open(R + 'guid.txt').read().strip()
def put(n, o): json.dump(o, open(C + n, 'w', encoding='utf-8', newline='\n'), ensure_ascii=False); print(n, hashlib.sha256(open(C + n, 'rb').read()).hexdigest()[:16])
if sys.argv[1] == 'fixed':
    put('args-empty.json', {}); put('args-guid.json', {'company_guid': G}); put('args-vt.json', {'company_guid': G, 'kind': 'voucher_types'})
    put('args-egress.json', {'limit': 256})
elif sys.argv[1] == 'build':
    put(f'args-build-{sys.argv[2]}.json', {'company_guid': G, 'vouchers': json.load(open(R + f'lab/1108-a2/batches/{sys.argv[2]}.json', encoding='utf-8'))})
elif sys.argv[1] == 'batch':
    put(f'args-post-{sys.argv[2]}.json', {'company_guid': G, 'batch_id': sys.argv[3]}); put(f'args-verify-{sys.argv[2]}.json', {'company_guid': G, 'batch_id': sys.argv[3]})
