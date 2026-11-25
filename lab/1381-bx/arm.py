# Lab helper (row 1381-BX, in the shape of row 2162-PA's pa.py at lab/2162-pa-sheet-draft f682a6aa; not product code): one
# arm of the row, under one start line, so that nothing is typed while its box is up. No server runs and Tally is not
# called: the box comes from the dialog child alone (src-tauri/src/lib.rs:112-126 at 48a64965).
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
#      iconic=False, visible=True and topmost=True, box.ps1's rectangle of the box lies inside the monitor's work area,
#      the arm's button lies inside the box, and box.ps1 logged the box at z-order place 0. Only then does it start
#      boxcheck.py, which writes the arm's answer when the box's whole text, title and buttons are what was fed. The
#      guard's lines go to local/box/<tag>-guard.txt. The foreground and FOCUS lines are graded after, not guarded.
#   5. After box.ps1 has clicked, waits up to 60 s for startc.ps1's EXITED line, then grades the child's standard output,
#      exit code and error output from the files (grade(), below). With no click (box.ps1 exit other than 0) it does not
#      wait: the box is left up, the child running, and the arm is STOP-UNANSWERED.
# Usage: python -I -X utf8 arm.py <LN|SN|SC|SY>      python -I -X utf8 arm.py --selftest   (grade() only; any OS)
# Exit 0: every deciding line as the code says (the W lines are recorded, not deciding); 1: anything else that was read (a named STOP or a FAIL to record);
# 2: a failed read of this helper's own inputs (BUILD.txt, the exe, an input, a name already used).
import hashlib, importlib.util, os, re, subprocess, sys, threading, time, uuid
R = os.path.dirname(os.path.abspath(__file__)).replace('\\', '/')  # the row folder: C:/bridge-lab/1381-bx at the sitting
L = R + '/local'; BOX, RUN, EV = L + '/box', L + '/run', L + '/evidence'
ARMS = {'LN': ('masters', 'No', 'in/ledger-input.txt'), 'SN': ('stock', 'No', 'in/stock-input.txt'),
        'SC': ('stock', 'Cancel', 'in/stock-input.txt'), 'SY': ('stock', 'Yes', 'in/stock-input.txt')}
PREFIX = {'stock': 'bridge-create-stock-item:', 'masters': 'bridge-create-ledger:'}  # approved_import.rs:88 and :57
FIT = re.compile(r'box-fit: box=\((-?\d+),(-?\d+),(-?\d+),(-?\d+)\) work=\((-?\d+),(-?\d+),(-?\d+),(-?\d+)\) fits=(true|false)')
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
    fits = [FIT.fullmatch(l) for l in lines]
    clicked = re.findall(r'^\S+Z CLICKED (\S+) at ', blog, re.M)
    return [(f'box.ps1 clicked {ans}, once (clicked {clicked})', clicked == [ans]),
            ('box.ps1 read the box as gone after the click', re.search(r'^\S+Z after click: box visible False;', blog, re.M) is not None),
            (f'exit code {0 if ans == "Yes" else 1} (read {code})', code == (0 if ans == 'Yes' else 1)),
            (f'standard output {"exactly the token for this nonce and a line feed" if want else "empty"} ({len(out)} bytes read)', out == want),
            (f'error output: only box-fit lines, at least one ({len(lines)} line(s))', bool(lines) and all(fits)),
            ('every box-fit line says fits=true', bool(lines) and all(m is not None and m.group(9) == 'true' for m in fits))]


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
    tok = ('bridge-create-stock-item:' + n0 + '\n').encode()
    lg = lambda a: ok_log.format(a=a)
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
        ('another line in the error output', grade, ('stock', 'No', n0, b'', 1, fit + 'box-fit: restored from minimised\n', lg('No')), False),
        ('No clicked where Cancel was the arm', grade, ('stock', 'Cancel', n0, b'', 1, fit, lg('No')), False),
        ('no click logged', grade, ('stock', 'No', n0, b'', 1, fit, ''), False),
        ('cross: the same work area and rectangle', cross, (fit, work, rect), True),
        ('cross: a work area other than screen.ps1\'s', cross, (fit, (0, 0, 1920, 1040), rect), False),
        ('cross: a box rectangle other than box.ps1\'s', cross, (fit, work, (688, 160, 1252, 945)), False),
        ('cross: no box-fit line', cross, ('', work, rect), False),
    ]
    right = 0
    for name, fn, args, want in cases:
        g = fn(*args); got = all(ok for _, ok in g); right += got == want
        print(f'{name}: expected {"all right" if want else "a line wrong"}, got {"all right" if got else "a line wrong"} '
              f'{"OK" if got == want else "WRONG"}')
    print(f'{len(cases)} cases, {right} right'); return 0 if right == len(cases) == 21 else 1


