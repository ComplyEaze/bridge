# Lab helper (#2074-L, not product code): ONE ledger_masters read through the row's client (submit.py), then a summary:
# the answer's evidence bytes and duration, the round trip, the ledger total and next_offset, and for each probe ledger
# ("Lab Mark Probe", "Lab Mark Probe Two") and a count of every other ledger's mailing state, its compliance.mailing_details
# as the tool gives it. Usage: python -I -X utf8 lm.py <label> basic|mailing
import json, subprocess, sys
label, kind = sys.argv[1], sys.argv[2]
G = 'a30f23c3-4819-4334-96ab-ffa6fba70976'; R = '<LAB_DIR>'
args = {'company_guid': G} if kind == 'basic' else {'company_guid': G, 'fields': 'compliance', 'include_mailing_details': True}
out = subprocess.run(['python', '-I', '-X', 'utf8', '<lab tools>/submit.py', R, label, 'ledger_masters', json.dumps(args)], capture_output=True, text=True, encoding='utf-8')
first = out.stdout.splitlines()[0]; fn = first.split()[0]
j = json.load(open(f'{R}/answers/{fn}', encoding='utf-8')); r = j['response']['result']; sc = r.get('structuredContent', {}); v = sc.get('result', sc)
ev = sc.get('evidence', {})
print(f'{fn} isError {r.get("isError")} round_trip {j["round_trip_seconds"]} s; evidence bytes {ev.get("bytes")} duration_ms {ev.get("duration_ms")} state {ev.get("state")}')
if r.get('isError'): print(json.dumps(sc)[:1500]); sys.exit(1)
items = v.get('items', []); snap = v.get('snapshot', {})
print(f'total {v.get("total")} items {len(items)} next_offset {v.get("next_offset")} snapshot M {snap.get("master_alter_id")} V {snap.get("voucher_alter_id")} reused {snap.get("reused")}')
others = {}
for it in items:
    md = (it.get('compliance') or {}).get('mailing_details', it.get('mailing_details'))
    if it['name'] in ('Lab Mark Probe', 'Lab Mark Probe Two'):
        print(f'  {it["name"]}: mailing_details {json.dumps(md, ensure_ascii=False)[:1500]}')
    else:
        key = 'absent' if md is None else (md if isinstance(md, str) else (f"{md.get('observation')}/{len(md.get('entries') or [])}" if isinstance(md, dict) else f'list of {len(md)}'))
        others[str(key)] = others.get(str(key), 0) + 1
print(f'  other ledgers by mailing state: {others}')
if kind != 'basic' and items: print('  compliance keys of the first item:', sorted((items[0].get('compliance') or {}).keys()))
