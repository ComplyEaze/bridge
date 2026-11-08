# Lab helper (#2162 index rows, not product code): counts, in every file under a folder, the start-up company's name,
# number and GUIDs, in UTF-8 and UTF-16LE (the name also in upper and lower case). The start-up company is read from
# tally.ini's Load line, leaving out the three lab books; its name from its own Company.1800 (the record after the bytes
# 02 10 1b 00 00 0f and a length byte), its GUIDs from the same file. Prints only counts: no value is printed or written.
# Usage: python -I -X utf8 startup_check.py <folder>
import os, re, sys
INI = 'C:/Program Files/TallyPrime/tally.ini'
B = '<Tally data folder>'
LAB = ('BRIDGE BILLTRAIL LAB', 'BRIDGE PILOT LAB', 'BRIDGE CORPUS DENSE')
UU = re.compile(r'(?<![0-9A-Fa-f])[0-9A-Fa-f]{8}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{12}(?![0-9A-Fa-f])')
MARK = bytes.fromhex('02101b00000f')
def name_of(b):
    i = b.find(MARK)
    if i < 0: return None
    n = b[i + len(MARK)]
    return b[i + len(MARK) + 2:i + len(MARK) + 2 + n].decode('utf-16-le').rstrip('\x00')
loads = []
for line in open(INI, encoding='utf-8', errors='replace'):
    m = re.match(r'\s*load\s*=\s*(.*)', line, re.I)
    if m: loads += [x for x in re.split(r'[\s,;]+', m.group(1)) if x.isdigit()]
targets = []
for num in loads:
    p = os.path.join(B, num, 'Company.1800')
    if not os.path.isfile(p): continue
    b = open(p, 'rb').read(); nm = name_of(b)
    if nm in LAB: continue
    guids = sorted({g.lower() for enc in ('utf-16-le', 'utf-8') for g in UU.findall(b.decode(enc, 'ignore'))})
    targets.append((num, nm, guids))
print('start-up companies found (not lab books):', len(targets))
if not targets or any(not t[1] for t in targets): raise SystemExit('start-up company or its name not found: nothing counted')
root = sys.argv[1]
total = 0; files = 0
for t_i, (num, nm, guids) in enumerate(targets, 1):
    pats = {'number': [num.encode(), num.encode('utf-16-le')],
            'name': [v.encode(e) for v in {nm, nm.upper(), nm.lower()} for e in ('utf-8', 'utf-16-le')],
            'GUID': [v.encode(e) for g in guids for v in (g, g.upper()) for e in ('utf-8', 'utf-16-le')]}
    counts = {k: 0 for k in pats}
    for dp, _, ns in os.walk(root):
        if '.git' in dp.split(os.sep): continue
        for n in ns:
            d = open(os.path.join(dp, n), 'rb').read(); files += 1 if t_i == 1 else 0
            for k, ps in pats.items(): counts[k] += sum(d.count(p) for p in ps)
    print(f'company {t_i}: name length {len(nm)}, GUIDs {len(guids)}; hits: number {counts["number"]}, name {counts["name"]}, GUID {counts["GUID"]}')
    total += sum(counts.values())
print('files scanned', files, '| total hits', total)
