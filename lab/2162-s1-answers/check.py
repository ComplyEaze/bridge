# Lab helper (2162-S1, not product code): the row's record by an XML parser: for R, C and the control, bytes, sha256, VOUCHER rows, ISPOSTDATED direct children and text, ISCANCELLED, ISOPTIONAL, the three VCHGSTSTATUS flags, ISGSTOVERRIDDEN, ALTERID; each pair byte-identical; R with its ISPOSTDATED line and line break removed against C. Writes evidence/check.txt.
import hashlib, re
import xml.etree.ElementTree as ET
L = '<lab folder>/reads/'
out = open('<lab folder>/evidence/check.txt', 'w', encoding='utf-8', newline='\n')
def p(*a):
    s = ' '.join(str(x) for x in a); print(s); out.write(s + '\n')
F = ('ISCANCELLED', 'ISOPTIONAL', 'VCHGSTSTATUSISINCLUDED', 'VCHGSTSTATUSISUNCERTAIN', 'VCHGSTSTATUSISEXCLUDED', 'ISGSTOVERRIDDEN', 'ALTERID', 'VOUCHERNUMBER', 'DATE')
def rd(n): return open(L + n + '.xml', 'rb').read()
def tree(b):
    s = b.decode('utf-16-le', 'replace').lstrip(chr(0xfeff)); s = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', s); s = re.sub(r'&#([0-9]);', '', s); return ET.fromstring(s)
for n in ('R-0045-1', 'R-0045-2', 'C-0045-1', 'C-0045-2', 'R-0010-ctl'):
    b = rd(n); vs = [v for v in tree(b).iter('VOUCHER') if v.attrib or len(v)]
    p(n, 'bytes', len(b), 'sha256', hashlib.sha256(b).hexdigest(), 'VOUCHER rows', len(vs))
    for v in vs:
        pd = v.findall('ISPOSTDATED')
        p('   ISPOSTDATED direct children', len(pd), [e.text for e in pd], '|', {f: (v.findtext(f) or '').strip() for f in F})
p('R pair byte-identical', rd('R-0045-1') == rd('R-0045-2'), '| C pair byte-identical', rd('C-0045-1') == rd('C-0045-2'))
r = rd('R-0045-1').decode('utf-16-le'); c = rd('C-0045-1').decode('utf-16-le')
lines = [l for l in r.splitlines(True) if '<ISPOSTDATED' in l]
p('ISPOSTDATED lines in R', len(lines), [l.strip() for l in lines])
p('R with its ISPOSTDATED line and line break removed equals C byte for byte', ''.join(l for l in r.splitlines(True) if '<ISPOSTDATED' not in l).encode('utf-16-le') == c.encode('utf-16-le'))
for n in ('marks-0', 'marks-1'):
    s = rd(n).decode('utf-16-le', 'replace'); p(n, re.findall(r'<(ALTVCHID|ALTMSTID)[^>]*>\s*([^<]*)', s))
