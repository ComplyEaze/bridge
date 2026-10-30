# Lab helper (#1688, not product code): the table of one set of THE BRIDGE READS, from the saved answers (no call):
# each validate_masters row (requested, match_state, alias_of, folded_twins, through_alias and every other field),
# ledger_masters's total, snapshot and items, and every egress record of the set's calls with its request_trail sends
# (kind, outcome, held_ms, request and response bytes). Also checks each answer came from the lab's own server.
# Usage: python -I -X utf8 btable.py <set>
import json, os, sys
W = '<LAB_DIR>'; SD = W + '/srv'; E = W + '/evidence'; SET = sys.argv[1]
own_pid = open(SD + '/server.pid').read().strip()
def ans(suffix):
    fn = [f for f in sorted(os.listdir(SD + '/answers')) if f.endswith(f'-{SET}-{suffix}.json')][-1]
    return fn, json.load(open(f'{SD}/answers/{fn}', encoding='utf-8'))
out = [f'THE BRIDGE READS, set {SET}']
for suffix in ('validate', 'lm', 'egress'):
    fn, j = ans(suffix)
    res = j['response'].get('result') or {}; sc = res.get('structuredContent') or {}; body = sc.get('result') or {}
    out.append(f'== {fn}: server_pid {j["server_pid"]} (own server {own_pid}: {str(j["server_pid"]) == own_pid}) {j["server_info"]}; isError {res.get("isError")}; round trip {j["round_trip_seconds"]} s')
    if res.get('isError'): out.append('   refusal: ' + json.dumps(sc, ensure_ascii=False)); continue
    if suffix == 'validate':
        out.append(f'   catalogue_evidence_sha256 {body.get("catalogue_evidence_sha256")}; rows {len(body.get("masters", []))}')
        for i, m in enumerate(body.get('masters', []), 1):
            main = {k: m.get(k) for k in ('requested', 'match_state', 'alias_of', 'folded_twins', 'through_alias')}
            rest = {k: v for k, v in m.items() if k not in main}
            out.append(f'   {i:2}. ' + json.dumps(main, ensure_ascii=False) + ' | ' + json.dumps(rest, ensure_ascii=False))
    elif suffix == 'lm':
        out.append(f'   total {body.get("total")}; snapshot {json.dumps(body.get("snapshot"))}')
        for it in body.get('items', []): out.append('   ' + json.dumps(it, ensure_ascii=False))
    else:
        for raw in body.get('records', []):
            rec = json.loads(raw) if isinstance(raw, str) else raw
            if rec.get('record_type') == 'response_prepared' or rec.get('request_trail'):
                rt = rec.get('request_trail') or {}
                trail = rt.get('last', []) if isinstance(rt, dict) else rt
                extra = {k: v for k, v in rt.items() if k != 'last'} if isinstance(rt, dict) else {}
                out.append(f'   {rec.get("ts")} {rec.get("record_type")} {rec.get("tool")} {json.dumps(extra)}: ' + ('; '.join(
                    f'{t.get("kind")} {t.get("outcome")} held_ms {t.get("held_ms")} request {t.get("request_bytes")} response {t.get("response_bytes")}' for t in trail) or 'no request_trail'))
            else:
                out.append(f'   {rec.get("ts")} {rec.get("record_type")}')
open(f'{E}/{SET}-bridge.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
print('\n'.join(out))
