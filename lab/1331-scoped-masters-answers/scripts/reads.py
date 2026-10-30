# Lab runner (#1331 B2/B4, not product code): the sheet's reads at one size, in its order, through send.py, then the
# byte-identity checks against Part A (a difference outside CMPINFO listed by element path) and the B2 gate.
# Usage: python -I -X utf8 reads.py <prefix> <catalogue bound> <expected NUMLEDGERS>
import hashlib, json, re, subprocess, sys
W = '<LAB_DIR>'; E = W + '/evidence'
pre, kbound, nled = sys.argv[1], sys.argv[2], sys.argv[3]
steps = [('m0', 'm-marks.xml', '8192', ''), ('extent', 'e-extent.xml', '8192', ''), ('catalogue', 'k-catalogue-v2.xml', kbound, ''),
         ('groups', 'g-groups.xml', '2000000', ''), ('g1', 'g1-rates-guid-five.xml', '200000', 'capture-after'),
         ('g1-again', 'g1-rates-guid-five.xml', '200000', 'capture-after'), ('n1', 'n1-rates-name-five.xml', '200000', 'capture-after'),
         ('g3', 'g3-rates-guid-absent-only.xml', '200000', 'capture-after'), ('g4', 'g4-master-guid-five.xml', '200000', 'capture-after'),
         ('g5', 'g5-balance-guid-five.xml', '200000', 'capture-after'), ('p2', 'p2-rates-parents-three.xml', '200000', ''), ('m1', 'm-marks.xml', '8192', '')]
secs = {}
for lab, req, bound, cap in steps:
    r = subprocess.run(['python', '-I', '-X', 'utf8', W + '/send.py', f'{pre}-{lab}', req, bound] + ([cap] if cap else []), capture_output=True, text=True)
    print(r.stdout.strip()[:900], flush=True)
    if r.returncode != 0: print(f'runner: stopped at {pre}-{lab}'); sys.exit(2)
    m = re.search(r'printed "200 (\d+) ([0-9.]+)"', r.stdout); secs[lab] = (int(m.group(1)), float(m.group(2)))
def body(l): return open(f'{E}/{l}.xml', 'rb').read()
def nocmp(b): return re.sub(r'<CMPINFO>.*?</CMPINFO>', '', (b[2:] if b[:2] == b'\xff\xfe' else b).decode('utf-16-le'), flags=re.S)
out = []
for lab, a in (('g1', 'A5-g1'), ('g1-again', 'A5-g1'), ('n1', 'A9-n1'), ('g3', 'A7-g3'), ('g4', 'A8-g4'), ('g5', 'A8b-g5'), ('p2', 'A12-p2'), ('groups', 'A3-groups')):
    x, y = body(f'{pre}-{lab}'), body(a)
    out.append(f'{pre}-{lab} against {a}: byte-identical {x == y}' + ('' if x == y else f'; equal outside CMPINFO {nocmp(x) == nocmp(y)}; sizes {len(x)} / {len(y)}'))
ext = open(f'{E}/{pre}-extent.table.txt', encoding='utf-8').read(); n = re.search(r'NUMLEDGERS (\d+)', ext).group(1)
out.append(f'NUMLEDGERS {n} (expected {nled}): {"equal" if n == nled else "DIFFERENT"}')
cat = open(f'{E}/{pre}-catalogue.table.txt', encoding='utf-8').read(); rows = re.search(r'rows per tag (\{.*?\})', cat).group(1)
out.append(f'catalogue rows {rows}')
a2 = {l for l in open(f'{E}/A2-catalogue.table.txt', encoding='utf-8') if l.startswith('LEDGER ')}
cl = {l for l in cat.splitlines(True) if l.startswith('LEDGER ')}
out.append(f'the 18 Part A catalogue rows are all in this catalogue unchanged (name, GUID, parent, bill-wise): {a2 <= cl}')
out.append('bytes and seconds: ' + json.dumps(secs))
gate = secs['catalogue'][0] > 15000000 or secs['catalogue'][1] > 30 or any(s > 120 for _, s in secs.values())
out.append(f'gate (catalogue > 15,000,000 B or > 30 s, or a read > 120 s): {"HELD" if gate else "passed"}')
open(f'{E}/{pre}-checks.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n'); print('\n'.join(out))
