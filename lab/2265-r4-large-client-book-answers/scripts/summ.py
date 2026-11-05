# Lab helper (#2265 R4 on THE CLIENT BOOK, not product code): reads one saved tool answer and prints only shapes and counts:
# seconds, answer file bytes, isError, rpc error presence, state, reason, error code / cause / failed_request (their keys and
# short codes only), Tally bytes and duration from evidence, ledger_masters_observed, vouchers_observed, and for every list
# its length and every dict with a 'total' its total. Never a name, an amount, a narration or a GUID.
# Usage: python -I -X utf8 summ.py <answer file>
import json, os, re, sys
f = sys.argv[1]; j = json.load(open(f, encoding='utf-8')); resp = j['response']; r = resp.get('result') or {}
sc = r.get('structuredContent') or {}; res = sc.get('result') if isinstance(sc.get('result'), dict) else {}
ev = sc.get('evidence') or {}
def code(x): return x if isinstance(x, str) and re.fullmatch(r'[a-z0-9_.:-]{1,80}', x) else ('<text>' if x else x)
out = [f'{j["tool"]}: seconds {j["round_trip_seconds"]}; answer file {os.path.getsize(f)} B; isError {r.get("isError")}; rpc error {bool(resp.get("error"))}',
       f'  evidence: state {ev.get("state")}, Tally bytes {ev.get("bytes")}, duration_ms {ev.get("duration_ms")}, reason_code {code(ev.get("reason_code"))}; truncated {sc.get("truncated")}']
err = res.get('error') if isinstance(res.get('error'), dict) else None
if err: out.append('  error: ' + ', '.join(f'{k} {code(v) if not isinstance(v, (dict, list)) else type(v).__name__}' for k, v in err.items() if k in ('code', 'cause', 'failed_request', 'retryable')))
fr = res.get('failed_request') or (err or {}).get('failed_request')
if fr is not None: out.append(f'  failed_request: {json.dumps(fr)[:200] if not isinstance(fr, str) else code(fr)}')
out.append(f'  state {res.get("state")}; reason {code(res.get("reason"))}; ledger_masters_observed {res.get("ledger_masters_observed")}; vouchers_observed {res.get("vouchers_observed")}; total {res.get("total")}; next_offset {res.get("next_offset")}')
for k, v in sorted(res.items()):
    if isinstance(v, list): out.append(f'  {k}: list of {len(v)}')
    elif isinstance(v, dict) and 'total' in v: out.append(f'  {k}: total {v.get("total")}, listed {len(v.get("listed") or [])}, listed_truncated {v.get("listed_truncated")}')
    elif isinstance(v, (int, float)) and not isinstance(v, bool) and k not in ('ledger_masters_observed', 'vouchers_observed', 'total', 'offset', 'next_offset'): out.append(f'  {k}: {v}')
print('\n'.join(out))
