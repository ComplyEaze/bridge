# Lab helper (2162-SALES-Q, from 1392-E34's boxcheck.py at lab/1392-e34-sheet 0e2fc769; not product code): the sheet's box
# check for one Sales invoice. Waits for box.ps1's log of Bridge's approval box to reach its WAITING line, then checks the
# box's text against this company, its GUID, this batch's id, its voucher count (one), and the built invoice: its type,
# date, number, customer, every entry line and both totals, read from the build answer and its saved XML (sha256 equal to
# the build answer's). Only when every check passes does it write Yes to box/<tag>.answer; otherwise it writes No and exits
# 1 (a stop). Writes its checks to box/<tag>-check.out, which never holds the box's company-number line.
# Usage: python -I -X utf8 boxcheck.py <tag> <batch_id> <count>
# Kept from 1392-E34 as they were: the log is read as bytes and taken only when it decodes whole as UTF-8, ends with a line
# break, and its WAITING line is whole; a refused or cut read is read again, for at most 30 s of such reads in a row, after
# which it writes No. Any error of this helper writes No (fail closed), with the error on stderr.
# Changed for this row: the folders, the GUID and the company; only the invoice text is accepted (the one-voucher text
# of other types and the several-voucher text are refused). The invoice text is the format string of
# agent_import_post.rs:3655 at 8c3fb9f2 (its test, agent_import_post_tests.rs:4709-4731), and row 2440-CP's box on this
# build (lab/2440-composed-answers, evidence/s8-window-a.txt) reads the same line for line: a date without a leading zero
# (shown_date), names quoted as JSON strings, entry lines "Dr 1051.00  \"<ledger>\"", totals without trailing zeros.
import re, sys, time, os, traceback
from datetime import datetime, timezone
tag, batch, n = sys.argv[1], sys.argv[2], int(sys.argv[3])
D = 'C:/bridge-lab/2162-sales-q/box'; log = f'{D}/{tag}-box.txt'
G = '6b43e498-430c-4d5c-bfef-d32e2ab93c85'; C = 'BRIDGE PILOT LAB'
MON = ('Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec')
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
def built(batch):
    # The one build answer of this launch naming the batch, and its saved XML read as bytes (UTF-8, or UTF-16 with a BOM).
    import glob, hashlib, json, xml.etree.ElementTree as ET
    found = []
    for f in glob.glob('C:/bridge-lab/2162-sales-q/run-g/answers/' + '*.json'):
        j = json.load(open(f, encoding='utf-8'))
        r = ((j.get('response') or {}).get('result') or {}).get('structuredContent') or {}
        r = r.get('result') if isinstance(r.get('result'), dict) else {}
        if j.get('tool') == 'build_import_xml' and r.get('batch_id') == batch: found.append(r)
    if len(found) != 1 or found[0].get('voucher_count') != 1 or not found[0].get('path'): return None, None
    b = found[0]; xd = open(b['path'], 'rb').read()
    if hashlib.sha256(xd).hexdigest() != b.get('sha256'): return None, None
    text = xd.decode('utf-16') if xd[:2] in (b'\xff\xfe', b'\xfe\xff') else xd.decode('utf-8-sig')
    vs = ET.fromstring(re.sub(r'^\s*<\?xml[^>]*\?>', '', text)).findall('.//VOUCHER')
    return (b, vs[0]) if len(vs) == 1 else (None, None)
