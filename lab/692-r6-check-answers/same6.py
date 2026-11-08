# Lab helper (692-R6, not product code): the sheet's same6.py. Reads the kept stock answers with expat (never a text
# search) and prints only verdicts, counts and lengths: never a name or an amount.
# - Pairs: whether two answers hold the same top-level rows, element for element (each child of ENVELOPE as its tag,
#   attributes and whitespace-stripped text, recursively), and each side's row count (DSPACCNAME children).
# - For each answer: the rows with a quantity (DSPCLQTY not empty) and no value (DSPCLAMTA empty).
# - Requests: whether each w*-stock-summary request equals the shipped envelope for its as-of date. The envelope is
#   rebuilt here from render_report_envelope's format string (native_statement_reports.rs:240-246 at 76705cfb), with the
#   id "Stock Summary", extra "" (render_built_in_report_request), and the period 1 April of the as-of date's financial
#   year to the as-of date (agent_stock_summary.rs:28). The company is the one name the requests carry, escaped as
#   xml_escape does; only its length is printed.
# Usage: python -I -X utf8 same6.py <answers folder> <requests folder>
import os, sys
import xml.parsers.expat
A, Q = sys.argv[1], sys.argv[2]

def tree(data):
    root = {'tag': None, 'attrs': (), 'text': '', 'kids': []}; stack = [root]
    p = xml.parsers.expat.ParserCreate()
    def start(tag, attrs):
        n = {'tag': tag, 'attrs': tuple(sorted(attrs.items())), 'text': '', 'kids': []}; stack[-1]['kids'].append(n); stack.append(n)
    def end(tag): stack.pop()
    def chars(s): stack[-1]['text'] += s
    p.StartElementHandler, p.EndElementHandler, p.CharacterDataHandler = start, end, chars
    p.Parse(data.decode('utf-16'), True)
    assert len(root['kids']) == 1 and root['kids'][0]['tag'] == 'ENVELOPE'
    return root['kids'][0]

def canon(n): return (n['tag'], n['attrs'], n['text'].strip(), tuple(canon(k) for k in n['kids']))
def find(n, tag): return [k for k in n['kids'] if k['tag'] == tag]
def deep(n, tag):
    out = [n] if n['tag'] == tag else []
    for k in n['kids']: out += deep(k, tag)
    return out

def answer(name):
    d = open(os.path.join(A, name + '.utf16le.xml'), 'rb').read(); e = tree(d)
    rows = len(find(e, 'DSPACCNAME'))
    noval = 0
    for info in find(e, 'DSPSTKINFO'):
        for cl in deep(info, 'DSPSTKCL'):
            q = [x['text'].strip() for x in find(cl, 'DSPCLQTY')]; a = [x['text'].strip() for x in find(cl, 'DSPCLAMTA')]
            if any(q) and not any(a): noval += 1
    return e, rows, noval, len(d)

def pair(step, x, y):
    ex, rx, nx, bx = answer(x); ey, ry, ny, by = answer(y)
    same = [canon(k) for k in ex['kids']] == [canon(k) for k in ey['kids']]
    print(f'{step}: {x} ({bx} B, rows {rx}, rows with a quantity and no value {nx}) against {y} ({by} B, rows {ry}, without a value {ny}): '
          f'top-level elements {len(ex["kids"])} and {len(ey["kids"])}; element for element equal: {"yes" if same else "NO"}')
    return same

print('Step 1, w4 against w2:')
s1 = [pair('  1a', 'r1-w4-stock-summary', 'r1-w2-stock-summary'), pair('  1b', 'r1-w4-negative-stock', 'r1-w2-negative-stock')]
print('Step 2, w5 against w2:')
s2 = [pair('  2a', 'r5-w5-stock-summary', 'r1-w2-stock-summary'), pair('  2b', 'r5-w5-negative-stock', 'r1-w2-negative-stock')]
print('Step 3, R3\'s row without a value:')
_, r3rows, r3nv, r3b = answer('r3-w2-stock-summary')
print(f'  r3-w2-stock-summary ({r3b} B): rows {r3rows}; rows with a quantity and no value {r3nv}; EXPECT 1: {"met" if r3nv == 1 else "NOT met"}')
_, n3rows, n3nv, n3b = answer('r3-w2-negative-stock')
print(f'  r3-w2-negative-stock ({n3b} B), for the record: rows {n3rows}; rows with a quantity and no value {n3nv}')
for n in ('r1-w1-stock-summary', 'r1-w3-stock-summary'):
    _, r, nv, b = answer(n); print(f'  {n} ({b} B), for the record: rows {r}; rows with a quantity and no value {nv}')

print('Step 4, the requests against the shipped envelope:')
FMT = ('<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Data</TYPE><ID>{id}</ID></HEADER><BODY><DESC>'
       '<STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY>'
       '<SVFROMDATE TYPE="Date">{frm}</SVFROMDATE><SVTODATE TYPE="Date">{to}</SVTODATE>{extra}</STATICVARIABLES></DESC></BODY></ENVELOPE>')
def esc(s): return s.replace('&', '&amp;').replace('<', '&lt;').replace('>', '&gt;').replace('"', '&quot;').replace("'", '&apos;')
def fy_start(asof): y, m = int(asof[:4]), int(asof[4:6]); return f'{y if m >= 4 else y - 1}0401'
names = set(); texts = {}
for w in ('w1', 'w2', 'w3', 'w4', 'w5'):
    d = open(os.path.join(Q, f'{w}-stock-summary.utf16le.xml'), 'rb').read()
    t = d.decode('utf-16'); texts[w] = (t, d[:2] == b'\xff\xfe')
    names.update(x['text'] for x in deep(tree(d), 'SVCURRENTCOMPANY'))
assert len(names) == 1, 'the requests carry more than one company'
company = esc(names.pop())
print(f'  company: one name in all five requests ({len(company)} characters escaped)')
for w, asof, need in (('w1', '20250331', True), ('w2', '20260331', True), ('w3', '20250930', True), ('w4', '20260331', False), ('w5', '20260331', False)):
    t, bom = texts[w]
    shipped = FMT.format(id='Stock Summary', company=company, frm=fy_start(asof), to=asof, extra='')
    eq = t == shipped
    tag = 'the sheet\'s step 4' if need else 'for the record (the sheet expects a different start)'
    print(f'  {w}-stock-summary (BOM {"yes" if bom else "no"}, {len(t)} characters) against the shipped request as of {asof} '
          f'(period {fy_start(asof)} to {asof}): equal: {"yes" if eq else "NO"} [{tag}]')
ok = all(s1) and all(s2) and r3nv == 1
print(f'Summary: step 1 {"equal" if all(s1) else "UNEQUAL"}; step 2 {"equal" if all(s2) else "UNEQUAL"}; step 3 {r3nv} (EXPECT 1)')
