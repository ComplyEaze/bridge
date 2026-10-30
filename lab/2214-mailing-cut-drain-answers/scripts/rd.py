# Lab helper (#2214, not product code): one STANDARD or MAILING read through the row's client, every page by offset (each
# page a call of its own), then egress_log for each call, then a copy of the drain record. Prints per call: wall seconds,
# isError, code, cause, retry_after_s, total, evidence bytes, rows, snapshot marks, next_offset; per trail: sent, omitted,
# failed, every kept send (kind, outcome, held_ms, request and response bytes), and the master posts (the two posts with
# the largest request bytes). Everything also goes to evidence/reads-log.txt.
# Usage: python -I -X utf8 rd.py <label> standard|mailing [--one-page]
import hashlib, json, os, sys, time
R = '<LAB_DIR>'; G = '<company guid>'
DEBT = os.path.join(os.environ['USERPROFILE'], '.complyeaze-bridge', 'coordination', 'native-dispatch-leases',
                    'a905deaa97bc7d8acb82123c4057a3890b174daf0295ee5c10be17f6fa0f0a01.wire.debt')
def now(): t = time.time(); return time.strftime('%Y-%m-%dT%H:%M:%S', time.gmtime(t)) + f'.{int(t * 1000) % 1000:03d}Z'
def log(s):
    with open(R + '/evidence/reads-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
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
    return fn, j, ts, time.monotonic() - t0
def debt(label):
    os.makedirs(R + '/debt', exist_ok=True); seq = len(os.listdir(R + '/debt'))
    try: d = open(DEBT, 'rb').read()
    except FileNotFoundError: d = None
    open(f'{R}/debt/{seq:03d}-{label}.' + ('json' if d is not None else 'absent.txt'), 'wb').write(d if d is not None else b'absent\n')
    log(f'  drain record after {label} at {now()}: ' + ('absent' if d is None else f'{len(d)} B sha256 {hashlib.sha256(d).hexdigest()} {d.decode("utf-8", "replace")}'))
def trail_of(efn, ej, tool_ts):
    sc = ej['response']['result']['structuredContent']; recs = [json.loads(r) for r in sc['result']['records']]
    mine = [r for r in recs if r.get('record_type') == 'response_prepared' and r.get('tool') == 'ledger_masters']
    return mine[0] if mine else None
def summarise(label, fn, j, ts, wall):
    r = j['response'].get('result') or {}; sc = r.get('structuredContent') or {}; v = sc.get('result', {}) if isinstance(sc.get('result'), dict) else {}
    err = v.get('error') if isinstance(v.get('error'), dict) else None; ev = sc.get('evidence', {}); snap = v.get('snapshot', {})
    info = {'fn': fn, 'sent': ts, 'wall_s': round(wall, 3), 'round_trip_s': j.get('round_trip_seconds'), 'isError': r.get('isError'),
            'code': (err or {}).get('code'), 'cause': (err or {}).get('cause'), 'retry_after_s': (err or {}).get('retry_after_s'),
            'next_step': (err or {}).get('next_step'), 'evidence_bytes': ev.get('bytes'), 'evidence_state': ev.get('state'), 'reason_code': ev.get('reason_code'),
            'total': v.get('total'), 'rows': len(v.get('items', []) or []), 'next_offset': v.get('next_offset'),
            'marks': (snap.get('master_alter_id'), snap.get('voucher_alter_id'), snap.get('reused')) if snap else None}
    if err: info['remediation'] = (err.get('remediation') or '')[:400]
    log(f'{label}: ' + json.dumps(info, ensure_ascii=False))
    return info, v
def egress(label):
    efn, ej, ets, ewall = call(f'{label}-egress_log', 'egress_log', {'limit': 4})
    t = trail_of(efn, ej, ets)
    if not t: log(f'  {efn}: no ledger_masters receipt found'); return None
    tr = t.get('request_trail') or {}; last = tr.get('last', [])
    # the ledger master read pair: on this build its request is 1,736 B (STANDARD) or 1,784 B (MAILING), the only post whose
    # request changes with include_mailing_details (Part R); the sheet's 'largest request bytes' picks a 2,524 B company read
    mp = [x for x in last if x['kind'] == 'post' and x.get('request_bytes') in (1736, 1784)]
    log(f'  {efn}: receipt {t.get("ts")} error {(t.get("error") or {}).get("code")} trail sent {tr.get("sent")} omitted {tr.get("omitted")} failed {tr.get("failed")} last_failed_seq {tr.get("last_failed_seq")}')
    log('    sends: ' + '; '.join(f'{x["seq"]}:{x["kind"]}/{x["outcome"]}/{x.get("held_ms")}ms/req {x.get("request_bytes")}/resp {x.get("response_bytes")}' for x in last))
    log('    master posts: ' + '; '.join(f'seq {x["seq"]} {x["outcome"]} held {x.get("held_ms")} ms req {x.get("request_bytes")} resp {x.get("response_bytes")}' for x in mp))
    return tr
label, kind = sys.argv[1], sys.argv[2]; one = '--one-page' in sys.argv
args = {'company_guid': G, 'fields': 'compliance'}
if kind == 'mailing': args['include_mailing_details'] = True
off = 0; page = 0
while True:
    a = dict(args);
    if off: a['offset'] = off
    pl = f'{label}' + (f'-p{page}' if page else '')
    fn, j, ts, wall = call(pl, 'ledger_masters', a)
    info, v = summarise(pl, fn, j, ts, wall)
    egress(pl)
    if info['isError'] or one or not info['next_offset']: break
    off = info['next_offset']; page += 1
debt(label)
