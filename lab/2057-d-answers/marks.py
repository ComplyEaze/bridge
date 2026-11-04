# Lab helper (2057-D, not product code): prints the snapshot marks of one ledger_masters answer saved by call.py.
# Usage: python -I -X utf8 marks.py <answer file name under srv/answers>
import json, sys
s = json.load(open('<LAB_DIR>/srv/answers/' + sys.argv[1], encoding='utf-8'))['response']['result']['structuredContent']['result']['snapshot']
print(sys.argv[1], 'M', s.get('master_alter_id'), 'V', s.get('voucher_alter_id'), 'reused', s.get('reused'))
