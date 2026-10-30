# Lab helper (#2027 step 2, not product code): ONE ledger_masters basic read (first page, no snapshot_id) through the
# row's client, then M, V, reused, total, next_offset and whether a ledger named BRIDGE IGST is in the list.
# Usage: python -I -X utf8 lm.py <label>
import json, subprocess, sys
R = '<LAB_DIR>'; G = '<company guid>'
out = subprocess.run(['python', '-I', '-X', 'utf8', '<lab tools>/submit.py', R, sys.argv[1], 'ledger_masters', json.dumps({'company_guid': G})], capture_output=True, text=True, encoding='utf-8')
fn = out.stdout.splitlines()[0].split()[0]
j = json.load(open(f'{R}/answers/{fn}', encoding='utf-8')); r = j['response']['result']; sc = r.get('structuredContent', {}); v = sc.get('result', sc)
if r.get('isError'): print(fn, 'isError', json.dumps(sc)[:1500]); sys.exit(1)
snap = v.get('snapshot', {}); names = [it['name'] for it in v.get('items', [])]
print(f'{fn} M {snap.get("master_alter_id")} V {snap.get("voucher_alter_id")} reused {snap.get("reused")} total {v.get("total")} items {len(names)} next_offset {v.get("next_offset")} BRIDGE IGST listed: {"BRIDGE IGST" in names}')
