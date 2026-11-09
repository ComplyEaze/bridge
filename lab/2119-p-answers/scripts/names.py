# Lab helper (row 2119-P, not product code): reads each party's name as the sheet says (the file's first line with exactly
# one line terminator removed, \r\n or \n, nothing else trimmed), checks it by the tool's name rule (LedgerScopeName at
# d04f67a9: ASCII letters and digits, single inner spaces, no leading or trailing space, at most 1024 characters, no
# formula word), and writes args\p-<label>.json with the sheet's closed set. Prints only the label, the name's length,
# the file's bytes and sha256, and the check; never the name. The args files stay on the lab PC.
# Usage: python -I -X utf8 names.py
import hashlib, json
FORMULA = {'and', 'or', 'not', 'is', 'in', 'contains', 'like', 'between', 'starting', 'ending', 'with', 'equal', 'greater', 'less', 'than'}
SRC = {'top1': '<lab root>/2377-b/req2/top3.txt', 'p1': '<lab root>/2119-l/lo/party.txt'}
COUNT = {'top1': '250', 'p1': '248'}
RECORDED = {'top1': 'ac313c65557d9b42f61d1a7a0a256c1773a182a03323d913ffebd83285376eeb', 'p1': '078b01c5dedf292fb5ddaa99be3cf344406fd8b88b427b1acc785494e3099b60'}
def check(n):
    if not n: return 'empty'
    if len(n) > 1024: return 'too long'
    prev = True
    for c in n:
        if not (c.isascii() and c.isalnum()) and c != ' ': return 'character'
        if c == ' ' and prev: return 'spacing'
        prev = c == ' '
    if prev: return 'spacing'
    if any(w.lower() in FORMULA for w in n.split(' ')): return 'formula word'
    return 'ok'
for label, path in SRC.items():
    b = open(path, 'rb').read(); t = b.decode('utf-8')
    i = t.find('\n')
    first = t if i < 0 else t[:i + 1]
    if first.endswith('\r\n'): name = first[:-2]
    elif first.endswith('\n'): name = first[:-1]
    else: name = first
    # The recorded word of 9 Nov: the line is TAB-separated; the name is field 1, used only if the line has
    # exactly one TAB, field 2 equals the sheet's count for that file, field 1 passes the rule unchanged, and the file's
    # sha256 equals the one its row recorded.
    h = hashlib.sha256(b).hexdigest(); line = name
    tabs = line.count('\t'); f1, _, f2 = line.partition('\t')
    checks = {'one TAB': tabs == 1, 'field 2 count': f2 == COUNT[label], 'file sha256': h == RECORDED[label]}
    res = check(f1)
    print(f'{label}: file {len(b)} B sha256 {h}; whole line {len(line)} characters (rule on whole line: {check(line)}); '
          f'field 1 {len(f1)} characters, rule check {res}; ' + '; '.join(f'{k} {v}' for k, v in checks.items()))
    name = f1
    if res == 'ok' and all(checks.values()):
        a = {'company_guid': 'd45bc1b0-e5e3-4261-b3b2-cce3915f42d3', 'as_of': '20260331', 'party': name, 'detail': 'open_bills'}
        open(f'<lab root>/2119-p/args/p-{label}.json', 'w', encoding='utf-8', newline='\n').write(json.dumps(a))
