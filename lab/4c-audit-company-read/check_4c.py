# Structural checks of the 4c company-object response (parse only; no text search).
# Usage: python -I check_4c.py <response file>
import sys
import xml.etree.ElementTree as ET

raw = open(sys.argv[1], 'rb').read()
enc = 'utf-16-le' if raw[:2] in (b'<\x00', b'\xff\xfe') else 'utf-8'
text = raw.decode(enc)
if text.startswith('﻿'):
    text = text[1:]
GUID = 'a30f23c3-4819-4334-96ab-ffa6fba70976'
BOOKSFROM = '20260401'

end = text.rfind('</ENVELOPE>')
trailing = text[end + len('</ENVELOPE>'):] if end >= 0 else None
root = ET.fromstring(text)
status = root.findtext('HEADER/STATUS')
print('encoding', enc, 'chars', len(text))
print('(1) HEADER/STATUS =', repr(status), '; after </ENVELOPE>:', repr(trailing), '->',
      'YES' if status == '1' and trailing is not None and trailing.strip() == '' else 'NO')

msgs = root.findall('BODY/DATA/TALLYMESSAGE')
kids = [list(m) for m in msgs]
companies = [c for k in kids for c in k if c.tag == 'COMPANY']
print('(2) DATA/TALLYMESSAGE count =', len(msgs), '; children per message =',
      [[c.tag for c in k] for k in kids], '->',
      'YES' if len(msgs) == 1 and len(kids[0]) == 1 and kids[0][0].tag == 'COMPANY' else 'NO')

c = companies[0] if companies else None
if c is not None:
    g = c.findall('GUID')
    b = c.findall('BOOKSFROM')
    print('(3) COMPANY NAME attribute =', repr(c.get('NAME')), '; GUID count =', len(g), '; BOOKSFROM count =', len(b), '->',
          'YES' if c.get('NAME') is not None and len(g) == 1 and len(b) == 1 else 'NO')
    gv = g[0].text if g else None
    bv = b[0].text if b else None
    print('(4) GUID =', repr(gv), '; BOOKSFROM =', repr(bv), '->', 'YES' if gv == GUID and bv == BOOKSFROM else 'NO')
    others = [e for e in root.iter('COMPANY') if e is not c]
    bad = [(e.get('NAME'), [x.tag for x in e if x.tag in ('GUID', 'BOOKSFROM')]) for e in others
           if e.find('GUID') is not None or e.find('BOOKSFROM') is not None]
    print('(5) other COMPANY elements =', len(others), [(e.get('NAME'), len(list(e))) for e in others],
          '; carrying GUID/BOOKSFROM =', bad, '->', 'YES' if not bad else 'NO')
    print('MEASURED: COMPANY child elements =', len(list(c)), '; tags =', [x.tag for x in c])
    print('MEASURED: ISINTEGRATED =', [x.text for x in c.findall('ISINTEGRATED')], '; NAME children =', [x.text for x in c.iter('NAME')])
else:
    print('no COMPANY element')
