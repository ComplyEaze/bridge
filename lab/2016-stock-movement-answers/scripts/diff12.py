# Lab helper (#2016 stock movement, not product code): step 12's record: every element of the first answer's VOUCHER that is
# absent from or different in the second answer's VOUCHER, read by structure (LAB-COMMON C4; never a text search). Each
# element is keyed by its path of tags, with a repeat index for repeated siblings; leaves are compared by trimmed text.
# Usage: python -I diff12.py <answer A> <answer B>
import re
import sys
import xml.etree.ElementTree as ET


def voucher(path):
    text = open(path, 'rb').read().decode('utf-16-le')
    text = re.sub(r'&#(0*([0-8]|1[124]|1[4-9]|2[0-9]|3[01]));', '', text)
    text = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', text)
    coll = ET.fromstring(text).find('./BODY/DATA/COLLECTION')
    rows = [e for e in coll if e.tag == 'VOUCHER']
    if len(rows) != 1:
        raise SystemExit(f'{path}: {len(rows)} VOUCHER rows (one expected)')
    return rows[0]


def flat(e, prefix, out):
    seen = {}
    for c in e:
        seen[c.tag] = seen.get(c.tag, 0) + 1
        key = f'{prefix}/{c.tag}[{seen[c.tag]}]'
        if len(c):
            out[key] = '<list>' if c.tag.endswith('.LIST') else '<element with children>'
            flat(c, key, out)
        else:
            out[key] = (c.text or '').strip()
    for k, v in e.attrib.items():
        out[f'{prefix}/@{k}'] = v


a, b = {}, {}
flat(voucher(sys.argv[1]), 'VOUCHER', a)
flat(voucher(sys.argv[2]), 'VOUCHER', b)
absent = [k for k in a if k not in b]
diff = [k for k in a if k in b and a[k] != b[k]]
print(f'elements in A: {len(a)}; in B: {len(b)}; in A, absent from B: {len(absent)}; in both, different: {len(diff)}')
print('-- absent from B:')
for k in absent:
    print(f'{k} = {a[k]!r}')
print('-- different (A -> B):')
for k in diff:
    print(f'{k}: {a[k]!r} -> {b[k]!r}')
