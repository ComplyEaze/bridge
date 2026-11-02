# Lab helper (#626 R2, not product code): writes this row's requests from the earlier 626-T shapes, with the company, names, dates and
# amounts changed. Each is UTF-16LE with a byte-order mark. The line break is the reference &#13;&#10;, never a literal CR or LF.
import hashlib, os, re, uuid
L = '<lab folder>'; T = L + '/templates/'; R = L + '/requests/'
os.makedirs(R, exist_ok=True)
CO = 'BRIDGE TWIN R2 LAB'; OLD = 'BRIDGE LINE BREAK LAB'; BRK = '&#13;&#10;'
def load(n): return open(T + n + '.utf16le.xml', 'rb').read().decode('utf-16')
def save(n, t):
    b = b'\xff\xfe' + t.encode('utf-16-le'); open(R + n + '.utf16le.xml', 'wb').write(b)
    print(n, len(b), hashlib.sha256(b).hexdigest()[:16])
for n in ('marks', 'ledgers', 'vch-aug'):
    t = load(n); assert t.count(OLD) == 1; save(n, t.replace(OLD, CO))
led = load('led1'); assert led.count('Lab Line Break Ledger') == 2 and led.count(OLD) == 1
imp = load('imp0'); assert imp.count('Lab Plain Ledger') == 2 and imp.count(OLD) == 1
def create(n, name): save(n, led.replace(OLD, CO).replace('Lab Line Break Ledger', name))
def payment(n, name, day, amount, rid):
    t = imp.replace(OLD, CO).replace('Lab Plain Ledger', name).replace('20260804', f'202608{day:02d}').replace('626-T control', f'626 R2 {n}')
    t = t.replace('-610.00', f'-{amount}.00').replace('>610.00', f'>{amount}.00')
    t = re.sub(r'REMOTEID="[^"]*"', f'REMOTEID="{rid}"', t)
    save(n, t)
create('led-a1', 'Twin A'); create('led-a2', 'Twin A' + BRK)
create('led-b1', 'Twin B' + BRK); create('led-b2', 'Twin B')
rid = '626f2000-0000-4000-8000-0000000000{:02x}'
payment('imp-a-plain', 'Twin A', 4, 711, rid.format(1)); payment('imp-a-break', 'Twin A' + BRK, 5, 712, rid.format(2))
payment('imp-b-break', 'Twin B' + BRK, 6, 713, rid.format(3)); payment('imp-b-plain', 'Twin B', 7, 714, rid.format(4))
