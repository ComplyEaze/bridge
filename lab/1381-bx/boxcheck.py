# Lab helper (row 1381-BX, in the shape of row 2162-PA's boxcheck.py at lab/2162-pa-sheet-draft f682a6aa; not product code):
# the whole-text check of one dialog child's box. Waits for box.ps1's log to reach its WAITING line, then checks the box
# against what this arm fed the child: the title its mode shows, its class, its three buttons, and its text, line by
# line and in order, equal to the arm's input after its nonce and count lines. Only when every check passes does it write
# the arm's answer (Yes, No or Cancel) to box/<tag>.answer, written whole under another name and renamed in, so box.ps1
# never reads it empty. Otherwise it writes NO answer file and exits 1: nothing is pressed (box.ps1 presses nothing when
# no valid answer file comes by its -AnswerBy, in this row's copy). Its checks go to box/<tag>-check.out.
# Usage: python -I -X utf8 boxcheck.py <tag> <stock|masters> <Yes|No|Cancel>
# The text checked (code at 48a64965): the child shows dialog_input's preview (src-tauri/src/tally/approved_import.rs:3273)
# with MessageBoxW (show_create_stock_item :3884, show_create_ledger :3820), the preview being what the parent wrote after
# the nonce and count lines (nonce_bound_dialog :3127). The title is stock_item_words (:99-104), STOCK_ITEM_TITLE (:91), or
# masters_words (:76-85), MASTERS_TITLE (:60). The buttons are MB_YESNOCANCEL; No the default (MB_DEFBUTTON2), read by the
# guard in arm.py from boxstate.ps1's FOCUS line, not here.
# Kept from 2162-PA as they were: the log is read as bytes and taken only when it decodes whole as UTF-8, ends with a line
# break, and its WAITING line is whole; a refused or cut read is read again, for at most 30 s of such reads in a row, after
# which it writes no answer. Any error of this helper writes no answer (fail closed), with the error on stderr.
import os, re, sys, time, traceback
from datetime import datetime, timezone
R = os.path.dirname(os.path.abspath(__file__)).replace('\\', '/')  # the row folder: C:/bridge-lab/1381-bx at the sitting
D = R + '/local/box'; RUN = R + '/local/run'
TITLES = {'stock': 'ComplyEaze Bridge \u2014 create one stock item?', 'masters': 'ComplyEaze Bridge \u2014 create one ledger?'}
ANSWERS = ('Yes', 'No', 'Cancel')
UUID = re.compile(r'[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}')
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
def answer(tag, ans, out):
    # The arm's answer is written only when every check passed; any other outcome leaves no answer file.
    if ans in ANSWERS:
        with open(f'{D}/{tag}.answer.tmp', 'w', encoding='utf-8') as f:
            f.write(ans)
        os.replace(f'{D}/{tag}.answer.tmp', f'{D}/{tag}.answer')
        out.append(f'{now()} answer file written: {ans}')
    else:
        out.append(f'{now()} no answer file written: nothing is pressed, the box is left up (STOP-CHECK)')
    open(f'{D}/{tag}-check.out', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
    print('\n'.join(out))
def read_whole(log):
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
STAMP = re.compile(r'^\d{4}-\d\d-\d\dT\S+Z ')
CHILD = re.compile(r'^\S+Z child (\S+) rect (-?\d+),(-?\d+),(-?\d+),(-?\d+) text: (.*)$')
def children(t):
    # Every child box.ps1 logged: (class, rect, text lines, CR count). A text runs on over the unstamped lines up to the
    # next stamped line; a child whose text is not ended by a stamped line (a cut log) is returned with text None. A CR at
    # the end of a line is dropped and counted, except the last, which is Add-Content's own line end.
    lines, out = t.split('\n'), []
    for i, l in enumerate(lines):
        m = CHILD.match(l.rstrip('\r'))
        if not m: continue
        raw = [l]
        for nxt in lines[i + 1:]:
            if STAMP.match(nxt): break
            raw.append(nxt)
        else: raw = None
        text = None if raw is None else [m.group(6)] + [x.rstrip('\r') for x in raw[1:]]
        cr = 0 if raw is None else sum(x.endswith('\r') for x in raw[:-1])
        out.append((m.group(1), tuple(int(m.group(k)) for k in range(2, 6)), text, cr))
    return out
def inside(a, b): return b[0] <= a[0] < a[2] <= b[2] and b[1] <= a[1] < a[3] <= b[3]
try:
    if len(sys.argv) != 4 or sys.argv[2] not in TITLES or sys.argv[3] not in ANSWERS:
        raise ValueError('usage: boxcheck.py <tag> <stock|masters> <Yes|No|Cancel>')
    tag, mode, ans = sys.argv[1:]
    t, reads = read_whole(f'{D}/{tag}-box.txt')
    raw = open(f'{RUN}/{tag}-in.txt', 'rb').read().decode('utf-8', 'strict')
    nonce, count, text = (raw.split('\n', 2) + ['', '', ''])[:3]
    want = text.split('\n')
    seen = re.search(r"^\S+Z BOX SEEN 0x[0-9a-f]+ pid \d+ title '(.*)' class (\S+)\r?$", t, re.M)
    rect = re.search(r'^\S+Z rect (-?\d+),(-?\d+),(-?\d+),(-?\d+) \(screen ', t, re.M)
    box = tuple(int(rect.group(k)) for k in range(1, 5)) if rect else None
    kids = children(t)
    buttons = [(r, ''.join(x.replace('&', '') for x in (txt or ['?']))) for c, r, txt, _ in kids if c == 'Button']
    texts = [(r, txt, cr) for c, r, txt, cr in kids if c == 'Static' and txt and txt[0].startswith('Press Y ')]
    shown = texts[0][1] if len(texts) == 1 else None
    checks = [('box seen', seen is not None),
              (f'title exactly {TITLES[mode]!r}', seen is not None and seen.group(1) == TITLES[mode]),
              ('class #32770', seen is not None and seen.group(2) == '#32770'),
              ('the input: a nonce line, the count 1, a text', UUID.fullmatch(nonce) is not None and count == '1' and text != ''),
              (f'exactly three buttons, Yes, No and Cancel (found {[b for _, b in buttons]})',
               sorted(b for _, b in buttons) == sorted(ANSWERS) and len(buttons) == 3),
              (f'exactly one text child, beginning with the answer text (found {len(texts)})', len(texts) == 1),
              (f'the text is the input line for line: {len(shown) if shown else 0} lines read, {len(want)} fed', shown == want),
              ('the box rectangle was logged', box is not None),
              ('the text child lies inside the box', box is not None and len(texts) == 1 and inside(texts[0][0], box)),
              ('every button lies inside the box, below the text', box is not None and len(texts) == 1 and len(buttons) == 3
               and all(inside(r, box) and r[1] >= texts[0][0][3] for r, _ in buttons))]
    if shown is not None and shown != want:
        diff = next((i for i, (a, b) in enumerate(zip(shown, want)) if a != b), min(len(shown), len(want)))
        checks.append((f'(first differing line: {diff + 1})', False))
    ok = all(x for _, x in checks)
    out = [f'{now()} check of {tag}, mode {mode}, answer {ans} (log read whole after {reads} read(s); '
           f'{len(text.encode("utf-8"))} bytes of text fed; line breaks read as CR LF: {texts[0][2] if len(texts) == 1 else "-"})']
    out += [('PASS ' if x else 'FAIL ') + k for k, x in checks]
    out.append('ALL PASS' if ok else 'NOT ALL PASS')
    answer(tag, ans if ok else None, out)
    sys.exit(0 if ok else 1)
except SystemExit: raise
except BaseException as e:
    traceback.print_exc()
    try: answer(sys.argv[1] if len(sys.argv) > 1 else 'unknown', None, [f'{now()} HELPER ERROR {type(e).__name__}: {e}', 'NOT ALL PASS (fail closed)'])
    finally: sys.exit(2)
