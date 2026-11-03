# Lab helper (#2176 N2, not product code): the repeat's seed by the same builder and count, indexes past N2b's. Then rereads the book's narrations for that range. Writes evidence/seeds.txt and evidence/ranges-2.txt.
import hashlib, importlib.util, re, subprocess, sys
W = '<lab folder>/'; E = '<earlier lab folder>/earlier/lab/1713-abandoned-ledger-create'
sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location('gen', E + '/gen.py'); gen = importlib.util.module_from_spec(spec); spec.loader.exec_module(gen)
row, start = 'N2b-2', 113200
body = gen.envelope('Vouchers', ''.join(gen.journal(i) for i in range(start, start + 2700)))
raw = gen.utf16(body); open(f'{W}seed/{row}.xml', 'wb').write(raw)
t = f'{row} start {start} bytes {len(raw)} vouchers {body.count("<VOUCHER ")} sha256 {hashlib.sha256(raw).hexdigest()}'
print(t); open(W + 'evidence/seeds.txt', 'a', encoding='utf-8', newline='\n').write(t + '\n')
req = W + 'evidence/ranges-request.xml'; ans = W + 'evidence/ranges-answer-2.xml'
r = subprocess.run(['curl.exe', '-sS', '--max-time', '300', '-w', '%{http_code} %{size_download} %{time_total}', '--data-binary', '@' + req, '-H', 'Content-Type: text/xml; charset=utf-16', 'http://127.0.0.1:9001', '-o', ans], capture_output=True, text=True)
s = open(ans, 'rb').read().decode('utf-16-le', 'replace')
ids = sorted(int(x) for x in re.findall(r'Lab bulk (\d+)', s))
line = f'printed {r.stdout} indexes {len(ids)} distinct {len(set(ids))} highest {ids[-1]} control 107800 to 113199 found {sum(1 for i in ids if 107800 <= i <= 113199)} inside {start} to {start + 2699}: {sum(1 for i in ids if start <= i < start + 2700)}'
print(line); open(W + 'evidence/ranges-2.txt', 'w', encoding='utf-8', newline='\n').write(line + '\n')
