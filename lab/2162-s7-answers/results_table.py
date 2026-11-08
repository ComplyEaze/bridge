# Lab helper (2162-S7, not product code): builds RESULTS.md's per-send table from count7.py's last whole output
# (evidence/count-cend.txt), then puts it into RESULTS-template.md at {TABLE} and writes RESULTS.md.
# Usage: python -I -X utf8 results_table.py
import re
R = '<LAB_DIR>'
t = open(R + '/evidence/count-cend.txt', encoding='utf-8-sig').read().splitlines()
info, order = {}, []
cur = None
for l in t:
    m = re.match(r'^a-(\w+?)-(.+?)\.xml: (\d+) bytes, sha256 ([0-9a-f]{64})$', l)
    if m:
        cur = m.group(1); order.append(cur)
        info[cur] = dict(file=m.group(2), bytes=int(m.group(3)), sha=m.group(4)[:8], rows=None, sales=None)
        continue
    m = re.match(r'^a-(\w+?)-.+?\.xml: curl (\d+), HTTP (\d+), first byte ([\d.]+) s, curl total ([\d.]+) s, wall ([\d.]+) s, (\d+) bytes$', l)
    if m:
        info[m.group(1)].update(curl=m.group(2), http=m.group(3), fb=m.group(4), tot=m.group(5), wall=m.group(6)); cur = None; continue
    if cur and l.startswith('  rows '): info[cur]['rows'] = int(l.split()[1])
    if cur and l.startswith('  BRIDGEVCHISSALES:'):
        y = re.search(r"'Yes' (\d+)", l); n = re.search(r"'No' (\d+)", l)
        info[cur]['sales'] = f"{y.group(1) if y else 0} / {n.group(1) if n else 0}"
seen = {}
rows = ['| send | request | rows | Sales Yes / No | bytes | sha256 | curl first byte / total (s) | wall (s) |', '|---|---|---:|---:|---:|---|---:|---:|']
for k in order:
    d = info[k]
    assert d['curl'] == '0' and d['http'] == '200', k
    same = f" (= {seen[d['sha']]})" if d['sha'] in seen else ''
    seen.setdefault(d['sha'], k)
    r = '\u2013' if d['rows'] is None else f"{d['rows']:,}"
    s = '\u2013' if d['sales'] is None else d['sales']
    rows.append(f"| {k} | {d['file']} | {r} | {s} | {d['bytes']:,} | {d['sha']}\u2026{same} | {d['fb']} / {d['tot']} | {d['wall']} |")
tpl = open(R + '/RESULTS-template.md', encoding='utf-8').read()
assert tpl.count('{TABLE}') == 1
open(R + '/RESULTS.md', 'w', encoding='utf-8', newline='\n').write(tpl.replace('{TABLE}', '\n'.join(rows)))
print(len(order), 'sends in the table')
