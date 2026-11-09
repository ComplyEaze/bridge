# Lab helper (row 703-LO, not product code): one party's outcome line and request trail table, counts and sizes only, no name.
# Usage: python -I -X utf8 lo.py <label> <outstandings answer> <egress_log answer>  -> prints; the caller saves it.
import json, sys
label, af, ef = sys.argv[1:4]
def sc(f):
    j = json.load(open(f, encoding='utf-8')); r = j['response']['result']; return j, (r.get('structuredContent') or json.loads(r['content'][0]['text']))
j, o = sc(af); r = o['result']; isErr = j['response']['result'].get('isError')
whole = j['round_trip_seconds']
bills = r.get('bills') or []; oa = r.get('on_account') or []
tie = r.get('checks', {}).get('bills_sum_to_ledger_closing')
print(f"{label}: isError {isErr}; written {j['call_sent_utc']} read {j['answer_read_utc']}; whole time {whole} s; state {r.get('state')}; basis {r.get('basis')}; detail {r.get('detail')}; as_of {r.get('as_of')}; "
      f"bills rows {len(bills)}; on_account rows {len(oa)}; bills plus on_account {len(bills) + len(oa)}; checks bills_sum_to_ledger_closing {tie}, company_marks_unchanged {r.get('checks', {}).get('company_marks_unchanged')}; "
      f"ledger_match.matched {r.get('ledger_match', {}).get('matched')}; verification {r.get('verification')}")
_, e = sc(ef); recs = [json.loads(x) if isinstance(x, str) else x for x in e['result']['records']]
rp = [x for x in recs if x.get('record_type') == 'response_prepared']  # in the egress log's order: the most recent first
mine = [x for x in rp if x.get('tool') == 'outstandings'][0]; prev = rp[rp.index(mine) + 1] if rp.index(mine) + 1 < len(rp) else None

t = mine['request_trail']; ents = t['last']  # one call's own trail: seq counts from 1 in each call; the record keeps 32 entries
print(f"trail (the most recent outstandings response_prepared record, receipt {mine['receipt_id']}): sent {t['sent']}, failed {t['failed']}, omitted {t['omitted']}; entries kept {len(ents)}, seq {ents[0]['seq']} to {ents[-1]['seq']}")
posts = [x for x in ents if x['kind'] == 'post']
# Pairs among the kept entries (by inference, by position and size as row 2119-P did): snapshot = consecutive equal-sized posts of 1290 B (row 2119-P's size);
# Ledger Outstandings = the consecutive equal-sized pair with the largest answers (its answers grow with the party's bills).
snap = lo = None; best = -1
for i in range(len(posts) - 1):
    a, b = posts[i], posts[i + 1]
    if a['request_bytes'] != b['request_bytes'] or a['response_bytes'] != b['response_bytes']: continue
    if snap is None and a['request_bytes'] == 1290: snap = (a['seq'], b['seq'])
    if a['response_bytes'] > best: best = a['response_bytes']; lo = (a['seq'], b['seq'])
if lo == snap: lo = None
print('| seq | kind | request_bytes | response_bytes | held_ms | outcome | pair (by inference) |'); print('|---|---|---|---|---|---|---|')
for x in ents:
    m = 'snapshot' if snap and x['seq'] in snap else 'Ledger Outstandings' if lo and x['seq'] in lo else ''
    print(f"| {x['seq']} | {x['kind']} | {x.get('request_bytes', '')} | {x['response_bytes']} | {x['held_ms']} | {x['outcome']} | {m} |")
held = {x['seq']: x['held_ms'] for x in ents}
s_ms = sum(held[s] for s in snap) if snap else None; l_ms = sum(held[s] for s in lo) if lo else None
rest = round(whole * 1000 - (s_ms or 0) - (l_ms or 0))
print(f"{label} summary: bills {len(bills) + len(oa)}; whole {whole} s; Ledger Outstandings pair seq {lo} held_ms {[held[s] for s in lo] if lo else None} sum {l_ms}; snapshot pair seq {snap} held_ms {[held[s] for s in snap] if snap else None} sum {s_ms}; rest of the call {rest} ms; posts in this call {len(posts)}, held_ms sum of all posts {sum(x['held_ms'] for x in posts)}")
