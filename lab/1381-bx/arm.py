# Lab helper (row 1381-BX, in the shape of row 2162-PA's pa.py at lab/2162-pa-sheet-draft f682a6aa; not product code): one
# arm of the row, under one start line, so that nothing is typed while its box is up. No server runs and Tally is not
# called: the box comes from the dialog child alone (src-tauri/src/lib.rs:112-126 at 48a64965).
#   0. Hashes its helpers (HELPERS: box.ps1, startc.ps1, boxstate.ps1, screen.ps1, boxcheck.py, build.py) against
#      SHA256.txt before loading build.py: any other bytes, STOP-HELPER-HASH, nothing launched. sys.dont_write_bytecode
#      is set first, so loading build.py writes no __pycache__ into the row folder (-I alone does not stop it).
#   1. Reads BUILD.txt (build.py's parse; STOP while a field is <TO FILL>) and hashes the exe on GRANTED PATH against
#      EXE SHA256; hashes the arm's committed input against SHA256.txt; refuses if any name it writes exists or if a
#      bridge_mcp.exe is already running. Writes local/run/<tag>-in.txt: the committed input with a fresh nonce
#      (uuid4) on its first line and every byte after that line unchanged (checked), and its sha256 beside it.
#   2. screen.ps1 into local/run/<tag>-screen.txt: the Windows release and every monitor's rectangle, work area and DPI.
#      Exactly one monitor is admitted; two or more: STOP-MONITORS before any launch.
#   3. Starts box.ps1 (this row's copy: Cancel is an answer, and no valid answer by -AnswerBy presses nothing), then, 2 s
#      later, startc.ps1 with the exe, the arm's mode, show 1 and the arm's input, the child's standard output and error
#      to local/run/<tag>-out.bin and local/run/<tag>-err.txt.
#   4. A guard, in its own thread: at box.ps1's WAITING line it requires the box's pid to be the child's (startc's
#      STARTED line), reads the box with boxstate.ps1 (byte-equal to row 2569-S's) into local/box/<tag>-R1.txt, and passes
#      only when that read exits 0 and holds exactly one box line (name starting "[ComplyEaze Bridge", class #32770) with
#      iconic=False, visible=True and topmost=True and a rect= equal to box.ps1's rectangle of the box (boxstate.ps1
#      given 30 s: past that the guard fails and it is left running, never ended), that rectangle lies inside the work area,
#      the arm's button lies inside the box, and box.ps1 logged the box at z-order place 0. Only then does it start
#      boxcheck.py, which writes the arm's answer when the box's whole text, title and buttons are what was fed. The
#      guard's lines go to local/box/<tag>-guard.txt. The foreground and FOCUS lines are graded after, not guarded.
#   5. After box.ps1 has clicked, waits up to 60 s for startc.ps1's EXITED line, then grades the child's standard output,
#      exit code and error output from the files (grade(), below). box.ps1 exit 3, 4 or 5 (no click, no box left up): it
#      waits up to 10 s for EXITED and reports the child's exit and every line of its error output before the STOP, so a
#      box never shown names its cause. box.ps1 exit 6: the box is left up, the child running, and nothing is waited on.
#      A 'box-fit: restored from minimised' line is admitted in the error output of every arm and recorded as W7.
#   ST, the too-large arm, has no answer: no guard, no check, no answer file is ever written for it, so box.ps1 never
#      clicks. The child is expected to cancel its own box (box_fit.rs) and exit 77; box.ps1 then ends with exit 3 (no
#      box seen) or 4 (the box gone before any click), after which arm.py waits for EXITED and grades with grade_st().
#      box.ps1 exit 6 (a box still up at 90 s) is STOP-UNANSWERED, the box left up.
# Usage: python -I -X utf8 arm.py <LN|SN|SC|SY|ST>   python -I -X utf8 arm.py --selftest   (the grading only; any OS)
# Exit 0: every deciding line as the code says (the W lines are recorded, not deciding); 1: anything else that was read (a named STOP or a FAIL to record);
# 2: a failed read of this helper's own inputs (BUILD.txt, the exe, an input, a name already used).
import sys
sys.dont_write_bytecode = True  # 1381-BX: before build.py is loaded; -I alone still writes __pycache__ into the row folder
import hashlib, importlib.util, os, re, subprocess, threading, time, uuid
R = os.path.dirname(os.path.abspath(__file__)).replace('\\', '/')  # the row folder: C:/bridge-lab/1381-bx at the sitting
L = R + '/local'; BOX, RUN, EV = L + '/box', L + '/run', L + '/evidence'
PRIV = L + '/private'  # box.ps1's foreground titles: kept on the lab PC, never on the evidence branch (B8)
ARMS = {'LN': ('masters', 'No', 'in/ledger-input.txt'), 'SN': ('stock', 'No', 'in/stock-input.txt'),
        'SC': ('stock', 'Cancel', 'in/stock-input.txt'), 'SY': ('stock', 'Yes', 'in/stock-input.txt'),
        'ST': ('stock', None, 'in/stock-large-input.txt')}
