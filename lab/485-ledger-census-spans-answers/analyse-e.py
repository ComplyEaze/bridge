# Lab helper (#485 E, not product code): the comparisons the part E sheet lists, from saved answers read by LAB-COMMON C4.
import os
import sys
from decimal import Decimal
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from compare import load, txt

A = os.path.join(os.path.dirname(os.path.abspath(__file__)))
def vrows(p):
    return [r for r in load(os.path.join(A, p)) if r.tag == 'VOUCHER']
spans = [(0, 8192), (8192, 16384), (16384, 24576), (24576, 29900)]
company = 'd45bc1b0-e5e3-4261-b3b2-cce3915f42d3'

def check(rows, span, lo_date, hi_date, label):
    bad_alt = [txt(r, 'ALTERID') for r in rows if not (span[0] < int(txt(r, 'ALTERID')) <= span[1])]
    bad_date = [txt(r, 'DATE') for r in rows if not (lo_date <= txt(r, 'DATE') <= hi_date)]
    bad_guid = [txt(r, 'GUID') for r in rows if not txt(r, 'GUID').startswith(company)]
    print(f'{label}: rows {len(rows)}; ALTERID outside span {span}: {len(bad_alt)}; DATE outside {lo_date}..{hi_date}: {len(bad_date)}; GUID not the company\'s: {len(bad_guid)}')

d03 = {txt(r, 'GUID') for r in vrows('answers-d/d03_ledger_vouchers_dn_sales_20250515.answer.xml')}
u1 = set()
for i, n in enumerate(['e02_ledger_census_dn_sales_20250515_s1', 'e03_ledger_census_dn_sales_20250515_s2', 'e04_ledger_census_dn_sales_20250515_s3', 'e05_ledger_census_dn_sales_20250515_s4']):
    rows = vrows(f'answers-e/{n}.answer.xml')
    check(rows, spans[i], '20250515', '20250515', n)
    u1 |= {txt(r, 'GUID') for r in rows}
print(f'union e02-e05 {len(u1)} vs d03 {len(d03)}: equal {u1 == d03}; only in union {len(u1 - d03)}; only in d03 {len(d03 - u1)}')

u2, may = set(), set()
for i, n in enumerate(['e06_ledger_census_dn_party_001_fy_s1', 'e07_ledger_census_dn_party_001_fy_s2', 'e08_ledger_census_dn_party_001_fy_s3', 'e09_ledger_census_dn_party_001_fy_s4']):
    rows = vrows(f'answers-e/{n}.answer.xml')
    check(rows, spans[i], '20250401', '20260331', n)
    u2 |= {txt(r, 'GUID') for r in rows}
    may |= {txt(r, 'GUID') for r in rows if txt(r, 'DATE').startswith('202505')}
e10 = vrows('answers-e/e10_ledger_vouchers_dn_party_001_fy.answer.xml')
s10 = {txt(r, 'GUID') for r in e10}
miss10 = {f: sum(1 for r in e10 if not txt(r, f)) for f in ('GUID', 'ALTERID', 'MASTERID')}
print(f'e10 rows {len(e10)} (distinct GUIDs {len(s10)}), empty-or-absent {miss10}')
print(f'union e06-e09 {len(u2)} vs e10 {len(s10)}: equal {u2 == s10}; only in union {len(u2 - s10)}; only in e10 {len(s10 - u2)}')

e11 = vrows('answers-e/e11_ledger_movement_dn_party_001_202505.answer.xml')
s11 = {txt(r, 'GUID') for r in e11}
print(f'e11 rows {len(e11)}; dates {min(txt(r, "DATE") for r in e11)}..{max(txt(r, "DATE") for r in e11)}')
print(f'e11 GUIDs {len(s11)} vs e06-e09 rows dated May 2025 {len(may)}: equal {s11 == may}; only in e11 {len(s11 - may)}; only in census {len(may - s11)}')
dr = cr = Decimal(0)
lines = 0
table = []
for r in sorted(e11, key=lambda r: int(txt(r, 'MASTERID'))):
    entries = []
    for le in r.iter():
        if le.tag.endswith('LEDGERENTRIES.LIST'):
            name = ''.join(le.find('LEDGERNAME').itertext()) if le.find('LEDGERNAME') is not None else None
            amt = txt(le, 'AMOUNT')
            pos = txt(le, 'ISDEEMEDPOSITIVE')
            entries.append((name, amt, pos))
            if name == 'DN Party 001':
                lines += 1
                a = Decimal(amt)
                if a < 0: dr += a
                else: cr += a
    table.append((txt(r, 'DATE'), txt(r, 'VOUCHERTYPENAME'), txt(r, 'VOUCHERNUMBER'), txt(r, 'MASTERID'), txt(r, 'ALTERID'), txt(r, 'GUID'), entries))
print(f'e11 DN Party 001 lines {lines}; sum of negative (debit) amounts {dr}; sum of positive (credit) amounts {cr}')
print('committed one-month TB (parse record on master): DN Party 001 debit -933428.20, credit empty')
print(f'debit equal: {dr == Decimal("-933428.20")}; credit equal (empty = 0): {cr == 0}')
with open(os.path.join(A, 'tables', 'e11-voucher-table.txt'), 'w', encoding='utf-8', newline='\n') as f:
    for t in table:
        f.write(' | '.join(t[:6]) + '\n')
        for e in t[6]:
            f.write(f'    LEDGERNAME={e[0]!r} ({len(e[0]) if e[0] else 0} chars) AMOUNT={e[1]} ISDEEMEDPOSITIVE={e[2]}\n')
