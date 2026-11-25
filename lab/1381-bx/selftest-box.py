# Lab helper (row 1381-BX, in the shape of row 2162-PA's selftest-box.py at lab/2162-pa-sheet-draft f682a6aa; not product
# code): the self-test of this row's boxcheck.py. It runs boxcheck.py as written but for one line: its row folder R becomes
# <scratch>, so its box folder is <scratch>/local/box and its input folder <scratch>/local/run. <scratch> is named on the
# command line: at the sitting <row>/local/selftest, elsewhere a folder outside the repository (1381-BX: a scratch inside the
# sheet folder but outside its local/ is refused, so a run never adds a file the sheet would carry). Nothing is written
# outside <scratch>; the row's own box logs and inputs are only read.
#   cases <row> <scratch>             before any box of the sitting (step 0), and on any machine. No box of this build was
#                                     ever logged by box.ps1, so the logs are WRITTEN here in box.ps1's log form, holding the
#                                     row's own two inputs (in/stock-input.txt and in/ledger-input.txt, a fresh nonce on the
#                                     first line as arm.py writes it). One right box gives the arm's answer; each change
#                                     alone gives no answer file. 26 cases; any other count or any case wrong prints A CASE
#                                     IS WRONG and exits 1 (the row stops). They are no evidence of what a real box holds.
#   replay <row> <scratch> <tag> <mode> <ans>   after an arm: the check once more on that arm's real box log and input. EXPECT <ans>.
#                                     Its answer file is written under <scratch>, where no box.ps1 reads it.
# Usage: python -I -X utf8 selftest-box.py cases C:/bridge-lab/1381-bx C:/bridge-lab/1381-bx/local/selftest   (forward slashes)
import os, shutil, subprocess, sys, uuid
R_LINE = "R = os.path.dirname(os.path.abspath(__file__)).replace('\\\\', '/')  # the row folder: C:/bridge-lab/1381-bx at the sitting"
if sys.argv[1] == 'one':  # one case, in its own process, as boxcheck.py runs: one <row> <scratch> <tag> <mode> <answer>
    row, scratch = sys.argv[2], sys.argv[3]
    src = open(f'{row}/boxcheck.py', encoding='utf-8').read()
    if src.count(R_LINE) != 1: sys.exit('HALT: boxcheck.py does not hold its R line once')
    src = src.replace(R_LINE, f"R = '{scratch}'")
    sys.argv = ['boxcheck.py'] + sys.argv[4:]
    exec(compile(src, 'boxcheck.py', 'exec'))
if len(sys.argv) < 4 or sys.argv[1] not in ('cases', 'replay'):
    sys.exit('usage: selftest-box.py cases <row> <scratch> | replay <row> <scratch> <tag> <mode> <ans>')
mode, row, scratch = sys.argv[1], sys.argv[2].rstrip('/'), sys.argv[3].rstrip('/')
_r, _s = os.path.abspath(row).replace('\\', '/'), os.path.abspath(scratch).replace('\\', '/')
if (_s + '/').startswith(_r + '/') and not _s.startswith(_r + '/local/'):  # 1381-BX
    sys.exit(f'HALT: the scratch {scratch} is inside the sheet folder but not under its local/: nothing written')
T = f'{scratch}/local'; B, RUN = f'{T}/box', f'{T}/run'
os.makedirs(B, exist_ok=True); os.makedirs(RUN, exist_ok=True)
def start(tag, kind, ans):
    for x in ('-box.txt', '.answer', '.answer.tmp', '-check.out', '-check.err'):
        if os.path.exists(f'{B}/{tag}{x}'): os.remove(f'{B}/{tag}{x}')
    return subprocess.Popen([sys.executable, '-I', '-X', 'utf8', os.path.abspath(__file__), 'one', row, scratch, tag, kind, ans],
                            stdout=subprocess.DEVNULL, stderr=open(f'{B}/{tag}-check.err', 'w'))
def result(tag, p, want, note):
    rc = p.wait(); a = open(f'{B}/{tag}.answer').read() if os.path.exists(f'{B}/{tag}.answer') else '(none)'
    line = f'{tag}: expected {want}, got {a}, exit {rc} {note}' + (' OK' if a == want else ' WRONG')
    print(line, flush=True); return line
