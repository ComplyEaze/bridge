# Lab helper (#2176 N1, not product code): seeds of 2700 Journals for N1a-2 and N1b from the earlier sheet's builder, with REMOTEIDs past
# every Journal already in the book; the runner's expected count moved to 2700; the local watcher baseline after focus settles, and the
# top 130 rows of the window left out of the comparison. The 1713 kit's own watch.ps1 is not touched.
import hashlib, importlib.util, re, sys
W = '<lab folder>'; E = '<earlier lab folder>/earlier/lab/1713-abandoned-ledger-create'
sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location('gen', E + '/gen.py'); gen = importlib.util.module_from_spec(spec); spec.loader.exec_module(gen)
for row, start in (('N1a-2', 102400), ('N1b', 105100)):
    body = gen.envelope('Vouchers', ''.join(gen.journal(i) for i in range(start, start + 2700)))
    raw = gen.utf16(body)
    open(f'{W}/seed/{row}.xml', 'wb').write(raw)
    print(row, len(raw), hashlib.sha256(raw).hexdigest()[:16], body.count('<VOUCHER '))
s = open(W + '/rows_n1.py', encoding='utf-8').read()
a = "code, text = readpy('import', out + '/A.answer', '1200')"
assert s.count(a) == 1
open(W + '/rows_n1.py', 'w', encoding='utf-8', newline='\n').write(s.replace(a, "code, text = readpy('import', out + '/A.answer', '2700')"))
w = open(W + '/watch.ps1', 'rb').read().decode('utf-8-sig')
a = '$baseImage = Grab;'
assert w.count(a) == 1
w = w.replace(a, 'Start-Sleep -Seconds 4; $baseImage = Grab;')
a = 'for ($gy = 4; $gy -lt'
assert w.count(a) == 1
w = w.replace(a, 'for ($gy = 132; $gy -lt')
open(W + '/watch.ps1', 'w', encoding='utf-8', newline='\r\n').write(w)
print('watch sha', hashlib.sha256(open(W + '/watch.ps1', 'rb').read()).hexdigest()[:16], 'kit sha', hashlib.sha256(open('<earlier lab folder>/watch.ps1', 'rb').read()).hexdigest()[:16])
