# Lab helper (1029-R, not product code): prints the Record table's fields from one saved tool answer: for post_import the
# approval state, attempt_recorded, dispatch state, response_state, resent, Tally's counters and any error (code,
# retry_after_s, next_step); for verify_import its status counts; for egress_log the records' kinds and any deadline or
# refused part. Long digit runs are printed as <long>. Usage: python -I -X utf8 fields.py <answer file>
import collections, json, re, sys
j = json.load(open(sys.argv[1], encoding='utf-8')); r = j['response'].get('result') or {}
sc = r.get('structuredContent') or {}; res = sc.get('result') if isinstance(sc, dict) else None
def p(*a): print(re.sub(r'\d{10,}', '<long>', ' '.join(str(x) for x in a)))
p('tool', j['tool'], 'sent', j['call_sent_utc'], 'secs', j['round_trip_seconds'], 'isError', r.get('isError'))
if not isinstance(res, dict): p('no result:', json.dumps(sc or r)[:1500]); sys.exit()
p('keys', sorted(res))
if j['tool'] == 'post_import':
    d = res.get('dispatch') or {}
    p('approval', json.dumps(res.get('approval')))
    p('attempt_recorded', res.get('attempt_recorded'), '| dispatch.state', d.get('state'), '| response_state', d.get('response_state'), '| resent', d.get('resent'))
    p('counters', {k: v for k, v in (d.get('counters') or {}).items() if k != 'counter_presence'})
    p('error', json.dumps(res.get('error') or sc.get('error')))
    for k in ('state', 'verification', 'next_step', 'retry_after_s'):
        if k in res: p(k, json.dumps(res[k])[:600])
elif j['tool'] == 'verify_import':
    for k, v in res.items():
        if k in ('state', 'status', 'verdict', 'counts', 'status_counts', 'summary', 'next_step', 'error', 'retry_after_s'): p(k, json.dumps(v)[:600])
    for k, v in res.items():
        if isinstance(v, list) and v and isinstance(v[0], dict):
            c = collections.Counter(x.get('status') or x.get('state') for x in v); p(k, len(v), dict(c))
elif j['tool'] == 'egress_log':
    for k, v in res.items():
        if isinstance(v, list):
            p(k, len(v))
            for x in v: p('  ', json.dumps(x)[:260])
        else: p(k, json.dumps(v)[:300])
