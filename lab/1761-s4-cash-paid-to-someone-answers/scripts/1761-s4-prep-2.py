# Lab helper (#1761 slice 4 row, 1761-s4-prep.py by mk-1761s4-prep2.py with the sheet at 5e9580d8 and the data and BK1 folders; not product code): stages the row folder before the sitting. Writes nothing to Tally.
# Byte copies of the stdio client, call helper and box watcher; the statement copy; step 1 to 3 argument files
# (company_guid is added at step 0). Refuses to run if the row folder already exists.
import hashlib, json, os, shutil, sys
R = '<LAB_DIR>'
S = '<SHEET_DIR>/lab/1761-s4-cash-paid-to-someone'
if os.path.exists(R): sys.exit(R + ' exists; not staged again')
for d in ('', 'inbox', 'answers', 'step0', 'evidence', 'args', 'box', 'sheet', 'shots', 'data', 'backups/BK1'): os.makedirs(f'{R}/{d}')
def sha(p): return hashlib.sha256(open(p, 'rb').read()).hexdigest()
log = []
for src, dst in (('<lab folder>/1761l/client.py', 'client.py'), ('<lab folder>/1761l/call.py', 'call.py'),
                 ('<lab folder>/852c2/box.ps1', 'box.ps1'), (S + '/statement.csv', 'statement.csv')):
    shutil.copyfile(src, f'{R}/{dst}'); assert sha(src) == sha(f'{R}/{dst}'); log.append(f'{dst} {os.path.getsize(src)} B sha256 {sha(src)} (byte copy)')
for f in ('LAB-COMMON.md', 'RUN-SHEET.md', 'SHA256.txt', 'statement.csv', '.gitattributes'): shutil.copyfile(f'{S}/{f}', f'{R}/sheet/{f}')
assert sha(f'{R}/statement.csv') == 'ebca906726de102cb23dcf15767de2cee5bba0ff4c401debd6e7b9fb3bf776ff'
base = {'statement_path': '<LAB_DIR>/statement.csv', 'controls_source': 'prior_statement',
        'opening_balance': '50,000.00', 'closing_balance': '35,000.00', 'account_label': 'Lab CSV 4321',
        'bank_ledger': 'BRIDGE Bank A', 'suspense_ledger': 'BRIDGE Suspense'}
A, B = 'st-20250408-be621b1463b52f1b', 'st-20250409-ef7d14f34a7b0412'
s2 = dict(base, cash_answers=[{'bridge_txn_id': A, 'answer': 'paid_to_someone', 'ledger': 'BRIDGE Contractor A', 'cash_ledger': 'BRIDGE Bank B'},
                              {'bridge_txn_id': B, 'answer': 'paid_to_someone', 'ledger': 'BRIDGE Wages', 'cash_ledger': 'Cash'}])
s3 = dict(base, cash_answers=[{'bridge_txn_id': A, 'answer': 'paid_to_someone', 'ledger': 'BRIDGE Contractor A', 'cash_ledger': 'Cash'},
                              {'bridge_txn_id': B, 'answer': 'paid_to_someone', 'ledger': 'BRIDGE Wages', 'cash_ledger': 'Cash'}])
for name, a in (('none', {}), ('s1-parse', base), ('s2-parse', s2), ('s3-parse', s3)):
    with open(f'{R}/args/{name}.json', 'w', encoding='utf-8', newline='\n') as f: json.dump(a, f, ensure_ascii=False, indent=1)
    log.append(f'args/{name}.json sha256 {sha(f"{R}/args/{name}.json")}')
with open(f'{R}/evidence/prep.txt', 'w', encoding='utf-8', newline='\n') as f: f.write('\n'.join(log) + '\n')
print('\n'.join(log))
