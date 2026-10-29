# Lab helper (#1233 stock census slice, not product code): reads each answer by parser (C4: only the characters XML forbids are
# dropped; the UDF prefix declared). ROWS are the STOCKITEM (c*) or STOCKGROUP (g1, k2) children of ENVELOPE/BODY/DATA/COLLECTION.
# Writes tables/<label>.tsv (GUID, BRIDGECOMPANYGUID, ALTERID if any) and prints the sheet's step-3 records.
import os, re, xml.etree.ElementTree as ET
W = '<lab folder>'; CG = '1edb9b05-8d35-4c0c-9959-dce655730463'
os.makedirs(W + '/tables', exist_ok=True)
def load(label):
    t = open(f'{W}/answers/{label}.xml', 'rb').read().decode('utf-16-le').lstrip('\ufeff')
    t = re.sub(r'&#(x[0-9A-Fa-f]+|[0-9]+);', lambda m: '' if (int(m.group(1)[1:], 16) if m.group(1)[0] == 'x' else int(m.group(1))) in set(range(0, 9)) | {11, 12} | set(range(14, 32)) else m.group(0), t)
    t = re.sub('[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t).replace('<ENVELOPE', '<ENVELOPE xmlns:UDF="urn:x-tally-udf"', 1)
    return ET.fromstring(t)
out = {}
for label, tag in (('c1', 'STOCKITEM'), ('c2', 'STOCKITEM'), ('c3', 'STOCKITEM'), ('c4', 'STOCKITEM'), ('g1', 'STOCKGROUP'), ('c1r', 'STOCKITEM'), ('k2', 'STOCKGROUP')):
    root = load(label); coll = root.find('./BODY/DATA/COLLECTION'); status = root.findtext('./HEADER/STATUS')
    rows = [] if coll is None else [r for r in coll if r.tag == tag]
    others = [] if coll is None else sorted({r.tag for r in coll if r.tag != tag})
    recs = [(r.findtext('GUID') or '', r.findtext('BRIDGECOMPANYGUID') or '', (r.findtext('ALTERID') or '').strip()) for r in rows]
    shapes = sorted({(tuple(sorted({c.tag for c in r})), tuple(sorted(r.attrib))) for r in rows})
    out[label] = recs
    with open(f'{W}/tables/{label}.tsv', 'w', encoding='utf-8', newline='\n') as f:
        f.write('GUID\tBRIDGECOMPANYGUID\tALTERID\n' + ''.join(f'{g}\t{b}\t{a}\n' for g, b, a in recs))
    guids = [g for g, _, _ in recs]
    print(f'{label}: COLLECTION present {coll is not None} attrs {dict(coll.attrib) if coll is not None else None}; HEADER/STATUS {status!r}; rows {len(rows)}; other children {others}; bytes {os.path.getsize(f"{W}/answers/{label}.xml")}')
    print(f'   repeated GUIDs {len(guids) - len(set(guids))}; GUIDs not prefixed by the company {sum(1 for g in guids if not g.lower().startswith(CG))}; BRIDGECOMPANYGUID not the company {sum(1 for _, b, _ in recs if b != CG)}; untrimmed GUIDs {sum(1 for g in guids if g != g.strip())}')
    print(f'   row shapes (child names, attributes): {shapes[:3]}{" ..." if len(shapes) > 3 else ""}')
u = [g.lower() for lab in ('c1', 'c2', 'c3') for g, _, _ in out[lab]]
print(f'c1+c2+c3 rows {len(u)}; distinct {len(set(u))}; equals NUMSTOCKITEMS 55: {len(u) == 55 and len(set(u)) == 55}')
print(f'c1r same GUIDs in the same order as c1: {[g for g, _, _ in out["c1r"]] == [g for g, _, _ in out["c1"]]}')
gs = {g.lower() for g, _, _ in out['g1']}; ks = {g.lower() for g, _, _ in out['k2']}
print(f'g1 rows {len(out["g1"])}, k2 rows {len(out["k2"])}; set equal {gs == ks}; only in g1 {sorted(gs - ks)}; only in k2 {sorted(ks - gs)}')
for lab in ('c1', 'c2', 'c3'):
    if not out[lab]: print(f'{lab} empty; bytes equal c4: {open(f"{W}/answers/{lab}.xml","rb").read() == open(f"{W}/answers/c4.xml","rb").read()}')
