# Lab runner (#1584 C rows and OUT-FIRST XC rows, not product code): one contention row. If a holder is named, it is
# started (in: from this shell; out: the WMI method, through step.ps1) with its own log and stop file, and the call is
# sent only once its log shows HELD (BUSY or OPEN_FAILED = the row is void). The call (tally_status) goes to the named
# client's inbox; after the answer the stop file is written and the log must show RELEASED before this runner returns.
# Void too if RELEASED is earlier than call_sent_utc + round_trip_seconds. A line per row goes to evidence\C-rows.txt.
# Usage: python -I -X utf8 crow.py <row> <client folder> <seq-name> [<holder side> <holder file>] [--seconds 900]
import json, os, re, subprocess, sys, time
from datetime import datetime, timedelta, timezone
W = 'C:\\bridge-lab\\1584'
row, client, name = sys.argv[1], sys.argv[2], sys.argv[3]
hside = sys.argv[4] if len(sys.argv) > 5 else None
hfile = sys.argv[5] if len(sys.argv) > 5 else None
def now(): return datetime.now(timezone.utc)
def logline(s):
    with open(os.path.join(W, 'evidence', 'C-rows.txt'), 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
def events(path):
    # read briefly, retrying a busy file: a read that collides with the holder's own write can cost the holder a log line
    for _ in range(20):
        if not os.path.exists(path): return []
        try:
            with open(path, encoding='ascii', errors='replace') as f: return [l.strip() for l in f if l.strip()]
        except PermissionError: time.sleep(0.25)
    return []
void = []; hlog = None
if hside:
    hlog = os.path.join(W, 'evidence', f'{row}-{hside}-hold.log'); stop = os.path.join(W, f'stop-{row}-{hside}.txt')
    if os.path.exists(hlog) or os.path.exists(stop): logline(f'{row}: holder log or stop file exists; nothing started'); sys.exit(2)
    line = (f'powershell -NoProfile -NonInteractive -ExecutionPolicy Bypass -File C:\\bridge-lab\\1584\\hold-lock.ps1 -Path "{hfile}" '
            f'-Mode Exclusive -Seconds 900 -Log {hlog} -StopFile {stop}')
    subprocess.run(['powershell', '-NoProfile', '-File', os.path.join(W, 'step.ps1'), '-Name', f'{row}-hold-{hside}', '-Side', hside, '-Lines', line], capture_output=True, text=True)
    time.sleep(3)  # START and HELD are written within a second of the start; reading later avoids colliding with them
    t0 = time.monotonic()
    while True:
        ev = events(hlog)
        if any(e.split(' ', 1)[1].startswith(('HELD', 'BUSY', 'OPEN_FAILED', 'INSTRUMENT_FAILED')) for e in ev if ' ' in e): break
        if time.monotonic() - t0 > 60: logline(f'{row}: no holder event within 60 s'); sys.exit(3)
        time.sleep(1)
    first = [e for e in ev if not e.split(' ', 1)[1].startswith('START')][0]
    if 'HELD' not in first:
        logline(f'{row}: VOID, holder {hside} on {hfile}: {first}'); sys.exit(4)
seq_path = os.path.join(W, client, 'answers', name + '.json')
r = subprocess.run(['python', '-I', '-X', 'utf8', os.path.join(W, 'req.py'), 'call', os.path.join(W, client), name, 'tally_status', '{}'], capture_output=True, text=True)
j = json.load(open(seq_path, encoding='utf-8'))
res = j['response'].get('result') or {}; sc = res.get('structuredContent') or {}
err = (sc.get('result') or {}).get('error') if isinstance(sc.get('result'), dict) else None
code = (err or {}).get('code') if err else None
if not code and res.get('isError'): code = json.dumps(sc)[:200]
sent = datetime.strptime(j['call_sent_utc'], '%Y-%m-%dT%H:%M:%SZ').replace(tzinfo=timezone.utc)
end_of_call = sent + timedelta(seconds=j['round_trip_seconds'])
hold_note = ''
if hside:
    open(stop, 'w').close()
    time.sleep(2)
    t0 = time.monotonic()
    while not any('RELEASED' in e for e in events(hlog)):
        if time.monotonic() - t0 > 30: logline(f'{row}: no RELEASED within 30 s of the stop file'); break
        time.sleep(1)
    ev = events(hlog)
    held = [e for e in ev if e.split(' ', 1)[1].startswith('HELD')][0]
    rel = [e for e in ev if 'RELEASED' in e]
    if rel:
        rel_t = datetime.fromisoformat(re.sub(r'(\.\d{6})\d+', r'\1', rel[0].split(' ')[0]).replace('Z', '+00:00'))
        # the sheet's rule, exactly: void if RELEASED is earlier than call_sent_utc + round_trip_seconds (the stop file is
        # written only after the answer has arrived, so the release follows the answer by construction)
        if rel_t < end_of_call: void.append('the hold was released before the call ended')
    hold_note = f'; holder {hside} {hfile}: {held}; {rel[0] if rel else "no RELEASED"}'
logline(f'{row}: client {client} {name}: call_sent_utc {j["call_sent_utc"]} round_trip {j["round_trip_seconds"]} s isError {res.get("isError")} code {code}{hold_note}' + (f'; VOID: {void}' if void else ''))
