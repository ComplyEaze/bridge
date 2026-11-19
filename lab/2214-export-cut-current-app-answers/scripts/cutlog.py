# Lab helper (2214 export cut on the current app; new, not product code): reads proxy.jsonl, writes nothing. For each
# request the proxy forwarded, from a given line on, prints: line of its forwarded event, UTC, conn, request, its kind from
# the body the proxy kept (mailing master, standard master, other post, status), bytes sent, and what became of it:
# answered (with bytes and seconds), CUT (a bridge_cut line), or UNANSWERED. Then every other event whole, and a summary:
# the first bridge_cut, its seconds from its own forwarded event, and the master posts (mailing or standard) forwarded
# after it, with their UTC seconds and connections. A master post is a request whose body holds <ID>List of Ledgers</ID>.
# Usage: python -I -X utf8 cutlog.py <path of proxy.jsonl> <first line number, 1-based>
import json, os, sys
path, first = sys.argv[1], int(sys.argv[2])
bodies = os.path.join(os.path.dirname(os.path.abspath(path)), 'bodies')
def kind(sha, line=''):
    if line.startswith('GET'): return 'status'
    p = os.path.join(bodies, sha) if sha else ''
    if not p or not os.path.exists(p): return 'post(body not kept)'
    b = open(p, 'rb').read()
    t = b.decode('utf-16-le', 'replace') if b[:2] in (b'\xff\xfe', b'<\x00') else b.decode('utf-8', 'replace')
    if '<ID>List of Ledgers</ID>' in t:
        return 'mailing master' if 'LEDMAILINGDETAILS.LIST' in t else 'standard master'
    return 'post'
events = []
for i, l in enumerate(open(path, encoding='utf-8'), 1):
    if i >= first: events.append((i, json.loads(l)))
fwd, end, rows = {}, {}, []
for i, e in events:
    ev = e.get('event')
    if ev == 'forwarded': fwd[(e['conn'], e['request'])] = (i, e)
    elif ev == 'bridge_cut': end[(e['conn'], e['request'])] = ('CUT', e, kind(e.get('sent_sha256')))
    elif ev is None and 'line' in e: end[(e['conn'], e['request'])] = ('answered', e, kind(e['sent_sha256'], e['line']))
cut = None
for (conn, req), (i, e) in fwd.items():
    state, x, k = end.get((conn, req), ('UNANSWERED', e, None))
    k = k or 'unknown'
    secs = round(x['at'] - e['at'], 3) if state != 'UNANSWERED' else None
    ans = f" answered {x['answered']}" if state == 'answered' else ''
    print(f"{i} {e['utc']} at {e['at']:.3f} conn {conn} req {req} {k} sent {e['sent']} front {e.get('front')} {state}{ans} {secs} s")
    rows.append((i, e['utc'], conn, req, k, state, secs))
    if state == 'CUT' and cut is None: cut = (i, e['utc'], conn, req, k, secs)
for i, e in events:
    if e.get('event') not in (None, 'forwarded', 'first_byte'): print(i, json.dumps(e, ensure_ascii=False))
if cut:
    print(f"SUMMARY first bridge_cut: forwarded line {cut[0]}, conn {cut[2]} req {cut[3]} {cut[4]}, {cut[5]} s after its forwarded event (forwarded at UTC {cut[1]})")
    later = [r for r in rows if r[0] > cut[0] and r[4] in ('mailing master', 'standard master')]
    same = [r for r in rows if r[2] == cut[2] and r[0] > cut[0]]
    print(f"SUMMARY master posts forwarded after the cut: {len(later)}; on the cut's own connection after it: {len(same)}")
    for r in later: print(f"SUMMARY   line {r[0]} {r[1]} conn {r[2]} req {r[3]} {r[4]} {r[5]}")
else:
    print('SUMMARY no bridge_cut')
