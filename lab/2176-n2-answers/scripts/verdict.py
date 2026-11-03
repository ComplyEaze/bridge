# Lab helper (#2176 N1, not product code): one row's verdict from its saved files, with orderread.py for the probe, as recompute.py does for
# the K rows. Prints the sheet's "what counts" facts and appends one line to evidence/times-final.txt. Usage: python -I verdict.py <row>
import glob, json, os, re, subprocess, sys
W = '<lab folder>'; O = '<earlier lab folder>'; E = O + '/earlier/lab/1713-abandoned-ledger-create'
GUID = '5f4825c4-5d89-47fa-9872-6397cca3a591'
row = sys.argv[1]; name = f'Lab Order {row}'
main = f'{W}/runs/{row}/main'
evs = [json.loads(l) for l in open(main + '/events.jsonl', encoding='utf-8')]
def ev(kind, n): return next((e for e in evs if e.get('ev') == kind and e.get('name') == n), None)
def probe(path):
    r = subprocess.run([sys.executable, '-I', O + '/orderread.py', path, name], capture_output=True, text=True)
    m = re.search(r'probe=(\w+)', r.stdout); return m.group(1) if m else 'unread:' + r.stderr.strip()[-60:]
def mark(path):
    r = subprocess.run([sys.executable, '-I', E + '/read.py', 'marks', path, GUID], capture_output=True, text=True)
    return re.search(r'ALTVCHID (\S+) ALTMSTID (\d+)', r.stdout).groups()
a_sent, a_fb, a_end = ev('send', 'A'), ev('first_byte', 'A'), ev('end', 'A')
b_wr, b_end = ev('written', 'B'), ev('end', 'B')
c_wr, c_fb, c_end = ev('written', 'C'), ev('first_byte', 'C'), ev('end', 'C')
d = round(a_fb['t'] - a_sent['t'], 3) if a_fb and a_sent else None
void = []
if os.path.exists(main + '/stop'): void.append('stop file')
if not b_wr: void.append('B not written')
if not b_end or b_end.get('how') != 'abandoned': void.append(f'B end {b_end and b_end.get("how")}')
if not (c_wr and a_fb and c_wr['t'] < a_fb['t']): void.append('C not written before A first_byte')
if not (c_fb and a_fb and c_fb['t'] > a_fb['t']): void.append('C first_byte not after A first_byte')
if not c_end or c_end.get('how') != 'answered': void.append(f'C end {c_end and c_end.get("how")}')
if d is None or d < 55: void.append(f'D {d} under 55')
print({'D': d, 'A end': a_end and a_end.get('how'), 'B written': b_wr and b_wr['t'], 'B end': b_end and (b_end.get('how'), b_end['t']), 'C written': c_wr and c_wr['t'], 'C first_byte': c_fb and c_fb['t'], 'C end': c_end and (c_end.get('how'), c_end['t']), 'A first_byte': a_fb and a_fb['t']})
c_state = probe(main + '/C.answer') if c_end and c_end.get('how') == 'answered' else 'no answer'
obs = []
for label in ('obs1', 'obs5', 'obs15', 'obs30'):
    ls = sorted(glob.glob(f'{W}/runs/{row}/{label}-ledgers-*/ledgers.answer')); ms = sorted(glob.glob(f'{W}/runs/{row}/{label}-marks-*/marks.answer'))
    obs.append((label, probe(ls[-1]) if ls else 'not read', mark(ms[-1]) if ms else None))
pre = [f for f in glob.glob(f'{W}/answers/{row}-pre-marks*') if 'headers' not in f and f.endswith(('.xml', '.answer', '.response'))]
pre_mark = mark(pre[0]) if pre else None
print('pre marks', pre_mark, 'C', c_state); [print(o) for o in obs]
seen = [o[1] for o in obs]
if void: v = 'VOID'
elif c_state == 'present' and all(s == 'present' for s in seen): v = 'PASS-present'
elif c_state == 'absent' and all(s == 'absent' for s in seen) and pre_mark and all(o[2] and o[2][1] == pre_mark[1] for o in obs): v = 'PASS-dropped'
elif c_state == 'absent' and any(s == 'present' for s in seen): v = 'FAIL'
else: v = 'NOTE'
line = f'{row}: D {d} s; B {b_end and b_end.get("how")}; C {c_state}; observations {seen}; marks before {pre_mark}, after {[o[2] for o in obs]}; {v}' + (f' ({"; ".join(void)})' if void else '')
open(W + '/evidence/times-final.txt', 'a', encoding='utf-8', newline='\n').write(line + '\n')
print('VERDICT', line)
