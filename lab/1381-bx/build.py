# Lab helper (row 1381-BX; not product code). Python 3.10, standard library only. The row's one build, read from
# BUILD.txt and nowhere else, checked and staged on the granted path; every act and hash goes to local/evidence.
#   check                 BUILD.txt holds each field once, in its shape; prints every field still <TO FILL>.
#   verify <artifact zip> the zip's bytes and sha256 against ARTIFACT BYTES and ZIP DIGEST; bridge-tally.mcpb and
#                         mcpb-smoke.json, each exactly once at the zip's root; the .mcpb's sha256 against MCPB SHA256 and
#                         the json's archive_sha256; the one member bin/x86_64-pc-windows-msvc/bridge_mcp.exe of the .mcpb
#                         (the entry point scripts/package-mcpb.mjs stages for win32-x64) written to local/build/bridge_mcp.exe
#                         and its sha256 against EXE SHA256 and the json's binary_sha256.
#   stage                 with no bridge_mcp.exe running (tasklist): the exe on GRANTED PATH must hash to ASIDE SHA256; it
#                         is moved to local/aside/bridge_mcp.exe and hashed there; local/build/bridge_mcp.exe is copied onto
#                         GRANTED PATH (written beside it, then renamed in) and hashed there against EXE SHA256.
#   restore               at the close, on the run coordinator's word: the reverse. The row's exe is moved from GRANTED
#                         PATH to local/build/returned/bridge_mcp.exe, the aside exe back onto GRANTED PATH, each hashed.
#   --selftest            the logic above on synthetic files in a temporary folder (runs on any OS; touches no lab path).
# Every act appends one line to local/evidence/copies.txt (UTC, act, source, target, bytes, sha256). Nothing is deleted.
# Exit 0 done; 1 a check failed or a field is still <TO FILL> (named, nothing moved after it); 2 a file could not be read.
import hashlib, io, json, os, re, shutil, subprocess, sys, tempfile, time, zipfile

R = os.path.dirname(os.path.abspath(__file__)).replace('\\', '/')  # the row folder: C:/bridge-lab/1381-bx at the sitting
FIELDS = {'BUILD COMMIT': r'[0-9a-f]{40}', 'CODE COMMIT': r'[0-9a-f]{40}', 'CI RUN': r'[0-9]{6,}',
          'ARTIFACT NAME': r'unsigned-bundle-smoke-windows-latest', 'ARTIFACT ID': r'[0-9]{4,}', 'ARTIFACT BYTES': r'[0-9]+',
          'ZIP DIGEST': r'[0-9a-f]{64}', 'MCPB SHA256': r'[0-9a-f]{64}', 'EXE SHA256': r'[0-9a-f]{64}',
          'GRANTED PATH': r'C:/bridge-lab/2569-r/exe-new/mcpb/bin/x86_64-pc-windows-msvc/bridge_mcp\.exe',
          'ASIDE SHA256': r'[0-9a-f]{64}'}
EXE_MEMBER = 'bin/x86_64-pc-windows-msvc/bridge_mcp.exe'


class Fail(Exception):
    def __init__(self, code, why):
        super().__init__(why); self.code = code


def utc(): return time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())
def sha(data): return hashlib.sha256(data).hexdigest()


def sha_file(path):
    try:
        with open(path, 'rb') as f:
            return sha(f.read()), os.path.getsize(path)
    except OSError as e:
        raise Fail(2, f'{path}: {type(e).__name__}')


def parse(text):
    """The fields of BUILD.txt and the names still <TO FILL>. A field missing, twice, unknown or out of shape is exit 2."""
    found, todo = {}, []
    for line in text.splitlines():
        if not line or line.startswith('#'):
            continue
        m = re.fullmatch(r'([A-Z0-9 ]+): (.+)', line)
        if not m or m.group(1) not in FIELDS or m.group(1) in found:
            raise Fail(2, f'BUILD.txt line not one known field, once: {line[:60]!r}')
        name, value = m.groups(); found[name] = value
        if value.startswith('<TO FILL'):
            todo.append(name)
        elif not re.fullmatch(FIELDS[name], value):
            raise Fail(2, f'BUILD.txt {name} is not in its shape')
    missing = sorted(set(FIELDS) - set(found))
    if missing:
        raise Fail(2, f'BUILD.txt lacks {", ".join(missing)}')
    return found, todo


