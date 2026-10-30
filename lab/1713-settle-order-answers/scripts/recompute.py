# Lab helper (#1713 order rows, not product code): the row verdicts recomputed from the saved files with orderread.py's
# reading (the live driver used read.py's probe list, which holds only "Lab Probe" names). For each row: order.py's events
# (the sheet's "what counts"), the pre-row MARKS and LEDGERS, C's answer, and each observation's last try. Writes
# evidence/times-final.txt, one line per row, with the verdict word. Usage: python -I recompute.py K0a K1a ...
import glob, json, os, re, subprocess, sys
W = '<lab folder>'; E = W + '/earlier/lab/1713-abandoned-ledger-create'; GUID = '5f4825c4-5d89-47fa-9872-6397cca3a591'
def probe(path, name):
    r = subprocess.run([sys.executable, '-I', W + '/orderread.py', path, name], capture_output=True, text=True)
    m = re.search(r'probe=(\w+)', r.stdout); return (m.group(1) if m else 'unread'), r.stdout.strip()
def mark(path):
    r = subprocess.run([sys.executable, '-I', E + '/read.py', 'marks', path, GUID], capture_output=True, text=True)
    m = re.search(r'ALTMSTID (\d+)', r.stdout); return m.group(1) if m else '?'
out = []
for row in sys.argv[1:]:
    main = f'{W}/runs/{row}/main'; evs = [json.loads(l) for l in open(main + '/events.jsonl', encoding='utf-8')]
    def ev(kind, name): return next((e for e in evs if e.get('ev') == kind and e.get('name') == name), None)
    a_send, a_fb, b_wr, b_end, c_wr, c_fb, c_end = ev('send', 'A'), ev('first_byte', 'A'), ev('written', 'B'), ev('end', 'B'), ev('written', 'C'), ev('first_byte', 'C'), ev('end', 'C')
    d = round(a_fb['t'] - a_send['t'], 3) if a_fb and a_send else None
    void = []
    if os.path.exists(main + '/stop'): void.append('stop by the watcher')
    want_b = 'answered' if row.startswith('L1B') else 'abandoned'
    if not b_wr: void.append('B not written')
    if not b_end or b_end.get('how') != want_b: void.append(f'B {b_end.get("how") if b_end else None}')
    if not (c_wr and a_fb and c_wr['t'] < a_fb['t']): void.append('VOID-late')
    if not (c_fb and a_fb and c_fb['t'] > a_fb['t']): void.append('C first byte not after A first byte')
    if not c_end or c_end.get('how') != 'answered': void.append(f'C {c_end.get("how") if c_end else None}')
    if d is None or d < 55: void.append(f'D {d}')
    pre_state, _ = probe(f'{W}/answers/{row}-pre-ledgers.xml', f'Lab Order {row}'); pre_mark = mark(f'{W}/answers/{row}-pre-marks.xml')
    c_state, c_line = probe(main + '/C.answer', f'Lab Order {row}') if c_end and c_end.get('how') == 'answered' else ('unread', '')
    obs = []
    for label in ('obs1', 'obs5', 'obs15', 'obs30'):
        led = sorted(glob.glob(f'{W}/runs/{row}/{label}-ledgers-*/ledgers.answer'), key=lambda p: int(p.split('-')[-1].split(os.sep)[0].split('/')[0]))
        mk = sorted(glob.glob(f'{W}/runs/{row}/{label}-marks-*/marks.answer'))
        obs.append((label, probe(led[-1], f'Lab Order {row}')[0] if led else 'unread', mark(mk[-1]) if mk else '?'))
    seen = [o[1] for o in obs]
    if void: verdict = 'VOID'
    elif c_state == 'present' and all(s == 'present' for s in seen): verdict = 'PASS-present'
    elif c_state == 'absent' and all(s == 'absent' for s in seen) and all(o[2] == pre_mark for o in obs): verdict = 'PASS-dropped'
    elif c_state == 'absent' and any(s == 'present' for s in seen): verdict = 'FAIL'
    elif 'TWICE' in [c_state] + seen: verdict = 'FAIL'
    else: verdict = 'NOTE'
    line = (f'{row}: D {d} s; B {b_end.get("how") if b_end else None} (written t {b_wr["t"] if b_wr else None}); C written t {c_wr["t"] if c_wr else None}, '
            f'A first byte t {a_fb["t"] if a_fb else None}, C first byte t {c_fb["t"] if c_fb else None}; pre-row probe {pre_state}, mark {pre_mark}; '
            f'C probe {c_state} ({c_line}); observations ' + ', '.join(f'{l} {s} mark {m}' for l, s, m in obs) + f'; {verdict}' + (f' ({"; ".join(void)})' if void else ''))
    out.append(line); print(line)
open(W + '/evidence/times-final.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
