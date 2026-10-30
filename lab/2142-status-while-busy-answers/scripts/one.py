# Lab helper (#2142, not product code; row 2119-B's one.py with the folder changed): queue one tool call through this row's stdio client and print a
# short line (the marks for ledger_masters). Usage: python -I one.py <name> <tool> '<json arguments>'. Reads only.
import json
import os
import sys

sys.path.insert(0, '<lab folder>')
from call import call  # noqa: E402

fn, secs, is_err, body = call(sys.argv[1], sys.argv[2], json.loads(sys.argv[3]))
print(fn, f'{secs:.3f}s', 'isError', is_err)
if isinstance(body, dict):
    snap = (body.get('result') or {}).get('snapshot') or {}
    if snap:
        print('snapshot', json.dumps({k: snap.get(k) for k in ('master_alter_id', 'voucher_alter_id', 'reused')}))
        if snap.get('reused') is not False:
            sys.exit('HALT: snapshot.reused is not false')
print(json.dumps(body)[:600] if not isinstance(body, str) else body[:600])
