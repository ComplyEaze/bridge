# Lab helper (2162-SALES-Q, from 1392-E34's boxcheck.py at lab/1392-e34-sheet 0e2fc769; not product code): the sheet's box
# check for one Sales invoice. Waits for box.ps1's log of Bridge's approval box to reach its WAITING line, then checks the
# box's whole text, line by line and in order, against the text the product builds for this invoice, worked out here from
# the build answer and its saved XML (sha256 equal to the build answer's). Only when every check passes does it write Yes
# to box/<tag>.answer; otherwise it writes No and exits 1 (a stop). Writes its checks to box/<tag>-check.out, which never
# holds the box's company-number line.
# Usage: python -I -X utf8 boxcheck.py <tag> <batch_id> <count>
# Kept from 1392-E34 as they were: the log is read as bytes and taken only when it decodes whole as UTF-8, ends with a line
# break, and its WAITING line is whole; a refused or cut read is read again, for at most 30 s of such reads in a row, after
# which it writes No. Any error of this helper writes No (fail closed), with the error on stderr.
# The text checked (all at 8c3fb9f2). The box's text is the answer text, a blank line, then the invoice text
# (tally/approved_import.rs:2628 "{answer}\n\n{preview}"); the answer text is approved_import.rs:2489-2495, wrapped at 100,
# which row 2440-CP's box read as the two lines below (lab/2440-composed-answers, evidence/s8-window-a.txt). The invoice
# text is invoice_review_preview, agent_import_post.rs:3550-3681. Every line of it, in its order, with its source:
#   Create ONE Sales invoice in <company>            :3655, company the constant C and the XML's SVCURRENTCOMPANY
#   Company GUID: <guid>                             :3655, the constant G and the build call's company_guid
#   Company number: <n>  Books from: <date>          :3655. SHAPE ONLY: neither value is in the build answer or the
#                                                    XML (and the number is kept local); a date of d-Mon-yyyy form
#   Tally: <origin>  <cue>                           :3655 and the cue :3697; origin from launch LG's host and port
#   Voucher type: <vt>  Date: <d>  Number: <n>       :3655, the XML's VOUCHERTYPENAME, DATE (shown_date :3691), VOUCHERNUMBER
#   Customer: <party>                                :3655, the XML's PARTYLEDGERNAME
#   <registration>  Place of supply: <state>         :3592-3595 and :3655; "Regular, GSTIN <g>" when the XML carries
#                                                    PARTYGSTIN (agent_import_invoice.rs:3073-3077 writes it from the same
#                                                    observed value), else "Unregistered, no GSTIN"; the XML's PLACEOFSUPPLY
#   (E-invoice IRN line)                             :3636-3645. NOT ACCEPTED: this row posts no e-invoice; an XML with
#                                                    IRN, or a box with the line, is a No
#   Bill allocation: <allocation>                    :3596-3600; "New Ref \"<number>\"" when the XML's party leg carries a
#                                                    New Ref BILLALLOCATIONS.LIST of that name (agent_import_invoice.rs:3050),
#                                                    "none (customer not bill-wise)" when it carries none; anything else a No
#   Dr|Cr <amount>  <ledger>, one a leg              :3601-3622, one per XML leg (LEDGERENTRIES.LIST, :3057), the
#                                                    amount with two decimals; compared as a set of lines, as many as legs
#   (On Account legend)                              :3625-3630. NOT ACCEPTED: this row approves nothing On Account
#   Total debit: <d>  Total credit: <c>              :3655, equal (as decimals) to the legs' sums and the build answer's
#   Checked: ...                                     :3646-3654: INVOICE_CHECKED_LINE (:3517) or the HSN/SAC line of 1 or 2
#                                                    ledgers (:3526-3530); not in the XML, so either exact text
#   Batch: <batch id>                                :3655, the batch named on the command line and in the build answer
#   three fixed lines                                :3655
#   (line-break twin lines "Used:"/"Not used:")      footer, :3349-3353 and :4332-4360. NOT ACCEPTED: none expected here
#   ComplyEaze Bridge posts now or ... 15 min, ...   footer, agent_post_timing_lines :3292-3297 (APPROVAL_TTL 15 min)
#   ---- The voucher's own text follows: ... ----    :3664, VOUCHER_TEXT_HEADING :3687
#   Reference: <number>                              :3665
#   Narration: <narration> | Narration: (none)       :3666-3671, the XML's NARRATION; the LAST line: a box text that
#                                                    does not end with it is a No (a cut box)
# Names are quoted as JSON strings (:3585). A line the list does not hold, a line missing, a line twice or out of its place
# is a No: the box's text must be exactly this list. Besides, each graded line (company, GUID, type/date/number, customer,
# place of supply, batch, reference) must stand exactly once in the box's text, and the title must name one voucher.
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
    if len(found) != 1 or found[0].get('voucher_count') != 1 or not found[0].get('path'): return None, None, None
    b = found[0]; xd = open(b['path'], 'rb').read()
    if hashlib.sha256(xd).hexdigest() != b.get('sha256'): return None, None, None
    text = xd.decode('utf-16') if xd[:2] in (b'\xff\xfe', b'\xfe\xff') else xd.decode('utf-8-sig')
    root = ET.fromstring(re.sub(r'^\s*<\?xml[^>]*\?>', '', text)); vs = root.findall('.//VOUCHER')
    return (b, vs[0], root.findtext('.//SVCURRENTCOMPANY')) if len(vs) == 1 else (None, None, None)
