# Lab helper (#1211, not product code): one parse of a statement (its full sha256 checked first; a different file = stop)
# and the statement_tie_out for it, then egress_log for the line that call added. Prints the parse's main fields and the
# tie-out's three figures, window, evidence state and request trail; appends a summary to evidence\tie-log.txt.
# Usage: python -I -X utf8 tie.py <label> <A|B> <stage> [<from> <to>]
import hashlib, json, os, subprocess, sys
W = '<LAB_DIR>'; SD = W + '/srv'
G = '9c97970c-e25d-4348-9740-6ddb1caaa56d'
STMT = {'A': ('<STATEMENT A folder>/lab-1628-ubi-3m.pdf', '506e05ebfb0f81d6e0fa4b0cc867bbaeae8a69d130c02615ea48a86559894fc9', '10000.00', '11000.00'),
        'B': ('<sheet checkout>/lab/1211-tie-out-evidence/statement/ubi-1211-r3.pdf', '33609063f8d64e2d3136081f1a175343011434befff4f319a33d6f905d2c650a', '12300.00', '13950.00')}
PW = '<STATEMENT A folder>/statement.password'
label, which, stage = sys.argv[1], sys.argv[2], sys.argv[3]
win = sys.argv[4:6]
path, want, op, cl = STMT[which]
def log(s):
    with open(W + '/evidence/tie-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
    print(s, flush=True)
got = hashlib.sha256(open(path, 'rb').read()).hexdigest()
if got != want: log(f'{label}: stop: STATEMENT {which} sha256 {got} differs'); sys.exit(2)
def call(name, tool, args):
    subprocess.run(['python', '-I', '-X', 'utf8', W + '/call.py', name, tool, json.dumps(args)], capture_output=True, text=True)
    fn = [f for f in sorted(os.listdir(SD + '/answers')) if f.endswith(f'-{name}.json')][-1]
    j = json.load(open(f'{SD}/answers/{fn}', encoding='utf-8'))
    r = j['response'].get('result') or {}
    sc = r.get('structuredContent') or {}
    return fn, r, (sc.get('result') if isinstance(sc.get('result'), dict) else sc)
args = {'statement_path': path.replace('/', '\\'), 'password_file': PW.replace('/', '\\'), 'bank': 'ubi', 'account_label': 'Lab UBI 7788',
        'opening_balance': op, 'closing_balance': cl, 'bank_ledger': 'Union Bank', 'suspense_ledger': 'Suspense'}
if win: args['from'], args['to'] = win
fn, r, p = call(f'{label}-parse', 'parse_bank_statement', args)
log(f'{label}: STATEMENT {which} sha256 ok; parse {fn} isError {r.get("isError")}: proposals_id {p.get("proposals_id")} sha256 {p.get("sha256")}; '
    f'statement_rows {p.get("statement_rows")} rows_in_window {p.get("rows_in_window")} vouchers {p.get("vouchers")} skipped {p.get("skipped")} '
    f'cash_questions_open {p.get("cash_questions_open")}; reconciled {json.dumps(p.get("reconciled"))}')
if r.get('isError') or not p.get('proposals_id'):
    log(f'{label}: parse refused: {json.dumps(r)[:1500]}'); sys.exit(3)
if stage == '-': sys.exit(0)
fn2, r2, t = call(f'{label}-tie-{stage}', 'statement_tie_out', {'company_guid': G, 'proposals_id': p['proposals_id'], 'proposals_sha256': p['sha256'], 'stage': stage})
sc = (r2.get('structuredContent') or {})
figs = {k: t.get(k) for k in ('opening_gap', 'closing_gap', 'change_in_window')}
log(f'{label}: tie_out {stage} {fn2} isError {r2.get("isError")}: ' + json.dumps(figs) + f'; window {json.dumps(t.get("window"))}; '
    f'rows_without_voucher {t.get("rows_without_voucher")}; evidence {json.dumps(sc.get("evidence"))}; '
    f'next_step {json.dumps(t.get("next_step"))} next_steps {json.dumps(t.get("next_steps"))}')
if r2.get('isError'): log(f'{label}: tie_out answer: {json.dumps(r2)[:2000]}')
fn3, r3, e = call(f'{label}-egress', 'egress_log', {'limit': 4})
recs = [json.loads(x) if isinstance(x, str) else x for x in (e.get('records') or [])]
mine = [d for d in recs if d.get('tool') == 'statement_tie_out']
if not mine: log(f'{label}: egress_log {fn3}: no statement_tie_out line among the last 4: {json.dumps(e)[:1500]}'); sys.exit(4)
d = mine[0]; tr = d.get('request_trail') or {}
kinds = {}
for q in tr.get('last') or []: kinds[(q.get('kind'), q.get('request_bytes'))] = kinds.get((q.get('kind'), q.get('request_bytes')), 0) + 1
log(f'{label}: egress_log {fn3}: statement_tie_out line ts {d.get("ts")} receipt {d.get("receipt_id")} response_sha256 {d.get("response_sha256")} '
    f'(= the answer: {d.get("bytes_prepared")} B); request_trail sent {tr.get("sent")} failed {tr.get("failed")} omitted {tr.get("omitted")}; '
    f'kinds ' + '; '.join(f'{k} {b} B x{n}' if b else f'{k} x{n}' for (k, b), n in sorted(kinds.items(), key=str)))
