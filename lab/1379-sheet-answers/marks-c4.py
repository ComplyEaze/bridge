# Lab helper (1379 Part R, not product code): LAB-COMMON C4 for one m-marks-S answer. Decodes UTF-16LE, drops only the
# characters XML forbids (raw or as references), parses as XML, and takes the COMPANY rows under ENVELOPE/BODY/DATA/COLLECTION.
# Prints the row count, the GUID, and step 1's fields, trimmed. Exits 2 if the row is not exactly one with THE COMPANY's GUID.
# Usage: python -I -X utf8 marks-c4.py raw/<answer>.xml
import re, sys
import xml.etree.ElementTree as ET
GUID = '1edb9b05-8d35-4c0c-9959-dce655730463'
FIELDS = ['ALTMSTID', 'ALTVCHID', 'ISINVENTORYON', 'ISBATCHWISEON', 'ISINTEGRATED', 'NUMGODOWNS', 'NUMSTOCKITEMS', 'NUMUNITS',
          'NUMVOUCHERTYPES', 'BOOKSFROM', 'LASTVOUCHERDATE']
d = open(sys.argv[1], 'rb').read()
t = d.decode('utf-16-le')
if t.startswith('﻿'): t = t[1:]
BAD = set(range(0, 9)) | {11, 12} | set(range(14, 32))
t = ''.join(c for c in t if ord(c) not in BAD)
t = re.sub(r'&#(x[0-9A-Fa-f]+|[0-9]+);', lambda m: '' if int(m.group(1)[1:], 16 if m.group(1)[0] == 'x' else 10) in BAD else m.group(0)
           if m.group(1)[0] == 'x' else ('' if int(m.group(1)) in BAD else m.group(0)), t)
root = ET.fromstring(t)
rows = root.findall('./BODY/DATA/COLLECTION/COMPANY')
print(f'{sys.argv[1]}: {len(d)} B; COMPANY rows {len(rows)}')
ok = len(rows) == 1 and (rows[0].findtext('GUID') or '').strip() == GUID
for r in rows:
    print('  GUID', (r.findtext('GUID') or '').strip())
    for f in FIELDS:
        e = r.find(f)
        print(f'  {f}', 'ABSENT' if e is None else repr((e.text or '').strip()))
print('C4:', 'one COMPANY row with THE COMPANY\'s GUID' if ok else 'NOT one row with THE COMPANY\'s GUID: stop')
sys.exit(0 if ok else 2)
