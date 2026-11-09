# Lab helper (row 703-G, not product code): the sheet's records from one run folder's answers: marks, vouchers total and
# snapshot, each gst_status total, counts, state and next_offset. Prints no party, amount or GSTIN.
import json, os, sys
A = f'<row folder>/{sys.argv[1]}/answers'
def body(n):
    j = json.load(open(f'{A}/{n}', encoding='utf-8')); r = j['response'].get('result') or {}
    return r.get('isError'), json.loads(r['content'][0]['text'])
for n in sorted(os.listdir(A)):
    if 'marks' in n:
        e, d = body(n); s = d.get('result', {}).get('snapshot') or {}
        print(n, 'isError', e, 'marks: master_alter_id', s.get('master_alter_id'), 'voucher_alter_id', s.get('voucher_alter_id'), 'reused', s.get('reused'), 'ledgers', d.get('result', {}).get('total'))
    elif 'vouchers' in n:
        e, d = body(n); res = d.get('result', {})
        print(n, 'isError', e, 'total', res.get('total'), 'state', res.get('state'), res.get('reason'), 'snapshot', json.dumps(res.get('snapshot')))
    elif 'gst' in n:
        e, d = body(n); res = d.get('result', {})
        print(n, 'isError', e, 'total', res.get('total'), 'state', res.get('state'), 'counts', json.dumps(res.get('counts')), 'next_offset', res.get('next_offset'))
