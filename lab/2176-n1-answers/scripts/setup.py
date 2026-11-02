# Lab helper (#2176 N1, not product code): prepares the row's folder: copies of the K rows' helpers with the working folder changed and
# row N1's step A as the seeded import; seeds two import files of 1200 Journals from the earlier sheet's own builder, with
# REMOTEIDs and narrations past the 48000 in the book. Prints sizes and sha256.
import hashlib, importlib.util, os, re, sys
O = '<earlier lab folder>'; W = '<lab folder>'; E = O + '/earlier/lab/1713-abandoned-ledger-create'
for d in ('plans', 'runs', 'evidence', 'answers', 'shots', 'seed'):
    os.makedirs(W + '/' + d, exist_ok=True)
for n in ('c3.py', 'capwin.ps1', 'watch.ps1', 'step0.ps1'):
    b = open(f'{O}/{n}', 'rb').read()
    for a, c in ((b'<earlier lab folder>', b'<lab folder>'), (b'<earlier lab folder>', b'<lab folder>')):
        b = b.replace(a, c)
    open(f'{W}/{n}', 'wb').write(b)
s = open(O + '/rows.py', encoding='utf-8').read()
s = s.replace("W = '<earlier lab folder>'; E = W + '/earlier/lab/1713-abandoned-ledger-create'", f"W = '<lab folder>'; E = '{E}'")
s = s.replace("ORDER = W + '/sheet/lab/1713-settle-order/order.py'", "ORDER = W + '/order.py'")
s = s.replace("VARIANT = {'K0'", "VARIANT = {'N1': ('new', 'new', 20), 'K0'")
old = "steps = [{'name': 'A', 'at': 1.0, 'conn': 'new', 'file': R + '/busy-year.xml', 'sha256': sha(R + '/busy-year.xml'), 'limit': None, 'write': False, 'max_bytes': 400000000},"
assert old in s
s = s.replace(old, "seed = f'{W}/seed/{row}.xml'\n    steps = [{'name': 'A', 'at': 1.0, 'conn': 'new', 'file': seed, 'sha256': sha(seed), 'limit': None, 'write': True, 'max_bytes': 2000000},")
old = "code, text = readpy('busy', out + '/A.answer', GUID, '48000'); log(row, f'A read: exit {code}: {text}')"
assert old in s
s = s.replace(old, "code, text = readpy('import', out + '/A.answer', '1200'); log(row, f'A read: exit {code}: {text}')")
s = re.sub(r"^# Lab helper \(#1713 order rows, not product code\): runs the sheet's timed rows", "# Lab helper (#2176 N1, not product code; the K rows' runner with A as a seeded import): runs the sheet's timed rows", s, count=1, flags=re.M)
open(W + '/rows_n1.py', 'w', encoding='utf-8', newline='\n').write(s)
spec = importlib.util.spec_from_file_location('gen', E + '/gen.py'); gen = importlib.util.module_from_spec(spec)
sys.dont_write_bytecode = True; spec.loader.exec_module(gen)
for row, start in (('N1a', 100000), ('N1b', 101200)):
    body = gen.envelope('Vouchers', ''.join(gen.journal(i) for i in range(start, start + 1200)))
    raw = gen.utf16(body)
    open(f'{W}/seed/{row}.xml', 'wb').write(raw)
    print(row, 'seed', len(raw), hashlib.sha256(raw).hexdigest()[:16], 'first remote', re.search(r'REMOTEID="([^"]+)"', body).group(1)[:50], 'first date', re.search(r'<DATE>(\d+)', body).group(1), 'dates', len(set(re.findall(r'<DATE>(\d+)', body))))
print('order.py sha', hashlib.sha256(open(W + '/order.py', 'rb').read()).hexdigest()[:16])
