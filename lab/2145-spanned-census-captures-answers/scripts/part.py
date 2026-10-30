# Lab helper (#2145, not product code): one part's reads in the sheet's order, through send.py (one request at a time), then
# the part's marks again and compare2145.py for each window (whole output kept in evidence/compare-<window>.txt).
# The book must be loaded alone first (C1, by the lab on Tally's screen). Stops at the first send.py exit other than 0.
# Usage: python -I part.py A|P|G|S
import os, subprocess, sys
W = '<lab folder>'
G_DAYS = ['03', '04', '05', '06', '07', '08', '10', '11', '12', '13', '14', '15', '16', '17']
G_REC = ['4a11daa46a426a8e37dd13e737cb7635e2b436f63db14295ffa3753c70d6a91c', '6880219726e715cef3b69df2f76c6bb425baef1ab6c6de2c9168c90e263241e3',
         'f14596976cdd6d333d95b7c3d43cad509be920ea1a17a51905a915fd021c2bea', 'd31317eca2f31dfde6f68c17f3728094429d7efe8ed0804f5e68a481ae56b6e1',
         '110ea43ba917d97f3d60badedd397ff8ec3608d3221f67f04dd575f88420e40a', '670bfb45fc5a42e92e0c7565cfb1830b4add211e22154e8db00781dfb48e573b',
         'f472d38bf28d364f2f5e30ee81c60948eea3a4b6ae03c13a7670cdd860702e92', 'a210c0e15803a643f9dec486bb065b9dc559abea1ea494278f15dec700628f03',
         '64e55b940b5389cb39b05e1e722828fd23197bfd4ea22a321b431f5499f0b4b9', '7df652a8bd20772fd5520f6fd9a54ad70fbc6a63d3fd2cfd1330f9f40361f467',
         '538bba68d236530c1cad07a1c87bea3a5464ec31b936b950fa497bf88f8d9e5c', '4757b4a8c4e9f333cf2e1c953ebffce335f2e2f7029431352a63a8add0b1109d',
         'e0bf18db5dd8ccb1a76cb3a3ec656dccc548ec796f6c89d16fdb7a3122741130', '0a1701b562804002b19728fa60ec9f4107423d36ed9d8c5609d6b461038dab6b']
# window: (name, control stem, spanned stem, RECORDED, M)
PARTS = {
    'A': [('a-wa1', 'a-wa1_c1_unspanned', 'a-wa1_c2_spanned_0_1676', '0d010d39400090149aa24d8238f15ce77bda51e35eeb2cf201259b9c93a4c28d', '1676'),
          ('a-d3batch', 'a-d3-0401_c1_unspanned', 'a-d3batch_c2_spanned_0_1469', '285f230b7bb2c77644832b7dc9d68d641dfea375e3a571ba2e4e3a391e9c5b9f', '1469'),
          ('a-d3cancel', 'a-d3-0401_c1_unspanned', 'a-d3cancel_c2_spanned_0_1685', 'f7bfe767de3ce652d99ae202d4d29b044283cee88281f60b47c43b82e47bcb00', '1685'),
          ('a-l1', 'a-l1_c1_unspanned', 'a-l1_c2_spanned_0_1790', 'eaf4b24a00e82083a96efa50e3a5034230d704d2b3c770cdcad67f1138c54a9f', '1790')],
    'P': [('p-audit', 'p-audit_c1_unspanned', 'p-audit_c2_spanned_0_68', '0aab1a3bb6a64ac0643ef3dd6529a470248c2f0fecbb4674a868f32b63dd4f52', '68')],
    'G': [(f'g-{d}nov', f'g-{d}nov_c1_unspanned', f'g-{d}nov_c2_spanned_0_64', rec, '64') for d, rec in zip(G_DAYS, G_REC)],
    'S': [('s-0421', 's-0421_c1_unspanned', 's-0421_c2_spanned_0_3', 'none', '3')]}
part = sys.argv[1]; key = part.lower()
def send(*stems):
    r = subprocess.run([sys.executable, '-I', W + '/send.py', part, *stems])
    if r.returncode != 0: print(f'part.py: send.py exit {r.returncode}: the part stops here'); sys.exit(r.returncode)
GUIDS = {'A': '17a10910-773c-42c6-bd66-7bba9a392536', 'P': '6b43e498-430c-4d5c-bfef-d32e2ab93c85', 'G': 'aae372c5-a894-491c-8843-5532e43d2d2f', 'S': '1edb9b05-8d35-4c0c-9959-dce655730463'}
HIGHEST_M = {'A': 1790, 'P': 68, 'G': 64, 'S': 3}
sys.path.insert(0, W); import importlib.util
def marks_rows(path):
    spec = importlib.util.spec_from_file_location('sendmod_marks', W + '/send.py')
    src = open(W + '/send.py', encoding='utf-8').read(); ns = {}
    exec(src.split('part = sys.argv[1]')[0], ns)  # send.py's definitions only (its marks() reader), nothing sent
    return ns['marks'](path)
send(f'{key}-marks')
import glob
first = marks_rows(max(glob.glob(f'{W}/answers/{part}/{key}-marks*.answer.xml'), key=os.path.getmtime))
print(f'part {part} first marks: {first}')
if len(first) != 1 or not first[0].startswith(GUIDS[part] + ' '):
    print('stop: not exactly one COMPANY row with THE COMPANY\'s GUID'); sys.exit(5)
vch = first[0].split('ALTVCHID ')[1].split()[0]
if not vch.isdigit() or int(vch) < HIGHEST_M[part]:
    print(f'stop: ALTVCHID {vch!r} below the part\'s highest M {HIGHEST_M[part]}'); sys.exit(5)
sent = set()
for name, control, spanned, rec, m in PARTS[part]:
    send(*([control] if control not in sent else []), spanned); sent.add(control)
send(f'{key}-marks')
worst = 0
for name, control, spanned, rec, m in PARTS[part]:
    r = subprocess.run([sys.executable, '-I', W + '/sheet/compare2145.py', rec, m, f'{W}/answers/{part}/{control}.answer.xml', f'{W}/answers/{part}/{spanned}.answer.xml'], capture_output=True, text=True)
    open(f'{W}/evidence/compare-{name}.txt', 'w', encoding='utf-8', newline='\n').write(f'command: compare2145.py {rec} {m} {control} {spanned}\nexit: {r.returncode}\n{r.stdout}{r.stderr}')
    last = [l for l in r.stdout.splitlines() if l.strip()][-6:]
    print(f'{name}: compare exit {r.returncode}; ' + ' | '.join(last)); worst = max(worst, r.returncode)
print(f'part {part} done; worst compare exit {worst}')
