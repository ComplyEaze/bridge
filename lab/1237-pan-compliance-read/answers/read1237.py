# Lab helper (#1237, not product code): read a saved compliance-listing answer by structure (C4).
# Lists every LEDGER row under ENVELOPE/BODY/DATA/COLLECTION with NAME, PARENT, ALTERID and, for
# INCOMETAXNUMBER, PARTYGSTIN and NAMEONPAN, exactly one of: absent, empty, or the text (with its raw characters).
# Usage: python -I read1237.py <answer.xml> [<out.json>]
import json
import re
import sys
import xml.etree.ElementTree as ET

WATCH = ('BRIDGE PAN Party A', 'BRIDGE PAN Party B', 'BRIDGE PAN Party C')


def load(path):
    b = open(path, 'rb').read()
    if b[:2] == b'\xff\xfe':
        b = b[2:]
    return b.decode('utf-16-le') if b[1:2] == b'\x00' else b.decode('utf-8')


def state(row, tag):
    els = [e for e in row if e.tag == tag]
    if not els:
        return 'absent'
    if len(els) > 1:
        return 'several: ' + repr([(e.text or '') for e in els])
    t = els[0].text
    return 'empty' if t is None or t == '' else 'text ' + repr(t)


t = load(sys.argv[1])
root = ET.fromstring(re.sub(r'&#4;', '[4]', t))
coll = root.find('./BODY/DATA/COLLECTION')
rows = [] if coll is None else [r for r in coll if r.tag == 'LEDGER']
out = {'rows': len(rows), 'incometaxnumber_elements_present': 0, 'ledgers': []}
for r in rows:
    pe = r.find('PARENT')
    ae = r.find('ALTERID')
    item = {'name': r.get('NAME'), 'parent': (pe.text or '').strip() if pe is not None else None,
            'alterid': (ae.text or '').strip() if ae is not None else None,
            'INCOMETAXNUMBER': state(r, 'INCOMETAXNUMBER')}
    out['incometaxnumber_elements_present'] += sum(1 for e in r if e.tag == 'INCOMETAXNUMBER')
    if item['name'] in WATCH:
        item['PARTYGSTIN'] = state(r, 'PARTYGSTIN')
        item['NAMEONPAN'] = state(r, 'NAMEONPAN')
    out['ledgers'].append(item)
if len(sys.argv) > 2:
    json.dump(out, open(sys.argv[2], 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
print('rows', out['rows'], 'INCOMETAXNUMBER elements present', out['incometaxnumber_elements_present'])
for i in out['ledgers']:
    if i['name'] in WATCH or i['INCOMETAXNUMBER'] != 'absent':
        print(json.dumps(i, ensure_ascii=False))
