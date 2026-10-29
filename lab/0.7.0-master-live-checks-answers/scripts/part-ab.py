# Lab helper (0.7.0 master live checks, not product code): steps 3 to 6 of part A or B, one call at a time through this
# row's stdio client, and the sheet's EXPECT lines checked on each saved answer. Reads only.
# Usage: python -I part-ab.py <A|B> <company_guid>
import hashlib
import json
import os
import sys
from decimal import Decimal

ROOT = os.path.join('<lab folder>')
sys.path.insert(0, ROOT)
from call import call  # noqa: E402

part, guid = sys.argv[1].lower(), sys.argv[2]
out = []


def say(s):
    out.append(s)
    print(s)


def saved(fn):
    p = os.path.join(ROOT, 'answers', fn)
    data = open(p, 'rb').read()
    w = json.loads(data)
    return w, len(data), hashlib.sha256(data).hexdigest()


def amt(v):
    if v is None or v == '' or (isinstance(v, dict)):
        return Decimal(0)
    return Decimal(str(v))


fn, secs, err, body = call(f'{part}3-trial_balance', 'trial_balance', {'company_guid': guid, 'from': '20250401', 'to': '20260331', 'limit': 100})
w, size, sha = saved(fn)
sc = w['response']['result'].get('structuredContent') or {}
say(f'{part.upper()}3 {fn} {secs:.3f}s isError {err} file {size} B sha256 {sha}')
res = body['result']
rows = res.get('rows') or res.get('ledgers') or res.get('items') or []
flagged = [r for r in rows if 'movements_differ_by' in r]
say(f'{part.upper()}3 rows {len(rows)}; rows with movements_differ_by: {[r.get("name") or r.get("ledger") for r in flagged]}')
cs = [r for r in rows if (r.get('name') or r.get('ledger')) == 'Closing Stock']
if cs:
    r = cs[0]
    keys = sorted(r)
    say(f'{part.upper()}3 Closing Stock row keys {keys}')
say(f'{part.upper()}3 movements_differ_ledgers {res.get("movements_differ_ledgers")}')
lead = (sc.get('headline') or {}).get('lead', '')
want = '1 ledger does not add up: the closing is not the opening plus the debit and credit totals (see movements_differ_by); each is shown as Tally sent it.'
say(f'{part.upper()}3 headline lead holds the text: {want in lead}')
lim = res.get('limitations') or []
say(f'{part.upper()}3 limitations[0:2] begin: {[str(x)[:60] for x in lim[:2]]}')
say(f'{part.upper()}3 evidence {json.dumps(body.get("evidence"))}')
tb_ev = body.get('evidence') or {}
gap = cs[0].get('movements_differ_by') if cs else None
json.dump({'closing_stock_row': cs[0] if cs else None}, open(os.path.join(ROOT, 'evidence', f'{part}3-closing-stock-row.json'), 'w'))

for step, tool, args, code in ((4, 'profit_and_loss', {'company_guid': guid, 'from': '20250401', 'to': '20260331'}, 'profit_and_loss_read_failed'),
                               (5, 'balance_sheet', {'company_guid': guid, 'from': '20250401', 'to': '20260331'}, 'balance_sheet_read_failed')):
    fn, secs, err, body = call(f'{part}{step}-{tool}', tool, args)
    w, size, sha = saved(fn)
    sc = w['response']['result'].get('structuredContent') or {}
    e = ((sc.get('result') or {}).get('error')) or {}
    say(f'{part.upper()}{step} {fn} {secs:.3f}s isError {err} file {size} B sha256 {sha}')
    say(f'{part.upper()}{step} code {e.get("code")} (want {code}); cause {e.get("cause")}; ledger {e.get("ledger")!r}; gap equals {part.upper()}3 movements_differ_by: {e.get("gap") == gap}')
    rem = e.get('remediation') or ''
    say(f'{part.upper()}{step} remediation begins as cited: {rem.startswith("No statement figure was returned: ComplyEaze Bridge derives the statements only from Trial Balance rows that add up")}')
    ev = sc.get('evidence') or {}
    say(f'{part.upper()}{step} evidence bytes {ev.get("bytes")} request_sha256 {ev.get("request_sha256")} response_sha256 {ev.get("response_sha256")}; {part.upper()}3 trial_balance evidence bytes {tb_ev.get("bytes")} response_sha256 {tb_ev.get("response_sha256")}')

fn, secs, err, body = call(f'{part}6-marks', 'ledger_masters', {'company_guid': guid})
snap = body['result']['snapshot']
say(f'{part.upper()}6 {fn} marks M {snap["master_alter_id"]} V {snap["voucher_alter_id"]} reused {snap["reused"]}')
with open(os.path.join(ROOT, 'evidence', f'part-{part}-checks.txt'), 'w', encoding='utf-8', newline='\n') as f:
    f.write('\n'.join(out) + '\n')