def fail(why): print('FAILED READ:', why, flush=True); sys.exit(2)


def sha(path): return hashlib.sha256(open(path, 'rb').read()).hexdigest()


def main(tag):
    mode, ans, rel = ARMS[tag]
    spec = importlib.util.spec_from_file_location('build', R + '/build.py'); build = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(build)
    try:
        b, _ = build.load(R)
    except build.Fail as e:
        print(('STOP: ' if e.code == 1 else 'FAILED READ: ') + str(e)); sys.exit(e.code)
    exe = b['GRANTED PATH']
    if not os.path.isfile(exe) or sha(exe) != b['EXE SHA256']:
        print('STOP-EXE-HASH: the exe on the granted path is missing or does not hash to EXE SHA256; nothing launched'); sys.exit(1)
    rows = {}
    for line in open(R + '/SHA256.txt', encoding='utf-8').read().splitlines():
        m = re.fullmatch(r'([0-9a-f]{64})  ([0-9]+)  (.+)', line)
        if m: rows[m.group(3)] = m.group(1)
    if rows.get(rel) is None or sha(f'{R}/{rel}') != rows[rel]:
        fail(f'{rel} does not hash as SHA256.txt says')
    names = [f'{BOX}/{tag}{x}' for x in ('-box.txt', '.answer', '.answer.tmp', '-check.out', '-check.err', '-R1.txt', '-guard.txt', '-ps.out')] + \
            [f'{RUN}/{tag}{x}' for x in ('-in.txt', '-in.sha256', '-screen.txt', '-startc.txt', '-out.bin', '-err.txt')] + [f'{EV}/{tag}-arm.txt']
    for p in names:
        if os.path.exists(p): fail(f'{p} exists; a name is used once')
    try:
        stray = build.running(build.tasklist)
    except build.Fail as e:
        print('FAILED READ: ' + str(e)); sys.exit(2)
    if stray:
        print('STOP-STRAY: a bridge_mcp.exe is already running; nothing launched'); sys.exit(1)
    for d in (BOX, RUN, EV): os.makedirs(d, exist_ok=True)
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
    def inside(a, w): return a is not None and w[0] <= a[0] < a[2] <= w[2] and w[1] <= a[1] < a[3] <= w[3]
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
        p = subprocess.run(['powershell', '-NoProfile', '-File', R + '/boxstate.ps1', '-ProcId', pid, '-Out', out],
                           capture_output=True, text=True, encoding='utf-8', errors='replace', creationflags=NOWIN)
        try: read = open(out, encoding='utf-8-sig').read()
        except OSError: read = ''
        lines = [l for l in read.splitlines() if l.startswith('WINDOW ') and ' name=[ComplyEaze Bridge' in l and ' class=#32770 ' in l]
        g.append(f'{utc()} R1 boxstate.ps1 on pid {pid}: exit {p.returncode}; box lines {len(lines)}')
        state = lines[0] if len(lines) == 1 else ''
        checks = [(f'the box is the child\'s (box pid {pid}, startc pid {started()})', pid == started()),
                  ('boxstate exit 0', p.returncode == 0), ('exactly one box line', len(lines) == 1),
                  ('iconic=False', ' iconic=False ' in state), ('visible=True', ' visible=True ' in state),
                  ('topmost=True', ' topmost=True ' in state),
                  (f'the box rect {rect} inside the work area {work}', inside(rect, work)),
                  (f'the {ans} button rect {br} inside the box', inside(br, rect)),
                  ('the box nearest the top of the z-order (place 0), so the click lands on it', scr.group(7) == '0')]
        g += [('PASS ' if ok else 'FAIL ') + name for name, ok in checks]
        if not all(ok for _, ok in checks):
            g.append(f'{utc()} GUARD NOT PASSED: no check, no answer file, nothing pressed (STOP-GUARD)'); return
        g.append(f'{utc()} GUARD PASSED: boxcheck.py started')
        guard['ok'] = True
        c = subprocess.run([sys.executable, '-I', '-X', 'utf8', R + '/boxcheck.py', tag, mode, ans], cwd=R,
                           stdout=subprocess.DEVNULL, stderr=open(f'{BOX}/{tag}-check.err', 'w'))
        guard['check'] = c.returncode
        g.append(f'{utc()} boxcheck.py exit {c.returncode}')
    th = threading.Thread(target=run_guard); th.start()
    th.join(); box.wait()
    open(f'{BOX}/{tag}-guard.txt', 'x', encoding='utf-8', newline='\n').write('\n'.join(guard['lines']) + '\n')
    blog = open(f'{BOX}/{tag}-box.txt', encoding='utf-8-sig').read() if os.path.exists(f'{BOX}/{tag}-box.txt') else ''
    stamped = [l[:220] for l in blog.splitlines() if re.match(r'^\d{4}-\d\d-\d\dT\S+Z ', l) and ' child Static ' not in l]
    report += [f'--- box log {tag} (stamped lines, the text child left out; box.ps1 exit {box.returncode})'] + stamped
    report += ['--- guard'] + guard['lines']
    if guard['check'] is not None and os.path.exists(f'{BOX}/{tag}-check.out'):
        report += [f'--- check (exit {guard["check"]})'] + open(f'{BOX}/{tag}-check.out', encoding='utf-8').read().splitlines()
    code = None
    if box.returncode == 0:
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
        stop = {3: 'STOP-NO-BOX: no box seen within 30 s', 4: 'STOP-BOX-GONE: the box went before any click',
                5: 'STOP-NO-BUTTON: the answer\'s button was not found, nothing clicked',
                6: ('STOP-GUARD' if not guard['ok'] else 'STOP-CHECK' if guard['check'] != 0 else 'STOP-UNANSWERED')
                   + ': no answer file by 90 s, nothing clicked, the box left up'}.get(box.returncode, 'STOP-BOX-HELPER: box.ps1 failed')
        report.append(f'{stop} (box.ps1 exit {box.returncode}); nothing ends the child; startc.ps1 is not waited on')
    report += ['--- startc'] + open(f'{RUN}/{tag}-startc.txt', encoding='utf-8', errors='replace').read().splitlines()[:6]
    if box.returncode != 0:
        open(f'{EV}/{tag}-arm.txt', 'x', encoding='utf-8', newline='\n').write('\n'.join(report) + '\n')
        print('\n'.join(report)); sys.exit(1)
    out = open(f'{RUN}/{tag}-out.bin', 'rb').read() if os.path.exists(f'{RUN}/{tag}-out.bin') else None
    err = open(f'{RUN}/{tag}-err.txt', encoding='utf-8', errors='replace').read() if os.path.exists(f'{RUN}/{tag}-err.txt') else None
    if out is None or err is None: fail('the child\'s output or error file is missing')
    rect = re.search(r'^\S+Z rect (-?\d+),(-?\d+),(-?\d+),(-?\d+) ', blog, re.M)
    g = grade(mode, ans, nonce, out, code, err, blog)
    r1 = open(f'{BOX}/{tag}-R1.txt', encoding='utf-8-sig').read() if os.path.exists(f'{BOX}/{tag}-R1.txt') else ''
    line = next((l for l in r1.splitlines() if l.startswith('WINDOW ') and ' class=#32770 ' in l), '')
    focus = re.search(r'^FOCUS name=\[&?No\] .*pid=(\d+)$', r1, re.M)
    w = [('W1 the box not minimised at its first read (iconic=False)', ' iconic=False ' in line),
         ('W2 the box in front at its first read (foreground=True)', line.endswith(' foreground=True')),
         ('W3 the keyboard focus on No, the default button, in the box\'s process', focus is not None and seen_pid(blog) == focus.group(1))]
    w += cross(err, work, tuple(int(x) for x in rect.groups()) if rect else None)
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
    if len(sys.argv) != 2 or sys.argv[1] not in ARMS: print('usage: arm.py <LN|SN|SC|SY> | --selftest'); sys.exit(2)
    main(sys.argv[1])