PREFIX = {'stock': 'bridge-create-stock-item:', 'masters': 'bridge-create-ledger:'}  # approved_import.rs:88 and :57
FIT = re.compile(r'box-fit: box=\((-?\d+),(-?\d+),(-?\d+),(-?\d+)\) work=\((-?\d+),(-?\d+),(-?\d+),(-?\d+)\) fits=(true|false)')
NOTREAD = 'box-fit: not readable fits=false'  # box_fit.rs:549: an unmeasured box counts as not fitting
RESTORED = 'box-fit: restored from minimised'  # box_fit.rs:570: the hook restored a box shown minimised, then measured it
HELPERS = ('box.ps1', 'startc.ps1', 'boxstate.ps1', 'screen.ps1', 'boxcheck.py', 'build.py')  # hashed before every arm
BOXSTATE_WITHIN = 30  # seconds for boxstate.ps1; past that the guard fails and boxstate.ps1 is left, not ended
NOWIN = getattr(subprocess, 'CREATE_NO_WINDOW', 0)


def utc(): return time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())


def grade(mode, ans, nonce, out, code, err, blog):
    """The deciding lines of one arm, from the files. out: the child's standard output bytes; code: its exit code (None
    when startc printed no EXITED line); err: its error output text; blog: box.ps1's log text. Code at 48a64965:
    answer_with_token (approved_import.rs:3243) writes dialog_token (:3154), prefix + nonce + LF, only when the dialog
    returns true (Yes: show_create_stock_item :3884-3921, show_create_ledger :3820-3858); the child exits 0 then, else 1
    (lib.rs:125). A box that does not fit exits 77 (box_fit.rs:38); one that could not be measured or shown exits 0 with
    no token. box_fit.rs:533-551 writes one box-fit line to the error output at each activation of the box."""
    want = (PREFIX[mode] + nonce + '\n').encode() if ans == 'Yes' else b''
    lines = err.splitlines()
    fits = [m for m in (FIT.fullmatch(l) for l in lines) if m]
    known = [l == RESTORED or FIT.fullmatch(l) is not None for l in lines]  # RESTORED is recorded as W7, not deciding
    clicked = re.findall(r'^\S+Z CLICKED (\S+) at ', blog, re.M)
    return [(f'box.ps1 clicked {ans}, once (clicked {clicked})', clicked == [ans]),
            ('box.ps1 read the box as gone after the click', re.search(r'^\S+Z after click: box visible False;', blog, re.M) is not None),
            (f'exit code {0 if ans == "Yes" else 1} (read {code})', code == (0 if ans == 'Yes' else 1)),
            (f'standard output {"exactly the token for this nonce and a line feed" if want else "empty"} ({len(out)} bytes read)', out == want),
            (f'error output: only box-fit lines, at least one measurement ({len(lines)} line(s), {len(fits)} measurement(s))',
             bool(fits) and all(known)),
            ('every box-fit measurement says fits=true', bool(fits) and all(m.group(9) == 'true' for m in fits))]


def grade_st(out, code, err, blog):
    """The deciding lines of ST. Code at 48a64965: the fit hook measures the box at its activation; a box not wholly inside
    the work area is sent a Cancel (box_fit.rs:572-580) and remembered as not fitting; guarded() then exits 77 whatever
    the box's result (:105-112, :158-161), before answer_with_token can write a token. An unmeasured box writes
    NOTREAD and counts as not fitting. A box never measured, or not shown, exits 0 with nothing printed (unavailable)."""
    lines = err.splitlines()
    ok_line = [l in (NOTREAD, RESTORED) or FIT.fullmatch(l) is not None for l in lines]
    unfit = [l == NOTREAD or (FIT.fullmatch(l) is not None and FIT.fullmatch(l).group(9) == 'false') for l in lines]
    clicked = re.findall(r'^\S+Z CLICKED (\S+) at ', blog, re.M)
    return [(f'box.ps1 clicked nothing (clicked {clicked})', clicked == []),
            (f'exit code 77, the fit guard\'s (read {code})', code == 77),
            (f'standard output empty ({len(out)} bytes read)', out == b''),
            (f'error output: only box-fit lines ({len(lines)} line(s))', bool(lines) and all(ok_line)),
            ('at least one box-fit line says fits=false', any(unfit))]


def w7(err):
    """Recorded, not deciding: whether the box opened minimised, so that the child's hook restored it before measuring
    (box_fit.rs:570 writes the line RESTORED)."""
    n = err.splitlines().count(RESTORED)
    return [(f'W7 the box not opened minimised (no {RESTORED!r} line; read {n}: with one, the child\'s hook restored it)', n == 0)]


