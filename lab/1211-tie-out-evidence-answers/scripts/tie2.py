# Lab helper (#1211, not product code): statement_tie_out for a parse already made (its proposals_id and sha256 given),
# then egress_log for the line that call added; the same record as tie.py. Usage:
# python -I -X utf8 tie2.py <label> <stage> <proposals_id> <proposals_sha256>
import json, os, subprocess, sys
W = '<LAB_DIR>'; SD = W + '/srv'
G = '9c97970c-e25d-4348-9740-6ddb1caaa56d'
label, stage, pid, psha = sys.argv[1:5]
def log(s):
    with open(W + '/evidence/tie-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
def call(name, tool, args):
    subprocess.run(['python', '-I', '-X', 'utf8', W + '/call.py', name, tool, json.dumps(args)], capture_output=True, text=True)
    fn = [f for f in sorted(os.listdir(SD + '/answers')) if f.endswith(f'-{name}.json')][-1]
    j = json.load(open(f'{SD}/answers/{fn}', encoding='utf-8'))
    r = j['response'].get('result') or {}
    sc = r.get('structuredContent') or {}
    return fn, r, (sc.get('result') if isinstance(sc.get('result'), dict) else sc)
fn2, r2, t = call(f'{label}-tie-{stage}', 'statement_tie_out', {'company_guid': G, 'proposals_id': pid, 'proposals_sha256': psha, 'stage': stage})
sc = (r2.get('structuredContent') or {})
figs = {k: t.get(k) for k in ('opening_gap', 'closing_gap', 'change_in_window')}
log(f'{label}: tie_out {stage} {fn2} for {pid} isError {r2.get("isError")}: ' + json.dumps(figs) + f'; window {json.dumps(t.get("window"))}; '
    f'rows_without_voucher {t.get("rows_without_voucher")}; evidence {json.dumps(sc.get("evidence"))}; '
    f'next_step {json.dumps(t.get("next_step"))} next_steps {json.dumps(t.get("next_steps"))}')
if r2.get('isError'): log(f'{label}: tie_out answer: {json.dumps(r2)[:2000]}')
fn3, r3, e = call(f'{label}-egress', 'egress_log', {'limit': 4})
recs = [json.loads(x) if isinstance(x, str) else x for x in (e.get('records') or [])]
mine = [d for d in recs if d.get('tool') == 'statement_tie_out']
if not mine: log(f'{label}: egress_log {fn3}: no statement_tie_out line among the last 4'); sys.exit(4)
d = mine[0]; tr = d.get('request_trail') or {}
kinds = {}
for q in tr.get('last') or []: kinds[(q.get('kind'), q.get('request_bytes'))] = kinds.get((q.get('kind'), q.get('request_bytes')), 0) + 1
log(f'{label}: egress_log {fn3}: statement_tie_out line ts {d.get("ts")} receipt {d.get("receipt_id")} response_sha256 {d.get("response_sha256")} '
    f'({d.get("bytes_prepared")} B); request_trail sent {tr.get("sent")} failed {tr.get("failed")} omitted {tr.get("omitted")}; '
    f'kinds ' + '; '.join(f'{k} {b} B x{n}' if b else f'{k} x{n}' for (k, b), n in sorted(kinds.items(), key=str)))
