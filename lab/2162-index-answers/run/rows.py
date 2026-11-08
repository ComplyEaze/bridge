# Lab helper (#2162 index rows, not product code): prints an answer's bytes and its VOUCHER rows' key fields. Usage: python -I -X utf8 rows.py <answer>
import sys, xml.etree.ElementTree as ET
d = open(sys.argv[1], 'rb').read()
t = d.decode('utf-16') if d[:2] == b'\xff\xfe' or d[1:2] == b'\x00' else d.decode('utf-8')
rows = ET.fromstring(t.lstrip('﻿')).findall('./BODY/DATA/COLLECTION/*')
print(sys.argv[1], len(d), 'B; rows', len(rows))
for x in rows: print(' ', {k: (x.findtext(k) or '').strip() for k in ('DATE', 'VOUCHERTYPENAME', 'VOUCHERNUMBER', 'ISCANCELLED', 'MASTERID', 'GUID')})