def guard_checks(pid, startc_pid, rc, read, rect, work, br, place, ans):
    """The guard's lines, from what it read. rc: boxstate.ps1's exit, None when it had not ended in BOXSTATE_WITHIN s.
    read: its R1 file. rect: box.ps1's logged rectangle of the box; br: the answer's button rectangle; place: the box's
    z-order place in box.ps1's log. boxstate.ps1 and box.ps1 both read the box's GetWindowRect in physical pixels (at
    two sittings, lab 2569-R's boxstate.ps1 and lab 626-R3's box.ps1 logged the journal box as 688,162,1252,945), so
    R1's rect= must equal box.ps1's."""
    lines = [l for l in read.splitlines() if l.startswith('WINDOW ') and ' name=[ComplyEaze Bridge' in l and ' class=#32770 ' in l]
    state = lines[0] if len(lines) == 1 else ''
    m = re.search(r' rect=\((-?\d+),(-?\d+),(-?\d+),(-?\d+)\) ', state)
    r1 = tuple(int(x) for x in m.groups()) if m else None
    return [(f'the box is the child\'s (box pid {pid}, startc pid {startc_pid})', pid is not None and pid == startc_pid),
            (f'boxstate.ps1 ended within {BOXSTATE_WITHIN} s' + ('' if rc is not None else ': it is left running, not ended'),
             rc is not None),
            ('boxstate exit 0', rc == 0), (f'exactly one box line (read {len(lines)})', len(lines) == 1),
            ('iconic=False', ' iconic=False ' in state), ('visible=True', ' visible=True ' in state),
            ('topmost=True', ' topmost=True ' in state),
            (f'boxstate.ps1\'s rect {r1} equals box.ps1\'s {rect}', r1 is not None and r1 == rect),
            (f'the box rect {rect} inside the work area {work}', inside(rect, work)),
            (f'the {ans} button rect {br} inside the box', inside(br, rect)),
            ('the box nearest the top of the z-order (place 0), so the click lands on it', place == '0')]


def inside(a, w): return a is not None and w[0] <= a[0] < a[2] <= w[2] and w[1] <= a[1] < a[3] <= w[3]


def exited(startc_text):
    m = re.search(r'^EXITED code=(-?\d+) ', startc_text, re.M)
    return int(m.group(1)) if m else None


def no_click_report(rc, startc_text, err):
    """After box.ps1 exit 3, 4 or 5 with no click: the child's exit and its whole error output, so a box never shown
    names its cause (box_fit.rs:129-140: 'box-fit: not per-monitor DPI aware ...: no box shown', exit 0). err: the
    error file's text, or None when it could not be read."""
    code = exited(startc_text)
    out = [f'child exit {code}' if code is not None else
           'no EXITED line within 10 s: the child may still run; nothing ends it']
    if err is None:
        out.append('the child\'s error file could not be read')
    else:
        out += [f'--- child error output ({len(err.splitlines())} line(s))'] + err.splitlines()
    return out


def helper_misses(rows, digest):
    """The helpers whose bytes do not hash as SHA256.txt says. digest(name) gives a file's sha256 or None."""
    return [n for n in HELPERS if rows.get(n) is None or digest(n) != rows[n]]


def cross(err, work, rect):
    """Recorded, not deciding: the child's own box-fit numbers against screen.ps1's work area and box.ps1's rectangle
    (all physical pixels). The child measures at activation (box_fit.rs on_activate), box.ps1 after the box is seen."""
    fits = [m for m in (FIT.fullmatch(l) for l in err.splitlines()) if m]
    return [(f'W4 every box-fit work area equals screen.ps1\'s {work}',
             bool(fits) and all(tuple(int(m.group(k)) for k in range(5, 9)) == work for m in fits)),
            (f'W5 the first box-fit box equals box.ps1\'s rectangle {rect}',
             bool(fits) and tuple(int(fits[0].group(k)) for k in range(1, 5)) == rect)]


