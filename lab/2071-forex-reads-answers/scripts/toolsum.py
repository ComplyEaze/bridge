# Lab helper (#2071, not product code): the fields the sheet asks for from one saved tool answer: isError, code, cause,
# remediation, bill_row, partial and partial_reasons, every list of set-aside names by its key, the rows returned, and the
# evidence bytes and response_sha256. Usage: python -I -X utf8 toolsum.py <answer file>
import json, sys
j = json.load(open(sys.argv[1], encoding='utf-8')); r = j['response'].get('result') or {}; sc = r.get('structuredContent') or {}
res = sc.get('result') if isinstance(sc.get('result'), dict) else {}
err = res.get('error') or {}
ev = sc.get('evidence') or {}
out = {'tool': j['tool'], 'isError': r.get('isError'), 'code': err.get('code'), 'cause': err.get('cause'), 'remediation': err.get('remediation'),
       'bill_row': err.get('bill_row'), 'partial': res.get('partial', sc.get('partial')), 'partial_reasons': res.get('partial_reasons', sc.get('partial_reasons')),
       'set_aside': {k: v for k, v in res.items() if k.endswith('_excluded')},
       'rows': {k: len(v) for k, v in res.items() if isinstance(v, list)}, 'total': res.get('total', res.get('total_ledgers')), 'state': res.get('state'),
       'evidence_bytes': ev.get('bytes'), 'response_sha256': ev.get('response_sha256'), 'evidence_state': ev.get('state'), 'reason_code': ev.get('reason_code')}
if j['tool'] == 'trial_balance': out['ledgers'] = [l.get('ledger') for l in res.get('ledgers', [])]
if j['tool'] == 'ledger_masters': out['items'] = [l.get('name') for l in res.get('items', [])]
if j['tool'] == 'outstandings' and not err: out['result_keys'] = sorted(res.keys())
print(json.dumps(out, ensure_ascii=False))
