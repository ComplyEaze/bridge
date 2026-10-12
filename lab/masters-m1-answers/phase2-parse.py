import sys, re, xml.etree.ElementTree as ET
E = sys.argv[1]
def load(n):
    b = open(f"{E}/nr-{n}.response", 'rb').read()
    h = open(f"{E}/nr-{n}.headers", 'rb').read().decode('latin-1')
    cs = re.search(r'charset=([\w-]+)', h, re.I).group(1)
    t = b.decode(cs)
    bad = [hex(ord(c)) for c in t if ord(c) < 32 and c not in '\t\r\n']
    clean = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '?', t)
    refs = sorted(set(m.group(0) for m in re.finditer(r'&#(\d+);', t) if int(m.group(1)) < 32 and int(m.group(1)) not in (9, 10, 13)))
    for x in refs: clean = clean.replace(x, '?')
    bad = bad + ['ref %s x%d' % (x, t.count(x)) for x in refs]
    return len(b), cs, ET.fromstring(clean), sorted(set(bad))
def names(el):
    out = []
    for lnl in el.iter('LANGUAGENAME.LIST'):
        for nl in lnl.iter('NAME.LIST'):
            for n in nl.findall('NAME'):
                out.append(n.text)
    return out
for n in ['R-MARKS', 'R-V2', 'R-V3', 'R-GRP']:
    size, cs, r, bad = load(n)
    coll = r.find('BODY/DATA/COLLECTION')
    led = coll.findall('LEDGER') if coll is not None else []
    grp = coll.findall('GROUP') if coll is not None else []
    cmp_ = coll.findall('COMPANY') if coll is not None else []
    print(f"== {n}: {size} bytes, charset {cs}, ctrl chars {bad}; COLLECTION rows LEDGER={len(led)} GROUP={len(grp)} COMPANY={len(cmp_)}; other child tags={sorted(set(c.tag for c in coll) - {'LEDGER','GROUP','COMPANY'}) if coll is not None else None}")
    for L in led:
        nm = L.get('NAME')
        line = f"   LEDGER {nm!r} ALTERID={'present:'+L.findtext('ALTERID').strip() if L.find('ALTERID') is not None else 'ABSENT'} PARENT={L.findtext('PARENT')!r} GUID={L.findtext('GUID')}"
        if n != 'R-GRP':
            print(line)
        if nm == 'MLab Alias Target':
            print(f"      NAME list (doc order): {names(L)}")
    for G in grp:
        nm = G.get('NAME')
        if nm == 'MLab Group Target' or n != 'R-GRP':
            print(f"   GROUP {nm!r} GUID={G.findtext('GUID')} ALTERID={G.findtext('ALTERID')!r} PARENT={G.findtext('PARENT')!r} names={names(G)}")
    if n == 'R-GRP':
        print("   all group names:", [g.get('NAME') for g in grp])
    if n == 'R-V3':
        have = sum(1 for L in led if L.find('ALTERID') is not None)
        print(f"   R-V3 ALTERID on {have} of {len(led)} ledger rows")
