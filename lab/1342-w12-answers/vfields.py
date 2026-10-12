# Print selected fields (by element name, structurally) from the COLLECTION/VOUCHER of W11 read-back answers.
# Usage: python -I vfields.py <answer> [<answer> ...]
import re
import sys
import xml.etree.ElementTree as ET

sys.stdout.reconfigure(encoding='utf-8')
NAMES = ['VOUCHERNUMBER', 'ALTERID', 'HSNSOURCETYPE', 'HSNLEDGERSOURCE', 'HSNGROUPSOURCE', 'GSTHSNNAME',
         'GSTHSNINFERAPPLICABILITY', 'VCHGSTSTATUSISINCLUDED', 'VCHGSTSTATUSISUNCERTAIN', 'VCHGSTSTATUSISEXCLUDED']
for p in sys.argv[1:]:
    t = open(p, 'rb').read().decode('utf-16-le').lstrip('﻿').replace('&#4;', '')
    t = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t)
    v = ET.fromstring(t).findall('.//COLLECTION/VOUCHER')
    assert len(v) == 1, p
    row = []
    for n in NAMES:
        vals = [(e.text or '').strip() for e in v[0].iter(n)]
        row.append('%s=%s' % (n, '|'.join(vals) if vals else '<absent>'))
    print(p.split('\\')[-1], '; '.join(row))
