# Lab helper (1108-A2 sitting 2, not product code; written whole by an editor): prints, from one saved post_import or
# verify_import answer, the fields the sheet's step 6.4 names: created, attempt_recorded, dispatch (state, response_state,
# resent), dispatch.response.outcome's counters and tally_line_errors, error.code and next_step, each voucher's status,
# attribution and next step, duplicates; and verification_status and counts. Reads the file only; THE COMPANY's GUID and
# number are printed as <company guid> and <company number>.
# Usage: python -I -X utf8 fields.py <answer file>
import json, re, sys
R = '<LAB_DIR>'
G = open(R + '/guid.txt').read().strip(); NUM = open(R + '/company-number.txt').read().strip()
j = json.load(open(sys.argv[1], encoding='utf-8')); sc = j['response']['result']['structuredContent']; r = sc['result']
def show(x): return re.sub(r'(?<!\d)' + NUM + r'(?!\d)', '<company number>', json.dumps(x, ensure_ascii=False).replace(G, '<company guid>'))
print(f"{j['tool']} isError {j['response']['result'].get('isError')} round trip {j['round_trip_seconds']} s, sent {j['call_sent_utc']}")
d = r.get('dispatch') or {}
print('created:', show(r.get('created')), '| attempt_recorded:', show(r.get('attempt_recorded')))
print('dispatch: state', show(d.get('state')), '| response_state', show(d.get('response_state')), '| resent', show(d.get('resent')))
o = ((d.get('response') or {}).get('outcome')) or d.get('outcome') or {}
print('dispatch.response.outcome counters:', show(o.get('counters') if o else d.get('counters')))
print('tally_line_errors:', show(o.get('tally_line_errors') if o else d.get('tally_line_errors')))
if o: print('outcome other keys:', show({k: v for k, v in o.items() if k not in ('counters', 'tally_line_errors')}))
e = r.get('error') or {}
print('error.code:', show(e.get('code')), '| error.message:', show(e.get('message')))
print('next_step:', show(r.get('next_step')))
print('verification_status:', show(r.get('verification_status')), '| counts (non-zero):', show({k: v for k, v in (r.get('counts') or {}).items() if v}))
for v in (r.get('vouchers') or r.get('items') or []):
    print('voucher', show(v.get('bridge_txn_id')), '| status', show(v.get('status')), '| attribution', show(v.get('attribution')),
          '| next_step', show(v.get('next_step')), '| other', show({k: x for k, x in v.items() if k in ('unmatched_cause', 'matches', 'ambiguous_within_batch', 'reason', 'cause', 'guid', 'master_id')}))
print('duplicates:', show(r.get('duplicates')))
print('post_span_binding:', show(r.get('post_span_binding')))
