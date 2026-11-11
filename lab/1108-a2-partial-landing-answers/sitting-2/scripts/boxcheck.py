# Lab helper (1108-A2, from 1392-E's boxcheck.py, from 1029-R; not product code): the sheet's box check, Difference 3 as the run lead
# corrected it before the first write. Waits for box.ps1's log of Bridge's approval box to reach its WAITING line (read as bytes,
# taken only when it decodes whole as UTF-8 and ends with a line break; a cut read is read again for at most 30 s). Yes only if:
# (a) the box text holds exactly one line beginning "Create ", and that whole line is `Create ONE Journal in "BRIDGE STOCK CREATE LAB"`
# (one voucher) or `Create 3 vouchers in "BRIDGE STOCK CREATE LAB"` (three); (b) the whole lines `Company GUID: <THE COMPANY's GUID>`
# and `Batch: <the batch id>`; (c) the whole line `Total debit: X  Total credit: X`, X as the build writes it (10, 66 or 94, given
# as the argument; any other spelling, .00 included, fails). Per-voucher lines are not checked. Otherwise it writes No (the sheet's
# Decline) and exits 1 (a stop). Any error of this helper writes No (fail closed). Writes its checks to box/<tag>-check.out.
# Usage: python -I -X utf8 boxcheck.py <tag> <batch_id> <count 1|3> <total>
import os, re, sys, time, traceback
from datetime import datetime, timezone
tag, batch, n, total = sys.argv[1], sys.argv[2], int(sys.argv[3]), sys.argv[4]
D = '<LAB_DIR>/box'; log = f'{D}/{tag}-box.txt'
G = open('<LAB_DIR>/guid.txt').read().strip(); C = 'BRIDGE STOCK CREATE LAB'
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
def answer(ans, out):
    open(f'{D}/{tag}.answer', 'w', encoding='utf-8').write(ans)
    out.append(f'{now()} answer file written: {ans}')
    open(f'{D}/{tag}-check.out', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
    print('\n'.join(out).replace(G, '<company guid>'))
def read_whole():
    t, bad_since, reads = '', None, 0
    for _ in range(1800):
        if os.path.exists(log):
            reads += 1
            try:
                t = open(log, 'rb').read().decode('utf-8-sig', 'strict')
                if not t.endswith('\n'): raise ValueError('log not ended by a line break')
                bad_since = None
            except (OSError, UnicodeDecodeError, ValueError) as e:
                t = ''; bad_since = bad_since or time.monotonic()
                print(f'{now()} read {reads} not whole: {type(e).__name__}: {e}', file=sys.stderr)
                if time.monotonic() - bad_since > 30: raise RuntimeError('no whole read of the box log in 30 s')
            if re.search(r'^\S+Z (WAITING for \S.*|NO BOX seen.*)\n', t, re.M): return t, reads
        time.sleep(0.25)
    raise RuntimeError('no WAITING line in the box log')
try:
    if n not in (1, 3) or total not in ('10', '66', '94'): raise ValueError('count must be 1 or 3 and total 10, 66 or 94')
    t, reads = read_whole()
    lines = [l.rstrip('\r') for l in t.split('\n')]
    want = f'Create ONE Journal in "{C}"' if n == 1 else f'Create 3 vouchers in "{C}"'
    creates = [l for l in lines if l.startswith('Create ')]
    checks = [
        ('box seen', 'BOX SEEN' in t),
        (f'(a) one Create line, whole: {want}', creates == [want]),
        ('(b) Company GUID line, whole', f'Company GUID: {G}' in lines),
        (f'(b) Batch line, whole: Batch: {batch}', f'Batch: {batch}' in lines),
        (f'(c) totals line, whole: Total debit: {total}  Total credit: {total}', f'Total debit: {total}  Total credit: {total}' in lines),
    ]
    ok = all(v for _, v in checks)
    out = [f'{now()} check of {tag} for batch {batch}, {n} voucher(s), total {total} (log read whole after {reads} read(s))']
    out += [('PASS ' if v else 'FAIL ') + k for k, v in checks]
    out += [f'  Create lines seen: {creates}', 'ALL PASS' if ok else 'NOT ALL PASS']
    answer('Yes' if ok else 'No', out)
    sys.exit(0 if ok else 1)
except SystemExit: raise
except BaseException as e:
    traceback.print_exc()
    try: answer('No', [f'{now()} check of {tag} for batch {batch}: HELPER ERROR {type(e).__name__}: {e}', 'NOT ALL PASS (fail closed)'])
    finally: sys.exit(2)
