# Lab runner (#1688, not product code): THE BRIDGE READS of one set (B0, BK, BI, BC), through call.py to the lab's own
# 0.7.0 server: validate_masters with the sheet's 18 names (typed here exactly, in order), ledger_masters fields=basic,
# egress_log limit 10. Then a table per set (evidence\<set>-bridge.txt): each validate row's fields, ledger_masters's
# total, snapshot and items, and each egress record's request_trail sends. The server answering is checked to be the
# lab's own (server_pid and serverInfo in each saved answer). Usage: python -I -X utf8 breads.py <set>
import json, os, subprocess, sys
W = '<LAB_DIR>'; SD = W + '/srv'; E = W + '/evidence'
SET = sys.argv[1]
G = '021bda0c-b1b1-44c8-8279-5212ce87c9c7'
NAMES = ['Lab Plain Ledger', 'Lab Plain Alias', 'LAB PLAIN ALIAS', 'Lab Other Ledger', 'Lab Other First', 'Lab Other Second',
         'lab other ledger', 'Lab Sales', 'Lab Caf\u00e9 Sales', 'Lab Alias Shape', 'Lab Shape Alias', 'Lab Alias Script',
         '\u0932\u0948\u092c \u0916\u093e\u0924\u093e', 'Lab Alias Clash', 'Lab Alias Fold', 'LAB SALES', 'Lab Alias Shared', 'Lab No Such Ledger']
calls = [(f'{SET}-validate', 'validate_masters', {'company_guid': G, 'ledgers': NAMES}),
         (f'{SET}-lm', 'ledger_masters', {'company_guid': G, 'fields': 'basic'}),
         (f'{SET}-egress', 'egress_log', {'limit': 10})]
for label, tool, args in calls:
    r = subprocess.run(['python', '-I', '-X', 'utf8', W + '/call.py', label, tool, json.dumps(args, ensure_ascii=True)], capture_output=True, text=True)
    print(r.stdout.splitlines()[0] if r.stdout else r.stderr[:500], flush=True)
subprocess.run(['python', '-I', '-X', 'utf8', W + '/btable.py', SET])