def load(root, need_filled=True):
    try:
        text = open(f'{root}/BUILD.txt', encoding='utf-8').read()
    except OSError as e:
        raise Fail(2, f'BUILD.txt: {type(e).__name__}')
    b, todo = parse(text)
    if need_filled and todo:
        raise Fail(1, f'BUILD.txt still holds <TO FILL> in {", ".join(todo)}: STOP-BUILD-UNFILLED')
    return b, todo


def record(root, act, src, dst, path):
    digest, size = sha_file(path)
    os.makedirs(f'{root}/local/evidence', exist_ok=True)
    with open(f'{root}/local/evidence/copies.txt', 'a', encoding='utf-8', newline='\n') as f:
        f.write(f'{utc()}\t{act}\t{src}\t{dst}\t{size}\t{digest}\n')
    return digest


def verify(root, zpath, out):
    b, _ = load(root)
    data = open(zpath, 'rb').read() if os.path.isfile(zpath) else None
    if data is None:
        raise Fail(2, f'{zpath}: not a file')
    out.append(f'zip {len(data)} bytes sha256 {sha(data)}')
    if len(data) != int(b['ARTIFACT BYTES']) or sha(data) != b['ZIP DIGEST']:
        raise Fail(1, 'the zip differs from ARTIFACT BYTES or ZIP DIGEST: STOP-BUILD-HASH')
    try:
        z = zipfile.ZipFile(io.BytesIO(data))
        names = z.namelist()
        if names.count('bridge-tally.mcpb') != 1 or names.count('mcpb-smoke.json') != 1:
            raise Fail(1, 'the zip does not hold bridge-tally.mcpb and mcpb-smoke.json once each at its root: STOP-BUILD-LAYOUT')
        mcpb, smoke = z.read('bridge-tally.mcpb'), json.loads(z.read('mcpb-smoke.json').decode('utf-8'))
        out.append(f'mcpb {len(mcpb)} bytes sha256 {sha(mcpb)}; json archive_sha256 {smoke.get("archive_sha256")} '
                   f'binary_sha256 {smoke.get("binary_sha256")}')
        if not (sha(mcpb) == b['MCPB SHA256'] == smoke.get('archive_sha256')):
            raise Fail(1, 'the .mcpb differs from MCPB SHA256 or archive_sha256: STOP-BUILD-HASH')
        inner = zipfile.ZipFile(io.BytesIO(mcpb))
        hits = [n for n in inner.namelist() if n == EXE_MEMBER]
        if len(hits) != 1:
            raise Fail(1, f'the .mcpb holds {len(hits)} members named {EXE_MEMBER}: STOP-BUILD-LAYOUT')
        exe = inner.read(EXE_MEMBER)
    except (zipfile.BadZipFile, ValueError, KeyError) as e:
        raise Fail(2, f'the artifact could not be read: {type(e).__name__}')
    out.append(f'exe {len(exe)} bytes sha256 {sha(exe)}')
    if not (sha(exe) == b['EXE SHA256'] == smoke.get('binary_sha256')):
        raise Fail(1, 'the exe differs from EXE SHA256 or binary_sha256: STOP-BUILD-HASH')
    os.makedirs(f'{root}/local/build', exist_ok=True)
    if os.path.exists(f'{root}/local/build/bridge_mcp.exe'):
        raise Fail(1, 'local/build/bridge_mcp.exe exists; never overwritten: STOP-BUILD')
    with open(f'{root}/local/build/bridge_mcp.exe', 'xb') as f:
        f.write(exe)
    record(root, 'extract', f'{zpath}!bridge-tally.mcpb!{EXE_MEMBER}', 'local/build/bridge_mcp.exe', f'{root}/local/build/bridge_mcp.exe')
    out.append('VERIFIED: zip, mcpb and exe equal BUILD.txt and mcpb-smoke.json; exe written to local/build/bridge_mcp.exe')


def running(lister):
    p = lister()
    if p.returncode != 0:
        raise Fail(2, f'tasklist failed, exit {p.returncode}: the command failed, never "none running"')
    lines = [l for l in p.stdout.splitlines() if l.strip()]
    if len(lines) == 1 and lines[0].startswith('INFO:'):
        return 0
    return len([l for l in lines if l.lower().startswith('"bridge_mcp.exe"')]) or len(lines)


