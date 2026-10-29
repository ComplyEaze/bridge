# Lab helper (#690 L0b, not product code): sends one of the sheet's request files exactly as it is (sha256 checked against
# SHA256.txt first) with curl, writes the answer to answers/<seq>-<name>.xml with --output, logs the UTC times, size and sha256,
# and reads the answer with an XML parser (only the characters XML forbids are dropped before parsing; the UDF prefix is declared).
# Usage: python -I l0.py <step-label> <request name: marks|ledgers|groups|create-10>
import hashlib, json, os, re, subprocess, sys, time
import xml.etree.ElementTree as ET
from datetime import datetime, timezone
R = '<lab folder>'; S = R + '/sheet/lab/690-batch-ledger-create-l0b'
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
def sha(b): return hashlib.sha256(b).hexdigest()
want = {l.split()[1]: l.split()[0] for l in open(S + '/SHA256.txt', encoding='utf-8') if l.strip()}
label, name = sys.argv[1], sys.argv[2]
rel = f'requests/{name}.utf16le.xml'; req = open(S + '/' + rel, 'rb').read()
if sha(req) != want[rel]:
    print('stop: request sha256 differs'); sys.exit(3)
seq = '{:02d}'.format(len(os.listdir(R + '/answers')))
out = f'{R}/answers/{seq}-{label}-{name}.xml'
t0 = now()
p = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download} %{time_total}', '--data-binary', '@' + S + '/' + rel,
                    '-H', 'Content-Type: text/xml; charset=utf-16', 'http://127.0.0.1:9001', '--output', out], capture_output=True, text=True)
t1 = now(); body = open(out, 'rb').read()
line = f'{label} {name} request {len(req)} B sent {t0} answered {t1} curl_exit {p.returncode} printed "{p.stdout}" answer {os.path.basename(out)} {len(body)} B sha256 {sha(body)}'
with open(R + '/evidence/log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(line + '\n')
print(line)
t = body.decode('utf-16-le') if body[:2] == b'\xff\xfe' or (len(body) > 1 and body[1] == 0) else body.decode('utf-8')
t = t.lstrip('\ufeff')
t = re.sub(r'&#(x[0-9A-Fa-f]+|[0-9]+);', lambda m: '' if (int(m.group(1)[1:], 16) if m.group(1)[0] == 'x' else int(m.group(1))) in set(range(0, 9)) | {11, 12} | set(range(14, 32)) else m.group(0), t)
t = re.sub('[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t).replace('<ENVELOPE', '<ENVELOPE xmlns:UDF="urn:x-tally-udf"', 1)
root = ET.fromstring(t)
summary = {}
if name.startswith('create'):
    for k in ('CREATED', 'ALTERED', 'DELETED', 'COMBINED', 'IGNORED', 'ERRORS', 'CANCELLED', 'EXCEPTIONS', 'LASTMID', 'LASTVCHID'):
        el = root.find('.//' + k); summary[k] = el.text.strip() if el is not None and el.text else None
    summary['LINEERROR'] = [e.text for e in root.iter('LINEERROR')]
else:
    coll = root.find('./BODY/DATA/COLLECTION')
    rows = [] if coll is None else list(coll)
    for r in rows:
        nm = r.get('NAME') or (r.findtext('NAME') or '').strip()
        summary.setdefault(r.tag, []).append({'name': nm, 'guid': (r.findtext('GUID') or '').strip(), 'alterid': (r.findtext('ALTERID') or '').strip(),
                                             'altmstid': (r.findtext('ALTMSTID') or '').strip(), 'altvchid': (r.findtext('ALTVCHID') or '').strip(), 'parent': (r.findtext('PARENT') or '').strip()})
js = f'{R}/evidence/{seq}-{label}-{name}.json'
open(js, 'w', encoding='utf-8', newline='\n').write(json.dumps(summary, indent=1, ensure_ascii=False))
if name.startswith('create'): print(json.dumps(summary))
else: print({k: len(v) for k, v in summary.items()}, json.dumps(summary)[:700])
