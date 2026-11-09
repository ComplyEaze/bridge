# Lab helper (2074-2214 Part W, not product code): reads an .xlsx workbook with the standard library only (zipfile and
# xml.etree), no package installed. Prints, per sheet: its name, its size, cells A1 to F6, the header row (the first row
# whose cells include "Ledger" or "Name"), the row count below it, and the whole row of every ledger named on the command
# line. Prints the file's bytes and sha256 first. Nothing is written.
# Usage: python -I -X utf8 xlsx.py <workbook.xlsx> [ledger name ...]
import hashlib, re, sys, zipfile
import xml.etree.ElementTree as ET
NS = {'m': 'http://schemas.openxmlformats.org/spreadsheetml/2006/main', 'r': 'http://schemas.openxmlformats.org/officeDocument/2006/relationships'}
path, names = sys.argv[1], sys.argv[2:]
raw = open(path, 'rb').read(); print(f'{path.rsplit(chr(92), 1)[-1].rsplit("/", 1)[-1]}: {len(raw)} B sha256 {hashlib.sha256(raw).hexdigest()}')
z = zipfile.ZipFile(path)
ss = []
if 'xl/sharedStrings.xml' in z.namelist():
    for si in ET.fromstring(z.read('xl/sharedStrings.xml')).findall('m:si', NS):
        ss.append(''.join(t.text or '' for t in si.iter('{%s}t' % NS['m'])))
wb = ET.fromstring(z.read('xl/workbook.xml'))
rels = {r.get('Id'): r.get('Target') for r in ET.fromstring(z.read('xl/_rels/workbook.xml.rels'))}
def col(ref):
    n = 0
    for ch in re.match(r'[A-Z]+', ref).group():
        n = n * 26 + ord(ch) - 64
    return n
def value(c):
    t = c.get('t'); v = c.find('m:v', NS)
    if t == 's' and v is not None: return ss[int(v.text)]
    if t == 'inlineStr':
        return ''.join(x.text or '' for x in c.iter('{%s}t' % NS['m']))
    return v.text if v is not None else ''
for s in wb.find('m:sheets', NS):
    target = rels[s.get('{%s}id' % NS['r'])].lstrip('/'); target = target if target.startswith('xl/') else 'xl/' + target
    rows = {}
    for r in ET.fromstring(z.read(target)).iter('{%s}row' % NS['m']):
        rn = int(r.get('r')); rows[rn] = {}
        for c in r.findall('m:c', NS): rows[rn][col(c.get('r'))] = value(c)
    print(f'== sheet {s.get("name")!r}: {len(rows)} rows')
    for rn in range(1, 7):
        cells = rows.get(rn, {})
        print(f'  row {rn}: ' + ' | '.join(f'{chr(64 + k)}{rn}={cells[k]!r}' for k in sorted(cells) if k <= 6 and cells[k] != ''))
    head = next((rn for rn in sorted(rows) if any(str(v).strip() in ('Ledger', 'Ledger name', 'Name', 'Party') for v in rows[rn].values())), None)
    if head is None: continue
    hdr = rows[head]; print(f'  header row {head}: ' + ' | '.join(f'{k}:{hdr[k]}' for k in sorted(hdr)))
    body = [rn for rn in sorted(rows) if rn > head and any(v != '' for v in rows[rn].values())]
    print(f'  rows below the header: {len(body)}')
    for n in names:
        for rn in body:
            if n in [str(v).strip() for v in rows[rn].values()]:
                print(f'  {n!r} row {rn}: ' + ' | '.join(f'{hdr.get(k, k)}={rows[rn][k]!r}' for k in sorted(rows[rn])))
