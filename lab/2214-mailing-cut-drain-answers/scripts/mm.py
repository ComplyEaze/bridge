# Lab helper (#2214, not product code): the sheet's named exception 2: ONE curl read of requests/mailing-master.utf16le.xml by
# LAB-COMMON C3's line, with %{time_starttransfer} %{time_total} printed after its -w fields, no client timeout, headers kept.
# 'whole' (D5): no --max-filesize, bound 64 MiB, the body kept locally and its LEDGER rows counted by an XML parser.
# 'cap' (C1): adds --max-filesize 33554432; curl exit 63 is a result. Usage: python -I mm.py <label> whole|cap
import hashlib, os, re, subprocess, sys, time, xml.etree.ElementTree as ET
R = '<LAB_DIR>'; S = R + '/sheet'; os.makedirs(R + '/raw', exist_ok=True)
label, mode = sys.argv[1], sys.argv[2]
def sha(b): return hashlib.sha256(b).hexdigest()
def now(): t = time.time(); return time.strftime('%Y-%m-%dT%H:%M:%S', time.gmtime(t)) + f'.{int(t * 1000) % 1000:03d}Z'
want = {}
for l in open(S + '/SHA256.txt', encoding='utf-8'):
    p = l.split()
    if len(p) >= 3: want[p[-1]] = (p[0], int(p[1]))
req = S + '/requests/mailing-master.utf16le.xml'; rb = open(req, 'rb').read()
def log(s):
    with open(R + '/evidence/raw-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
if (sha(rb), len(rb)) != want['requests/mailing-master.utf16le.xml']: log(f'{label}: stop rule: mailing-master differs from SHA256.txt; nothing sent'); sys.exit(2)
seq = len([f for f in os.listdir(R + '/raw') if f.endswith('.xml')])
ans = f'{R}/raw/{seq:03d}-{label}.xml'; hdr = f'{R}/raw/{seq:03d}-{label}.headers.txt'
cmd = ['curl.exe', '-sS', '-w', '%{http_code} %{size_download} %{time_starttransfer} %{time_total}', '--data-binary', '@' + req, '-H', 'Content-Type: text/xml; charset=utf-16', '-D', hdr, 'http://127.0.0.1:9001', '-o', ans]
if mode == 'cap': cmd[1:1] = ['--max-filesize', '33554432']
t0 = now(); m0 = time.monotonic(); r = subprocess.run(cmd, capture_output=True, text=True); secs = time.monotonic() - m0
size = os.path.getsize(ans) if os.path.exists(ans) else 0
cl = None
if os.path.exists(hdr):
    for hl in open(hdr, encoding='latin-1'):
        if hl.upper().startswith('CONTENT-LENGTH:'): cl = int(hl.split(':', 1)[1])
body_sha = sha(open(ans, 'rb').read()) if size else 'none'
log(f'{label} ({mode}) mailing-master ({len(rb)} B, sha256 {sha(rb)}) sent {t0} ended {now()} {secs:.3f} s curl_exit {r.returncode} printed "{r.stdout.strip()}" CONTENT-LENGTH {cl} bytes {size} sha256 {body_sha}' + (f' stderr "{r.stderr.strip()}"' if r.stderr.strip() else ''))
if mode == 'whole' and r.returncode == 0 and size and size <= 64 * 1024 * 1024 and cl == size:
    t = open(ans, 'rb').read(); t = (t[2:] if t[:2] == b'\xff\xfe' else t).decode('utf-16-le')
    n4 = t.count('&#4;'); root = ET.fromstring(t.replace('&#4;', ''))
    rows = root.findall('.//LEDGER')
    log(f'{label}: LEDGER rows {len(rows)} (any depth); direct children of COLLECTION {len(root.findall("./BODY/DATA/COLLECTION/LEDGER"))}; &#4; {n4}')
