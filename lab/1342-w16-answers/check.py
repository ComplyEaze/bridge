# Check a W11 answer by parsing: complete ENVELOPE, a COLLECTION element, no LINEERROR. Usage: python -I check.py <answer>
import re
import sys
import xml.etree.ElementTree as ET

sys.stdout.reconfigure(encoding='utf-8')
raw = open(sys.argv[1], 'rb').read()
enc = 'utf-16-le' if raw[:2] in (b'<\x00', b'\xff\xfe') else 'utf-8'
text = raw.decode(enc).lstrip('﻿')
n4 = len(re.findall(r'&#4;', text))
text = text.replace('&#4;', '�#4;')
try:
    root = ET.fromstring(text)
except ET.ParseError as e:
    print('PARSE ERROR', e)
    sys.exit(2)
coll = list(root.iter('COLLECTION'))
lerr = [e.text for e in root.iter('LINEERROR')]
print(f'encoding {enc}; root {root.tag}; COLLECTION elements {len(coll)}; LINEERROR {lerr}; &#4; refs {n4}')
for c in coll:
    kids = [k.tag for k in c]
    print('  COLLECTION children:', len(kids), sorted(set(kids)))
print('OK' if root.tag == 'ENVELOPE' and coll and not lerr else 'STOP-CHECK')
