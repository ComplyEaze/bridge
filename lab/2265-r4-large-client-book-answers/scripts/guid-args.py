# Lab helper (#2265 R4 on THE CLIENT BOOK, not product code): from the saved list_companies answer, checks there is exactly one
# company and writes the three register argument files with its GUID, the month's from and to; prints only the company count,
# identity_state and the files written (the GUID and the name are never printed).
# Usage: python -I -X utf8 guid-args.py <list_companies answer> <from> <to>
import json, sys
r = json.load(open(sys.argv[1], encoding='utf-8'))['response']['result']['structuredContent']['result']; cs = r['companies']
print('companies', len(cs), 'identity_state', [c.get('identity_state') for c in cs], 'books_from', [c.get('books_from') for c in cs])
if len(cs) != 1: sys.exit('stop: not exactly one company')
for n in ('sales_register', 'purchase_register', 'gst_by_counter_ledger'):
    json.dump({'company_guid': cs[0]['guid'], 'from': sys.argv[2], 'to': sys.argv[3]}, open(f'<LAB_DIR>/calls/args-{n}.json', 'w', encoding='utf-8', newline='\n'))
    print('written calls/args-' + n + '.json')
