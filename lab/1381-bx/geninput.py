# Lab helper (row 1381-BX; not product code). Python 3.10+, standard library only. Writes and checks the stock-item box's
# child input, in/stock-input.txt, from the product's own source at the build's code commit, so that no line of it is typed.
#
# What the child reads (src-tauri/src/tally/approved_import.rs at 48a64965: nonce_bound_dialog :3127 writes
# "{nonce}\n{count}\n{preview}"; dialog_input :3273 splits it; the preview the stock dialog is given is displayed_text :2904,
# "{answer}\n\n{preview}", with the Windows answer_text :2764-2770). The test file pins that text on Windows:
# agent_master_review_tests.rs at 48a64965 holds STOCK_ITEM_PREVIEW (concat!, :361), the cfg(windows) STOCK_ITEM_ANSWER (:890),
# and the test the_stock_item_dialog_shows_its_answer_then_the_whole_preview, which on Windows asserts that the dialog text is
# "{STOCK_ITEM_ANSWER}\n\n{STOCK_ITEM_PREVIEW}", 32 lines and 1,455 bytes. The ignored test write_the_stock_item_box_input
# (:912-925) writes the file this helper writes, with the nonce synthetic_guid(0) and the count 1; run on Windows it gives
# these bytes, run elsewhere it gives the Mac alert's text (answer_text is cfg-split), so it cannot make this file on a Mac.
#
#   write <test source> <mac tool output> <out>   reads the literals from <test source> (decoding only \" \\ \n and plain
#       text: any other escape is refused), builds synthetic_guid(0) "\n1\n" WINDOWS_ANSWER "\n\n" PREVIEW, and checks it three
#       ways before writing <out> (never over an existing file): (1) the text after the 39-byte header is 1,455 bytes and 32
#       lines, the Windows assertions; (2) the Windows answer equals this helper's copy of wrapped() (approved_import.rs
#       :2798-2816) applied at 100 characters (:2792) to the answer_effect words (:2755-2758) in the Windows form (:2767);
#       (3) <mac tool output>, the ignored test's file as the product wrote it on a Mac, holds the same header, the cfg(not
#       (windows)) answer literal, a blank line and byte for byte the same preview.
#   check <test source> <file>                    the same checks (1) and (2), and that <file> is exactly those bytes.
# Exit 0 written or equal; 1 a check failed (named); 2 a file could not be read or parsed (never read as "differs").
import hashlib, os, re, sys

NONCE0 = '00000000-0000-4000-8000-000000000000'  # synthetic_guid(0), src-tauri/src/lib.rs:9-12 at 48a64965
WIN_LINES, WIN_BYTES = 32, 1455


def die(code, why):
    print(('FAILED READ: ' if code == 2 else 'CHECK FAILED: ') + why)
    sys.exit(code)


def rust_str(body):
    out, i = [], 0
    while i < len(body):
        c = body[i]
        if c == '\\':
            nxt = body[i + 1:i + 2]
            if nxt not in ('"', '\\', 'n'):
                die(2, f'escape \\{nxt} not admitted')
            out.append({'"': '"', '\\': '\\', 'n': '\n'}[nxt]); i += 2
        else:
            out.append(c); i += 1
    return ''.join(out)


LIT = re.compile(r'"((?:[^"\\]|\\.)*)"')


def literals(src):
    m = re.search(r'\nconst STOCK_ITEM_PREVIEW: &str = concat!\(\n(.*?)\n\);\n', src, re.S)
    if not m:
        die(2, 'STOCK_ITEM_PREVIEW concat! not found once')
    rows = [r.strip() for r in m.group(1).split('\n')]
    # Every row is one string literal and a comma; only the last row may omit its comma.
    if not rows or any(not r.endswith(',') for r in rows[:-1]):
        die(2, 'a STOCK_ITEM_PREVIEW row other than the last has no comma')
    rows = [r[:-1] if r.endswith(',') else r for r in rows]
    if any(not LIT.fullmatch(r) for r in rows):
        die(2, 'a STOCK_ITEM_PREVIEW row is not one string literal')
    preview = ''.join(rust_str(LIT.fullmatch(r).group(1)) for r in rows)
    answers = {}
    for cfg in ('not(windows)', 'windows'):
        found = re.findall(r'\n#\[cfg\(' + re.escape(cfg) + r'\)\]\nconst STOCK_ITEM_ANSWER: &str = "((?:[^"\\]|\\.)*)";\n', src)
        if len(found) != 1:
            die(2, f'the cfg({cfg}) STOCK_ITEM_ANSWER literal not found exactly once')
        answers[cfg] = rust_str(found[0])
    return preview, answers['windows'], answers['not(windows)']


def wrapped(text, limit):  # approved_import.rs:2798-2816, line for line
    lines, width = '', 0
    for word in text.split(' '):
        length = len(word)
        if width == 0:
            lines += word; width = length
        elif width + 1 + length <= limit:
            lines += ' ' + word; width += 1 + length
        else:
            lines += '\n' + word; width = length
    return lines


def build(src):
    preview, win, mac = literals(src)
    shown = f'{win}\n\n{preview}'
    data = f'{NONCE0}\n1\n{shown}'.encode('utf-8')
    checks = [(f'(1) the shown text is {WIN_LINES} lines and {WIN_BYTES} bytes (got {len(shown.splitlines())}, '
               f'{len(shown.encode())})', len(shown.splitlines()) == WIN_LINES and len(shown.encode()) == WIN_BYTES),
              ('(2) the Windows answer equals wrapped(answer, 100)',
               win == wrapped('Press Y (or click Yes) to create this stock item in Tally. Press N (or click No) to cancel. '
                              'Nothing is created until you answer.', 100))]
    return data, preview, mac, checks


def read(path):
    try:
        return open(path, 'rb').read()
    except OSError as e:
        die(2, f'{path}: {type(e).__name__}')


def main(argv):
    if len(argv) < 3 or argv[0] not in ('write', 'check') or len(argv) != (4 if argv[0] == 'write' else 3):
        die(2, 'usage: geninput.py write <test source> <mac tool output> <out> | check <test source> <file>')
    try:
        src = read(argv[1]).decode('utf-8')
    except UnicodeDecodeError:
        die(2, 'the test source is not UTF-8')
    data, preview, mac, checks = build(src)
    if argv[0] == 'write':
        tool = read(argv[2])
        want = f'{NONCE0}\n1\n{mac}\n\n{preview}'.encode('utf-8')
        checks.append(('(3) the Mac tool output is the header, the Mac answer, a blank line and the same preview',
                       tool == want))
    else:
        have = read(argv[2])
        checks.append((f'the file is exactly the generated bytes ({len(have)} bytes read, {len(data)} generated)',
                       have == data))
    for name, ok in checks:
        print(('PASS ' if ok else 'FAIL ') + name)
    if not all(ok for _, ok in checks):
        die(1, 'not every check passed; nothing written')
    if argv[0] == 'write':
        if os.path.exists(argv[3]):
            die(2, f'{argv[3]} exists; never overwritten')
        with open(argv[3], 'xb') as f:
            f.write(data)
    lf, verb = data.count(b'\n'), 'WROTE' if argv[0] == 'write' else 'EQUAL'
    print(f'{verb} {len(data)} bytes, {lf} LF, none at the end, '
          f'sha256 {hashlib.sha256(data).hexdigest()}; shown text {len(data) - 39} bytes, sha256 '
          f'{hashlib.sha256(data[39:]).hexdigest()}')
    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
