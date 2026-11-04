# Lab helper (1029-T, copied from 1029-R; not product code): for each kept answer named, the sha256 and size of its
# `result` member (structuredContent.result) as canonical JSON, json.dumps(obj, sort_keys=True, separators=(",",":"),
# ensure_ascii=False).encode("utf-8"), leaving out company, evidence, read_at and truncated. Its bytes go to no file; the
# helper refuses to print a line whose canonical bytes hold the company number. Writes evidence/kept-canonical.txt.
# Usage: python -I -X utf8 canon.py <answer> ...
import hashlib, json, os, sys
B = '<Tally data folder>'; NM = 'BRIDGE CORPUS DENSE'.encode('utf-16-le') + b'\x00\x00'
NUM = [d for d in os.listdir(B) if os.path.isfile(os.path.join(B, d, 'Company.1800')) and NM in open(os.path.join(B, d, 'Company.1800'), 'rb').read()][0]
out = []
for p in sys.argv[1:]:
    r = json.load(open(p, encoding='utf-8'))['response']['result']['structuredContent']['result']
    c = {k: v for k, v in r.items() if k not in ('company', 'evidence', 'read_at', 'truncated')}
    b = json.dumps(c, sort_keys=True, separators=(',', ':'), ensure_ascii=False).encode('utf-8')
    if NUM.encode() in b: out.append(f'{p}: canonical result still holds the company number; no line given'); continue
    out.append(f'{p}: result (canonical JSON, without company, evidence, read_at, truncated) {len(b)} B sha256 {hashlib.sha256(b).hexdigest()}')
open('<LAB_DIR>/evidence/kept-canonical.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
print('\n'.join(out))
