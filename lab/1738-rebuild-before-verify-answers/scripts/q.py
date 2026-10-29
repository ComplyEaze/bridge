# Lab helper (#1738 rebuild before verify, not product code): queue ONE call through this row's stdio client and print a
# short summary. Usage: python -I q.py <name> <tool> <args.json file>. The arguments come from a file, never a shell string.
import hashlib
import json
import os
import sys

ROOT = '<lab folder>'
sys.path.insert(0, ROOT)
from call import call  # noqa: E402

name, tool, argfile = sys.argv[1], sys.argv[2], sys.argv[3]
args = json.load(open(argfile, encoding='utf-8'))
fn, secs, err, body = call(name, tool, args)
data = open(os.path.join(ROOT, 'answers', fn), 'rb').read()
print(fn, f'{secs:.3f}s', 'isError', err, 'file', len(data), 'B sha256', hashlib.sha256(data).hexdigest())
print(json.dumps(body, ensure_ascii=False)[:int(os.environ.get('Q_CHARS', '1500'))])
