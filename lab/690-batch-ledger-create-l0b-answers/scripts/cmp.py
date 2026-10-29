# Lab helper (#690 L0b Part A, not product code): lists a summary's ledgers by ALTERID. Usage: python -I cmp.py <summary.json>
import json, sys
d = json.loads(open(sys.argv[1], encoding='utf-8-sig').read())
for x in sorted(d['LEDGER'], key=lambda x: int(x['alterid'] or 0)): print(x['alterid'], x['guid'][-8:], x['name'], '|', x['parent'])
