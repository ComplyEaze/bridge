# Lab helper (2074-2214 W row, 18 Nov; not product code): parses evidence/proxy/proxy.jsonl from a given line on. For each
# answered request: its conn, request number, line, bytes sent and answered, seconds from forwarded to answered, and its
# kind from the body kept in bodies/<sha256>: "mailing" when it names LEDMAILINGDETAILS.LIST, else "master" when it asks
# for a Ledger collection, else "post" or "status". Every other event (open, bridge_cut, proxy_error, tally_closed, close)
# is printed whole. Nothing is written.
# Usage: python -I -X utf8 proxylog.py <first line number, 1-based> [--posts-only]
import json, os, sys
R = '<lab folder>/evidence/proxy'
first = int(sys.argv[1]); posts_only = '--posts-only' in sys.argv
fwd = {}
def kind(sha, line):
    if line.startswith('GET'): return 'status'
    p = os.path.join(R, 'bodies', sha)
    if not os.path.exists(p): return 'post(body not kept)'
    b = open(p, 'rb').read()
    t = b.decode('utf-16-le', 'replace') if b[:2] in (b'\xff\xfe', b'<\x00') else b.decode('utf-8', 'replace')
    if 'LEDMAILINGDETAILS.LIST' in t: return 'mailing'
    u = t.upper()
    if '<TYPE>LEDGER</TYPE>' in u or 'TYPE="LEDGER"' in u or '<TYPE>LEDGER' in u: return 'master'
    return 'post'
n = 0
for i, l in enumerate(open(R + '/proxy.jsonl', encoding='utf-8'), 1):
    if i < first: continue
    e = json.loads(l)
    ev = e.get('event')
    if ev == 'forwarded': fwd[(e['conn'], e['request'])] = e['at']; continue
    if ev == 'first_byte': continue
    if ev is None and 'line' in e:
        k = kind(e['sent_sha256'], e['line'])
        if posts_only and k == 'status': continue
        dt = e['at'] - fwd.get((e['conn'], e['request']), e['at'])
        print(f"{i} {e['utc']} conn {e['conn']} req {e['request']} {k} sent {e['sent']} answered {e['answered']} {dt:.3f} s")
        n += 1
        continue
    print(i, json.dumps(e, ensure_ascii=False))
print('answered requests listed', n)
