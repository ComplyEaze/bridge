# Lab helper (row 1780-W S3, new; not product code; written whole by an editor): queues build_import_xml with the exact content of
# requests\build-1591-s1.json (its sha256 checked first) through this row's stdio client, and prints result.voucher_count, the totals,
# verification_preflight whole and result.batch_id. Nothing is posted. Usage: python -I -X utf8 s3.py
import hashlib, json, sys
sys.path.insert(0, '<LAB_DIR>')
from call import call  # noqa: E402
P = '<LAB_DIR>\\requests\\build-1591-s1.json'
d = open(P, 'rb').read()
assert hashlib.sha256(d).hexdigest() == '5f8a5ff24d416331ec2706774d178ba615759abafa87b7c408c279eca88bf13a', 'input differs'
fn, secs, is_err, body = call('s3-build', 'build_import_xml', json.loads(d.decode('utf-8')))
print(fn, f'{secs:.3f}s', 'isError', is_err)
if not isinstance(body, dict):
    print(str(body)[:2000]); sys.exit(2)
r = body.get('result') or {}
print('voucher_count', r.get('voucher_count'), 'total_debit', r.get('total_debit'), 'total_credit', r.get('total_credit'))
print('batch_id', r.get('batch_id'))
print('verification_preflight', json.dumps(r.get('verification_preflight', body.get('verification_preflight'))))
print('keys', sorted(r))
