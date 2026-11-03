# Lab helper (683-B, not product code): queues ONE call for the row's stdio client and waits for its answer.
# The arguments are the sheet's args file, copied byte for byte into the call. Prints only the shape: isError,
# the error code and cause for a refusal, and the answer's size; the answer itself stays in run/answers.
# Usage: python -I -X utf8 q.py <seq-name> <tool> <sheet args file name or ->
import json, os, sys, time
R = '<lab folder>/run/'; name, tool, af = sys.argv[1], sys.argv[2], sys.argv[3]
args = {} if af == '-' else json.load(open('<lab folder>/sheet/args/' + af, encoding='utf-8'))
fn = name + '.json'
if os.path.exists(R + 'inbox/' + fn) or os.path.exists(R + 'answers/' + fn): sys.exit(fn + ' already queued; never resent')
tmp = R + 'inbox/' + name + '.tmp'
open(tmp, 'w', encoding='utf-8', newline='\n').write(json.dumps({'tool': tool, 'arguments': args}))
os.replace(tmp, R + 'inbox/' + fn)
while not os.path.exists(R + 'answers/' + fn): time.sleep(1)
time.sleep(1)
a = json.load(open(R + 'answers/' + fn, encoding='utf-8'))
res = a['response']; r = res.get('result') or {}
sc = r.get('structuredContent') or {}
err = sc.get('error') if isinstance(sc, dict) else None
print(fn, 'secs', a['round_trip_seconds'], 'isError', r.get('isError'), 'rpc_error', bool(res.get('error')),
      'code', (err or {}).get('code') if isinstance(err, dict) else err, 'bytes', os.path.getsize(R + 'answers/' + fn))
