# Lab helper (#2265 R4 on THE CLIENT BOOK, not product code): from Bridge's agent-egress.jsonl, prints kinds, bytes and
# times only: each record's type, tool, rows and bytes prepared and its request trail counts, and for sales_register the
# kept tail of the trail (seq, kind, request bytes, response bytes, held ms). Writes evidence/egress-summary.txt.
import json, collections
L = [json.loads(l) for l in open('<LAB_DIR>/agent/agent-egress.jsonl', encoding='utf-8')]
out = [f'records {len(L)}']
for x in L:
    t = x.get('request_trail') or {}
    out.append(f'{x.get("ts")} {x.get("record_type")} {x.get("tool")} rows {x.get("rows_prepared")} bytes {x.get("bytes_prepared")} trail ' + (json.dumps({k: v for k, v in t.items() if k != "last"}) if t else '-'))
    last = t.get('last') or []
    if last:
        out.append('  kept tail: ' + json.dumps(dict(collections.Counter(f'{y.get("kind")} {y.get("outcome")}' for y in last))) + f'; request bytes {sum(y.get("request_bytes") or 0 for y in last)}, response bytes {sum(y.get("response_bytes") or 0 for y in last)} (largest {max((y.get("response_bytes") or 0) for y in last)}), held ms {sum(y.get("held_ms") or 0 for y in last)} (longest {max((y.get("held_ms") or 0) for y in last)})')
        out.append('  seq:kind:request/response/held_ms  ' + ' '.join(f'{y["seq"]}:{y["kind"][0]}:{y.get("request_bytes") or 0}/{y.get("response_bytes")}/{y.get("held_ms")}' for y in last))
open('<LAB_DIR>/evidence/egress-summary.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n'); print('\n'.join(out))
