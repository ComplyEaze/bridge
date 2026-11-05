# Lab helper (#2265 R4 on THE CLIENT BOOK, not product code): one raw read on 9001 by C3's curl line, the answer kept whole in
# raw/<label>.xml (lab PC only, never printed), and only shapes and counts printed: bytes, seconds, HTTP status, and per mode
# marks: COMPANY rows and ALTVCHID / ALTMSTID; count: the number of a tag's objects (nodes with attributes or children);
# extent: BOOKSFROM, LASTVOUCHERDATE, NUMLEDGERS, NUMGROUPS, NUMVOUCHERTYPES and the marks. Nothing else of the answer is
# printed or logged. An existing answer is never resent. Usage: python -I -X utf8 rd.py <label> <request> <mode> [tag]
import hashlib, os, re, subprocess, sys, time, xml.etree.ElementTree as ET
from datetime import datetime, timezone
R = '<LAB_DIR>/'; label, req, mode = sys.argv[1], sys.argv[2], sys.argv[3]; tag = sys.argv[4] if len(sys.argv) > 4 and not sys.argv[4].startswith('--') else None
ans = R + f'raw/{label}.xml'
reparse = '--reparse' in sys.argv
if os.path.exists(ans) and not reparse: sys.exit(f'stop: {label} exists; never resent')
if reparse and not os.path.exists(ans): sys.exit('stop: nothing to reparse')
rq = open(R + 'reqs/' + req, 'rb').read(); t0 = datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
r = subprocess.CompletedProcess([], 0, 'reparse - -', '') if reparse else subprocess.run(['curl.exe', '-sS', '--max-time', '900', '-w', '%{http_code} %{size_download} %{time_total}', '--data-binary', '@' + R + 'reqs/' + req,
                    '-H', 'Content-Type: text/xml; charset=utf-16', 'http://127.0.0.1:9001', '-o', ans], capture_output=True, text=True)
st = r.stdout.split(); b = open(ans, 'rb').read() if os.path.exists(ans) else b''
line = f'{t0} {label}: request {req} sha256 {hashlib.sha256(rq).hexdigest()[:16]} curl exit {r.returncode} status {st[0] if st else "-"} answer {len(b)} B seconds {st[2] if len(st) > 2 else "-"}'
def parse():
    t = (b[2:] if b[:2] == b'\xff\xfe' else b).decode('utf-16-le', 'replace')
    t = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2\d|3[01]));', '', t, flags=re.I)
    t = re.sub(r'<(/?)([A-Za-z_][\w.-]*):', r'<\1\2_', t)  # tags with a namespace prefix (e.g. user-defined fields) parse as plain names
    return ET.fromstring(t)
try:
    root = parse(); objs = lambda n: [v for v in root.iter(n) if v.attrib or len(v)]
    if mode == 'marks':
        cs = objs('COMPANY'); line += f'; COMPANY rows {len(cs)}; ' + '; '.join(f'V {(c.findtext("ALTVCHID") or "").strip() or "blank"} M {(c.findtext("ALTMSTID") or "").strip()}' for c in cs)
    elif mode == 'count': line += f'; {tag} objects {len(objs(tag))}'
    elif mode == 'extent':
        cs = objs('COMPANY'); line += f'; COMPANY rows {len(cs)}; ' + '; '.join(' '.join(f'{k} {(c.findtext(k) or "").strip()}' for k in ('BOOKSFROM', 'LASTVOUCHERDATE', 'NUMLEDGERS', 'NUMGROUPS', 'NUMVOUCHERTYPES', 'ALTVCHID', 'ALTMSTID')) for c in cs)
except Exception as e: line += f'; NOT READ ({type(e).__name__})'
open(R + 'evidence/read-log.txt', 'a', encoding='utf-8', newline='\n').write(line + '\n'); print(line)