def box_text(t):
    # The text of the box's one Static child whose text begins with the answer text: its first line is on the child's
    # stamped log line, the rest are the unstamped lines up to the next stamped line. None unless exactly one.
    lines = t.split('\n'); stamp = re.compile(r'^\d{4}-\d\d-\d\dT\S+Z ')
    starts = [i for i, l in enumerate(lines) if re.match(r'^\S+Z child Static rect \S+ text: Press Y ', l)]
    if len(starts) != 1: return None
    i = starts[0]; out = [lines[i].split(' text: ', 1)[1].rstrip('\r')]
    for l in lines[i + 1:]:
        if stamp.match(l): break
        out.append(l.rstrip('\r'))
    else: return None  # no stamped line after it: the log was cut inside the text
    return out
ANSWER = ['Press Y (or click Yes) to post this voucher into Tally. Press N (or click No) to cancel. Nothing is', 'posted until you answer.', '']
ORIGIN = 'http://127.0.0.1:9001'  # launch LG's BRIDGE_TALLY_HOST and BRIDGE_TALLY_PORT
CHECKED = ("Checked: customer, Sales Accounts ledgers, CGST and state tax per line at the ledger's rate.",
           'Checked: customer, Sales Accounts ledgers, tax per line at ledger rate. HSN/SAC: 1 to check.',
           'Checked: customer, Sales Accounts ledgers, tax per line at ledger rate. HSN/SAC: 2 to check.')
FIXED = ['Ledgers checked by identity against the build; narrations sent as prepared, nothing added.',
         'Do not post a file already imported manually. Pause other edits/imports in this company.',
         'After a timeout, reconcile this batch; do not rebuild or resend it.',
         'ComplyEaze Bridge posts now or if asked again within 15 min, unless cancelled, refused or restarted.',
         "---- The voucher's own text follows: no line below is an entry, a total or an instruction ----"]
