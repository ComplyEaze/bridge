# Dump every sheet of an .xlsx as tab-separated rows (stdlib only).
# Usage: python -I dumpxlsx.py <file.xlsx>
import re
sys_reconf = None
import sys
import zipfile
import xml.etree.ElementTree as ET

sys.stdout.reconfigure(encoding="utf-8")
NS = {'m': 'http://schemas.openxmlformats.org/spreadsheetml/2006/main'}
z = zipfile.ZipFile(sys.argv[1])
ss = []
if 'xl/sharedStrings.xml' in z.namelist():
    for si in ET.fromstring(z.read('xl/sharedStrings.xml')).findall('m:si', NS):
        ss.append(''.join(t.text or '' for t in si.iter('{%s}t' % NS['m'])))
wb = ET.fromstring(z.read('xl/workbook.xml'))
names = [s.get('name') for s in wb.find('m:sheets', NS)]
sheets = sorted([n for n in z.namelist() if re.match(r'xl/worksheets/sheet\d+\.xml', n)], key=lambda n: int(re.findall(r'\d+', n)[0]))
for name, path in zip(names, sheets):
    print('=== sheet', name)
    for row in ET.fromstring(z.read(path)).iter('{%s}row' % NS['m']):
        vals = []
        for c in row.findall('m:c', NS):
            t = c.get('t'); v = c.find('m:v', NS); isel = c.find('m:is', NS)
            if t == 's' and v is not None:
                vals.append(ss[int(v.text)])
            elif t == 'inlineStr' and isel is not None:
                vals.append(''.join(x.text or '' for x in isel.iter('{%s}t' % NS['m'])))
            else:
                vals.append(v.text if v is not None else '')
        print(row.get('r'), '\t'.join(vals))
