# Lab helper (725-D1, not product code): reads box.ps1's log of Bridge's approval box and checks its text against the
# sheet's step 5, one line per check: the company and count, the types, both dates, the debit and credit totals, one line
# per ledger with 200 entries for Cash and for DN Sales, the company GUID and the batch. Writes nothing to the box: the
# answer file is written by the lab only after reading this and the screen. Usage: python -I -X utf8 boxcheck.py <tag> <batch_id>
import re, sys
from decimal import Decimal
tag, batch = sys.argv[1], sys.argv[2]
t = open(f'<LAB_DIR>/box/{tag}-box.txt', encoding='utf-8-sig').read()
G = 'd45bc1b0-e5e3-4261-b3b2-cce3915f42d3'
def num(s): return Decimal(s.replace(',', ''))
def total(label):
    m = re.search(label + r':\s*([0-9,]+(?:\.[0-9]+)?)', t); return num(m.group(1)) if m else None
def ledger(name):
    m = re.search(r'Dr\s+([0-9,.]+)\s+Cr\s+([0-9,.]+)\s+(\d+)\s+entries\s+"' + re.escape(name) + '"', t)
    return (num(m.group(1)), num(m.group(2)), int(m.group(3))) if m else None
checks = [
    ('box seen', 'BOX SEEN' in t),
    ('Create 200 vouchers in "BRIDGE CORPUS DENSE"', 'Create 200 vouchers in "BRIDGE CORPUS DENSE"' in t),
    ('Types: 200 Journal', re.search(r'Types:\s*200 Journal\s*$', t, re.M) is not None),
    ('both dates 1-Apr-2026', re.search(r'Dates?:\s*1-Apr-2026(\s+to\s+1-Apr-2026)?\b', t) is not None),
    ('total debit 20150.00', total('Total debit') == Decimal('20150')),
    ('total credit 20150.00', total('Total credit') == Decimal('20150')),
    ('Cash: Dr 20150 Cr 0, 200 entries', ledger('Cash') == (Decimal('20150'), Decimal('0'), 200)),
    ('DN Sales: Dr 0 Cr 20150, 200 entries', ledger('DN Sales') == (Decimal('0'), Decimal('20150'), 200)),
    ('one line per ledger, two ledgers', len(re.findall(r'\d+\s+entries\s+"', t)) == 2),
    ('Company GUID', f'Company GUID: {G}' in t),
    ('Batch', f'Batch: {batch}' in t),
]
for name, ok in checks: print(('PASS ' if ok else 'FAIL ') + name)
print('ALL PASS' if all(ok for _, ok in checks) else 'NOT ALL PASS')
for k in ('Dates', 'Date:', 'Total debit', 'Total credit', 'Types', 'entries'):
    for l in t.splitlines():
        if k in l and 'Company number' not in l: print('  |', l.strip()[:160])
