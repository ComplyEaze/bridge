# Lab helper (#2176 N2, not product code): the row's working copies from N1's, with the folder changed and the one line ROW.md adds; the seeds by the earlier sheet's builder, 2,700 Journals each. order.py copied unchanged, its sha256 checked.
import hashlib, importlib.util, shutil, sys
W = '<lab folder>'; N = '<N1 folder>'; O = '<earlier lab folder>'; E = O + '/earlier/lab/1713-abandoned-ledger-create'
sys.dont_write_bytecode = True
def norm(b): return b.replace(b'\r\n', b'\n')
sane = lambda s: s.replace('<N1 folder>', '<lab folder>').replace('<earlier lab folder>', '<earlier lab folder>').replace('stop', 'stop')
k = open(W + '/kit/rows_n1.py', 'rb').read(); l = open(N + '/rows_n1.py', 'rb').read().decode('utf-8')
print('local runner equals the pushed one, line ends and the listed words aside:', norm(sane(l).encode()) == norm(k))
s = l
line = "VARIANT = {'N1': ('new', 'new', 20), 'K0': ('new', 'new', 20), 'K1': ('K1', 'new', 20), 'K2': ('new', 'K2', 20), 'K3': ('K1', 'K2', 20), 'L1B': ('new', 'new', None)}\n"
if line not in s: line = line.replace('\n', '\r\n')
assert s.count(line) == 1
s = s.replace(line, line + "VARIANT['N2'] = ('K1', 'new', 20)" + line[len(line.rstrip()):]).replace('<N1 folder>', W)
open(W + '/rows_n2.py', 'wb').write(s.encode('utf-8'))
for f in ('verdict.py', 'c3.py', 'watch.ps1', 'step0.ps1', 'capwin.ps1'):
    b = open(f'{N}/{f}', 'rb').read()
    b = b.replace(b'<N1 folder>', W.encode()).replace(b'<N1 folder>', W.replace('/', '\\').encode())
    open(f'{W}/{f}', 'wb').write(b)
shutil.copyfile(N + '/order.py', W + '/order.py')
print('order.py', hashlib.sha256(open(W + '/order.py', 'rb').read()).hexdigest())
spec = importlib.util.spec_from_file_location('gen', E + '/gen.py'); gen = importlib.util.module_from_spec(spec); spec.loader.exec_module(gen)
with open(W + '/evidence/seeds.txt', 'w', encoding='utf-8', newline='\n') as f:
    for row, start in (('N2a', 107800), ('N2b', 110500)):
        body = gen.envelope('Vouchers', ''.join(gen.journal(i) for i in range(start, start + 2700)))
        raw = gen.utf16(body); open(f'{W}/seed/{row}.xml', 'wb').write(raw)
        t = f'{row} start {start} bytes {len(raw)} vouchers {body.count("<VOUCHER ")} sha256 {hashlib.sha256(raw).hexdigest()}'
        print(t); f.write(t + '\n')
