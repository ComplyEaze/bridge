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
if n == 1:  # 1392-E, on the run lead's word: Bridge's one-voucher box reads 'post this voucher?' and 'Create ONE <type> in
    # "<company>"'; accepted only for a batch of one, and checked against the built batch: its saved XML (sha256 equal to the
    # build answer's) gives the type, the date and each line, Dr where ISDEEMEDPOSITIVE is Yes.
    import glob, hashlib, json, xml.etree.ElementTree as ET
    bf = [f for f in glob.glob('<LAB_DIR>/agent-run/answers/*-build.json')
          if (json.load(open(f, encoding='utf-8'))['response']['result'].get('structuredContent') or {}).get('result', {}).get('batch_id') == batch]
    b = json.load(open(bf[0], encoding='utf-8'))['response']['result']['structuredContent']['result'] if len(bf) == 1 else {}
    xd = open(b['path'], 'rb').read() if b.get('path') else b''
    v = ET.fromstring(xd).find('.//VOUCHER') if xd and hashlib.sha256(xd).hexdigest() == b.get('sha256') and b.get('voucher_count') == 1 else None
    if v is None: one = [('built batch read (one build answer, its XML sha256 equal, voucher_count 1)', False)]
    else:
        vt, dt = v.findtext('VOUCHERTYPENAME'), datetime.strptime(v.findtext('DATE'), '%Y%m%d').strftime('%d-%b-%Y')
        ents = [('Dr' if e.findtext('ISDEEMEDPOSITIVE') == 'Yes' else 'Cr', '%.2f' % abs(float(e.findtext('AMOUNT'))), e.findtext('LEDGERNAME')) for e in v.findall('ALLLEDGERENTRIES.LIST')]
        boxl = re.findall(r'^(Dr|Cr) (\d+\.\d\d)\s+"([^"]*)"\s*$', t, re.M)
        one = [('title names this voucher', re.search(r"title 'ComplyEaze Bridge . post this voucher\?'", t) is not None),
               (f'Create ONE {vt} in "{C}"', re.search(r'^Create ONE ' + re.escape(vt) + r' in "' + re.escape(C) + r'"\s*$', t, re.M) is not None),
               ('one Create line', len(re.findall(r'^Create ', t, re.M)) == 1),
               (f'Date: {dt}', re.search(r'^Date: ' + re.escape(dt) + r'\s', t, re.M) is not None),
               ('entry lines equal the built batch: ' + '; '.join(' '.join(x) for x in ents), sorted(boxl) == sorted(ents) and len(ents) > 0)]
else:
    one = [(f'title names {n} vouchers', re.search(r"title 'ComplyEaze Bridge . post " + str(n) + r" vouchers\?'", t) is not None),
           (f'Create {n} vouchers in "{C}"', f'Create {n} vouchers in "{C}"' in t),
           ('one Create line', len(re.findall(r'^Create \d+ vouchers in ', t, re.M)) == 1)]
checks = [
    ('box seen', 'BOX SEEN' in t), *one,
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
