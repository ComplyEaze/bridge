# Lab helper (#2142 drain repeat, not product code; row 2119-B's call.py with the folder changed): queue ONE call for this row's stdio client from Python values, no shell
# quoting. Usage (from another script): call(name, tool, args_dict) -> (isError, parsed answer text as JSON or None).
import json
import os
import time

ROOT = r'<lab folder>'
INBOX, ANSWERS = os.path.join(ROOT, 'inbox'), os.path.join(ROOT, 'answers-mcp')


def call(name, tool, args):
    seq = '{:03d}'.format(len([f for f in os.listdir(INBOX) if f.endswith('.json')]))
    fn = f'{seq}-{name}.json'
    with open(os.path.join(INBOX, fn), 'w', encoding='utf-8', newline='\n') as f:
        json.dump({'tool': tool, 'arguments': args}, f)
    out = os.path.join(ANSWERS, fn)
    while not os.path.exists(out):
        time.sleep(0.3)
    time.sleep(0.3)
    w = json.load(open(out, encoding='utf-8'))
    res = w['response'].get('result') or {}
    text = None
    for c in res.get('content', []):
        if c.get('type') == 'text':
            text = c['text']
    try:
        body = json.loads(text) if text else None
    except ValueError:
        body = text
    return fn, w['round_trip_seconds'], res.get('isError'), body


