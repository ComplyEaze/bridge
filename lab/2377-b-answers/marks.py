# Lab helper (2377-L Part B, not product code): reads a MARKS answer per LAB-COMMON C4: decode UTF-16LE, drop only the
# characters XML forbids, parse, and list each COMPANY row under ENVELOPE/BODY/DATA/COLLECTION with its NAME, GUID,
# ALTVCHID and ALTMSTID (trimmed). Exit 1 unless exactly one row.
# Usage: python -I marks.py <answer>
import re, sys
import xml.etree.ElementTree as ET
d = open(sys.argv[1], 'rb').read()
t = d.decode('utf-16-le')
if t.startswith('﻿'): t = t[1:]
t = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t)
t = re.sub(r'&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));', '', t, flags=re.I)
e = ET.fromstring(t)
rows = e.findall('./BODY/DATA/COLLECTION/COMPANY')
for r in rows:
    g = lambda k: (r.findtext(k) or '').strip()
    print(f"COMPANY row: NAME {r.get('NAME') or g('NAME')!r}, GUID {g('GUID')}, ALTVCHID {g('ALTVCHID')}, ALTMSTID {g('ALTMSTID')}")
print(f'COMPANY rows: {len(rows)}')
sys.exit(0 if len(rows) == 1 else 1)
