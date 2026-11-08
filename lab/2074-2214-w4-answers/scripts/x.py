# Lab helper (2074-2214 X cut, not product code): one X go, steps 1 to 7, through the row's client (run2), with the
# relay armed by file. Each Bridge call is followed by egress_log; the drain record is copied whole after each call and
# at D1. Raw MARKS (raw.py) only before the arm and after Whole: never while a drain may be owed. Stops on a refusal the
# sheet does not expect, after 20 draining refusals, or 3 minutes from D1 (C9). Everything goes to evidence/x-log.txt.
# Usage: python -I -X utf8 x.py <k> <front file name in fronts/>
import hashlib, json, math, os, subprocess, sys, time
R = '<lab folder>'; G = 'a30f23c3-4819-4334-96ab-ffa6fba70976'
DEBT = os.path.join(os.environ['USERPROFILE'], '.complyeaze-bridge', 'coordination', 'native-dispatch-leases',
                    '5a4833d61ba1083da1dccd41c0490142d921522815c2a3cc421c63542a1a2ad0.wire.debt')
k, front = sys.argv[1], sys.argv[2]
FRONT = f'<lab folder>\\fronts\\{front}'
def now(t=None): t = time.time() if t is None else t; return time.strftime('%Y-%m-%dT%H:%M:%S', time.gmtime(t)) + f'.{int(t * 1000) % 1000:03d}Z'
def log(s):
    with open(R + '/evidence/x-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
def call(name, tool, args):
    inbox, answers = R + '/run2/inbox', R + '/run2/answers'
    seq = len([f for f in os.listdir(inbox) if f.endswith('.json')]); fn = f'{seq:03d}-{name}.json'
    t0 = time.time(); m0 = time.monotonic()
    with open(f'{inbox}/{fn}', 'w', encoding='utf-8', newline='\n') as f: json.dump({'tool': tool, 'arguments': args}, f, ensure_ascii=False)
    out = f'{answers}/{fn}'
    while not os.path.exists(out): time.sleep(0.02)
    time.sleep(0.05)
    while True:
        try: j = json.load(open(out, encoding='utf-8')); break
        except ValueError: time.sleep(0.02)
    return fn, j, t0, time.time(), time.monotonic() - m0
def debt(label):
    os.makedirs(R + '/debt', exist_ok=True); seq = len(os.listdir(R + '/debt')); t = time.time()
    try: d = open(DEBT, 'rb').read()
    except FileNotFoundError: d = None
    open(f'{R}/debt/{seq:03d}-{label}.' + ('json' if d is not None else 'absent.txt'), 'wb').write(d if d is not None else b'absent\n')
    log(f'  drain record {label} at {now(t)}: ' + ('absent' if d is None else f'{len(d)} B sha256 {hashlib.sha256(d).hexdigest()} {d.decode("utf-8", "replace")}'))
    try: return json.loads(d) if d else None, t
    except ValueError: return None, t
def egress(label):
    efn, ej, *_ = call(f'{label}-egress_log', 'egress_log', {'limit': 4})
    recs = [json.loads(r) for r in ej['response']['result']['structuredContent']['result']['records']]
    mine = [r for r in recs if r.get('record_type') == 'response_prepared' and r.get('tool') == 'ledger_masters']
    if not mine: log(f'  {efn}: no ledger_masters receipt'); return None
    t = mine[0]; tr = t.get('request_trail') or {}; last = tr.get('last', [])
    mp = [x for x in last if x['kind'] == 'post' and x.get('request_bytes') in (1736, 1784)]
    log(f'  {efn}: receipt {t.get("ts")} error {(t.get("error") or {}).get("code")} trail sent {tr.get("sent")} omitted {tr.get("omitted")} failed {tr.get("failed")} last_failed_seq {tr.get("last_failed_seq")}')
    log('    sends: ' + '; '.join(f'{x["seq"]}:{x["kind"]}/{x["outcome"]}/{x.get("held_ms")}ms/req {x.get("request_bytes")}/resp {x.get("response_bytes")}' for x in last))
    log('    master posts: ' + ('; '.join(f'seq {x["seq"]} {x["outcome"]} held {x.get("held_ms")} ms req {x.get("request_bytes")} resp {x.get("response_bytes")}' for x in mp) or 'none'))
    return {'status_sends': sum(1 for x in last if x['kind'] == 'status'), 'posts': sum(1 for x in last if x['kind'] == 'post'), 'master': mp}
def read(label, mailing=False):
    a = {'company_guid': G, 'fields': 'compliance'}
    if mailing: a['include_mailing_details'] = True
    fn, j, t0, t1, wall = call(label, 'ledger_masters', a)
    r = j['response'].get('result') or {}; sc = r.get('structuredContent') or {}; v = sc.get('result', {}) if isinstance(sc.get('result'), dict) else {}
    err = v.get('error') if isinstance(v.get('error'), dict) else None; snap = v.get('snapshot', {})
    info = {'fn': fn, 'sent': now(t0), 'ended': now(t1), 'wall_s': round(wall, 3), 'isError': r.get('isError'), 'code': (err or {}).get('code'),
            'cause': (err or {}).get('cause'), 'failed_request': (err or {}).get('failed_request'), 'retry_after_s': (err or {}).get('retry_after_s'),
            'total': v.get('total'), 'rows': len(v.get('items', []) or []), 'next_offset': v.get('next_offset'),
            'marks': (snap.get('master_alter_id'), snap.get('voucher_alter_id'), snap.get('reused')) if snap else None}
    if err: info['next_step'] = err.get('next_step'); info['remediation'] = (err.get('remediation') or '')[:300]
    log(f'{label}: ' + json.dumps(info, ensure_ascii=False))
    return info, v, t1
def marks(label):
    r = subprocess.run([sys.executable, '-I', '-X', 'utf8', R + '/raw.py', 'marks', label], capture_output=True, text=True)
    log('  ' + r.stdout.strip().replace('\n', '\n  '))
log(f'# X{k} with {front} started {now()}')
# 1
d, _ = debt(f'x{k}-1-before')
if not d or d.get('state') != 'clear': log('HALT: drain record not clear at step 1'); sys.exit(2)
marks(f'x{k}-1-marks')
i1, _, _ = read(f'x{k}-1-standard')
if i1['isError']: log('HALT: step 1 STANDARD refused'); sys.exit(2)
# 2
open(R + '/arm/front', 'w', encoding='ascii', newline='').write(FRONT); log(f'  armed {now()} with {FRONT}')
im, _, tcut = read(f'x{k}-2-mailing', mailing=True)
# 4: D1 at once
d1, td1 = debt(f'x{k}-D1')
log(f'  D1 {now(td1)}: state {(d1 or {}).get("state")}')
egress(f'x{k}-2-mailing')
if not im['isError']: log('  the mailing read ANSWERED: VOID go (step 3 not met as a cut); continuing to D2/Whole for marks')
# 5: D2
n = 0; admitted = None
while True:
    n += 1
    info, v, t1 = read(f'x{k}-D2-{n:02d}')
    tr = egress(f'x{k}-D2-{n:02d}'); debt(f'x{k}-D2-{n:02d}-after')
    if not info['isError']: admitted = (info, v, t1, tr); break
    if info['code'] != 'tally_endpoint_draining': log(f'HALT: refusal {info["code"]} not expected'); sys.exit(3)
    if n >= 20 or time.time() - td1 > 180: log('HALT: C9, drain still refused after 20 calls or 3 minutes'); sys.exit(4)
    time.sleep(max(0, (info['retry_after_s'] or 1) - (time.time() - t1)))
# 6: D3
info, v, t1, tr = admitted
log(f'  D3: admitted at call {n}; gap D1 to its end {t1 - td1:.3f} s; master posts held {[x.get("held_ms") for x in (tr or {}).get("master", [])]} ms against 2026 ms')
# 7: Whole
iw, vw, _ = read(f'x{k}-whole')
egress(f'x{k}-whole')
strip = lambda x: {a: b for a, b in x.items() if a != 'snapshot'}
same = [strip(a) == strip(b) for a, b in zip(v.get('items', []), vw.get('items', []))]
log(f'  Whole: total {iw["total"]} (admitted {info["total"]}); rows {len(vw.get("items", []))}; equal rows {sum(same)} of {len(same)}; all equal {all(same) and len(v.get("items", [])) == len(vw.get("items", []))}')
marks(f'x{k}-7-marks')
debt(f'x{k}-7-after')
log(f'# X{k} ended {now()}')