def selftest():
    n0, nx = str(uuid.UUID(int=0x1381)), str(uuid.UUID(int=0x1382))
    ok_log = ("2026-11-26T10:00:05.000Z CLICKED {a} at 900,913 (physical) x; seen-to-click 3.1 s\r\n"
              "2026-11-26T10:00:06.000Z after click: box visible False; foreground x\r\n")
    fit = 'box-fit: box=(688,162,1252,945) work=(0,0,1920,1030) fits=true\n'
    work, rect = (0, 0, 1920, 1030), (688, 162, 1252, 945)
    unfit = 'box-fit: box=(688,0,1252,2512) work=(0,0,1920,1030) fits=false\n'
    tok = ('bridge-create-stock-item:' + n0 + '\n').encode()
    lg = lambda a: ok_log.format(a=a)
    r1 = ('read at x pid 4242\nwindows 1\nWINDOW handle=1 name=[ComplyEaze Bridge — create one stock item?] class=#32770 '
          'iconic=False visible=True topmost=True rect=(688,162,1252,945) visual_state=Normal foreground=True\n')
    btn = (1000, 900, 1100, 930)
    dpi = 'box-fit: not per-monitor DPI aware (Windows 10 version 1703 or later is needed): no box shown'
    cases = [  # (name, function, args, every line right?)
        ('Yes, the token, exit 0', grade, ('stock', 'Yes', n0, tok, 0, fit, lg('Yes')), True),
        ('No, nothing printed, exit 1', grade, ('stock', 'No', n0, b'', 1, fit, lg('No')), True),
        ('Cancel, nothing printed, exit 1', grade, ('stock', 'Cancel', n0, b'', 1, fit, lg('Cancel')), True),
        ('the ledger control, No, two box-fit lines', grade, ('masters', 'No', n0, b'', 1, fit + fit, lg('No')), True),
        ('Yes, the token of another nonce', grade, ('stock', 'Yes', n0, ('bridge-create-stock-item:' + nx + '\n').encode(), 0, fit, lg('Yes')), False),
        ('Yes, the ledger token', grade, ('stock', 'Yes', n0, ('bridge-create-ledger:' + n0 + '\n').encode(), 0, fit, lg('Yes')), False),
        ('Yes, the token with CR LF', grade, ('stock', 'Yes', n0, tok[:-1] + b'\r\n', 0, fit, lg('Yes')), False),
        ('Yes, the token twice', grade, ('stock', 'Yes', n0, tok + tok, 0, fit, lg('Yes')), False),
        ('Yes, nothing printed, exit 0 (unavailable)', grade, ('stock', 'Yes', n0, b'', 0, fit, lg('Yes')), False),
        ('No, a token printed', grade, ('stock', 'No', n0, tok, 1, fit, lg('No')), False),
        ('No, exit 0 (unavailable)', grade, ('stock', 'No', n0, b'', 0, fit, lg('No')), False),
        ('exit 77, the box did not fit', grade, ('stock', 'No', n0, b'', 77, fit.replace('true', 'false'), lg('No')), False),
        ('no EXITED line', grade, ('stock', 'No', n0, b'', None, fit, lg('No')), False),
        ('no box-fit line (an input refused before any box)', grade, ('stock', 'No', n0, b'', 1, '', lg('No')), False),
        ('another line in the error output', grade, ('stock', 'No', n0, b'', 1, fit + 'thread main panicked\n', lg('No')), False),
        ('a box restored from minimised, then measured fitting', grade, ('stock', 'No', n0, b'', 1, RESTORED + '\n' + fit, lg('No')), True),
        ('the restored line alone, no measurement', grade, ('stock', 'No', n0, b'', 1, RESTORED + '\n', lg('No')), False),
        ('W7: no restored line', w7, (fit,), True),
        ('W7: the box was restored from minimised', w7, (RESTORED + '\n' + fit,), False),
        ('No clicked where Cancel was the arm', grade, ('stock', 'Cancel', n0, b'', 1, fit, lg('No')), False),
        ('no click logged', grade, ('stock', 'No', n0, b'', 1, fit, ''), False),
        ('cross: the same work area and rectangle', cross, (fit, work, rect), True),
        ('cross: a work area other than screen.ps1\'s', cross, (fit, (0, 0, 1920, 1040), rect), False),
        ('cross: a box rectangle other than box.ps1\'s', cross, (fit, work, (688, 160, 1252, 945)), False),
        ('cross: no box-fit line', cross, ('', work, rect), False),
        ('ST: refused by the fit guard, exit 77', grade_st, (b'', 77, unfit, ''), True),
        ('ST: refused, the box not readable', grade_st, (b'', 77, NOTREAD + '\n', ''), True),
        ('ST: seen not to fit, then measured again', grade_st, (b'', 77, unfit + fit, ''), True),
        ('ST: exit 0, nothing printed (unavailable)', grade_st, (b'', 0, '', ''), False),
        ('ST: exit 77 with every line fits=true', grade_st, (b'', 77, fit, ''), False),
        ('ST: a click logged', grade_st, (b'', 77, unfit, lg('No')), False),
        ('ST: a token printed', grade_st, (tok, 77, unfit, ''), False),
        ('ST: no box-fit line', grade_st, (b'', 77, '', ''), False),
        ('ST: exit 1', grade_st, (b'', 1, unfit, ''), False),
        ('ST: restored from minimised, then not fitting', grade_st, (b'', 77, RESTORED + '\n' + unfit, ''), True),
        ('guard: every line as read', guard_checks, ('4242', '4242', 0, r1, rect, work, btn, '0', 'No'), True),
        ('guard: boxstate.ps1\'s rect other than box.ps1\'s', guard_checks,
         ('4242', '4242', 0, r1.replace('rect=(688,162,', 'rect=(550,130,'), rect, work, btn, '0', 'No'), False),
        ('guard: boxstate.ps1 not ended within 30 s', guard_checks, ('4242', '4242', None, '', rect, work, btn, '0', 'No'), False),
        ('guard: the box another process\'s', guard_checks, ('4243', '4242', 0, r1, rect, work, btn, '0', 'No'), False),
        ('no click: the child names a box never shown', no_click_case,
         (3, 'STARTED pid=4242 x\nEXITED code=0 at x\n', dpi + '\n', ['child exit 0', dpi]), True),
        ('no click: no EXITED line, the error file unreadable', no_click_case,
         (3, 'STARTED pid=4242 x\n', None, ['no EXITED line within 10 s', 'could not be read']), True),
        ('helpers: each as SHA256.txt says', helper_case, ({n: 'a' * 64 for n in HELPERS}, {}), True),
        ('helpers: box.ps1 changed', helper_case, ({n: 'a' * 64 for n in HELPERS}, {'box.ps1': 'b' * 64}), False),
        ('loading build.py writes no __pycache__ into the row folder', bytecode_case, (), True),
        ('check_sheet.py names a __pycache__ found anyway', pycache_case, (), True),
    ]
    right = 0
    for name, fn, args, want in cases:
        g = fn(*args); got = all(ok for _, ok in g); right += got == want
        print(f'{name}: expected {"all right" if want else "a line wrong"}, got {"all right" if got else "a line wrong"} '
              f'{"OK" if got == want else "WRONG"}')
    print(f'{len(cases)} cases, {right} right'); return 0 if right == len(cases) == 45 else 1


