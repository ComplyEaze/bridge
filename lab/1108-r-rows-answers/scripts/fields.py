# Lab helper (rows 1108-R0 to R2, new; not product code): prints the fields the sheet's step 7.7 asks to record from one
# post_import or verify_import answer (answers kept whole by the client): dispatch state, response_state, resent, the
# outcome's counters and tally_line_errors; post_span_binding; error.code; next_step; each voucher's status, attribution,
# marker and next step from vouchers and unverified_vouchers (found anywhere in the answer by those keys); duplicates; and
# whether a key named created or attempt_recorded is present anywhere. Reads only. Usage: python -I -X utf8 fields.py <answer>
import json, sys
j = json.load(open(sys.argv[1], encoding='utf-8')); r = j['response']['result']['structuredContent']['result']
def walk(o, path=''):
    if isinstance(o, dict):
        for k, v in o.items(): yield path + '/' + k, k, v; yield from walk(v, path + '/' + k)
    elif isinstance(o, list):
        for i, v in enumerate(o): yield from walk(v, f'{path}[{i}]')
d = r.get('dispatch') or {}
print('call_sent_utc', j.get('call_sent_utc'), 'round_trip_seconds', j.get('round_trip_seconds'), 'isError', j['response']['result'].get('isError'))
print('verification_status', r.get('verification_status'))
print('dispatch.state', d.get('state'), '| response_state', d.get('response_state'), '| resent', d.get('resent'))
oc = (d.get('response') or {}).get('outcome') or d
print('outcome counters', json.dumps({k: v for k, v in (oc.get('counters') or {}).items() if k != 'counter_presence'}))
print('tally_line_errors', json.dumps(oc.get('tally_line_errors', d.get('tally_line_errors')))[:600])
print('post_span_binding', json.dumps(r.get('post_span_binding'))[:900])
print('error.code', (r.get('error') or {}).get('code'), '| next_step', json.dumps(r.get('next_step'))[:400])
print('counts', json.dumps(r.get('counts')))
for key in ('vouchers', 'unverified_vouchers', 'items'):
    for v in (r.get(key) or []):
        if isinstance(v, dict):
            print(key, json.dumps({k: v.get(k) for k in ('bridge_txn_id', 'status', 'attribution', 'marker', 'next_step') if k in v})[:500])
print('duplicates', json.dumps(r.get('duplicates'))[:400])
print('approval', json.dumps(r.get('approval')), '| attempt_recorded present', any(k == 'attempt_recorded' for _, k, _ in walk(r)),
      '| created present', any(k == 'created' for _, k, _ in walk(r)), '| top-level attempt_recorded', r.get('attempt_recorded', '(absent)'))