if mode == 'replay':
    tag, kind, ans = sys.argv[4], sys.argv[5], sys.argv[6]
    shutil.copyfile(f'{row}/local/run/{tag}-in.txt', f'{RUN}/r-{tag}-in.txt')
    p = start('r-' + tag, kind, ans); open(f'{B}/r-{tag}-box.txt', 'wb').write(open(f'{row}/local/box/{tag}-box.txt', 'rb').read())
    line = result('r-' + tag, p, ans, f'(the real box log {tag}-box.txt and the arm\'s real input)')
    print(open(f'{B}/r-{tag}-check.out', encoding='utf-8').read().strip())
    print('REPLAY ALL PASS' if line.endswith(' OK') else 'REPLAY NOT ALL PASS'); sys.exit(0 if line.endswith(' OK') else 1)
TITLE = {'stock': 'ComplyEaze Bridge \u2014 create one stock item?', 'masters': 'ComplyEaze Bridge \u2014 create one ledger?'}
def fed(kind):
    raw = open(f'{row}/in/{"stock" if kind == "stock" else "ledger"}-input.txt', 'rb').read()
    return f'{uuid.uuid4()}\n'.encode() + raw.split(b'\n', 1)[1]
S = '2026-11-26T10:00:0{}.000Z '
BUTTONS = [('&Yes', (840, 900, 928, 926)), ('&No', (938, 900, 1026, 926)), ('Cancel', (1036, 900, 1124, 926))]
def log(kind, title=None, cls='#32770', buttons=BUTTONS, lines=None, text_rect=(760, 220, 1230, 880), box=(688, 162, 1252, 945),
        crlf=False, extra_text=False, nobox=False):
    # box.ps1's log form (box.ps1:76-83): BOX SEEN, rect, foreground (title as length and sha256), one line per child, capture, WAITING; Add-Content
    # writes CRLF after each logged line and the child's own text keeps the line breaks it holds.
    if nobox:
        return (S.format(0) + 'watch start\r\n' + S.format(9) + 'NO BOX seen within 30 s\r\n').encode('utf-8-sig')
    text = lines if lines is not None else fed_text[kind]
    body = '\r\n'.join(text) if crlf else '\n'.join(text)
    out = [S.format(0) + 'watch start: tally window 0x0; see within 30 s; answer by 90 s',
           S.format(1) + f"BOX SEEN 0x1a2b pid 4242 title '{title if title is not None else TITLE[kind]}' class {cls}",
           S.format(1) + f'rect {box[0]},{box[1]},{box[2]},{box[3]} (screen 1920x1080); exstyle 0x10108 topmost 1; z-order place box 0 tally -1 (lower = nearer the top); above tally False',
           S.format(1) + 'foreground 0x1a2b pid 4242 bridge_mcp title 1 chars sha256 ' + '2d711642b726b04401627ca9fbac32f5c8530fb1903cc4db02258717921a4881']
    out += [S.format(1) + f'child Button rect {r[0]},{r[1]},{r[2]},{r[3]} text: {b}' for b, r in buttons]
    out.append(S.format(1) + 'child Static rect 700,220,732,252 text: ')
    out.append(S.format(1) + f'child Static rect {text_rect[0]},{text_rect[1]},{text_rect[2]},{text_rect[3]} text: {body}')
    if extra_text:
        out.append(S.format(1) + f'child Static rect 760,880,1230,890 text: {body}')
    out += [S.format(2) + 'capture x-open.png 1920x1080 1 B sha256 0', S.format(2) + 'WAITING for answer']
    return ('\r\n'.join(out) + '\r\n').encode('utf-8-sig')