def no_click_case(rc, startc_text, err, must):
    lines = no_click_report(rc, startc_text, err)
    return [(f'the report holds {m!r}', any(m in l for l in lines)) for m in must]


def helper_case(rows, changed):
    return [('no helper missed', helper_misses(rows, lambda n: changed.get(n, rows[n])) == [])]


def bytecode_case():
    load('build')
    return [('sys.dont_write_bytecode is set', sys.dont_write_bytecode is True),
            ('no __pycache__ in the row folder after loading build.py', not os.path.exists(R + '/__pycache__'))]


def pycache_case():
    cs = load('check_sheet')
    return [('a __pycache__ path is named', cs.pycache_hits(['arm.py', '__pycache__/build.cpython-310.pyc']) == ['__pycache__']),
            ('none named when there is none', cs.pycache_hits(['arm.py', 'in/stock-input.txt']) == [])]


def fail(why): print('FAILED READ:', why, flush=True); sys.exit(2)


def sha(path): return hashlib.sha256(open(path, 'rb').read()).hexdigest()


def load(name):
    """A row file as a module; sys.dont_write_bytecode (top of this file) keeps its __pycache__ out of the row folder."""
    spec = importlib.util.spec_from_file_location(name, f'{R}/{name}.py'); mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod); return mod


def manifest_rows():
    rows = {}
    for line in open(R + '/SHA256.txt', encoding='utf-8').read().splitlines():
        m = re.fullmatch(r'([0-9a-f]{64})  ([0-9]+)  (.+)', line)
        if m: rows[m.group(3)] = m.group(1)
    return rows


