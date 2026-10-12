# Inspect a W14 company object/fetch answer: ENVELOPE/BODY presence, tag count, and every element whose name
# contains HSN, TOLER, GSTCLASS or GSTDETAIL (name, text, children). Usage: python -I objscan.py <answer>
import sys, xml.etree.ElementTree as ET
raw = open(sys.argv[1], 'rb').read()
t = raw.decode('utf-16-le').lstrip('﻿').replace('&#4;', '')
r = ET.fromstring(t)
print('root', r.tag, '| BODY', r.find('BODY') is not None, '| LINEERROR', [e.text for e in r.iter('LINEERROR')], '| elements', sum(1 for _ in r.iter()))
comp = [e for e in r.iter('COMPANY')]
print('COMPANY elements', len(comp), [c.get('NAME') for c in comp][:3])
for e in r.iter():
    u = e.tag.upper()
    if any(k in u for k in ('HSN', 'TOLER', 'GSTCLASS', 'GSTDETAIL', 'GSTAPPLIC')):
        kids = [c.tag for c in e]
        print(' ', e.tag, repr((e.text or '').strip())[:80], ('children ' + str(kids[:12])) if kids else '')
