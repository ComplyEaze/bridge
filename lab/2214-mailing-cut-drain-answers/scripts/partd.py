# Lab helper (#2214, not product code): Part D through the row's client. 'cut <rung> <label>': MAILING (M_n); if it answers,
# every page is read and the run ends (no cut). If it is refused as the sheet expects (party_ledger_master_read_failed),
# D1: the drain record is copied at once, then egress_log; D2: within 5 s STANDARD, again after each refusal's
# retry_after_s (at most 20 calls or 3 minutes: the bound is a stop); D3: the first STANDARD that answers is read whole
# (every page), compared with S_<rung> (total, every row as parsed JSON, snapshot marks), then one raw MARKS (marks.py).
# 'std <rung> <label>': the D2/D3 loop alone (after C1 or D5). Every call is logged to evidence/partd-log.txt with its
# wall seconds, code, cause, retry_after_s, its trail (sent, omitted, failed, each kept send) and the drain record after it.
import hashlib, json, os, subprocess, sys, time
R = '<LAB_DIR>'; G = '<company guid>'
DEBT = os.path.join(os.environ['USERPROFILE'], '.complyeaze-bridge', 'coordination', 'native-dispatch-leases',
                    'a905deaa97bc7d8acb82123c4057a3890b174daf0295ee5c10be17f6fa0f0a01.wire.debt')
