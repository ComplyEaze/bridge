# Lab helper (row 2119-P, not product code): reads one outstandings answer and one egress_log answer and prints the
# sheet's EXPECT fields and the trail table, never a party name. The sum of bills[].closing is a decimal sum; it is
# compared with the sum the earlier row kept (given on the command line) and only "equal" or "not equal" is printed;
# the sum itself goes to evidence/<label>-sum-local.txt, which stays on the lab PC.
# Usage: python -I -X utf8 out.py <label> <outstandings answer> <egress answer> <expected bills> <expected sum>
import json, sys
from decimal import Decimal
label, fa, fe, nb, esum = sys.argv[1:6]
import hashlib
R = '<lab root>/2119-p'
j = json.load(open(fa, encoding='utf-8')); resp = j['response']['result']; sc = resp['structuredContent']; r = sc['result']
bills = r.get('bills') or []; s = sum(Decimal(b['closing']) for b in bills) if bills else Decimal(0)
open(f'{R}/evidence/{label}-sum-local.txt', 'w', encoding='utf-8', newline='\n').write(f'{label}: sum of bills[].closing {s}; kept sum {esum}\n')
lm = r.get('ledger_match') or {}
exp = {'isError': (resp.get('isError'), False), 'state': (r.get('state'), 'observed'), 'basis': (r.get('basis'), 'tally_ledger_outstandings'),
       'detail': (r.get('detail'), 'open_bills'), 'as_of': (r.get('as_of'), '20260331'), 'bills': (len(bills), int(nb)),
       'on_account': (len(r.get('on_account') or []), 0), 'checks.bills_sum_to_ledger_closing': ((r.get('checks') or {}).get('bills_sum_to_ledger_closing'), 'tied'),
       'checks.company_marks_unchanged': ((r.get('checks') or {}).get('company_marks_unchanged'), 'checked'), 'ledger_match.matched': (lm.get('matched'), 'exact'),
       'sum equal to kept': (s == Decimal(esum), True)}
lines = [f'{label}: whole time {j["round_trip_seconds"]} s (written {j["call_sent_utc"]}, read {j["answer_read_utc"]}); answer bytes {len(open(fa, "rb").read())}; truncated {sc.get("truncated")}; verification {r.get("verification")}']
for k, (got, want) in exp.items(): lines.append(f'  {k}: {got} (EXPECT {want}) {"met" if got == want else "NOT MET"}')
lines.append(f'  similar_ledgers_total {lm.get("similar_ledgers_total")}; evidence {json.dumps({k: v for k, v in (sc.get("evidence") or {}).items() if k in ("bytes", "duration_ms", "state", "reason_code")})}')
# the trail: the egress_log record of this call, response_prepared
e = json.load(open(fe, encoding='utf-8'))['response']['result']['structuredContent']['result']
recs = [json.loads(x) if isinstance(x, str) else x for x in (e.get('records') or [])]
mine = [x for x in recs if x.get('tool') == 'outstandings' and x.get('record_type') == 'response_prepared' and j['call_sent_utc'] <= x.get('ts', '') <= j['answer_read_utc']]
lines.append(f'  egress_log records {len(recs)}; outstandings response_prepared records {len(mine)}')
for x in mine[-1:]:
    t = x.get('request_trail') or {}
    lines.append(f'  request_trail: sent {t.get("sent")} failed {t.get("failed")} omitted {t.get("omitted")}; record time {x.get("at") or x.get("ts") or x.get("time")}')
    lines.append('  trail keys: ' + ', '.join(f'{k}={v}' for k, v in t.items() if not isinstance(v, list)))
    lines.append('  | seq | kind | request_bytes | response_bytes | held_ms | outcome |')
    lines.append('  |---|---|---|---|---|---|')
    for en in t.get('last') or []:
        lines.append(f'  | {en.get("seq")} | {en.get("kind")} | {en.get("request_bytes")} | {en.get("response_bytes")} | {en.get("held_ms")} | {en.get("outcome")} |')
out = '\n'.join(lines); print(out)
open(f'{R}/evidence/{label}-outcome.txt', 'w', encoding='utf-8', newline='\n').write(out + '\n')
