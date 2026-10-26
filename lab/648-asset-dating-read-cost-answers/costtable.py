import json, glob, os
rows = []
for part in ('A', 'B'):
    for p in sorted(glob.glob(os.path.join('answers', part, '*.json'))):
        if p.endswith('.egress.json'):
            continue
        d = json.load(open(p, encoding='utf-8'))
        name = os.path.basename(p)[:-5]
        args = d['arguments']
        win = '%s..%s' % (args.get('from', ''), args.get('to', '')) if 'from' in args else ''
        r = d['response']['result']
        t = json.loads(r['content'][0]['text'])
        ev = t.get('evidence', {})
        eg = json.load(open(p[:-5] + '.egress.json', encoding='utf-8'))
        et = json.loads(eg['result']['content'][0]['text'])
        recs = [json.loads(x) for x in et['result']['records']]
        mine = [x for x in recs if x.get('record_type') == 'response_prepared' and x.get('tool') == d['tool']][:1]
        trail = mine[0].get('request_trail', {}) if mine else {}
        last = trail.get('last', [])
        posts = [x for x in last if x.get('kind') == 'post']
        rows.append({'part': part, 'call': name, 'tool': d['tool'], 'window': win, 'round_trip_s': d['round_trip_seconds'],
                     'requests_sent': trail.get('sent'), 'posts': len(posts), 'status_checks': len(last) - len(posts),
                     'request_bytes_total': sum(x.get('request_bytes') or 0 for x in posts),
                     'response_bytes_total': sum(x.get('response_bytes') or 0 for x in last),
                     'largest_response': max([x.get('response_bytes') or 0 for x in last] or [0]),
                     'bytes_prepared': mine[0].get('bytes_prepared') if mine else None,
                     'evidence_bytes': ev.get('bytes'), 'evidence_ms': ev.get('duration_ms'), 'state': ev.get('state')})
json.dump(rows, open(os.path.join('tables', 'cost-table.json'), 'w', encoding='utf-8', newline='\n'), indent=1)
lines = ['| Part | Call | Tool | Window | Round trip (s) | Requests (posts + status) | Response bytes (all) | Largest response | Answer bytes | Tally read (ms) |', '|---|---|---|---|---|---|---|---|---|---|']
for x in rows:
    lines.append('| %s | %s | %s | %s | %.3f | %s (%s + %s) | %s | %s | %s | %s |' % (x['part'], x['call'], x['tool'], x['window'], x['round_trip_s'], x['requests_sent'], x['posts'], x['status_checks'], x['response_bytes_total'], x['largest_response'], x['bytes_prepared'], x['evidence_ms']))
open(os.path.join('tables', 'cost-table.md'), 'w', encoding='utf-8', newline='\n').write('\n'.join(lines) + '\n')
print('\n'.join(lines))
