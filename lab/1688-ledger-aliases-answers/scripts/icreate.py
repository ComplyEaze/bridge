# Lab runner (#1688 Part I, not product code): ONE create of the sheet. MARKS first (rd.py: exactly THE COMPANY, ALTMSTID
# equal to the last recorded), then the create (rd.py: sha256 against SHA256.txt, sent once), a capture of the whole Tally
# window, then THE READS with the ALTMSTID the answer implies (CREATED 1 or ALTERED 1: +1; ERRORS 1: unchanged), and every
# IDX row whose ALTERID, names, parent or ISBILLWISEON differ from the IDX before. Any other answer is an I-HALT (exit 3);
# ALTERED 1 pauses the row (exit 4) after the reads. Usage: python -I -X utf8 icreate.py <step, e.g. I0> <file> <ALTMSTID> <IDX before label>
import json, re, subprocess, sys
W = '<LAB_DIR>'; E = W + '/evidence'
step, fn, mst, idx_before = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
def run(args):
    r = subprocess.run(['python', '-I', '-X', 'utf8'] + args, capture_output=True, text=True)
    print(r.stdout.strip()[:1500], flush=True); return r
if run([W + '/rd.py', f'{step}-pre-marks', 'requests/marks.utf16le.xml', mst]).returncode != 0: print('runner: stopped at the MARKS before'); sys.exit(2)
r = run([W + '/rd.py', step, 'requests/creates/' + fn, mst])
if r.returncode != 0 and 'counters' not in r.stdout: print('runner: the create was not sent or failed'); sys.exit(2)
print(subprocess.run(['python', '-I', W + '/cap.py', f'{step}-after'], capture_output=True, text=True).stdout.strip())
cnt = json.loads(re.search(r'counters (\{.*?\})', open(f'{E}/{step}.table.txt', encoding='utf-8').read()).group(1))
le = re.search(r'LINEERROR (\[.*?\])', open(f'{E}/{step}.table.txt', encoding='utf-8').read()).group(1)
zero = lambda ks: all(cnt[k] == '0' for k in ('CREATED', 'ALTERED', 'DELETED', 'COMBINED', 'IGNORED', 'ERRORS', 'CANCELLED', 'EXCEPTIONS') if k not in ks)
if cnt['CREATED'] == '1' and zero(('CREATED',)): kind, after = 'CREATED', str(int(mst) + 1)
elif cnt['ALTERED'] == '1' and zero(('ALTERED',)): kind, after = 'ALTERED', str(int(mst) + 1)
elif cnt['ERRORS'] == '1' and zero(('ERRORS',)) and le != '[]': kind, after = 'ERRORS', mst
else: kind, after = 'I-HALT', mst
print(f'{step}: answer {kind}; counters {cnt}; LINEERROR {le}; THE READS expect ALTMSTID {after}')
if run([W + '/reads4.py', step + '-reads', after]).returncode != 0: print('runner: stopped in THE READS (a mark moved otherwise than the sheet says)'); sys.exit(2)
def rows(label):
    out, cur = {}, None
    for l in open(f'{E}/{label}.table.txt', encoding='utf-8'):
        if l.startswith('LEDGER'):
            g = re.search(r'GUID (\S+)', l).group(1); cur = g; out[g] = [l.strip()]
        elif l.startswith('    name') and cur: out[cur].append(l.strip())
    return out
a, b = rows(idx_before), rows(step + '-reads-idx')
changes = []
for g in sorted(set(a) | set(b)):
    if a.get(g) != b.get(g): changes.append(f'  {g}:\n    before {a.get(g)}\n    after  {b.get(g)}')
text = f'{step} IDX rows changed against {idx_before}:\n' + ('\n'.join(changes) if changes else '  none')
open(f'{E}/{step}-idx-changes.txt', 'w', encoding='utf-8', newline='\n').write(text + '\n'); print(text)
if kind == 'I-HALT': sys.exit(3)
if kind == 'ALTERED': sys.exit(4)