try:
    t, reads = read_whole()
    one = []
    b, v = built(batch) if n == 1 else (None, None)
    if v is None: one = [('built invoice read (one build answer naming the batch, its XML sha256 equal, voucher_count 1, one VOUCHER)', False)]
    else:
        import json
        from decimal import Decimal
        q = lambda s: json.dumps(s, ensure_ascii=False)
        d = datetime.strptime(v.findtext('DATE'), '%Y%m%d'); dt = f'{d.day}-{MON[d.month - 1]}-{d.year}'
        vt, num, party = v.findtext('VOUCHERTYPENAME'), v.findtext('VOUCHERNUMBER'), v.findtext('PARTYLEDGERNAME')
        legs = v.findall('LEDGERENTRIES.LIST') + v.findall('ALLLEDGERENTRIES.LIST')
        ents = [('Dr' if e.findtext('ISDEEMEDPOSITIVE') == 'Yes' else 'Cr', '%.2f' % abs(Decimal(e.findtext('AMOUNT'))), e.findtext('LEDGERNAME')) for e in legs]
        dr = sum(Decimal(a) for s, a, _ in ents if s == 'Dr'); cr = sum(Decimal(a) for s, a, _ in ents if s == 'Cr')
        boxl = re.findall(r'^(Dr|Cr) (\d+\.\d\d)\s+"([^"]*)"\s*$', t, re.M)
        tot = re.findall(r'^Total debit: (\d+(?:\.\d+)?)  Total credit: (\d+(?:\.\d+)?)\s*$', t, re.M)
        def totals_ok():
            if len(tot) != 1: return False
            try: wd, wc = Decimal(str(b['total_debit'])), Decimal(str(b['total_credit']))
            except Exception: return False
            return Decimal(tot[0][0]) == dr == wd and Decimal(tot[0][1]) == cr == wc and dr == cr
        one = [('title names this voucher', re.search(r"title 'ComplyEaze Bridge . post this voucher\?'", t) is not None),
               (f'Create ONE Sales invoice in {q(C)}', re.search(r'^Create ONE Sales invoice in ' + re.escape(q(C)) + r'\s*$', t, re.M) is not None),
               ('one Create line', len(re.findall(r'^Create ', t, re.M)) == 1),
               (f'Voucher type: {q(vt)}  Date: {dt}  Number: {q(num)}',
                re.search(r'^Voucher type: ' + re.escape(q(vt)) + r'  Date: ' + re.escape(dt) + r'  Number: ' + re.escape(q(num)) + r'\s*$', t, re.M) is not None),
               ('one Number line', len(re.findall(r'Number: "', t)) == 1),
               (f'Customer: {q(party)}', re.search(r'^Customer: ' + re.escape(q(party)) + r'\s*$', t, re.M) is not None),
               ('entry lines equal the built invoice: ' + '; '.join(' '.join(x) for x in ents), sorted(boxl) == sorted(ents) and len(ents) > 0),
               (f'totals equal the built invoice and the build answer: debit {dr} credit {cr}', totals_ok())]
    checks = [
        ('box seen', 'BOX SEEN' in t), *one,
        ('Company GUID', re.search(r'^Company GUID: ' + re.escape(G) + r'\s*$', t, re.M) is not None),
        ('Batch', re.search(r'^Batch: ' + re.escape(batch) + r'\s*$', t, re.M) is not None),
        ('one Batch line', len(re.findall(r'^Batch: ', t, re.M)) == 1),
    ]
    ok = all(v for _, v in checks) and n == 1
    out = [f'{now()} check of {tag} for batch {batch}, {n} vouchers (log read whole after {reads} read(s))'] + [('PASS ' if v else 'FAIL ') + k for k, v in checks]
    if n != 1: out.append('FAIL voucher count 1 (this row posts one invoice a batch)')
    out.append('ALL PASS' if ok else 'NOT ALL PASS')
    out += ['  | ' + l.strip()[:160] for l in t.splitlines() if l.startswith('Total debit')]
    answer('Yes' if ok else 'No', out)
    sys.exit(0 if ok else 1)
except SystemExit: raise
except BaseException as e:
    traceback.print_exc()
    try: answer('No', [f'{now()} check of {tag} for batch {batch}: HELPER ERROR {type(e).__name__}: {e}', 'NOT ALL PASS (fail closed)'])
    finally: sys.exit(2)
