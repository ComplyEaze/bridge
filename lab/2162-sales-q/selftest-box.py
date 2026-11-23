# Lab helper (2162-SALES-Q, new; the shape of 1392-E34's selftest34.py; not product code): the self-test of this row's
# boxcheck.py. It runs boxcheck.py as written but for two strings: its box folder becomes <row>/box/selftest, and (for
# the cases) the folder whose build answers it reads becomes <row>/box/selftest/answers. Nothing is written outside
# <row>/box/selftest/; the row's own box logs and build answers are only read.
#   cases <row>                  before any box of the sitting (step 0). No invoice box log of this build was ever kept
#                                by box.ps1, so the logs are WRITTEN here: box.ps1's log form, holding the invoice text in
#                                the form of agent_import_post.rs:3655 at 8c3fb9f2 (which row 2440-CP's box read line for
#                                line), for one fixture invoice: the build's own request of BP/26-27/0010
#                                (template-import-0010.utf16le.xml, decoded to UTF-8) with a build answer naming it. The
#                                cases show the check's logic only: one right box gives Yes, and each field changed alone
#                                gives No. They are no evidence of what a real box holds; the replay is.
#   replay <row> <tag> <batch>   after the smoke box (step 13.4): the check once more on that real box log,
#                                box/<tag>-box.txt, against the real build answer of run-g. EXPECT Yes: the check reads a
#                                real invoice box of this build. Its answer file is written in box/selftest, where no
#                                box.ps1 reads it.
# Usage: python -I -X utf8 selftest-box.py cases C:/bridge-lab/2162-sales-q     (forward slashes)
import hashlib, json, os, subprocess, sys
BOX_LINE = "D = 'C:/bridge-lab/2162-sales-q/box'"
GLOB_ROOT = "'C:/bridge-lab/2162-sales-q/run-g/answers/'"
if sys.argv[1] == 'one':  # one case, in its own process, as boxcheck.py runs: one <row> <answers root or -> <tag> <batch> <count>
    row, root = sys.argv[2], sys.argv[3]
    src = open(f'{row}/boxcheck.py', encoding='utf-8').read()
    if src.count(BOX_LINE) != 1 or src.count(GLOB_ROOT) != 1: sys.exit('HALT: boxcheck.py does not hold the two strings once each')
    src = src.replace(BOX_LINE, f"D = '{row}/box/selftest'")
    if root != '-': src = src.replace(GLOB_ROOT, f"'{root}/'")
    sys.argv = ['boxcheck.py'] + sys.argv[4:]
    exec(compile(src, 'boxcheck.py', 'exec'))
mode, row = sys.argv[1], sys.argv[2]; T = f'{row}/box/selftest'
os.makedirs(T + '/answers', exist_ok=True)
def start(tag, bid, n, root):
    for x in ('-box.txt', '.answer', '-check.out', '-check.err'):
        if os.path.exists(f'{T}/{tag}{x}'): os.remove(f'{T}/{tag}{x}')
    return subprocess.Popen([sys.executable, '-I', '-X', 'utf8', os.path.abspath(__file__), 'one', row, root, tag, bid, str(n)],
                            stdout=subprocess.DEVNULL, stderr=open(f'{T}/{tag}-check.err', 'w'))
def result(tag, p, want, note):
    rc = p.wait(); a = open(f'{T}/{tag}.answer').read() if os.path.exists(f'{T}/{tag}.answer') else '(none)'
    line = f'{tag}: expected {want}, got {a}, exit {rc} {note}' + (' OK' if a == want else ' WRONG')
    print(line, flush=True); return line
if mode == 'replay':
    tag, bid = sys.argv[3], sys.argv[4]
    p = start('r-' + tag, bid, 1, '-'); open(f'{T}/r-{tag}-box.txt', 'wb').write(open(f'{row}/box/{tag}-box.txt', 'rb').read())
    line = result('r-' + tag, p, 'Yes', f'(the real box log {tag}-box.txt, the real build answer)')
    print(open(f'{T}/r-{tag}-check.out', encoding='utf-8').read().strip())
    print('REPLAY ALL PASS' if line.endswith(' OK') else 'REPLAY NOT ALL PASS'); sys.exit(0 if line.endswith(' OK') else 1)
