# Lab helper (#1356, not product code): for each answer, parses it by structure (UTF-16LE, forbidden characters dropped,
# as XML) and prints the row element under ENVELOPE/BODY/DATA/COLLECTION, the row count, and how many rows carry a
# non-empty GUID, ALTERID and MASTERID. Read only. Usage: python -I rows.py <answer> [...]
import re
import sys
import xml.etree.ElementTree as ET
from collections import Counter

for path in sys.argv[1:]:
    raw = open(path, 'rb').read()
    try:
        t = raw.decode('utf-16-le').lstrip('﻿')
        t = re.sub(r'&#(0*([0-8]|1[124]|1[4-9]|2[0-9]|3[01]));', '', t)
        t = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t)
        root = ET.fromstring(t)
    except Exception as e:
        print(f'{path}: {len(raw)} B; does not parse: {e}')
        continue
    coll = root.find('./BODY/DATA/COLLECTION')
    rows = [] if coll is None else list(coll)
    tags = Counter(r.tag for r in rows)
    full = sum(1 for r in rows if all((r.findtext(k) or '').strip() for k in ('GUID', 'ALTERID', 'MASTERID')))
    print(f'{path}: {len(raw)} B; parses; rows {dict(tags)}; with GUID, ALTERID and MASTERID non-empty: {full}')