inputs = {k: fed(k) for k in ('stock', 'masters')}
fed_text = {k: v.decode('utf-8').split('\n', 2)[2].split('\n') for k, v in inputs.items()}
st = fed_text['stock']
CASES = [  # (tag, mode, answer, input bytes, log kwargs, expected answer file, note)
    ('s-yes', 'stock', 'Yes', None, {}, 'Yes', 'the right stock box, answer Yes'),
    ('s-no', 'stock', 'No', None, {}, 'No', 'the right stock box, answer No'),
    ('s-cancel', 'stock', 'Cancel', None, {}, 'Cancel', 'the right stock box, answer Cancel'),
    ('l-no', 'masters', 'No', None, {}, 'No', 'the right ledger box, answer No'),
    ('s-crlf', 'stock', 'Yes', None, {'crlf': True}, 'Yes', 'the right stock text read back with CR LF breaks'),
    ('s-ledger-title', 'stock', 'Yes', None, {'title': TITLE['masters']}, '(none)', 'the ledger title on the stock box'),
    ('l-stock-title', 'masters', 'No', None, {'title': TITLE['stock']}, '(none)', 'the stock title on the ledger box'),
    ('s-title-hyphen', 'stock', 'Yes', None, {'title': 'ComplyEaze Bridge - create one stock item?'}, '(none)', 'a hyphen for the dash'),
    ('s-class', 'stock', 'Yes', None, {'cls': 'Static'}, '(none)', 'a window that is not a dialog'),
    ('s-no-cancel', 'stock', 'Yes', None, {'buttons': BUTTONS[:2]}, '(none)', 'two buttons only'),
    ('s-four', 'stock', 'Yes', None, {'buttons': BUTTONS + [('Help', (600, 900, 680, 926))]}, '(none)', 'a fourth button'),
    ('s-ok', 'stock', 'Yes', None, {'buttons': [('OK', BUTTONS[0][1])] + BUTTONS[1:]}, '(none)', 'OK in place of Yes'),
    ('s-rate', 'stock', 'Yes', None, {'lines': [l.replace('rate: "18"', 'rate: "28"') for l in st]}, '(none)', 'one rate changed'),
    ('s-hsn', 'stock', 'Yes', None, {'lines': [l.replace('39239090', '39239091') for l in st]}, '(none)', 'the HSN code changed'),
    ('s-last-missing', 'stock', 'Yes', None, {'lines': st[:-1]}, '(none)', 'the last line missing (a cut box)'),
    ('s-first-missing', 'stock', 'Yes', None, {'lines': st[1:]}, '(none)', 'the first answer line missing'),
    ('s-blank-missing', 'stock', 'Yes', None, {'lines': st[:2] + st[3:]}, '(none)', 'the blank line under the answer missing'),
    ('s-added', 'stock', 'Yes', None, {'lines': st + ['Lab line']}, '(none)', 'a line added at the end'),
    ('s-doubled', 'stock', 'Yes', None, {'lines': st[:10] + st[9:]}, '(none)', 'a line doubled'),
    ('s-swapped', 'stock', 'Yes', None, {'lines': st[:16] + [st[17], st[16]] + st[18:]}, '(none)', 'two rate rows swapped'),
    ('s-two-texts', 'stock', 'Yes', None, {'extra_text': True}, '(none)', 'two text children'),
    ('s-text-outside', 'stock', 'Yes', None, {'text_rect': (760, 220, 1300, 880)}, '(none)', 'the text child past the box edge'),
    ('s-button-over', 'stock', 'Yes', None, {'buttons': [(b, (r[0], 860, r[2], 886)) for b, r in BUTTONS]}, '(none)', 'buttons over the text'),
    ('s-nobox', 'stock', 'Yes', None, {'nobox': True}, '(none)', 'no box seen'),
    ('s-bad-nonce', 'stock', 'Yes', b'not-a-nonce\n' + inputs['stock'].split(b'\n', 1)[1], {}, '(none)', 'an input whose nonce is not a UUID'),
    ('s-stock-on-ledger', 'masters', 'No', inputs['stock'], {'title': TITLE['masters']}, '(none)', 'the ledger box showing the stock text'),
]
results = []
for tag, kind, ans, data, kw, want, note in CASES:
    open(f'{RUN}/{tag}-in.txt', 'wb').write(data if data is not None else inputs[kind])
    p = start(tag, kind, ans)
    open(f'{B}/{tag}-box.txt', 'wb').write(log(kind, **kw))
    results.append(result(tag, p, want, f'({note})'))
right = sum(r.endswith(' OK') for r in results)
print(f'{len(results)} cases ran, {right} right')
ok = len(results) == 26 and right == 26
print('ALL CASES RIGHT' if ok else 'A CASE IS WRONG'); sys.exit(0 if ok else 1)
