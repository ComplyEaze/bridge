# Lab helper (1392-E34, changed from 1392-E's args.py, lab original 1652 B sha256 1fcd876147c76fc83979eb39ca0ed743e985d573cdb25cea05d51101659ed32f;
# not product code). Changes: the row folder; 'fixed' writes only the four statements this row parses (b3, a3, b4, a4), with
# statement_path moved from 1392-E's folder to this row's and password_file set to the one file under the user profile
# (resolved here from USERPROFILE, never typed or printed); the K build from batches/k.json in place of the F build.
# Usage: python -I -X utf8 args.py fixed | build-from <parse answer> <name> | batch <name> <batch_id>
import hashlib, json, os, sys
C = 'C:/bridge-lab/1392e34/calls/'; G = 'd6e4c24f-8c7a-4797-93ec-e44bdff60bc9'
OLD_PDF, NEW_PDF = 'C:\\bridge-lab\\1392e\\pdf\\', 'C:\\bridge-lab\\1392e34\\pdf\\'
def put(n, o): json.dump(o, open(C + n, 'w', encoding='utf-8', newline='\n'), ensure_ascii=False); print(n, hashlib.sha256(open(C + n, 'rb').read()).hexdigest()[:16])
if sys.argv[1] == 'fixed':
    pw = os.path.join(os.environ['USERPROFILE'], 'lab-1392e34-pw', 'pw.txt')
    if not os.path.isfile(pw): sys.exit('HALT: the password file under the user profile is missing')
    put('args-empty.json', {}); put('args-guid.json', {'company_guid': G}); put('args-vt.json', {'company_guid': G, 'kind': 'voucher_types'})
    put('args-egress.json', {'limit': 256})
    calls = json.load(open(C + 'calls-printed.txt', encoding='utf-8-sig'))
    for k in ('b3', 'a3', 'b4', 'a4'):
        v = dict(calls[k])
        if not v['statement_path'].startswith(OLD_PDF): sys.exit(f'HALT: {k} statement_path is not under the 1392-E folder')
        v['statement_path'] = NEW_PDF + v['statement_path'][len(OLD_PDF):]; v['password_file'] = pw
        put(f'args-parse-{k}.json', v)
    put('args-build-k.json', {'company_guid': G, 'vouchers': json.load(open('C:/bridge-lab/1392e34/run/batches/k.json', encoding='utf-8'))})
elif sys.argv[1] == 'build-from':
    r = json.load(open(sys.argv[2], encoding='utf-8'))['response']['result']['structuredContent']['result']
    put(f'args-build-{sys.argv[3]}.json', {'company_guid': G, 'proposals_id': r['proposals_id'], 'proposals_sha256': r['sha256']})
elif sys.argv[1] == 'batch':
    put(f'args-post-{sys.argv[2]}.json', {'company_guid': G, 'batch_id': sys.argv[3]}); put(f'args-verify-{sys.argv[2]}.json', {'company_guid': G, 'batch_id': sys.argv[3]})