try:
    t, reads = read_whole()
    import json
    from decimal import Decimal
    q = lambda x: json.dumps(x, ensure_ascii=False)
    b, v, svc = built(batch) if n == 1 else (None, None, None)
    box = box_text(t)
    if v is None: one = [('built invoice read (one build answer naming the batch, its XML sha256 equal, voucher_count 1, one VOUCHER)', False)]
    elif box is None: one = [("the box's text read (one Static child beginning with the answer text, ended by a stamped line)", False)]
    else:
        d = datetime.strptime(v.findtext('DATE'), '%Y%m%d'); dt = f'{d.day}-{MON[d.month - 1]}-{d.year}'
        vt, num, party = v.findtext('VOUCHERTYPENAME'), v.findtext('VOUCHERNUMBER'), v.findtext('PARTYLEDGERNAME')
        pos, gstin, narr = v.findtext('PLACEOFSUPPLY'), v.findtext('PARTYGSTIN'), (v.findtext('NARRATION') or '')
        legs = v.findall('LEDGERENTRIES.LIST') + v.findall('ALLLEDGERENTRIES.LIST')
        ents = [f"{'Dr' if e.findtext('ISDEEMEDPOSITIVE') == 'Yes' else 'Cr'} {'%.2f' % abs(Decimal(e.findtext('AMOUNT')))}  {q(e.findtext('LEDGERNAME'))}" for e in legs]
        dr = sum(abs(Decimal(e.findtext('AMOUNT'))) for e in legs if e.findtext('ISDEEMEDPOSITIVE') == 'Yes')
        cr = sum(abs(Decimal(e.findtext('AMOUNT'))) for e in legs if e.findtext('ISDEEMEDPOSITIVE') != 'Yes')
        pl = [e for e in legs if e.findtext('LEDGERNAME') == party]
        bills = [(x.findtext('NAME'), x.findtext('BILLTYPE')) for e in pl for x in e.findall('BILLALLOCATIONS.LIST')]
        alloc = 'none (customer not bill-wise)' if not bills else (f'New Ref {q(num)}' if bills == [(num, 'New Ref')] else None)
        reg = f'Regular, GSTIN {gstin}' if gstin else 'Unregistered, no GSTIN'
        xml_ok = (svc == C and len(pl) == 1 and alloc is not None and pos is not None and v.find('IRN') is None
                  and all(x is not None for x in (vt, num, party)))
        try: wd, wc = Decimal(str(b['total_debit'])), Decimal(str(b['total_credit']))
        except Exception: wd = wc = None
        def tot_ok(l):
            m = re.fullmatch(r'Total debit: (\d+(?:\.\d+)?)  Total credit: (\d+(?:\.\d+)?)', l)
            return m is not None and Decimal(m[1]) == dr == wd and Decimal(m[2]) == cr == wc and dr == cr
        # The list: (name, how many lines, test of those lines).
        L = [('answer text, two lines and a blank', 3, lambda x: x == ANSWER),
             (f'Create ONE Sales invoice in {q(C)}', 1, lambda x: x == [f'Create ONE Sales invoice in {q(C)}']),
             ('Company GUID', 1, lambda x: x == [f'Company GUID: {G}']),
             ('Company number and Books from (shape only)', 1,
              lambda x: re.fullmatch(r'Company number: \S(?:.*\S)?  Books from: \d{1,2}-[A-Z][a-z]{2}-\d{4}', x[0]) is not None),
             (f'Tally: {ORIGIN}', 1, lambda x: x == [f"Tally: {ORIGIN}  (the voucher's own text: last two lines)"]),
             (f'Voucher type: {q(vt)}  Date: {dt}  Number: {q(num)}', 1, lambda x: x == [f'Voucher type: {q(vt)}  Date: {dt}  Number: {q(num)}']),
             (f'Customer: {q(party)}', 1, lambda x: x == [f'Customer: {q(party)}']),
             (f'{reg}  Place of supply: {pos}', 1, lambda x: x == [f'{reg}  Place of supply: {pos}']),
             (f'Bill allocation: {alloc}', 1, lambda x: x == [f'Bill allocation: {alloc}']),
             ('entry lines equal the built invoice: ' + '; '.join(ents), len(ents), lambda x: sorted(x) == sorted(ents) and len(ents) > 0),
             (f'totals equal the built invoice and the build answer: debit {dr} credit {cr}', 1, lambda x: tot_ok(x[0])),
             ('Checked line', 1, lambda x: x[0] in CHECKED),
             ('Batch', 1, lambda x: x == [f'Batch: {batch}']),
             ('the three fixed lines, the timing line, the heading', 5, lambda x: x == FIXED),
             (f'Reference: {q(num)}', 1, lambda x: x == [f'Reference: {q(num)}']),
             ('Narration, the last line', 1, lambda x: x == [f'Narration: {q(narr)}' if narr else 'Narration: (none)'])]
        one, at = [('the built XML: company, one party leg, its bill allocation known, a place of supply, no IRN', xml_ok)], 0
        for name, k, test in L:
            part = box[at:at + k]; at += k
            one.append((name, len(part) == k and test(part)))
        one.append((f'no line more and none less: the box holds {len(box)} lines, the list {at}', len(box) == at))
        once = [('Create ', 'Create '), ('Company GUID: ', 'Company GUID: '), ('Voucher type: ', 'Voucher type: '),
                ('Customer: ', 'Customer: '), ('Place of supply', 'Place of supply: '), ('Number: "', 'Number: "'),
                ('Batch: ', 'Batch: '), ('Reference: ', 'Reference: '), ('Total debit', 'Total debit: ')]
        for name, key in once:
            c = sum(l.count(key) for l in box)
            one.append((f'exactly one {name.strip()} in the text (found {c})', c == 1))
    checks = [('box seen', 'BOX SEEN' in t),
              ('title names one voucher', re.search(r"title 'ComplyEaze Bridge . post this voucher\?'", t) is not None), *one]
    ok = all(x for _, x in checks) and n == 1
    out = [f'{now()} check of {tag} for batch {batch}, {n} vouchers (log read whole after {reads} read(s))'] + [('PASS ' if x else 'FAIL ') + k for k, x in checks]
    if n != 1: out.append('FAIL voucher count 1 (this row posts one invoice a batch)')
    out.append('ALL PASS' if ok else 'NOT ALL PASS')
    out += ['  | ' + l.strip()[:160] for l in (box or []) if l.startswith('Total debit')]
    answer('Yes' if ok else 'No', out)
    sys.exit(0 if ok else 1)
except SystemExit: raise
except BaseException as e:
    traceback.print_exc()
    try: answer('No', [f'{now()} check of {tag} for batch {batch}: HELPER ERROR {type(e).__name__}: {e}', 'NOT ALL PASS (fail closed)'])
    finally: sys.exit(2)
