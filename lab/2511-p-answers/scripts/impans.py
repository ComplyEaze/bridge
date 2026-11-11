# Lab helper (row 2511-P, new; not product code): reads an import answer as LAB-COMMON C4 says (UTF-16LE when it starts
# with a BOM or its second byte is 0, else UTF-8; drops only XML's forbidden characters; an XML parser) and prints its
# bytes, sha256, the encoding read, HEADER/STATUS, every counter (CREATED, ALTERED, DELETED, LASTVCHID, LASTMID, COMBINED,
# IGNORED, ERRORS, CANCELLED, EXCEPTIONS) or <absent>, every LINEERROR, and a verdict: CREATED 1 AND OTHERS 0, or NOT AS
# EXPECT naming why. A file that does not parse is stop (exit 1); NOT AS EXPECT exits 2. Usage: python -I -X utf8 impans.py <answer>
import hashlib, re, sys
import xml.etree.ElementTree as ET
raw = open(sys.argv[1], 'rb').read()
if raw[:2] == b'\xff\xfe': text = raw[2:].decode('utf-16-le'); enc = 'UTF-16LE with BOM'
elif len(raw) > 1 and raw[1] == 0: text = raw.decode('utf-16-le'); enc = 'UTF-16LE, no BOM'
else: text = raw.decode('utf-8'); enc = 'UTF-8'
FORB = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));", re.I)
text = FORB.sub('', text); text = re.sub(r'^\s*<\?xml[^>]*\?>', '', text)
print(f'{sys.argv[1]}: {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}, read as {enc}')
try: root = ET.fromstring(text)
except ET.ParseError as e: print('stop: does not parse:', e); print(repr(text[:300])); sys.exit(1)
st = root.find('HEADER/STATUS'); print('HEADER/STATUS', '<absent>' if st is None else repr((st.text or '').strip()))
C = ('CREATED', 'ALTERED', 'DELETED', 'LASTVCHID', 'LASTMID', 'COMBINED', 'IGNORED', 'ERRORS', 'CANCELLED', 'EXCEPTIONS')
v = {}
for c in C:
    e = next(root.iter(c), None); v[c] = None if e is None else (e.text or '').strip(); print(f'{c} {"<absent>" if e is None else repr(v[c])}')
le = [(x.text or '') for x in root.iter('LINEERROR')]; print('LINEERROR count', len(le)); [print('LINEERROR', repr(x)) for x in le]
why = [c for c in C if v[c] is None and c not in ('LASTVCHID', 'LASTMID')]
why += [c for c in ('ALTERED', 'DELETED', 'COMBINED', 'IGNORED', 'ERRORS', 'CANCELLED', 'EXCEPTIONS') if v[c] not in (None, '0')]
if v['CREATED'] != '1': why.append('CREATED')
if le: why.append('LINEERROR')
if why: print('NOT AS EXPECT:', why); sys.exit(2)
print('CREATED 1 AND OTHERS 0'); sys.exit(0)