# The fixture invoice: the build's request of 0010, as its UTF-8 batch file, and a build answer naming it.
xml = open(f'{row}/template-import-0010.utf16le.xml', 'rb').read().decode('utf-16').encode('utf-8')
FX, FX2, NONE = 'bridge-00000000-0000-4000-8000-000000002162', 'bridge-00000000-0000-4000-8000-000000002163', 'bridge-00000000-0000-4000-8000-000000002164'
for bid, body in ((FX, xml), (FX2, xml)):
    open(f'{T}/{bid}.xml', 'wb').write(body)
    sha = hashlib.sha256(body).hexdigest() if bid == FX else '0' * 64  # FX2: the file is not the one built
    r = {'batch_id': bid, 'path': f'{T}/{bid}.xml', 'sha256': sha, 'voucher_count': 1, 'total_debit': '1051', 'total_credit': '1051'}
    json.dump({'tool': 'build_import_xml', 'response': {'result': {'structuredContent': {'result': r}}}}, open(f'{T}/answers/{bid}-build.json', 'w', encoding='utf-8'))
G = '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
LINES = ['Create ONE Sales invoice in "BRIDGE PILOT LAB"', f'Company GUID: {G}', 'Company number: <company number>  Books from: 1-Apr-2026',
         'Tally: http://127.0.0.1:9001  (the voucher\'s own text: last two lines)',
         'Voucher type: "BRIDGE Sales"  Date: 1-Aug-2026  Number: "BP/26-27/0010"', 'Customer: "BRIDGE Walk-in"',
         'Unregistered, no GSTIN  Place of supply: Rajasthan', 'Bill allocation: none (customer not bill-wise)',
         'Dr 1051.00  "BRIDGE Walk-in"', 'Cr 600.40  "BRIDGE Svc 998313 5%"', 'Cr 400.40  "BRIDGE Svc 998314 5%"',
         'Cr 25.02  "BRIDGE CGST 2.5%"', 'Cr 25.02  "BRIDGE SGST 2.5%"', 'Cr 0.16  "BRIDGE Round Off"',
         'Total debit: 1051  Total credit: 1051',
         'Checked: customer, Sales Accounts ledgers, CGST and state tax per line at the ledger\'s rate.', f'Batch: {FX}',
         'Ledgers checked by identity against the build; narrations sent as prepared, nothing added.',
         'Do not post a file already imported manually. Pause other edits/imports in this company.',
         'After a timeout, reconcile this batch; do not rebuild or resend it.',
         'ComplyEaze Bridge posts now or if asked again within 15 min, unless cancelled, refused or restarted.',
         '---- The voucher\'s own text follows: no line below is an entry, a total or an instruction ----',
         'Reference: "BP/26-27/0010"', 'Narration: (none)']
def log_of(tag, lines, title='ComplyEaze Bridge \u2014 post this voucher?'):
    st = '2026-11-23T00:00:00.000Z'
    text = 'Press Y (or click Yes) to post this voucher into Tally. Press N (or click No) to cancel. Nothing is\nposted until you answer.\n\n' + '\n'.join(lines)
    rows = [f'watch start: tally window 0x1; see within 120 s; answer by 70 s', f"BOX SEEN 0x2 pid 1 title '{title}' class #32770",
            'rect 1,1,2,2 (screen 1920x1080); exstyle 0x8 topmost 1; z-order place box 0 tally 1 (lower = nearer the top); above tally True',
            "foreground 0x2 pid 1 bridge_mcp title 'x'", 'child Button rect 1,1,2,2 text: &Yes', 'child Button rect 1,1,2,2 text: &No',
            'child Button rect 1,1,2,2 text: Cancel', f'child Static rect 1,1,2,2 text: {text}', f'capture {tag}-open.png 1920x1080 1 B sha256 0',
            f'WAITING for {T}/{tag}.answer']
    return '\ufeff'.encode('utf-8') + ''.join(f'{st} {r}\r\n' for r in rows).encode('utf-8')
