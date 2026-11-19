# Lab helper (2440-CP): one post_import approval attempt through call.py, following each pending answer after its
# retry_after_s (the tool's own flow, also an approved-without-send answer at once); stops at any other answer.
# Usage: python -I -X utf8 postloop.py <label prefix> <batch_id>
import json, subprocess, sys, time, glob, os
pre, batch = sys.argv[1:3]; R = '<lab folder>/run'
args = json.dumps({'company_guid': 'eb2f5190-292e-4ed8-bfd3-eace839fe699', 'batch_id': batch})
for i in range(1, 30):
    subprocess.run([sys.executable, '-I', '-X', 'utf8', R + '/call.py', f'{pre}-{i}', 'post_import', args])
    fn = sorted(glob.glob(f'{R}/answers/*-{pre}-{i}.json'))[-1]
    sc = json.load(open(fn, encoding='utf-8'))['response']['result'].get('structuredContent') or {}
    res = sc.get('result') or {}; ap = res.get('approval') or {}
    st = ap.get('state'); print(f'POSTLOOP {i} approval={st} dispatch={(res.get("dispatch") or {}).get("state")} error={(res.get("error") or {}).get("code")}', flush=True)
    if st == 'pending': time.sleep(ap.get('retry_after_s', 15)); continue
    if st == 'approved' and (res.get('dispatch') or {}).get('state') == 'not_dispatched': continue
    break
