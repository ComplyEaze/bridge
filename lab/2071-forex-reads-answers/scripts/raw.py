# Lab helper (#2071 forex reads, not product code): ONE raw read by the sheet's curl line, run from sheet\. The request's
# sha256 is checked against SHA256.txt first (a difference = stop, nothing sent). Logs the ISO time, curl's exit code and
# -w line to ..\evidence\log.txt, then bytes, sha256 and CONTENT-LENGTH agreement. A non-zero exit or non-200 = stop.
# With a bound (the matching tool answer's evidence.bytes), an answer larger than it = stop. For marks: the COMPANY row's
# NAME, GUID, ALTMSTID and ALTVCHID. With a previous answer file: whether this answer is byte-identical to it.
# Usage: python -I -X utf8 raw.py <label> <request name> [bound|-] [previous answer path]
import hashlib, os, subprocess, sys, xml.etree.ElementTree as ET
from datetime import datetime, timezone
W = '<LAB_DIR>'; S = W + '/sheet'
label, name = sys.argv[1], sys.argv[2]
bound = int(sys.argv[3]) if len(sys.argv) > 3 and sys.argv[3] != '-' else None
prev = sys.argv[4] if len(sys.argv) > 4 else None
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
def sha(b): return hashlib.sha256(b).hexdigest()
def log(s):
    with open(W + '/evidence/log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
def stop(m): log('stop: ' + m); sys.exit(2)
rel = f'requests/{name}.utf16le.xml'
want = {l.split()[1]: l.split()[0] for l in open(S + '/SHA256.txt', encoding='utf-8') if len(l.split()) >= 2}
rb = open(S + '/' + rel, 'rb').read()
if want.get(rel) != sha(rb): stop(f'{label}: {rel} differs from SHA256.txt; nothing sent')
resp, hdr = f'{W}/evidence/{label}.response', f'{W}/evidence/{label}.headers'
if os.path.exists(resp): stop(f'{label}: {resp} exists; never resent')
cmd = ['curl.exe', '-sS', '--data-binary', f'@requests\\{name}.utf16le.xml', '-H', 'Content-Type: text/xml; charset=utf-16',
       '-o', f'..\\evidence\\{label}.response', '-D', f'..\\evidence\\{label}.headers', '-w', '%{http_code} %{size_download}\\n', 'http://localhost:9001/']
t0 = now()
r = subprocess.run(cmd, cwd=S, capture_output=True, text=True)
body = open(resp, 'rb').read() if os.path.exists(resp) else b''
cl = None
if os.path.exists(hdr):
    for hl in open(hdr, encoding='latin-1'):
        if hl.upper().startswith('CONTENT-LENGTH:'): cl = int(hl.split(':', 1)[1])
line = (f'{t0} {label} ({rel}, request sha256 {sha(rb)}): curl exit {r.returncode}; -w {r.stdout.strip()!r}; bytes {len(body)}; '
        f'sha256 {sha(body) if body else "none"}; CONTENT-LENGTH {cl}; equal {cl == len(body)}')
if prev:
    pb = open(prev, 'rb').read()
    line += f'; byte-identical to the earlier sitting\'s {os.path.basename(prev)} ({len(pb)} B, sha256 {sha(pb)}): {pb == body}'
if bound is not None: line += f'; tool evidence.bytes bound {bound}'
if r.stderr.strip(): line += f'; stderr {r.stderr.strip()!r}'
log(line)
if r.returncode != 0 or not r.stdout.startswith('200 '): stop(f'{label}: transport failure')
if bound is not None and len(body) > bound: stop(f'{label}: {len(body)} B is larger than the tool answer\'s {bound} B')
if name == 'marks':
    t = (body[2:] if body[:2] == b'\xff\xfe' else body).decode('utf-16-le').replace('&#4;', '')
    for c in ET.fromstring(t).iter('COMPANY'):
        if c.find('GUID') is None and c.get('NAME') is None: continue
        f = {k: (c.findtext(k) or '').strip() if c.find(k) is not None else 'absent' for k in ('GUID', 'ALTMSTID', 'ALTVCHID')}
        log(f'  {label}: COMPANY NAME="{c.get("NAME")}" ' + ' '.join(f'{k} {v}' for k, v in f.items()))
