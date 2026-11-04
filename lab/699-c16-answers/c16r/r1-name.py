# Lab helper (699-C16R, not product code): R1. Parses one kept answer by structure (UTF-16LE, characters XML forbids dropped,
# parsed as XML) and finds every VOUCHER whose subtree holds a NAME element with a GUID-shaped value containing a run of 10 or
# more digits. Prints, per voucher, VOUCHERTYPENAME, VOUCHERNUMBER, DATE and the tag of the element directly above that NAME.
# Never prints the value. Usage: python -I -X utf8 r1-name.py <answer file>
import re, sys, xml.etree.ElementTree as ET
a = open(sys.argv[1], 'rb').read()
t = (a[2:] if a[:2] == b'\xff\xfe' else a).decode('utf-16-le')
BAD = set(range(0, 9)) | {11, 12} | set(range(14, 32))
t = re.sub(r'&#([xX][0-9a-fA-F]+|[0-9]+);', lambda m: '' if (int(m.group(1)[1:], 16) if m.group(1)[0] in 'xX' else int(m.group(1))) in BAD else m.group(0), t)
t = ''.join(ch for ch in t if ord(ch) not in BAD)
root = ET.fromstring(t)
GUID = re.compile(r'^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}')
found = []
for v in root.iter('VOUCHER'):
    parent = {c: p for p in v.iter() for c in p}
    for n in v.iter('NAME'):
        val = (n.text or '').strip()
        if GUID.match(val) and re.search(r'\d{10,}', val):
            found.append((v.findtext('VOUCHERTYPENAME'), v.findtext('VOUCHERNUMBER'), v.findtext('DATE'), parent[n].tag if n in parent else '<VOUCHER itself>'))
vouchers = {f[:3] for f in found}
print('VOUCHER rows in the answer:', len(list(root.iter('VOUCHER'))), '| NAME hits:', len(found), '| vouchers holding one:', len(vouchers))
for f in found: print('VOUCHERTYPENAME', f[0], '| VOUCHERNUMBER', f[1], '| DATE', f[2], '| list element above the NAME:', f[3])
