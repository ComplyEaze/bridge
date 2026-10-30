# Lab helper (#2142 drain repeat, not product code; row 2119-B's client.py with the folder, build and answers folder changed): one launch of the staged bridge_mcp.exe over MCP stdio, kept until inbox\stop.
# Its environment comes from launch-env.json in this folder (the row's launch configuration; the lab sets the posting
# setting there on the owner's word of 28 Oct and the go-ahead for #1709-settle, synthetic books only). Watches inbox\ for <seq>-<name>.json = {"tool": ..., "arguments": {...}},
# sends one at a time in name order, waits for each answer (no timeout, never resent), saves answers\<seq>-<name>.json
# whole. Log: client-log.txt (UTC).
import hashlib
import json
import os
import subprocess
import sys
import time

ROOT = r'<lab folder>'
EXE = r'<build folder>\bin\x86_64-pc-windows-msvc\bridge_mcp.exe'
EXE_SHA = '4c9bace38a82f77471d9b30d88e20ac92d3472a187b744cfeaf15537edf2df44'
LAUNCH = os.path.join(ROOT, 'launch-env.json')
INBOX, ANSWERS = os.path.join(ROOT, 'inbox'), os.path.join(ROOT, 'answers-mcp')
FORBIDDEN = ('BRIDGE_AGENT_ENABLE_BATCH_POST', 'BRIDGE_AGENT_MAX_BYTES', 'BRIDGE_AGENT_REDACTION')


def log(line):
    with open(os.path.join(ROOT, 'client-log.txt'), 'a', encoding='utf-8', newline='\n') as f:
        f.write(time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()) + ' ' + line + '\n')


def sha_of(path):
    with open(path, 'rb') as fh:
        return hashlib.sha256(fh.read()).hexdigest()


exe_sha = sha_of(EXE)
if exe_sha != EXE_SHA:
    log(f'stop: exe sha256 {exe_sha} differs; not started')
    sys.exit(2)
with open(LAUNCH, encoding='utf-8') as f:
    launch = json.load(f)
bad = [k for k in launch if k in FORBIDDEN]
if bad:
    log(f'stop: launch configuration holds {bad}; not started')
    sys.exit(2)
env = {k: v for k, v in os.environ.items() if not k.startswith('BRIDGE_')}
env.update({k: str(v) for k, v in launch.items()})
log(f'exe sha256 {exe_sha} ok; launch-env.json sha256 {sha_of(LAUNCH)}; keys {sorted(launch)}; '
    f'BRIDGE_AGENT_ENABLE_WRITES present: {"BRIDGE_AGENT_ENABLE_WRITES" in launch}')
stderr_path = os.path.join(ROOT, time.strftime('server-stderr-%Y%m%dT%H%M%SZ.log', time.gmtime()))
stderr_file = open(stderr_path, 'wb')
p = subprocess.Popen([EXE], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=stderr_file, env=env)
log(f'launched pid {p.pid}; stderr to {os.path.basename(stderr_path)}')
with open(os.path.join(ROOT, 'server.pid'), 'w') as f:
    f.write(str(p.pid))
next_id = [1]


def rpc(method, params):
    i = next_id[0]
    next_id[0] += 1
    p.stdin.write((json.dumps({'jsonrpc': '2.0', 'id': i, 'method': method, 'params': params}) + '\n').encode('utf-8'))
    p.stdin.flush()
    while True:
        line = p.stdout.readline()
        if not line:
            raise SystemExit('server closed stdout')
        m = json.loads(line)
        if m.get('id') == i:
            return m


init = rpc('initialize', {'protocolVersion': '2025-06-18', 'capabilities': {}, 'clientInfo': {'name': 'lab-client', 'version': '1'}})
p.stdin.write((json.dumps({'jsonrpc': '2.0', 'method': 'notifications/initialized'}) + '\n').encode('utf-8'))
p.stdin.flush()
server_info = init.get('result', {}).get('serverInfo')
log(f'initialized {server_info}')
done = set(f for f in os.listdir(ANSWERS) if f.endswith('.json'))
while True:
    if os.path.exists(os.path.join(INBOX, 'stop')):
        break
    todo = sorted(f for f in os.listdir(INBOX) if f.endswith('.json') and f not in done)
    if not todo:
        time.sleep(0.5)
        continue
    fn = todo[0]
    done.add(fn)
    out = os.path.join(ANSWERS, fn)
    if os.path.exists(out):
        log(f'skip {fn}: answer exists (never resend)')
        continue
    with open(os.path.join(INBOX, fn), encoding='utf-8') as f:
        req = json.load(f)
    t0w = time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())
    t0 = time.perf_counter()
    if req['tool'] == 'list':
        res = rpc('tools/list', {})
    else:
        res = rpc('tools/call', {'name': req['tool'], 'arguments': req.get('arguments', {})})
    secs = time.perf_counter() - t0
    body = {'server_info': server_info, 'server_pid': p.pid, 'tool': req['tool'], 'arguments': req.get('arguments', {}),
            'call_sent_utc': t0w, 'round_trip_seconds': round(secs, 3), 'response': res}
    with open(out, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(body, f, ensure_ascii=False)
    r = res.get('result') or {}
    log(f'{fn} tool={req["tool"]} secs={secs:.3f} isError={r.get("isError")} error={res.get("error")} saved_sha256={sha_of(out)}')
p.stdin.close()
try:
    p.wait(timeout=30)
except subprocess.TimeoutExpired:
    log('server did not exit within 30 s of stdin close; killed')
    p.kill()
    p.wait()
stderr_file.close()
log(f'session ended; server pid {p.pid} exit code {p.returncode}')
