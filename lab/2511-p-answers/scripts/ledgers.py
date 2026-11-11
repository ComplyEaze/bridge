# Lab helper (row 2511-P, new; not product code): reads a ledger catalogue answer as LAB-COMMON C4 says (UTF-16LE with or
# without a BOM; drops only XML's forbidden characters, raw or as references, keeping 9, 10 and 13; an XML parser, never a
# text search) and prints HEADER/STATUS, every LINEERROR, the count of LEDGER rows (children of ENVELOPE/BODY/DATA/COLLECTION),
# then per LEDGER: NAME (untrimmed, line breaks shown as \r \n, with its length), PARENT, ISBILLWISEON, CURRENCYNAME (or
# <absent>; '' for empty), and the names of every child element once. A file that does not parse, or has no COLLECTION, is
# stop (exit 1). Usage: python -I -X utf8 ledgers.py <answer>
import re, sys
import xml.etree.ElementTree as ET
raw = open(sys.argv[1], 'rb').read()
body = raw[2:] if raw[:2] == b'\xff\xfe' else raw
text = body.decode('utf-16-le')
FORB = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));", re.I)
n = len(FORB.findall(text)); text = FORB.sub('', text)
try: root = ET.fromstring(text)
except ET.ParseError as e: print('stop: does not parse:', e); sys.exit(1)
def show(s): return 'None' if s is None else repr(s.replace('\r', '\\r').replace('\n', '\\n'))
print(f'{sys.argv[1]}: {len(raw)} bytes; forbidden characters dropped: {n}')
st = root.find('HEADER/STATUS'); print('HEADER/STATUS', None if st is None else (st.text or '').strip())
for le in root.iter('LINEERROR'): print('LINEERROR', show(le.text))
col = root.find('BODY/DATA/COLLECTION')
if col is None: print('stop: no BODY/DATA/COLLECTION'); sys.exit(1)
rows = [c for c in col if c.tag == 'LEDGER']; other = [c.tag for c in col if c.tag != 'LEDGER']
print(f'LEDGER rows: {len(rows)}; other COLLECTION children: {other}')
for r in rows:
    nm = r.find('NAME'); name = r.get('NAME') if nm is None else nm.text
    def f(tag):
        e = r.find(tag)
        return '<absent>' if e is None else repr((e.text or '').strip())
    print(f'LEDGER NAME {show(name)} (len {len(name or "")}, attribute NAME {show(r.get("NAME"))}); PARENT {f("PARENT")}; '
          f'ISBILLWISEON {f("ISBILLWISEON")}; CURRENCYNAME {f("CURRENCYNAME")}')
    print('  children:', sorted({c.tag for c in r}))
