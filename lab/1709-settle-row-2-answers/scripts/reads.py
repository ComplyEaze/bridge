# Lab helper (#1709-settle second sitting, not product code): the sheet's MARKS and J-LIST reads through this row's stdio client.
# Usage: python -I reads.py marks <tag> | jlist <tag> | window <tag> <from> <to> | day <tag> <yyyymmdd>
# MARKS: vouchers 20250615 to 20250622, offset 0, no other optional argument; prints state, total and the snapshot's marks.
# J-LIST: vouchers with voucher_class Journal, one call per calendar month from Apr 2025 to Apr 2026 (Tally's current date at
# step 0), offset 0; writes tables/<tag>.tsv (DATE, VOUCHERNUMBER, MASTERID, GUID) and prints its sha256. A refused month is
# printed and the run stops, so it can be halved by hand.
import calendar
import hashlib
import json
import os
import sys

ROOT = '<lab folder>'
sys.path.insert(0, ROOT)
from call import call  # noqa: E402

GUID = '3a6bd6e1-b835-4bff-89dd-8a6af138c346'


def result_of(body):
    return (body.get('result') or {}) if isinstance(body, dict) else {}


def marks(tag):
    for attempt in (1, 2):
        fn, secs, err, body = call(f'{tag}-marks' + ('' if attempt == 1 else '-again'), 'vouchers',
                                   {'company_guid': GUID, 'from': '20250615', 'to': '20250622', 'offset': 0})
        r = result_of(body)
        snap = r.get('snapshot') or {}
        print(fn, f'{secs:.3f}s isError {err} state {r.get("state")} total {r.get("total")} snapshot',
              json.dumps({k: snap.get(k) for k in ('master_alter_id', 'voucher_alter_id', 'reused')}))
        if err or not snap:
            print('HALT: no snapshot (or an error) in a MARKS read')
            return 2
        if snap.get('reused') is False:
            return 0
    print('HALT: snapshot.reused still true on the second read')
    return 2


def jlist(tag):
    rows = []
    months = [(2025, m) for m in range(4, 13)] + [(2026, m) for m in range(1, 5)]
    for y, m in months:
        last = calendar.monthrange(y, m)[1]
        fn, secs, err, body = call(f'{tag}-{y}{m:02d}', 'vouchers',
                                   {'company_guid': GUID, 'from': f'{y}{m:02d}01', 'to': f'{y}{m:02d}{last:02d}',
                                    'voucher_class': 'Journal', 'offset': 0})
        r = result_of(body)
        items = r.get('items') or []
        withheld = r.get('withheld_vouchers') or []
        print(fn, f'{secs:.3f}s isError {err} state {r.get("state")} total {r.get("total")} items {len(items)} withheld {len(withheld)}')
        if err:
            print('HALT: refused month; halve it by hand:', json.dumps(body)[:400])
            return 2
        if r.get('total') is not None and r.get('total') != len(items) + len(withheld):
            print('HALT: total differs from the items on this page (a second page?)')
            return 2
        for v in items:
            rows.append((v.get('date'), v.get('voucher_number'), v.get('master_id'), v.get('guid'), 'item'))
        for v in withheld:
            rows.append((v.get('date'), v.get('voucher_number'), v.get('master_id'), v.get('guid'), 'withheld'))
    rows.sort(key=lambda t: int(t[2] or 0))
    os.makedirs(os.path.join(ROOT, 'tables'), exist_ok=True)
    out = os.path.join(ROOT, 'tables', f'{tag}.tsv')
    text = 'DATE\tVOUCHERNUMBER\tMASTERID\tGUID\tsource\n' + ''.join('\t'.join(str(x) for x in t) + '\n' for t in rows)
    with open(out, 'w', encoding='utf-8', newline='\n') as f:
        f.write(text)
    print(f'{tag}: {len(rows)} Journals; tables/{tag}.tsv sha256 {hashlib.sha256(text.encode()).hexdigest()}')
    for t in rows:
        print('  ', *t)
    return 0


def window(tag, frm, to):
    fn, secs, err, body = call(tag, 'vouchers', {'company_guid': GUID, 'from': frm, 'to': to, 'offset': 0})
    r = result_of(body)
    snap = r.get('snapshot') or {}
    print(fn, f'{secs:.3f}s isError {err} state {r.get("state")} reason {r.get("reason")} total {r.get("total")} snapshot',
          json.dumps({k: snap.get(k) for k in ('master_alter_id', 'voucher_alter_id', 'reused')}) if snap else 'none')
    for v in r.get('items') or []:
        print('  ', v.get('date'), v.get('voucher_type'), v.get('voucher_number'), v.get('master_id'), v.get('guid'))
    return 0


if __name__ == '__main__':
    a = sys.argv[1:]
    if a[0] == 'marks':
        sys.exit(marks(a[1]))
    if a[0] == 'jlist':
        sys.exit(jlist(a[1]))
    if a[0] == 'window':
        sys.exit(window(a[1], a[2], a[3]))
    sys.exit('usage')
