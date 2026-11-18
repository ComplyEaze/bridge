# Lab helper (2074-2214 W row, 18 Nov; not product code; xlsx.py's reader, printing a whole sheet): reads an .xlsx with the
# standard library only and prints every non-empty cell of one sheet, row by row, as REF='text' (cells over 300 characters
# cut, with their length). Prints C3 and C4 by name, empty or not. Prints the file's bytes and sha256 first. Nothing written.
# Usage: python -I -X utf8 xlsx2.py <workbook.xlsx> <sheet name>
import hashlib, re, sys, zipfile
import xml.etree.ElementTree as ET
p, want = sys.argv[1], sys.argv[2]
b = open(p, 'rb').read(); print(f'{p}: {len(b)} B sha256 {hashlib.sha256(b).hexdigest()}')
N = {'m': 'http://schemas.openxmlformats.org/spreadsheetml/2006/main',
     'r': 'http://schemas.openxmlformats.org/officeDocument/2006/relationships'}
z = zipfile.ZipFile(p)
ss = []
if 'xl/sharedStrings.xml' in z.namelist():
    for si in ET.fromstring(z.read('xl/sharedStrings.xml')).findall('m:si', N):
        ss.append(''.join(t.text or '' for t in si.iter('{%s}t' % N['m'])))
wb = ET.fromstring(z.read('xl/workbook.xml'))
rels = ET.fromstring(z.read('xl/_rels/workbook.xml.rels'))
rmap = {r.get('Id'): r.get('Target') for r in rels}
target = None
for s in wb.find('m:sheets', N):
    if s.get('name') == want: target = rmap[s.get('{%s}id' % N['r'])]
if target is None: raise SystemExit('no sheet ' + want)
target = target.lstrip('/'); target = target if target.startswith('xl/') else 'xl/' + target
sh = ET.fromstring(z.read(target))
cells = {}
for row in sh.iter('{%s}row' % N['m']):
    out = []
    for c in row.findall('m:c', N):
        ref, t = c.get('r'), c.get('t')
        v = c.find('m:v', N); is_ = c.find('m:is', N)
        if t == 's' and v is not None: val = ss[int(v.text)]
        elif t == 'inlineStr' and is_ is not None: val = ''.join(x.text or '' for x in is_.iter('{%s}t' % N['m']))
        elif v is not None: val = v.text or ''
        else: val = ''
        cells[ref] = val
        if val != '': out.append(f"{ref}='{val[:300]}'" + (f' ({len(val)} chars)' if len(val) > 300 else ''))
    print(f"row {row.get('r')}: " + ' | '.join(out))
for ref in ('C3', 'C4'): print(f"{ref}: {cells.get(ref, '')!r}")
