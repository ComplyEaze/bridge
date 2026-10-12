# usage: m1p3.py <evidence> write <step> <ID>   |  rows <step> <name1> [<name2>...]  |  marks <step> | grp <step> <name> | party <step> <name>
import sys, re, xml.etree.ElementTree as ET
E = sys.argv[1]; mode = sys.argv[2]; step = sys.argv[3]
def load(fn):
    b = open(f"{E}/{fn}.response", 'rb').read()
    h = open(f"{E}/{fn}.headers", 'rb').read().decode('latin-1')
    m = re.search(r'charset=([\w-]+)', h, re.I)
    cs = m.group(1) if m else 'utf-8'
    t = b.decode(cs)
    clean = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '?', t)
    clean = re.sub(r'&#(\d+);', lambda m: '?' if int(m.group(1)) < 32 and int(m.group(1)) not in (9, 10, 13) else m.group(0), clean)
    return len(b), cs, ET.fromstring(clean), t
def names(el):
    out = []
    for lnl in el.iter('LANGUAGENAME.LIST'):
        for nl in lnl.iter('NAME.LIST'):
            out += [n.text for n in nl.findall('NAME')]
    return out
norm = lambda s: re.sub(r'[\s\-/]', '', (s or '')).lower()
if mode == 'write':
    fn = f"{step}-{sys.argv[4]}"; size, cs, r, t = load(fn)
    print(f"{fn}: {size} bytes {cs}")
    for k in ['CREATED','ALTERED','DELETED','LASTVCHID','LASTMID','COMBINED','IGNORED','ERRORS','CANCELLED','EXCEPTIONS']:
        els = list(r.iter(k)); print(f"  {k}: {[e.text for e in els] if els else 'absent'}")
    for e in r.iter('LINEERROR'): print('  LINEERROR:', e.text)
    print('  raw:', t.strip()[:600])
elif mode == 'marks':
    fn = f"{step}-R-MARKS"; size, cs, r, t = load(fn)
    cs_ = r.findall('BODY/DATA/COLLECTION/COMPANY')
    print(f"{fn}: {size} bytes; COMPANY rows {len(cs_)}; " + "; ".join(f"{c.get('NAME')} GUID={c.findtext('GUID')} ALTMSTID={(c.findtext('ALTMSTID') or 'absent').strip()} ALTVCHID={(c.findtext('ALTVCHID') or 'absent').strip()}" for c in cs_))
elif mode == 'rows':
    fn = f"{step}-R-V3"; size, cs, r, t = load(fn)
    want = set(norm(n) for n in sys.argv[4:])
    led = r.findall('BODY/DATA/COLLECTION/LEDGER')
    print(f"{fn}: {size} bytes; LEDGER rows {len(led)}; all: {[l.get('NAME') for l in led]}")
    for L in led:
        ns = names(L) + [L.get('NAME')]
        if any(norm(n) in want for n in ns):
            print(f"  ROW NAME={L.get('NAME')!r} GUID={L.findtext('GUID')} ALTERID={(L.findtext('ALTERID') or 'ABSENT').strip()} PARENT={L.findtext('PARENT')!r} ISBILLWISEON={L.findtext('ISBILLWISEON')!r} NAMES={names(L)}")
elif mode == 'grp':
    fn = f"{step}-R-GRP"; size, cs, r, t = load(fn)
    g = r.findall('BODY/DATA/COLLECTION/GROUP')
    print(f"{fn}: {size} bytes; GROUP rows {len(g)}")
    for G in g:
        if norm(G.get('NAME')) in set(norm(n) for n in sys.argv[4:]) or any(norm(n) in set(norm(x) for x in sys.argv[4:]) for n in names(G)):
            print(f"  GROUP NAME={G.get('NAME')!r} GUID={G.findtext('GUID')} ALTERID={(G.findtext('ALTERID') or 'absent').strip()} PARENT={G.findtext('PARENT')!r} NAMES={names(G)}")
elif mode == 'party':
    fn = f"{step}-R-PARTY"; size, cs, r, t = load(fn)
    led = r.findall('BODY/DATA/COLLECTION/LEDGER')
    print(f"{fn}: {size} bytes; LEDGER rows {len(led)}")
    for L in led:
        if L.get('NAME') == sys.argv[4]:
            print('  ' + ET.tostring(L, encoding='unicode'))
