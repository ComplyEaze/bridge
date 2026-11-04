# Lab helper (1379 Part R, not product code): the sheet's "Record by parser" table, from the JSON answers as returned
# (r2 to r8, each one page). Reads fields by key, never by substring. Names from the book appear only in this table.
# A field the answer does not carry is written "not returned". Usage: python -I -X utf8 table.py > evidence/r-table.txt
import json
A = '<LAB_DIR>/srv/answers/'
def res(f): return json.load(open(A + f, encoding='utf-8'))['response']['result']['structuredContent']['result']
def j(x): return json.dumps(x, ensure_ascii=False, sort_keys=True)
NR = 'not returned'
print('1379 Part R: the parser table (JSON answers as returned; one page each). A field the answer does not carry reads "not returned".')

g = res('002-r2-godowns.json')['masters']
print(f'\n## r2 masters godowns: {len(g)} rows')
for m in g: print(f'- name {m["name"]!r}; parent {m["parent"]!r}; master_id {m["master_id"]}')
print(f'Exactly one godown: {"yes" if len(g) == 1 else "no"} ({len(g)}). Tally\'s default "Main Location" among them: {"yes" if any(m["name"] == "Main Location" for m in g) else "no"}.')

u = res('003-r3-units.json')['masters']
print(f'\n## r3 masters units: {len(u)} rows')
for m in u: print(f'- name {m["name"]!r}; decimal_places {m.get("decimal_places", NR)}; simple {m.get("simple", NR)}')

it = res('004-r4-stock-items.json')['masters']
print(f'\n## r4 masters stock_items: {len(it)} rows (keys returned per item: {sorted({k for m in it for k in m})}; base unit: {NR})')
for m in it:
    print(f'- name {m["name"]!r}; parent {m["parent"]!r}; base unit {m.get("base_units", m.get("base_unit", NR))}; '
          f'gst {j(m["gst"])}; hsn {j(m["hsn"])}; inherited_from_group_or_company {m.get("inherited_from_group_or_company", NR)!r}')
def has_rate(m): return any(e for e in m['gst'].get('entries', []))
def has_hsn(m): return any(e for e in m['hsn'].get('entries', []))
print(f'Counts: with a GST rate entry {sum(map(has_rate, it))}; with an HSN entry {sum(map(has_hsn, it))}; '
      f'with neither {sum(1 for m in it if not has_rate(m) and not has_hsn(m))}; of {len(it)}.')

v = res('005-r5-vchtypes.json')['masters']
s = [m for m in v if m.get('parent') == 'Sales']
print(f'\n## r5 masters voucher_types: {len(v)} rows; of the Sales class (parent "Sales"): {len(s)}')
for m in s: print(f'- name {m["name"]!r}; active {m.get("active")}; numbering_method {m.get("numbering_method")!r}; series_numbering {j(m.get("series_numbering"))}')

def ledgers(f, label):
    r = res(f); items = r['items']
    print(f'\n## {label}: {len(items)} rows; group_filter {j(r.get("group_filter"))}')
    return items
for m in ledgers('006-r6-sales-ledgers.json', 'r6 ledger_masters Sales Accounts (ancestry, compliance, gst details)'):
    c = m['compliance']
    print(f'- name {m["name"]!r}; parent {m["parent"]!r}; tax_type {c.get("tax_type")!r}; gst_duty_head {j(c.get("gst_duty_head"))}; '
          f'affects stock {m.get("affects_stock", c.get("affects_stock", NR))}')
    print(f'  gst_details {j(c.get("gst_details"))}')
for m in ledgers('007-r7-debtors.json', 'r7 ledger_masters Sundry Debtors (ancestry, compliance, gst details)'):
    c = m['compliance']
    print(f'- name {m["name"]!r}; parent {m["parent"]!r}; registration type {m.get("party_gstin_registration_type")!r}; '
          f'gst_registrations {j(c.get("gst_registrations"))}; GSTIN status {m.get("party_gstin_status")!r}; state {c.get("state")!r}; '
          f'bill-wise {m.get("bill_wise", c.get("bill_wise", NR))}')
for m in ledgers('008-r8-duties.json', 'r8 ledger_masters Duties & Taxes (ancestry, compliance, gst details)'):
    c = m['compliance']
    print(f'- name {m["name"]!r}; parent {m["parent"]!r}; tax_type {c.get("tax_type")!r}; gst_duty_head {j(c.get("gst_duty_head"))}; '
          f'own rate details (gst_details) {j(c.get("gst_details"))}')
