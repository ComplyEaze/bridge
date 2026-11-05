# Lab helper (1392-E, from 1029-R, not product code): the sheet's box check. Waits for box.ps1's log of Bridge's approval box to reach
# its WAITING line, then checks the box's text names this company, its GUID, this batch's id and this batch's voucher
# count. Only when every check passes does it write Yes to box/<tag>.answer; otherwise it writes No (the sheet's Decline)
# and exits 1 (a stop). Writes its checks to box/<tag>-check.out. Usage: python -I -X utf8 boxcheck.py <tag> <batch_id> <count>
import re, sys, time, os
from datetime import datetime, timezone
tag, batch, n = sys.argv[1], sys.argv[2], int(sys.argv[3])
D = '<LAB_DIR>/box'; log = f'{D}/{tag}-box.txt'
G = 'd6e4c24f-8c7a-4797-93ec-e44bdff60bc9'; C = 'BRIDGE TRANSFER LAB'
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
t = ''
for _ in range(1800):
    if os.path.exists(log):
        t = open(log, encoding='utf-8-sig').read()
        if 'WAITING for' in t or 'NO BOX seen' in t: break
    time.sleep(0.25)
checks = [
    ('box seen', 'BOX SEEN' in t),
    (f'title names {n} vouchers', re.search(r"title 'ComplyEaze Bridge . post " + str(n) + r" vouchers\?'", t) is not None),
    (f'Create {n} vouchers in "{C}"', f'Create {n} vouchers in "{C}"' in t),
    ('one Create line', len(re.findall(r'^Create \d+ vouchers in ', t, re.M)) == 1),
    ('Company GUID', f'Company GUID: {G}' in t),
    ('Batch', re.search(r'^Batch: ' + re.escape(batch) + r'\s*$', t, re.M) is not None),
]
ok = all(v for _, v in checks); ans = 'Yes' if ok else 'No'
out = [f'{now()} check of {tag} for batch {batch}, {n} vouchers'] + [('PASS ' if v else 'FAIL ') + k for k, v in checks]
out.append('ALL PASS' if ok else 'NOT ALL PASS')
for k in ('Types:', 'Dates:', 'Total debit', 'entries'):
    out += ['  | ' + l.strip()[:160] for l in t.splitlines() if k in l]
open(f'{D}/{tag}.answer', 'w', encoding='utf-8').write(ans)
out.append(f'{now()} answer file written: {ans}')
open(f'{D}/{tag}-check.out', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
print('\n'.join(out))
sys.exit(0 if ok else 1)
