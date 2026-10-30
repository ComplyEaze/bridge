# Lab helper (0.7.0 RC sitting, not product code): queue ONE call for client.py in <row>\inbox and wait for its answer
# (no timeout); print a short summary and the answer's text. Usage: submit.py <rowdir> <name> <tool> [<args json or @file>]
import json
import os
import sys
import time

row, name, tool = sys.argv[1], sys.argv[2], sys.argv[3]
raw = sys.argv[4] if len(sys.argv) > 4 else '{}'
if raw.startswith('@'):
    with open(raw[1:], encoding='utf-8') as f:
        raw = f.read()
args = json.loads(raw)
inbox, answers = os.path.join(row, 'inbox'), os.path.join(row, 'answers')
seq = len([f for f in os.listdir(inbox) if f.endswith('.json')])
fn = f'{seq:03d}-{name}.json'
with open(os.path.join(inbox, fn), 'w', encoding='utf-8', newline='\n') as f:
    json.dump({'tool': tool, 'arguments': args}, f, ensure_ascii=False)
out = os.path.join(answers, fn)
while not os.path.exists(out):
    time.sleep(0.3)
time.sleep(0.3)
with open(out, encoding='utf-8') as f:
    j = json.load(f)
r = j['response'].get('result') or {}
print(f"{fn} secs={j['round_trip_seconds']} isError={r.get('isError')} rpcError={j['response'].get('error')}")
if tool != 'list':
    for c in r.get('content', []):
        if c.get('type') == 'text':
            print(c['text'])