def main(tag):
    mode, ans, rel = ARMS[tag]
    try:
        rows = manifest_rows()
    except OSError as e:
        fail(f'SHA256.txt unreadable: {e}')
    def digest(n):
        try: return sha(f'{R}/{n}')
        except OSError: return None
    missed = helper_misses(rows, digest)
    if missed:
        print(f'STOP-HELPER-HASH: {", ".join(missed)} not as SHA256.txt says; nothing launched'); sys.exit(1)
    build = load('build')
    try:
        b, _ = build.load(R)
    except build.Fail as e:
        print(('STOP: ' if e.code == 1 else 'FAILED READ: ') + str(e)); sys.exit(e.code)
    exe = b['GRANTED PATH']
    if not os.path.isfile(exe) or sha(exe) != b['EXE SHA256']:
        print('STOP-EXE-HASH: the exe on the granted path is missing or does not hash to EXE SHA256; nothing launched'); sys.exit(1)
    if rows.get(rel) is None or sha(f'{R}/{rel}') != rows[rel]:
        fail(f'{rel} does not hash as SHA256.txt says')
    names = [f'{BOX}/{tag}{x}' for x in ('-box.txt', '.answer', '.answer.tmp', '-check.out', '-check.err', '-R1.txt', '-R1-ps.txt', '-guard.txt', '-ps.out')] + \
            [f'{RUN}/{tag}{x}' for x in ('-in.txt', '-in.sha256', '-screen.txt', '-startc.txt', '-out.bin', '-err.txt')] + [f'{EV}/{tag}-arm.txt', f'{PRIV}/{tag}-foreground.txt']
    for p in names:
        if os.path.exists(p): fail(f'{p} exists; a name is used once')
    try:
        stray = build.running(build.tasklist)
    except build.Fail as e:
        print('FAILED READ: ' + str(e)); sys.exit(2)
    if stray:
        print('STOP-STRAY: a bridge_mcp.exe is already running; nothing launched'); sys.exit(1)
    for d in (BOX, RUN, EV, PRIV): os.makedirs(d, exist_ok=True)
    committed = open(f'{R}/{rel}', 'rb').read(); nonce = str(uuid.uuid4())
    fed = nonce.encode() + b'\n' + committed.split(b'\n', 1)[1]
    if fed.split(b'\n', 1)[1] != committed.split(b'\n', 1)[1]: fail('the fed input differs after its first line')
    with open(f'{RUN}/{tag}-in.txt', 'xb') as f: f.write(fed)
    with open(f'{RUN}/{tag}-in.sha256', 'x', encoding='utf-8', newline='\n') as f:
        f.write(f'{sha(RUN + "/" + tag + "-in.txt")}  {len(fed)}  {tag}-in.txt (from {rel}, sha256 {rows[rel]}, first line replaced)\n')
    report = [f'{utc()} arm {tag}: mode {mode}, answer {ans}, input {rel} ({len(fed)} bytes fed, fresh nonce)']
    sc = subprocess.run(['powershell', '-NoProfile', '-File', R + '/screen.ps1', '-Out', f'{RUN}/{tag}-screen.txt'],
                        capture_output=True, text=True, encoding='utf-8', errors='replace', creationflags=NOWIN)
    st = open(f'{RUN}/{tag}-screen.txt', encoding='utf-8-sig').read() if os.path.exists(f'{RUN}/{tag}-screen.txt') else ''
    mons = re.findall(r'^MONITOR \d+ .* work=\((-?\d+),(-?\d+),(-?\d+),(-?\d+)\) dpi=', st, re.M)
    report += [f'screen.ps1 exit {sc.returncode}'] + [l for l in st.splitlines() if l.startswith(('OS ', 'DPI_AWARE', 'MONITOR'))]
    if sc.returncode != 0 or len(mons) != 1:
        report += [l[:220] for l in (sc.stdout + sc.stderr).splitlines() if l.startswith(('FAILED', 'RESULT'))][:10]
        report.append('STOP-MONITORS: screen.ps1 failed or read other than one monitor; nothing launched')
        print('\n'.join(report)); sys.exit(1)
    work = tuple(int(x) for x in mons[0])
    box = subprocess.Popen(['powershell', '-NoProfile', '-File', R + '/box.ps1', '-RowDir', L.replace('/', '\\'), '-Tag', tag,
                            '-SeeWithin', '30', '-AnswerBy', '90'], stdout=open(f'{BOX}/{tag}-ps.out', 'w'),
                           stderr=subprocess.STDOUT, creationflags=NOWIN)
    time.sleep(2)  # as pa.py: box.ps1 starts its watch before the box can open
    child = subprocess.Popen(['powershell', '-NoProfile', '-File', R + '/startc.ps1', '-Exe', exe.replace('/', '\\'), '-Mode', mode,
                              '-Show', '1', '-In', f'{RUN}/{tag}-in.txt'.replace('/', '\\'),
                              '-OutFile', f'{RUN}/{tag}-out.bin'.replace('/', '\\'), '-ErrFile', f'{RUN}/{tag}-err.txt'.replace('/', '\\')],
                             stdout=open(f'{RUN}/{tag}-startc.txt', 'w'), stderr=subprocess.STDOUT, creationflags=NOWIN)
    guard = {'ok': False, 'check': None, 'lines': []}
    def started():
        try: m = re.search(r'^STARTED pid=(\d+) ', open(f'{RUN}/{tag}-startc.txt', encoding='utf-8', errors='replace').read(), re.M)
        except OSError: m = None
        return m.group(1) if m else None
    def run_guard():
        g = guard['lines']; log = f'{BOX}/{tag}-box.txt'; t = ''
        end = time.monotonic() + 60
        while time.monotonic() < end:
            try: t = open(log, 'rb').read().decode('utf-8-sig')
            except (OSError, UnicodeDecodeError): t = ''
            if re.search(r'^\S+Z (WAITING for \S.*|NO BOX seen.*)\r?\n', t, re.M): break
            if box.poll() is not None: break
            time.sleep(0.25)
        if not re.search(r'^\S+Z WAITING for \S', t, re.M):
            g.append(f'{utc()} no WAITING line in the box log (box.ps1 exit {box.poll()}): no read, no check, no answer'); return
        seen = re.search(r'^\S+Z BOX SEEN 0x[0-9a-f]+ pid (\d+) ', t, re.M)
        scr = re.search(r'^\S+Z rect (-?\d+),(-?\d+),(-?\d+),(-?\d+) \(screen (\d+)x(\d+)\).*z-order place box (-?\d+) ', t, re.M)
        btn = [l for l in t.splitlines() if re.match(r'^\S+Z child Button rect -?\d+,-?\d+,-?\d+,-?\d+ text: &?' + ans + r'\s*$', l)]
        if not seen or not scr or len(btn) != 1:
            g.append(f'{utc()} the box log lacks the BOX SEEN pid, the rect line or exactly one {ans} button: no check, no answer'); return
        pid = seen.group(1); rect = tuple(int(scr.group(i)) for i in range(1, 5))
        br = tuple(int(x) for x in re.search(r'rect (-?\d+),(-?\d+),(-?\d+),(-?\d+)', btn[0]).groups())
        out = f'{BOX}/{tag}-R1.txt'
        p = subprocess.Popen(['powershell', '-NoProfile', '-File', R + '/boxstate.ps1', '-ProcId', pid, '-Out', out],
                             stdout=open(f'{BOX}/{tag}-R1-ps.txt', 'x'), stderr=subprocess.STDOUT, creationflags=NOWIN)
        try: rc = p.wait(timeout=BOXSTATE_WITHIN)
        except subprocess.TimeoutExpired: rc = None  # left running: nothing here ends a process (B4)
        try: read = open(out, encoding='utf-8-sig').read() if rc is not None else ''
        except OSError: read = ''
        g.append(f'{utc()} R1 boxstate.ps1 on pid {pid}: ' + (f'exit {rc}' if rc is not None else
                 f'not ended within {BOXSTATE_WITHIN} s, left running'))
        checks = guard_checks(pid, started(), rc, read, rect, work, br, scr.group(7), ans)
        g += [('PASS ' if ok else 'FAIL ') + name for name, ok in checks]
        if not all(ok for _, ok in checks):
            g.append(f'{utc()} GUARD NOT PASSED: no check, no answer file, nothing pressed (STOP-GUARD)'); return
        g.append(f'{utc()} GUARD PASSED: boxcheck.py started')
        guard['ok'] = True
        c = subprocess.run([sys.executable, '-I', '-X', 'utf8', R + '/boxcheck.py', tag, mode, ans], cwd=R,
                           stdout=subprocess.DEVNULL, stderr=open(f'{BOX}/{tag}-check.err', 'w'))
        guard['check'] = c.returncode
        g.append(f'{utc()} boxcheck.py exit {c.returncode}')
    if ans is None:  # ST: no guard, no check, no answer file, whatever the box shows
        guard['lines'].append(f'{utc()} {tag}: no guard, no check and no answer file for this arm; box.ps1 clicks nothing')
    else:
        th = threading.Thread(target=run_guard); th.start(); th.join()
    box.wait()
    open(f'{BOX}/{tag}-guard.txt', 'x', encoding='utf-8', newline='\n').write('\n'.join(guard['lines']) + '\n')
    blog = open(f'{BOX}/{tag}-box.txt', encoding='utf-8-sig').read() if os.path.exists(f'{BOX}/{tag}-box.txt') else ''
    stamped = [l[:220] for l in blog.splitlines() if re.match(r'^\d{4}-\d\d-\d\dT\S+Z ', l) and ' child Static ' not in l]
    report += [f'--- box log {tag} (stamped lines, the text child left out; box.ps1 exit {box.returncode})'] + stamped
    report += ['--- guard'] + guard['lines']
    if guard['check'] is not None and os.path.exists(f'{BOX}/{tag}-check.out'):
        report += [f'--- check (exit {guard["check"]})'] + open(f'{BOX}/{tag}-check.out', encoding='utf-8').read().splitlines()
    code = None
    if box.returncode == 0 or (ans is None and box.returncode in (3, 4)):
        end = time.monotonic() + 60
        while time.monotonic() < end:
            m = re.search(r'^EXITED code=(\d+) ', open(f'{RUN}/{tag}-startc.txt', encoding='utf-8', errors='replace').read(), re.M)
            if m: code = int(m.group(1)); break
            time.sleep(0.5)
        try:  # startc.ps1 holds the child's output files unshared until it ends, just after its EXITED line
            child.wait(timeout=30)
        except subprocess.TimeoutExpired:
            report.append('startc.ps1 had not ended 30 s after the wait for EXITED; its files are read as they are')
    else:
        if box.returncode in (3, 4, 5):  # no click and no box left up: the child's own account names the cause
            end = time.monotonic() + 10
            while time.monotonic() < end and exited(open(f'{RUN}/{tag}-startc.txt', encoding='utf-8', errors='replace').read()) is None:
                time.sleep(0.5)
            startc_text = open(f'{RUN}/{tag}-startc.txt', encoding='utf-8', errors='replace').read()
            if exited(startc_text) is not None:
                try: child.wait(timeout=10)  # startc.ps1 releases the child's files as it ends
                except subprocess.TimeoutExpired: pass
            try: err_text = open(f'{RUN}/{tag}-err.txt', encoding='utf-8', errors='replace').read()
            except OSError: err_text = None
            report += no_click_report(box.returncode, startc_text, err_text)
        stop = {3: 'STOP-NO-BOX: no box seen within 30 s', 4: 'STOP-BOX-GONE: the box went before any click',
                5: 'STOP-NO-BUTTON: the answer\'s button was not found, nothing clicked',
                6: ('STOP-UNANSWERED' if ans is None else 'STOP-GUARD' if not guard['ok'] else 'STOP-CHECK' if guard['check'] != 0 else 'STOP-UNANSWERED')
                   + ': no answer file by 90 s, nothing clicked, the box left up'}.get(box.returncode, 'STOP-BOX-HELPER: box.ps1 failed')
        report.append(f'{stop} (box.ps1 exit {box.returncode}); nothing ends the child'
                      + ('' if box.returncode in (3, 4, 5) else '; startc.ps1 is not waited on'))
    report += ['--- startc'] + open(f'{RUN}/{tag}-startc.txt', encoding='utf-8', errors='replace').read().splitlines()[:6]
    if box.returncode != 0 and not (ans is None and box.returncode in (3, 4)):
        open(f'{EV}/{tag}-arm.txt', 'x', encoding='utf-8', newline='\n').write('\n'.join(report) + '\n')
        print('\n'.join(report)); sys.exit(1)
    out = open(f'{RUN}/{tag}-out.bin', 'rb').read() if os.path.exists(f'{RUN}/{tag}-out.bin') else None
    err = open(f'{RUN}/{tag}-err.txt', encoding='utf-8', errors='replace').read() if os.path.exists(f'{RUN}/{tag}-err.txt') else None
    if out is None or err is None: fail('the child\'s output or error file is missing')
    rect = re.search(r'^\S+Z rect (-?\d+),(-?\d+),(-?\d+),(-?\d+) ', blog, re.M)
    if ans is None:
        g = grade_st(out, code, err, blog)
        w = [(f'W6 box.ps1 saw a box (BOX SEEN; box.ps1 exit {box.returncode}: 3 none seen, 4 gone before any click)',
              seen_pid(blog) is not None),
             cross(err, work, None)[0]] + w7(err)
        report += [f'--- graded (child standard output {len(out)} bytes; error output {len(err.splitlines())} line(s))']
        report += [l[:220] for l in err.splitlines()[:6]]
        if code == 0 and out == b'':
            report.append('UNAVAILABLE: exit 0 with nothing printed: too large refused, but not by the fit guard')
        report += ['deciding:'] + [('PASS ' if ok else 'FAIL ') + n for n, ok in g]
        report += ['recorded, not deciding:'] + [('PASS ' if ok else 'FAIL ') + n for n, ok in w]
        ok = all(x for _, x in g)
        report.append(f'ARM {tag} ' + ('AS THE CODE SAYS' if ok else 'NOT AS EXPECTED: record every FAIL line'))
        open(f'{EV}/{tag}-arm.txt', 'x', encoding='utf-8', newline='\n').write('\n'.join(report) + '\n')
        print('\n'.join(report)); sys.exit(0 if ok else 1)
    g = grade(mode, ans, nonce, out, code, err, blog)
    r1 = open(f'{BOX}/{tag}-R1.txt', encoding='utf-8-sig').read() if os.path.exists(f'{BOX}/{tag}-R1.txt') else ''
    line = next((l for l in r1.splitlines() if l.startswith('WINDOW ') and ' class=#32770 ' in l), '')
    focus = re.search(r'^FOCUS name=\[&?No\] .*pid=(\d+)$', r1, re.M)
    w = [('W1 the box not minimised at its first read (iconic=False)', ' iconic=False ' in line),
         ('W2 the box in front at its first read (foreground=True)', line.endswith(' foreground=True')),
         ('W3 the keyboard focus on No, the default button, in the box\'s process', focus is not None and seen_pid(blog) == focus.group(1))]
    w += cross(err, work, tuple(int(x) for x in rect.groups()) if rect else None) + w7(err)
    report += [f'--- graded (child standard output {len(out)} bytes: {out[:128]!r}; error output {len(err.splitlines())} line(s))']
    report += [l[:220] for l in err.splitlines()[:6]]
    report += ['deciding:'] + [('PASS ' if ok else 'FAIL ') + n for n, ok in g]
    report += ['recorded, not deciding:'] + [('PASS ' if ok else 'FAIL ') + n for n, ok in w]
    ok = all(x for _, x in g) and guard['ok'] and guard['check'] == 0
    report.append(f'ARM {tag} ' + ('AS THE CODE SAYS' if ok else 'NOT AS EXPECTED: record every FAIL line')
                  + f'; recorded lines: {sum(x for _, x in w)} of {len(w)} PASS')
    open(f'{EV}/{tag}-arm.txt', 'x', encoding='utf-8', newline='\n').write('\n'.join(report) + '\n')
    print('\n'.join(report)); sys.exit(0 if ok else 1)


def seen_pid(blog):
    m = re.search(r'^\S+Z BOX SEEN 0x[0-9a-f]+ pid (\d+) ', blog, re.M)
    return m.group(1) if m else None


if __name__ == '__main__':
    if sys.argv[1:] == ['--selftest']: sys.exit(selftest())
    if len(sys.argv) != 2 or sys.argv[1] not in ARMS: print('usage: arm.py <LN|SN|SC|SY|ST> | --selftest'); sys.exit(2)
    main(sys.argv[1])
