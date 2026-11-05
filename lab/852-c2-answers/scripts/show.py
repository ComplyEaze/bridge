# Lab helper (852-C2, from 1392-E, not product code): prints the parts of one saved tool answer the sheet's EXPECTs name (state,
# counts, skipped, counterparties, proposals id, batch id, voucher count, dispatch, approval, verification) and the whole
# transfers section. Reads the file only. Usage: python -I -X utf8 show.py <answer file>
import json, sys
j = json.load(open(sys.argv[1], encoding='utf-8')); resp = j['response']; r = resp.get('result') or {}
sc = r.get('structuredContent'); res = (sc or {}).get('result') if isinstance(sc, dict) else None
print(f"{j['tool']} secs {j['round_trip_seconds']} isError {r.get('isError')} rpc_error {resp.get('error')}")
if not isinstance(res, dict):
    print('no result object; structuredContent keys', list((sc or {}).keys()) if isinstance(sc, dict) else None)
    print(json.dumps(sc or r, ensure_ascii=False)[:3000]); sys.exit()
print('result keys', sorted(res))
for k in ('state', 'status', 'reconciliation', 'reconciled', 'verdict', 'reason', 'code', 'refusal_reason', 'next_step', 'retry_after_s', 'attempt_recorded',
          'proposals_id', 'sha256', 'batch_id', 'voucher_count', 'skipped', 'skipped_rows', 'rows', 'row_count', 'counterparties', 'verification_status', 'counts',
          'approval', 'dispatch', 'resent', 'controls', 'coverage', 'warnings'):
    if k in res:
        v = res[k]; s = json.dumps(v, ensure_ascii=False)
        print(f'{k}: {s if len(s) < 1500 else s[:1500] + " ..."}')
if 'transfers' in res: print('transfers:', json.dumps(res['transfers'], ensure_ascii=False, indent=1))
