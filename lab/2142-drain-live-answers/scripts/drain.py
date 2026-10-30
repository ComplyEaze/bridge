# Lab helper (#2142 drain live, not product code): the sheet's steps through the row's client (inbox/answers), with no
# raw request. Each call is queued, waited for (no timeout), and summarised: wall seconds, isError, the refusal code, cause,
# retry_after_s and next step if any, and the marks for ledger_masters. After each call the drain record is copied whole at
# once (debt\<seq>-<label>.json, or "absent"), with bytes and sha256. Usage:
#   python -I -X utf8 drain.py rest <label>          one ledger_masters (basic) and a copy of the record
#   python -I -X utf8 drain.py run <n>               steps 3, 4 and 5 in a row (the cut, the refusal, the admission)
#   python -I -X utf8 drain.py one <label> <tool>    one call (ledger_masters basic, or egress_log) and a copy
import hashlib, json, os, sys, time
R = '<LAB_DIR>'; G = '<company guid>'
DEBT = os.path.join(os.environ['USERPROFILE'], '.complyeaze-bridge', 'coordination', 'native-dispatch-leases',
                    'a905deaa97bc7d8acb82123c4057a3890b174daf0295ee5c10be17f6fa0f0a01.wire.debt')
os.makedirs(R + '/debt', exist_ok=True)
def now(): t = time.time(); return time.strftime('%Y-%m-%dT%H:%M:%S', time.gmtime(t)) + f'.{int(t * 1000) % 1000:03d}Z'
def log(s):
    with open(R + '/evidence/drain-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
def call(name, tool, args):
    inbox, answers = R + '/inbox', R + '/answers'
    seq = len([f for f in os.listdir(inbox) if f.endswith('.json')]); fn = f'{seq:03d}-{name}.json'
    t0 = time.monotonic(); ts = now()
    with open(f'{inbox}/{fn}', 'w', encoding='utf-8', newline='\n') as f: json.dump({'tool': tool, 'arguments': args}, f, ensure_ascii=False)
    out = f'{answers}/{fn}'
    while not os.path.exists(out): time.sleep(0.05)
    time.sleep(0.1)
    while True:
        try: j = json.load(open(out, encoding='utf-8')); break
        except ValueError: time.sleep(0.05)
    wall = time.monotonic() - t0
    r = j['response'].get('result') or {}; sc = r.get('structuredContent') or {}
    err = sc.get('error') if isinstance(sc.get('error'), dict) else ((sc.get('result') or {}).get('error') if isinstance((sc.get('result') or {}).get('error'), dict) else None)
    code = (err or {}).get('code') or sc.get('code'); v = sc.get('result', sc) if isinstance(sc, dict) else {}
    snap = v.get('snapshot', {}) if isinstance(v, dict) else {}
    info = {'fn': fn, 'sent': ts, 'wall_s': round(wall, 3), 'round_trip_s': j.get('round_trip_seconds'), 'isError': r.get('isError'), 'code': code}
    for k in ('retry_after_s', 'cause', 'next_step', 'reason', 'message'):
        for src in (err or {}, sc):
            if isinstance(src, dict) and k in src and k not in info: info[k] = src[k]
    if tool == 'ledger_masters' and not r.get('isError'): info['marks'] = (snap.get('master_alter_id'), snap.get('voucher_alter_id'), snap.get('reused'))
    if r.get('isError'): info['refusal'] = json.dumps(sc, ensure_ascii=False)[:900]
    if tool == 'egress_log': info['egress'] = json.dumps(sc, ensure_ascii=False)[:2500]
    log(f'{name}: ' + json.dumps(info, ensure_ascii=False))
    return info
def debt(label):
    seq = len(os.listdir(R + '/debt'))
    try: d = open(DEBT, 'rb').read()
    except FileNotFoundError: d = None
    p = f'{R}/debt/{seq:03d}-{label}.' + ('json' if d is not None else 'absent.txt')
    open(p, 'wb').write(d if d is not None else b'absent\n')
    log(f'  drain record after {label} at {now()}: ' + ('absent' if d is None else f'{len(d)} B sha256 {hashlib.sha256(d).hexdigest()} {d.decode("utf-8", "replace")}'))
    return d
LM = lambda: {'company_guid': G}
mode = sys.argv[1]
if mode == 'rest':
    call(sys.argv[2], 'ledger_masters', LM()); debt(sys.argv[2])
elif mode == 'one':
    lab, tool = sys.argv[2], sys.argv[3]
    call(lab, tool, LM() if tool == 'ledger_masters' else {'limit': 8}); debt(lab)
elif mode == 's5':
    n = sys.argv[2]; wait = float(sys.argv[3]); t5 = time.monotonic(); k = 0
    log(f'# run {n} step 5 started {now()} (resumed after the driver misread step 4)')
    while True:
        k += 1
        log(f'  waiting {wait} s (retry_after_s)'); time.sleep(wait)
        b = call(f'r{n}-s5-ledger_masters-{k:02d}', 'ledger_masters', LM()); debt(f'r{n}-s5-{k:02d}')
        if not b.get('isError'): call(f'r{n}-s5-egress_log', 'egress_log', {'limit': 8}); debt(f'r{n}-s5-egress'); break
        if b.get('code') != 'tally_endpoint_draining': log(f'  stop rule: a refusal the sheet does not expect: {b.get("code")}'); break
        if k >= 20 or time.monotonic() - t5 > 180: log(f'  stop rule: step 5 bound reached ({k} calls, {time.monotonic() - t5:.1f} s)'); break
        wait = b.get('retry_after_s')
    log(f'# run {n} step 5 ended {now()}')
elif mode == 'run':
    n = sys.argv[2]; t_start = time.monotonic()
    log(f'# run {n} started {now()}')
    o = call(f'r{n}-s3-outstandings', 'outstandings', {'company_guid': G, 'as_of': '20260331'}); debt(f'r{n}-s3')
    a = call(f'r{n}-s4-ledger_masters', 'ledger_masters', LM()); debt(f'r{n}-s4')
    call(f'r{n}-s4-egress_log', 'egress_log', {'limit': 8}); debt(f'r{n}-s4-egress')
    wait = a.get('retry_after_s'); t5 = time.monotonic(); k = 0
    if a.get('code') != 'tally_endpoint_draining': log(f'  step 4 not refused as draining: {a.get("code")}; step 5 not looped')
    else:
        while True:
            k += 1
            if not isinstance(wait, (int, float)): log(f'  no retry_after_s to wait for: {wait!r}'); break
            log(f'  waiting {wait} s (retry_after_s)'); time.sleep(wait)
            b = call(f'r{n}-s5-ledger_masters-{k:02d}', 'ledger_masters', LM()); debt(f'r{n}-s5-{k:02d}')
            if not b.get('isError'): call(f'r{n}-s5-egress_log', 'egress_log', {'limit': 8}); debt(f'r{n}-s5-egress'); break
            if b.get('code') != 'tally_endpoint_draining': log(f'  stop rule: a refusal the sheet does not expect: {b.get("code")}'); break
            if k >= 20 or time.monotonic() - t5 > 180: log(f'  stop rule: step 5 bound reached ({k} calls, {time.monotonic() - t5:.1f} s)'); break
            wait = b.get('retry_after_s')
    log(f'# run {n} ended {now()} ({time.monotonic() - t_start:.1f} s)')
