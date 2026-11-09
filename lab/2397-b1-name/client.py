# Lab helper for item 2397 B1 (not product code): lab/2246-split-journal's client.py, changed only where this
# sheet needs it (RUN-SHEET.md, "Files", lists each change). One launch of one build's bridge_mcp.exe, kept until
# <row>\inbox\stop. The launch configuration must hold exactly the four settings this sheet names, and only the
# tools named in ALLOWED are sent: reads only, no import, no post. Watches <row>\inbox\ for <seq>-<name>.json =
# {"tool": ..., "arguments": {...}}, sends one at a time in name order, waits for each answer (no timeout, never
# resent), saves <row>\answers\<seq>-<name>.json whole (written to a .tmp file, then renamed).
# Log: <row>\client-log.txt (UTC).
# Usage: python client.py <row folder> <b1|master>
# It logs the package family (or none) of itself and of the server it starts, as the 2246 client did.
# Request files are read as UTF-8 with or without a BOM; any failure is logged with its traceback before exit.
import hashlib
import json
import os
import subprocess
import sys
import time
import traceback
import ctypes
from ctypes import wintypes

ROOT = sys.argv[1]
ARGS = sys.argv[2:]
BUILDS = ('b1', 'master')
BUILD = ARGS[0] if len(ARGS) == 1 and ARGS[0] in BUILDS else None
LAB = r'C:\bridge-lab\2397-b1'
EXE = os.path.join(LAB, BUILD or 'none', 'bin', 'bridge_mcp.exe')
DATA_DIR = os.path.join(LAB, BUILD or 'none', 'data')
LAUNCH = os.path.join(ROOT, 'launch-env.json')
INBOX, ANSWERS = os.path.join(ROOT, 'inbox'), os.path.join(ROOT, 'answers')
ALLOWED = ('list', 'tally_status', 'list_companies', 'trial_balance')


def log(line):
    with open(os.path.join(ROOT, 'client-log.txt'), 'a', encoding='utf-8', newline='\n') as f:
        f.write(time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()) + ' ' + line + '\n')


def sha_of(path):
    with open(path, 'rb') as fh:
        return hashlib.sha256(fh.read()).hexdigest()


def package_family(pid=None):
    """The package family of this process (pid None) or of pid: a name, 'none', or 'error <n>'."""
    k32 = ctypes.WinDLL('kernel32', use_last_error=True)
    k32.OpenProcess.restype = wintypes.HANDLE
    k32.OpenProcess.argtypes = (wintypes.DWORD, wintypes.BOOL, wintypes.DWORD)
    k32.CloseHandle.argtypes = (wintypes.HANDLE,)
    length = wintypes.UINT(0)
    if pid is None:
        rc = k32.GetCurrentPackageFamilyName(ctypes.byref(length), None)
    else:
        handle = k32.OpenProcess(0x1000, False, pid)  # PROCESS_QUERY_LIMITED_INFORMATION
        if not handle:
            return f'error open {ctypes.get_last_error()}'
        try:
            rc = k32.GetPackageFamilyName(wintypes.HANDLE(handle), ctypes.byref(length), None)
            if rc == 122:
                name = ctypes.create_unicode_buffer(length.value)
                rc = k32.GetPackageFamilyName(wintypes.HANDLE(handle), ctypes.byref(length), name)
                return name.value if rc == 0 else f'error {rc}'
        finally:
            k32.CloseHandle(wintypes.HANDLE(handle))
        return 'none' if rc == 15700 else f'error {rc}'
    if rc == 122:
        name = ctypes.create_unicode_buffer(length.value)
        rc = k32.GetCurrentPackageFamilyName(ctypes.byref(length), name)
        return name.value if rc == 0 else f'error {rc}'
    return 'none' if rc == 15700 else f'error {rc}'


def main():
    global p, server_info, SIDE
    if BUILD is None:
        log(f'stop: arguments {ARGS!r} are not <b1|master>; not started')
        sys.exit(2)
    SIDE = BUILD
    with open(os.path.join(LAB, BUILD, 'exe-sha256.txt'), encoding='ascii') as f:
        expected_sha = f.read().strip()
    # The Terms setting is the owner's act: the client checks only that the owner wrote it, never its value.
    required = {'BRIDGE_TALLY_HOST': '127.0.0.1', 'BRIDGE_TALLY_PORT': '9001', 'BRIDGE_AGENT_DATA_DIR': DATA_DIR}
    exe_sha = sha_of(EXE)
    if exe_sha != expected_sha:
        log(f'stop: exe sha256 {exe_sha} differs; not started')
        sys.exit(2)
    with open(LAUNCH, encoding='utf-8-sig') as f:
        launch = json.load(f)
    if (sorted(launch) != sorted(list(required) + ['BRIDGE_TERMS_ACCEPTED'])
            or {k: launch.get(k) for k in required} != required):
        log(f'stop: launch configuration is not exactly the required one (keys {sorted(launch)}); not started')
        sys.exit(2)
    env = {k: v for k, v in os.environ.items() if not k.startswith('BRIDGE_')}
    env.update(launch)
    log(f'build {SIDE}; client pid {os.getpid()}; exe sha256 {exe_sha} ok; launch-env.json sha256 {sha_of(LAUNCH)}; '
        f'LOCALAPPDATA={env.get("LOCALAPPDATA")!r} APPDATA={env.get("APPDATA")!r} USERPROFILE={env.get("USERPROFILE")!r}')
    stderr_path = os.path.join(ROOT, time.strftime('server-stderr-%Y%m%dT%H%M%SZ.log', time.gmtime()))
    stderr_file = open(stderr_path, 'wb')
    p = subprocess.Popen([EXE], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=stderr_file, env=env)
    log(f'launched pid {p.pid}; stderr to {os.path.basename(stderr_path)}')
    log(f'package family: client {package_family()}; server {package_family(p.pid)}')
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
        with open(os.path.join(INBOX, fn), encoding='utf-8-sig') as f:
            req = json.load(f)
        if req.get('tool') not in ALLOWED:
            log(f'stop: {fn} names tool {req.get("tool")!r}, which this item never sends; nothing sent')
            break
        t0w = time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())
        t0 = time.perf_counter()
        if req['tool'] == 'list':
            res = rpc('tools/list', {})
        else:
            res = rpc('tools/call', {'name': req['tool'], 'arguments': req.get('arguments', {})})
        secs = time.perf_counter() - t0
        body = {'build': SIDE, 'server_info': server_info, 'server_pid': p.pid, 'tool': req['tool'],
                'arguments': req.get('arguments', {}), 'call_sent_utc': t0w, 'round_trip_seconds': round(secs, 3),
                'response': res}
        with open(out + '.tmp', 'w', encoding='utf-8', newline='\n') as f:
            json.dump(body, f, ensure_ascii=False)
        os.replace(out + '.tmp', out)
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


try:
    main()
except SystemExit as stop:
    if stop.code not in (None, 0, 2):
        log(f'stop: {stop.code}')
    raise
except BaseException:
    log('stop: exception\n' + traceback.format_exc())
    raise