def tasklist():
    return subprocess.run(['tasklist', '/FI', 'IMAGENAME eq bridge_mcp.exe', '/FO', 'CSV', '/NH'],
                          capture_output=True, text=True, errors='replace')


def move_checked(root, act, src, dst, want):
    if os.path.exists(dst):
        raise Fail(1, f'{dst} exists; never overwritten: STOP-STAGE')
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    os.replace(src, dst)
    if record(root, act, src, dst, dst) != want:
        raise Fail(1, f'{dst} does not hash as expected after {act}: STOP-STAGE')


def stage(root, granted, out, lister=tasklist):
    b, _ = load(root)
    if running(lister):
        raise Fail(1, 'a bridge_mcp.exe is running: STOP-STAGE')
    new, aside = f'{root}/local/build/bridge_mcp.exe', f'{root}/local/aside/bridge_mcp.exe'
    if sha_file(new)[0] != b['EXE SHA256']:
        raise Fail(1, 'local/build/bridge_mcp.exe does not hash to EXE SHA256 (run verify first): STOP-STAGE')
    have = sha_file(granted)[0]
    out.append(f'granted path before: sha256 {have}')
    if have != b['ASIDE SHA256']:
        raise Fail(1, 'the exe on the granted path does not hash to ASIDE SHA256: STOP-STAGE, nothing moved')
    move_checked(root, 'move-aside', granted, aside, b['ASIDE SHA256'])
    tmp = granted + '.1381-bx-tmp'
    if os.path.exists(tmp):
        raise Fail(1, f'{tmp} exists: STOP-STAGE')
    shutil.copyfile(new, tmp)
    os.replace(tmp, granted)
    if record(root, 'copy-in', 'local/build/bridge_mcp.exe', granted, granted) != b['EXE SHA256']:
        raise Fail(1, 'the granted path does not hash to EXE SHA256 after the copy: STOP-STAGE')
    out.append('STAGED: the aside exe in local/aside, the row\'s exe on the granted path, both hashed')


def restore(root, granted, out, lister=tasklist):
    b, _ = load(root)
    if running(lister):
        raise Fail(1, 'a bridge_mcp.exe is running: STOP-RESTORE')
    aside = f'{root}/local/aside/bridge_mcp.exe'
    if sha_file(granted)[0] != b['EXE SHA256'] or sha_file(aside)[0] != b['ASIDE SHA256']:
        raise Fail(1, 'the granted path or the aside exe does not hash as staged: STOP-RESTORE, nothing moved')
    move_checked(root, 'move-out', granted, f'{root}/local/build/returned/bridge_mcp.exe', b['EXE SHA256'])
    move_checked(root, 'move-back', aside, granted, b['ASIDE SHA256'])
    out.append('RESTORED: the aside exe back on the granted path, the row\'s exe in local/build/returned')


