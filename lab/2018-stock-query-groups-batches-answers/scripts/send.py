# Lab helper (#2018 L1-L3, not product code; the 2145 row's sender with this sheet's names and bounds): sends the sheet's request files one at a time by LAB-COMMON C3's curl line (the file's
# sha256 checked against SHA256.txt first; the sheet's size bound as --max-filesize; 600 s to answer), writes the answer and
# its headers under answers/<part>/, then GET /status with 60 s to answer (none = the gateway is held: stop, nothing more sent).
# A marks answer is read by structure (C4): exactly one COMPANY row, its GUID, ALTVCHID and ALTMSTID trimmed.
# Usage: python -I send.py <part> <request stem> [<request stem> ...]   (stems as in requests/, without .request.utf16le.xml)
import hashlib, os, re, subprocess, sys, xml.etree.ElementTree as ET
from datetime import datetime, timezone
W = '<lab folder>'; S = W + '/sheet'
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
def sha(p): return hashlib.sha256(open(p, 'rb').read()).hexdigest()
def log(s):
    with open(W + '/evidence/send-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
want = {}
for l in open(S + '/SHA256.txt', encoding='utf-8'):
    p = l.split()
    if len(p) >= 2 and not l.startswith('#'): want[p[-1]] = p[0]
def marks(path):
    t = open(path, 'rb').read().decode('utf-16-le').lstrip('\ufeff')
    t = re.sub(r'&#(x[0-9A-Fa-f]+|[0-9]+);', lambda m: '' if (int(m.group(1)[1:], 16) if m.group(1)[0] == 'x' else int(m.group(1))) in set(range(0, 9)) | {11, 12} | set(range(14, 32)) else m.group(0), t)
    t = re.sub('[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t).replace('<ENVELOPE', '<ENVELOPE xmlns:UDF="urn:x-tally-udf"', 1)
    coll = ET.fromstring(t).find('./BODY/DATA/COLLECTION')
    rows = [] if coll is None else [r for r in coll if r.tag == 'COMPANY']  # rows are the COLLECTION's children (CMPINFO holds an empty COMPANY)
    return [(r.findtext('GUID') or '').strip() + ' ALTVCHID ' + (r.findtext('ALTVCHID') or '').strip() + ' ALTMSTID ' + (r.findtext('ALTMSTID') or '').strip() for r in rows]
part = sys.argv[1]; os.makedirs(f'{W}/answers/{part}', exist_ok=True)
for stem in sys.argv[2:]:
    rel = f'requests/{stem}.utf16le.xml'; req = S + '/' + rel
    if sha(req) != want.get(rel): log(f'stop: {rel} sha256 differs from SHA256.txt; nothing sent'); sys.exit(3)
    bound = '8192' if stem.startswith('m-') else ('262144' if stem.startswith('c-') else '65536')
    tag = stem
    n = 1
    while os.path.exists(f'{W}/answers/{part}/{tag}.answer.xml'): n += 1; tag = f'{stem}-{n}'
    ans = f'{W}/answers/{part}/{tag}.answer.xml'; hdr = f'{W}/answers/{part}/{tag}.headers.txt'
    t0 = now()
    try:
        r = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download} %{time_total}', '--max-filesize', bound, '--data-binary', '@' + req,
                            '-H', 'Content-Type: text/xml; charset=utf-16', '-D', hdr, 'http://127.0.0.1:9001', '-o', ans], capture_output=True, text=True, timeout=600)
    except subprocess.TimeoutExpired:
        log(f'stop: {tag} no answer in 600 s (sent {t0}): the gateway is held; nothing more sent'); sys.exit(7)
    cl = ''.join(re.findall(r'CONTENT-LENGTH:\s*(\d+)', open(hdr, encoding='latin-1').read(), re.I)) if os.path.exists(hdr) else ''
    size = os.path.getsize(ans) if os.path.exists(ans) else -1
    log(f'{part} {tag} ({rel}, request sha256 {want[rel]}) bound {bound} sent {t0} answered {now()} curl_exit {r.returncode} printed "{r.stdout}" content_length {cl} bytes {size} sha256 {sha(ans) if size >= 0 else "none"}' + (f' stderr "{r.stderr.strip()}"' if r.stderr.strip() else ''))
    s0 = now()
    try:
        st = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download}', '-o', f'{W}/answers/{part}/{tag}-status.body', 'http://127.0.0.1:9001/status'], capture_output=True, text=True, timeout=60)
        log(f'{part} {tag} GET /status sent {s0} answered {now()} exit {st.returncode} printed {st.stdout}')
    except subprocess.TimeoutExpired:
        log(f'stop: GET /status after {tag} had no answer in 60 s: the gateway is held'); sys.exit(7)
    if r.returncode != 0 or not r.stdout.startswith('200 ') or str(size) != cl or size <= 0: log(f'stop: {tag} failed (C3)'); sys.exit(2)
    if stem.startswith('m-'): log(f'{part} {tag} COMPANY rows: {marks(ans)}')
