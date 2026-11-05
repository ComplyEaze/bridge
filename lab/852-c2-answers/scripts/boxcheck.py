# Lab helper (852-C2, from 1392-E, from 1029-R, not product code): the sheet's box check. Waits for box.ps1's log of Bridge's approval box to reach
# its WAITING line, then checks the box's text names this company, its GUID, this batch's id and this batch's voucher
# count. Only when every check passes does it write Yes to box/<tag>.answer; otherwise it writes No (the sheet's Decline)
# and exits 1 (a stop). Writes its checks to box/<tag>-check.out. Usage: python -I -X utf8 boxcheck.py <tag> <batch_id> <count>
# 1392-E, on the run lead's word: the log is read as bytes and taken only when it decodes whole as UTF-8, ends with a line
# break, and its WAITING line is whole; a refused or cut read is read again, for at most 30 s of such reads in a row, after
# which it writes No. Any error of this helper writes No (fail closed), with the error on stderr.
import re, sys, time, os, traceback
from datetime import datetime, timezone
tag, batch, n = sys.argv[1], sys.argv[2], int(sys.argv[3])
D = '<LAB_DIR>/box'; log = f'{D}/{tag}-box.txt'
G = '2ba6df28-d1e3-4e81-b6d2-eb50f0a693ab'; C = 'BRIDGE LARGE LAB'
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
def answer(ans, out):
    open(f'{D}/{tag}.answer', 'w', encoding='utf-8').write(ans)
    out.append(f'{now()} answer file written: {ans}')
    open(f'{D}/{tag}-check.out', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
    print('\n'.join(out))
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
    t, reads = read_whole()
    # 852-C2, on the run lead's word: 852-C1's content rules (this company, its GUID, this batch id, 50 vouchers, the plural
    # wording); only the guarded read above is 1392-E's. A count other than 50 fails.
    if True:
        one = [('count is 50', n == 50), (f'title names {n} vouchers', re.search(r"title 'ComplyEaze Bridge . post " + str(n) + r" vouchers\?'", t) is not None),
               (f'Create {n} vouchers in "{C}"', f'Create {n} vouchers in "{C}"' in t),
               ('one Create line', len(re.findall(r'^Create \d+ vouchers in ', t, re.M)) == 1)]
    checks = [
        ('box seen', 'BOX SEEN' in t), *one,
        ('Company GUID', f'Company GUID: {G}' in t),
        ('Batch', re.search(r'^Batch: ' + re.escape(batch) + r'\s*$', t, re.M) is not None),
    ]
    ok = all(v for _, v in checks)
    out = [f'{now()} check of {tag} for batch {batch}, {n} vouchers (log read whole after {reads} read(s))'] + [('PASS ' if v else 'FAIL ') + k for k, v in checks]
    out.append('ALL PASS' if ok else 'NOT ALL PASS')
    for k in ('Types:', 'Dates:', 'Total debit', 'entries'):
        out += ['  | ' + l.strip()[:160] for l in t.splitlines() if k in l]
    answer('Yes' if ok else 'No', out)
    sys.exit(0 if ok else 1)
except SystemExit: raise
except BaseException as e:
    traceback.print_exc()
    try: answer('No', [f'{now()} check of {tag} for batch {batch}: HELPER ERROR {type(e).__name__}: {e}', 'NOT ALL PASS (fail closed)'])
    finally: sys.exit(2)