def selftest():
    cases, right = 0, 0
    def case(name, fn, want):
        nonlocal cases, right
        cases += 1
        try:
            fn(); got = 0
        except Fail as e:
            got = e.code
        ok = got == want; right += ok
        print(f'{name}: expected exit {want}, got {got} {"OK" if ok else "WRONG"}')
    def expect(cond):
        if not cond:
            raise Fail(1, 'not as expected')
    with tempfile.TemporaryDirectory() as t:
        exe = b'MZ synthetic exe'
        inner = io.BytesIO(); zi = zipfile.ZipFile(inner, 'w'); zi.writestr(EXE_MEMBER, exe); zi.writestr('manifest.json', '{}'); zi.close()
        mcpb = inner.getvalue()
        smoke = json.dumps({'archive_sha256': sha(mcpb), 'binary_sha256': sha(exe)})
        outer = io.BytesIO(); zo = zipfile.ZipFile(outer, 'w'); zo.writestr('bridge-tally.mcpb', mcpb); zo.writestr('mcpb-smoke.json', smoke); zo.close()
        zbytes = outer.getvalue(); open(f'{t}/a.zip', 'wb').write(zbytes)
        granted = f'{t}/granted/bridge_mcp.exe'; os.makedirs(f'{t}/granted'); old = b'MZ the aside exe'; open(granted, 'wb').write(old)
        lines = {'BUILD COMMIT': 'a' * 40, 'CODE COMMIT': 'b' * 40, 'CI RUN': '1234567', 'ARTIFACT NAME': 'unsigned-bundle-smoke-windows-latest',
                 'ARTIFACT ID': '98765', 'ARTIFACT BYTES': str(len(zbytes)), 'ZIP DIGEST': sha(zbytes), 'MCPB SHA256': sha(mcpb),
                 'EXE SHA256': sha(exe), 'GRANTED PATH': 'C:/bridge-lab/2569-r/exe-new/mcpb/bin/x86_64-pc-windows-msvc/bridge_mcp.exe',
                 'ASIDE SHA256': sha(old)}
        def put(**change):
            d = dict(lines, **{k.replace('_', ' '): v for k, v in change.items()})
            open(f'{t}/BUILD.txt', 'w', encoding='utf-8').write('# synthetic\n' + ''.join(f'{k}: {v}\n' for k, v in d.items()))
        none = lambda: subprocess.CompletedProcess([], 0, 'INFO: No tasks are running which match the specified criteria.\n', '')
        one = lambda: subprocess.CompletedProcess([], 0, '"bridge_mcp.exe","4242","Console","1","9,000 K"\n', '')
        broke = lambda: subprocess.CompletedProcess([], 1, '', 'ERROR')
        put(); case('a filled BUILD.txt checks', lambda: load(t), 0)
        put(EXE_SHA256='<TO FILL: x>'); case('a field still <TO FILL> stops the row', lambda: load(t), 1)
        put(CI_RUN='12ab'); case('a field out of its shape is a failed read', lambda: load(t), 2)
        open(f'{t}/BUILD.txt', 'a').write('CI RUN: 1234567\n'); case('a field twice is a failed read', lambda: load(t), 2)
        put(ZIP_DIGEST='0' * 64); case('a zip whose digest differs stops', lambda: verify(t, f'{t}/a.zip', []), 1)
        put(MCPB_SHA256='0' * 64); case('a .mcpb whose hash differs stops', lambda: verify(t, f'{t}/a.zip', []), 1)
        put(EXE_SHA256='0' * 64); case('an exe whose hash differs stops', lambda: verify(t, f'{t}/a.zip', []), 1)
        put(); case('the right artifact verifies and writes the exe', lambda: verify(t, f'{t}/a.zip', []), 0)
        case('a second verify never overwrites the exe', lambda: verify(t, f'{t}/a.zip', []), 1)
        case('stage refuses while a bridge_mcp.exe runs', lambda: stage(t, granted, [], one), 1)
        case('stage refuses when tasklist fails', lambda: stage(t, granted, [], broke), 2)
        case('stage moves aside and copies in', lambda: stage(t, granted, [], none), 0)
        case('the granted path then holds the row\'s exe', lambda: expect(sha_file(granted)[0] == sha(exe)), 0)
        case('a second stage refuses: the granted path is not the aside exe', lambda: stage(t, granted, [], none), 1)
        case('restore puts the aside exe back', lambda: restore(t, granted, [], none), 0)
        case('the granted path then holds the aside exe', lambda: expect(sha_file(granted)[0] == sha(old)), 0)
        n = len(open(f'{t}/local/evidence/copies.txt').read().splitlines())
        case(f'copies.txt holds the five acts (found {n})', lambda: expect(n == 5), 0)
    print(f'{cases} cases, {right} right'); return 0 if cases == right else 1


def main(argv):
    if argv == ['--selftest']:
        return selftest()
    out = []
    try:
        if argv == ['check']:
            b, todo = load(R, need_filled=False)
            for name in FIELDS:
                print(f'{name}: {"<TO FILL>" if name in todo else b[name]}')
            print(f'BUILD.txt: {len(todo)} field(s) to fill' + (': STOP-BUILD-UNFILLED' if todo else ': filled'))
            return 1 if todo else 0
        b, _ = load(R)
        if len(argv) == 2 and argv[0] == 'verify':
            verify(R, argv[1], out)
        elif argv == ['stage']:
            stage(R, b['GRANTED PATH'], out)
        elif argv == ['restore']:
            restore(R, b['GRANTED PATH'], out)
        else:
            raise Fail(2, 'usage: build.py check | verify <zip> | stage | restore | --selftest')
    except Fail as e:
        print('\n'.join(out + [('FAILED READ: ' if e.code == 2 else 'STOP: ') + str(e)])); return e.code
    except OSError as e:
        print('\n'.join(out + [f'FAILED READ: {type(e).__name__}: {e}'])); return 2
    print('\n'.join(out)); return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
