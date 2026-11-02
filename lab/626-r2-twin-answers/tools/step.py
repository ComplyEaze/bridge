# Lab helper (#626 R2, not product code): one step: an optional write sent once by C3's curl line, then marks, ledgers and vouchers. Answers are
# saved whole. Names are read from the XML structure, with CR protected so the parser cannot normalise it, and printed with UTF-16 and
# UTF-8 hex. Usage: python -I -X utf8 step.py <step> [<write request stem>]
import hashlib, os, re, subprocess, sys, xml.etree.ElementTree as ET, datetime
L = '<lab folder>'; A = L + '/answers/'; R = L + '/requests/'
os.makedirs(A, exist_ok=True)
step = sys.argv[1]; write = sys.argv[2] if len(sys.argv) > 2 else None
def send(stem, tag):
    req = f'{R}{stem}.utf16le.xml'; ans = f'{A}{step}-{tag}.xml'; hdr = f'{A}{step}-{tag}.headers.txt'
    if os.path.exists(ans): sys.exit(f'stop: {ans} exists; never resent')
    t0 = datetime.datetime.now(datetime.timezone.utc).isoformat(timespec='milliseconds')
    r = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download}', '--data-binary', '@' + req, '-H', 'Content-Type: text/xml; charset=utf-16', '-D', hdr, 'http://127.0.0.1:9001', '-o', ans], capture_output=True, text=True)
    sha = hashlib.sha256(open(req, 'rb').read()).hexdigest()
    with open(L + '/send-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(f'{t0} {step}-{tag} {stem} req_sha256 {sha} exit {r.returncode} printed {r.stdout}\n')
    if r.returncode or not r.stdout.startswith('200 '): sys.exit(f'stop: {stem} transport {r.returncode} {r.stdout} {r.stderr}')
    return ans
def root(path):
    t = open(path, 'rb').read().decode('utf-16-le').lstrip('\ufeff')
    t = re.sub(r'&#4;|\x03', '', t).strip().replace('\r', '&#13;')
    return ET.fromstring(t)
def hx(s): return s.encode('utf-16-le').hex() if s is not None else None
def show(s):
    return None if s is None else f'{s!r} len {len(s)} utf16 {hx(s)} utf8 {s.encode("utf-8").hex()}'
if write:
    a = send(write, 'write'); r = root(a)
    print('WRITE', write, {k: (r.findtext('.//' + k) or '').strip() for k in ('CREATED', 'ALTERED', 'DELETED', 'COMBINED', 'IGNORED', 'ERRORS', 'CANCELLED', 'LASTVCHID', 'LASTMID')}, 'LINEERROR', [(e.text or '')[:160] for e in r.iter('LINEERROR')])
m = root(send('marks', 'marks'))
rows = [c for c in m.iter('COMPANY') if c.attrib or len(c)]
print('MARKS rows', len(rows), {k: (rows[0].findtext(k) or '').strip() for k in ('GUID', 'ALTVCHID', 'ALTMSTID')} if rows else None)
led = root(send('ledgers', 'ledgers'))
print('LEDGERS')
for row in [x for x in led.iter('LEDGER') if x.attrib]:
    n = row.get('NAME')
    names = [e.text for e in row.iter('NAME')]
    if not (n or '').startswith(('Twin', 'TWIN', 'twin')): print('  (other)', repr(n)); continue
    print('  attr', show(n)); print('     elements', [show(x) for x in names]); print('     GUID', (row.findtext('GUID') or '').strip(), 'PARENT', (row.findtext('PARENT') or '').strip(), 'ALTERID', (row.findtext('ALTERID') or '').strip())
v = root(send('vch-aug', 'vch'))
print('VOUCHERS')
for row in [x for x in v.iter('VOUCHER') if x.attrib or len(x)]:
    print('  ', {k: (row.findtext(k) or '').strip() for k in ('DATE', 'VOUCHERTYPENAME', 'VOUCHERNUMBER', 'NARRATION', 'ALTERID')}, 'party', show(row.findtext('PARTYLEDGERNAME')))
    for le in row.findall('.//ALLLEDGERENTRIES.LIST'):
        print('     entry', show(le.findtext('LEDGERNAME')), 'amount', (le.findtext('AMOUNT') or '').strip(), 'deemed', (le.findtext('ISDEEMEDPOSITIVE') or '').strip())
