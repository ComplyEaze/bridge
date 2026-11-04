# Lab helper (1379 Part R, not product code): one sheet call (r2 to r8) through call.py, then each further page while the
# answer says more rows follow: called again with the offset it gives and the snapshot_id it returns. One call at a time.
# Prints, for each page, isError, the error code if any, the rows on the page, total, next offset and snapshot id.
# Usage: python -I -X utf8 pages.py <label> <tool> <json args>
import glob, json, subprocess, sys
W = '<LAB_DIR>'
label, tool, args = sys.argv[1], sys.argv[2], json.loads(sys.argv[3])
args = dict(company_guid='1edb9b05-8d35-4c0c-9959-dce655730463', **args)
page = 1
while True:
    lab = label if page == 1 else f'{label}-p{page}'
    subprocess.run([sys.executable, '-I', '-X', 'utf8', W + '/call.py', lab, tool, json.dumps(args)], check=True, stdout=subprocess.DEVNULL)
    f = sorted(glob.glob(f'{W}/srv/answers/*-{lab}.json'))[-1]
    j = json.load(open(f, encoding='utf-8')); r = j['response'].get('result') or {}
    sc = r.get('structuredContent') or {}; res = sc.get('result') if isinstance(sc.get('result'), dict) else {}
    rows = res.get('masters', res.get('items'))
    nxt = res.get('next_offset'); snap = (res.get('snapshot') or {}).get('id')
    if nxt is None and rows is not None and isinstance(res.get('total'), int) and (res.get('offset') or 0) + len(rows) < res['total']:
        nxt = (res.get('offset') or 0) + len(rows)  # ledger_masters gives offset and total, not next_offset
    err = sc.get('error') or (sc.get('refusal') if isinstance(sc, dict) else None)
    print(f'{f.split("/")[-1]}: isError {r.get("isError")}; rpc error {j["response"].get("error")}; error {json.dumps(err)[:300] if err else None}; '
          f'rows {None if rows is None else len(rows)}; total {res.get("total")}; offset {res.get("offset")}; next_offset {nxt}; snapshot {snap}')
    if r.get('isError') or j['response'].get('error') or nxt is None:
        break
    args = dict(args, offset=nxt, snapshot_id=snap); page += 1
