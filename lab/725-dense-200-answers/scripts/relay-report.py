# Lab helper (725-D1, not product code): reads a\relay.jsonl by its parsed fields (never a substring search) and prints the
# sheet's counts: event lines; the import exchanges (no `event`, `number` not null); the exchanges before the first import
# and after it up to a given line, each with count, bytes sent, bytes answered, summed answer_s; and the exchanges from a
# given line on (the verify's), with their count, bytes and largest answer. Usage: python -I -X utf8 relay-report.py <verify start line>
import json, sys
L = [json.loads(l) for l in open('<LAB_DIR>/a/relay.jsonl', encoding='utf-8') if l.strip()]
v0 = int(sys.argv[1])
ex = [(i, x) for i, x in enumerate(L) if 'event' not in x]
imp = [(i, x) for i, x in ex if x.get('number') is not None]
print('lines', len(L), '| exchanges', len(ex), '| import exchanges', len(imp))
print('events', [(x['event'], x.get('cause')) for x in L if 'event' in x])
i_imp = imp[0][0] if imp else len(L)
def summ(name, rows):
    rows = [x for _, x in rows]
    print(f'{name}: {len(rows)} requests; sent {sum(x.get("sent") or 0 for x in rows)} B; answered {sum(x.get("answered") or 0 for x in rows)} B; '
          f'summed answer_s {round(sum(x.get("answer_s") or 0 for x in rows), 3)}; largest answer {max((x.get("answered") or 0 for x in rows), default=0)} B')
summ('before the import', [(i, x) for i, x in ex if i < i_imp])
summ('after the import, before the verify', [(i, x) for i, x in ex if i_imp < i < v0])
summ('the verify (from line %d)' % v0, [(i, x) for i, x in ex if i >= v0])