EXPECTED_CUT = 'party_ledger_master_read_failed'; DRAINING = 'tally_endpoint_draining'
def now(): t = time.time(); return time.strftime('%Y-%m-%dT%H:%M:%S', time.gmtime(t)) + f'.{int(t * 1000) % 1000:03d}Z'
def log(s):
    with open(R + '/evidence/partd-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
def call(name, tool, args):
    inbox, answers = R + '/inbox', R + '/answers'
    seq = len([f for f in os.listdir(inbox) if f.endswith('.json')]); fn = f'{seq:03d}-{name}.json'
    t0 = time.monotonic(); ts = now()
    with open(f'{inbox}/{fn}', 'w', encoding='utf-8', newline='\n') as f: json.dump({'tool': tool, 'arguments': args}, f, ensure_ascii=False)
    out = f'{answers}/{fn}'
    while not os.path.exists(out): time.sleep(0.02)
    time.sleep(0.05)
    while True:
        try: j = json.load(open(out, encoding='utf-8')); break
        except ValueError: time.sleep(0.05)
    t1 = time.monotonic()
    return fn, j, ts, t1 - t0, now()
def debt(label):
    os.makedirs(R + '/debt', exist_ok=True); seq = len(os.listdir(R + '/debt'))
    try: d = open(DEBT, 'rb').read()
    except FileNotFoundError: d = None
    open(f'{R}/debt/{seq:03d}-{label}.' + ('json' if d is not None else 'absent.txt'), 'wb').write(d if d is not None else b'absent\n')
    log(f'  drain record after {label} at {now()}: ' + ('absent' if d is None else f'{len(d)} B sha256 {hashlib.sha256(d).hexdigest()} {d.decode("utf-8", "replace")}'))
    return d
def summarise(label, fn, j, ts, wall, te):
    r = j['response'].get('result') or {}; sc = r.get('structuredContent') or {}; v = sc.get('result', {}) if isinstance(sc.get('result'), dict) else {}
    err = v.get('error') if isinstance(v.get('error'), dict) else None; ev = sc.get('evidence', {}); snap = v.get('snapshot', {})
    info = {'fn': fn, 'sent': ts, 'answered': te, 'wall_s': round(wall, 3), 'round_trip_s': j.get('round_trip_seconds'), 'isError': r.get('isError'),
            'code': (err or {}).get('code'), 'cause': (err or {}).get('cause'), 'retry_after_s': (err or {}).get('retry_after_s'), 'next_step': (err or {}).get('next_step'),
            'evidence_bytes': ev.get('bytes'), 'evidence_state': ev.get('state'), 'reason_code': ev.get('reason_code'), 'total': v.get('total'),
            'rows': len(v.get('items', []) or []), 'next_offset': v.get('next_offset'), 'marks': (snap.get('master_alter_id'), snap.get('voucher_alter_id'), snap.get('reused')) if snap else None}
    if err: info['remediation'] = (err.get('remediation') or '')[:500]
    log(f'{label}: ' + json.dumps(info, ensure_ascii=False))
    return info, v
def egress(label):
    efn, ej, *_ = call(f'{label}-egress_log', 'egress_log', {'limit': 4})
    recs = [json.loads(x) for x in ej['response']['result']['structuredContent']['result']['records']]
    t = next((x for x in recs if x.get('record_type') == 'response_prepared' and x.get('tool') == 'ledger_masters'), None)
    if not t: log(f'  {efn}: no ledger_masters receipt'); return None
    tr = t.get('request_trail') or {}; last = tr.get('last', [])
    mp = [x for x in last if x['kind'] == 'post' and x.get('request_bytes') in (1736, 1784)]
    log(f'  {efn}: receipt {t.get("ts")} error {(t.get("error") or {}).get("code")} trail sent {tr.get("sent")} omitted {tr.get("omitted")} failed {tr.get("failed")} last_failed_seq {tr.get("last_failed_seq")}')
    log('    sends: ' + '; '.join(f'{x["seq"]}:{x["kind"]}/{x["outcome"]}/{x.get("held_ms")}ms/req {x.get("request_bytes")}/resp {x.get("response_bytes")}' for x in last))
    log('    master posts: ' + '; '.join(f'seq {x["seq"]} {x["outcome"]} held {x.get("held_ms")} ms req {x.get("request_bytes")} resp {x.get("response_bytes")}' for x in mp))
    return tr
STD = {'company_guid': G, 'fields': 'compliance'}; MAIL = dict(STD, include_mailing_details=True)
def rows_of(fns):
    out = []
    for fn in fns:
        v = json.load(open(f'{R}/answers/{fn}', encoding='utf-8'))['response']['result']['structuredContent']['result']
        out += v.get('items', [])
    return out
def read_rest(label, first_fn, v, args):
    fns = [first_fn]; page = 0
    while v.get('next_offset'):
        page += 1; fn, j, ts, wall, te = call(f'{label}-p{page}', 'ledger_masters', dict(args, offset=v['next_offset']))
        info, v = summarise(f'{label}-p{page}', fn, j, ts, wall, te); egress(f'{label}-p{page}'); fns.append(fn)
        if info['isError']: log(f'  stop rule: page {page} refused: {info["code"]}'); sys.exit(3)
    return fns
def std_loop(rung, label, d1_end):
    ref = sorted(f for f in os.listdir(R + '/answers') if f.endswith(f'-S{rung}.json'))
    k = 0; t0 = time.monotonic(); wait = 0
    while True:
        k += 1
        if wait: log(f'  waiting {wait} s (retry_after_s)'); time.sleep(wait)
        fn, j, ts, wall, te = call(f'{label}-D2-{k:02d}', 'ledger_masters', STD)
        info, v = summarise(f'{label}-D2-{k:02d}', fn, j, ts, wall, te); debt(f'{label}-D2-{k:02d}'); egress(f'{label}-D2-{k:02d}')
        if not info['isError']:
            fns = read_rest(f'{label}-D2-{k:02d}', fn, v, STD)
            gap = time.monotonic() - d1_end if d1_end else None
            log(f'  D3 admitted at call {k}; gap from D1 end to this call end: {gap:.1f} s' if gap is not None else f'  D3 admitted at call {k}')
            if ref:
                a = rows_of([ref[-1]]); b = rows_of(fns)
                sv = json.load(open(f'{R}/answers/{ref[-1]}', encoding='utf-8'))['response']['result']['structuredContent']['result']
                same = [json.dumps(x, sort_keys=True) for x in a] == [json.dumps(x, sort_keys=True) for x in b]
                log(f'  whole? total {info["total"]} vs S{rung} {sv.get("total")}; rows {len(b)} vs {len(a)}; every row equal as parsed JSON: {same}; snapshot marks {info["marks"]} vs S{rung} {(sv["snapshot"].get("master_alter_id"), sv["snapshot"].get("voucher_alter_id"))}')
            debt(f'{label}-D3')
            r = subprocess.run(['python', '-I', '<LAB_DIR>/marks.py', f'{label}-after-D3'], capture_output=True, text=True); log('  ' + r.stdout.strip())
            return True
        if info['code'] != DRAINING: log(f'  stop rule: a refusal the sheet does not expect: {info["code"]}'); sys.exit(3)
        if k >= 20 or time.monotonic() - t0 > 180: log(f'  stop rule: D2 bound reached ({k} calls, {time.monotonic() - t0:.1f} s)'); sys.exit(4)
        wait = info.get('retry_after_s') or 10
mode = sys.argv[1]; rung = sys.argv[2]; label = sys.argv[3]
if mode == 'cut':
    fn, j, ts, wall, te = call(f'{label}', 'ledger_masters', MAIL); d1_end = time.monotonic()
    info, v = summarise(label, fn, j, ts, wall, te)
    if not info['isError']:
        egress(label); read_rest(label, fn, v, MAIL); debt(label); log(f'  {label} answered: no cut at this rung'); sys.exit(0)
    debt(f'{label}-D1')
    egress(label)
    if info['code'] != EXPECTED_CUT: log(f'  stop rule: a refusal the sheet does not expect: {info["code"]}'); sys.exit(3)
    log(f'  D1 the cut: {info["code"]} cause {info["cause"]}; D2 starts {time.monotonic() - d1_end:.2f} s after D1 answered')
    std_loop(rung, label, d1_end)
elif mode == 'std':
    std_loop(rung, label, time.monotonic())
