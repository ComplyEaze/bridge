# Lab helper (#648, not product code): call ONE tool of the staged bridge_mcp.exe over MCP stdio, time the tools/call
# round trip, then read egress_log (which sends nothing to Tally) in the same session, and save both whole.
# Usage: python -I call648.py <part A|B> <out-name> <tool> <args.json | ->
import hashlib
import json
import os
import subprocess
import sys
import time

EXE = r'<build folder>\bridge_mcp.exe'
EXE_SHA = '92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524'
ROOT = r'<item folder>'

part, name, tool = sys.argv[1], sys.argv[2], sys.argv[3]
args = {}
if len(sys.argv) > 4 and sys.argv[4] != '-':
    with open(sys.argv[4], encoding='utf-8') as f:
        args = json.load(f)
out_path = os.path.join(ROOT, 'answers', part, name + '.json')
if os.path.exists(out_path):
    sys.exit(f'exists: {out_path} (never repeat a call)')
with open(EXE, 'rb') as fh:
    if hashlib.sha256(fh.read()).hexdigest() != EXE_SHA:
        sys.exit('halt: exe hash differs; not started')
env = dict(os.environ)
for k in ('BRIDGE_AGENT_ENABLE_WRITES', 'BRIDGE_AGENT_ENABLE_IMPORT', 'BRIDGE_AGENT_REDACTION'):
    env.pop(k, None)
env.update({'BRIDGE_TERMS_ACCEPTED': 'true', 'BRIDGE_TALLY_HOST': '127.0.0.1', 'BRIDGE_TALLY_PORT': '9001',
            'BRIDGE_AGENT_DATA_DIR': os.path.join(ROOT, 'data-' + part)})
p = subprocess.Popen([EXE], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=env)


def send(msg):
    p.stdin.write((json.dumps(msg) + '\n').encode('utf-8'))
    p.stdin.flush()


def recv(want):
    while True:
        line = p.stdout.readline()
        if not line:
            raise SystemExit('server closed stdout; stderr: ' + p.stderr.read().decode('utf-8', 'replace')[:2000])
        m = json.loads(line)
        if m.get('id') == want:
            return m


send({'jsonrpc': '2.0', 'id': 1, 'method': 'initialize', 'params': {'protocolVersion': '2025-06-18', 'capabilities': {}, 'clientInfo': {'name': '<lab client name>', 'version': '1'}}})
init = recv(1)
send({'jsonrpc': '2.0', 'method': 'notifications/initialized'})
t0_wall = time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())
t0 = time.perf_counter()
send({'jsonrpc': '2.0', 'id': 2, 'method': 'tools/call', 'params': {'name': tool, 'arguments': args}})
res = recv(2)
secs = time.perf_counter() - t0
t1_wall = time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())
send({'jsonrpc': '2.0', 'id': 3, 'method': 'tools/call', 'params': {'name': 'egress_log', 'arguments': {'limit': 8}}})
eg = recv(3)
p.stdin.close()
try:
    p.wait(timeout=30)
except subprocess.TimeoutExpired:
    p.kill()
raw = json.dumps(res, ensure_ascii=False)
with open(out_path, 'w', encoding='utf-8', newline='\n') as f:
    json.dump({'server_info': init.get('result', {}).get('serverInfo'), 'tool': tool, 'arguments': args,
               'call_sent_utc': t0_wall, 'answer_utc': t1_wall, 'round_trip_seconds': round(secs, 3),
               'response': res}, f, ensure_ascii=False)
with open(os.path.join(ROOT, 'answers', part, name + '.egress.json'), 'w', encoding='utf-8', newline='\n') as f:
    json.dump(eg, f, ensure_ascii=False)
r = res.get('result') or {}
text = ''.join(c.get('text', '') for c in r.get('content', []) if c.get('type') == 'text')
codes = [c for c in ('request_deadline_exceeded', 'response_size_limit_exceeded') if c in text]
# cost from the egress record of this call
reqs = None
bytes_prepared = None
trail = []
try:
    et = json.loads(eg['result']['content'][0]['text'])
    recs = [json.loads(x) for x in et['result']['records']]
    mine = [x for x in recs if x.get('record_type') == 'response_prepared' and x.get('tool') == tool]
    if mine:
        bytes_prepared = mine[0].get('bytes_prepared')
        trail = mine[0].get('request_trail', {}).get('last', [])
        reqs = mine[0].get('request_trail', {}).get('sent')
except Exception as e:  # noqa: BLE001
    trail = ['egress parse failed: %s' % e]
line = '%s %s %s round_trip=%.3fs answer_text_bytes=%d bytes_prepared=%s trail_sent=%s isError=%s codes=%s' % (
    t0_wall, name, tool, secs, len(text.encode('utf-8')), bytes_prepared, reqs, r.get('isError'), codes)
with open(os.path.join(ROOT, 'cost-log.txt'), 'a', encoding='ascii', errors='replace', newline='\n') as f:
    f.write(line + '\n')
print(line)
if codes:
    print('halt: ' + ', '.join(codes))
print(text[:1200])