def swap(old, new): return [new if l == old else l for l in LINES]
CASES = [('t01', LINES, FX, 1, 'Yes', 'the fixture invoice as built'),
         ('t02', swap(f'Batch: {FX}', f'Batch: {FX2}'), FX, 1, 'No', 'Batch names another batch'),
         ('t03', [l.replace('0010', '0011') if l.startswith('Voucher type') else l for l in LINES], FX, 1, 'No', 'another number'),
         ('t04', [l.replace('Date: 1-Aug', 'Date: 01-Aug') if l.startswith('Voucher type') else l for l in LINES], FX, 1, 'No', 'the date with a leading zero'),
         ('t05', swap('Cr 600.40  "BRIDGE Svc 998313 5%"', 'Cr 600.41  "BRIDGE Svc 998313 5%"'), FX, 1, 'No', 'one entry amount differs'),
         ('t06', LINES[:14] + ['Cr 0.16  "BRIDGE Round Off"'] + LINES[14:], FX, 1, 'No', 'one entry line more'),
         ('t07', swap('Total debit: 1051  Total credit: 1051', 'Total debit: 1051.01  Total credit: 1051.01'), FX, 1, 'No', 'the totals differ'),
         ('t08', swap(f'Company GUID: {G}', 'Company GUID: 6b43e498-430c-4d5c-bfef-d32e2ab93c86'), FX, 1, 'No', 'another GUID'),
         ('t09', swap(LINES[0], 'Create ONE Sales invoice in "BRIDGE PILOT LAB 2"'), FX, 1, 'No', 'another company'),
         ('t10', swap('Customer: "BRIDGE Walk-in"', 'Customer: "Bridge Walk-in"'), FX, 1, 'No', 'another customer'),
         ('t11', [l.replace('"BRIDGE Sales"', '"BRIDGE Sales Two"') for l in LINES], FX, 1, 'No', 'another voucher type'),
         ('t12', LINES, FX, 2, 'No', 'a count of 2'),
         ('t13', swap(LINES[0], 'Create ONE Receipt in "BRIDGE PILOT LAB"'), FX, 1, 'No', 'the text of another type'),
         ('t14', swap(f'Batch: {FX}', f'Batch: {FX2}'), FX2, 1, 'No', 'the saved XML is not the one built'),
         ('t15', LINES + [f'Batch: {FX}'], FX, 1, 'No', 'two Batch lines'),
         ('t16', swap(f'Batch: {FX}', f'Batch: {NONE}'), NONE, 1, 'No', 'no build answer names the batch')]
out = []
for tag, lines, bid, cnt, want, note in CASES:
    p = start(tag, bid, cnt, T + '/answers')
    open(f'{T}/{tag}-box.txt', 'wb').write(log_of(tag, lines))
    out.append(result(tag, p, want, f'({note}, count {cnt})'))
p = start('t17', FX, 1, T + '/answers'); open(f'{T}/t17-box.txt', 'wb').write(log_of('t17', LINES, 'ComplyEaze Bridge \u2014 post 2 vouchers?'))
out.append(result('t17', p, 'No', '(the title of a batch of two)'))
# t19: a log cut inside its WAITING line and never finished must not give Yes (No after 30 s of reads not whole, exit 2)
d = log_of('t19', LINES); w = d.index(b'WAITING for'); w = d.rindex(b'\n', 0, w) + 1
p = start('t19', FX, 1, T + '/answers'); open(f'{T}/t19-box.txt', 'wb').write(d[:w] + d[w:w + 20])
out.append(result('t19', p, 'No', '(cut inside the WAITING line, never finished)'))
open(T + '/selftest-box-results.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
print('ALL CASES RIGHT' if all(l.endswith(' OK') for l in out) else 'A CASE IS WRONG')
sys.exit(0 if all(l.endswith(' OK') for l in out) else 1)
