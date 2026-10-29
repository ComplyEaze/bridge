# Lab helper (#1738 hand-import rebuild, not product code): one or more calls through this row's stdio client, each saved
# whole by the client; prints, per call, the file's sha256 and the parts the sheet asks to record (transfers whole).
# Usage: python -I -X utf8 step.py <name>=<tool>=<args file or {}> ...
import hashlib
import json
import os
import sys

ROOT = '<lab folder>'
sys.path.insert(0, ROOT)
from call import call  # noqa: E402

for spec in sys.argv[1:]:
    name, tool, src = spec.split('=', 2)
    args = {} if src == '{}' else json.load(open(os.path.join(ROOT, 'args', src), encoding='utf-8'))
    fn, secs, err, body = call(name, tool, args)
    data = open(os.path.join(ROOT, 'answers', fn), 'rb').read()
    print(f'== {fn} {secs:.3f}s isError {err} {len(data)} B sha256 {hashlib.sha256(data).hexdigest()}')
    r = body.get('result', {}) if isinstance(body, dict) else {}
    if 'error' in r:
        print('ERROR', json.dumps(r['error'], ensure_ascii=False))
    keys = [k for k in ('batch_id', 'state', 'verification_status', 'voucher_count', 'skipped_rows', 'proposals_id', 'sha256',
                        'rows_in_window', 'vouchers', 'skipped', 'path', 'dispatch', 'approval', 'attempt_recorded') if k in r]
    print({k: r[k] for k in keys})
    if 'transfers' in r:
        print('transfers', json.dumps(r['transfers'], ensure_ascii=False))
    for k in ('rows', 'vouchers_status', 'results', 'journal', 'next_step', 'next_steps', 'differences', 'attribution'):
        if k in r and k not in keys:
            print(k, json.dumps(r[k], ensure_ascii=False)[:2500])
