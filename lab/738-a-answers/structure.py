# Lab helper (738-A, copied from 699-R1 with its folder changed; not product code): one structure line for each file kept on the lab PC, values withheld: its bytes, and for
# an XML answer the element path counts under ENVELOPE (top two levels and the COLLECTION's children), for a JSON answer the top-
# level keys and, under the tool result, its keys with list lengths. Writes evidence/kept-structure.txt.
# Usage: python -I -X utf8 structure.py <file> ...
import collections, json, re, sys, xml.etree.ElementTree as ET
out = []
for p in sys.argv[1:]:
    d = open(p, 'rb').read(); line = f'{p}: {len(d)} B; '
    if p.endswith(('.xml', '.bin')):
        t = (d[2:] if d[:2] == b'\xff\xfe' else d).decode('utf-16-le' if (d[:2] == b'\xff\xfe' or d[1:2] == b'\x00') else 'utf-8', 'replace')
        t = re.sub('[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t)
        root = ET.fromstring(t.encode('utf-8'))
        c = collections.Counter()
        for a in root:
            c[f'{root.tag}/{a.tag}'] += 1
            for b in a: c[f'{root.tag}/{a.tag}/{b.tag}'] += 1
        for coll in root.iter('COLLECTION'):
            for x in coll:
                c[f'COLLECTION/{x.tag}'] += 1
                for y in x: c[f'COLLECTION/{x.tag}/{y.tag}'] += 1
        line += 'XML; ' + ', '.join(f'{k} x{v}' for k, v in sorted(c.items()))
    else:
        j = json.loads(d.decode('utf-8'))
        line += 'JSON; top-level keys ' + ', '.join(sorted(j))
        res = (((j.get('response') or {}).get('result') or {}).get('structuredContent') or {}).get('result')
        if isinstance(res, dict):
            line += '; result keys ' + ', '.join(f'{k}' + (f'[{len(v)}]' if isinstance(v, list) else '') for k, v in sorted(res.items()))
    out.append(line)
open('<LAB_DIR>/evidence/kept-structure.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
print('\n'.join(out))
