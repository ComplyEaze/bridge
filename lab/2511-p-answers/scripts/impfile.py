# Lab helper (row 2511-P, new; not product code): reads a built import file as LAB-COMMON C4 says (decoded by its BOM or
# as UTF-8 when it has none; drops only XML's forbidden characters, raw or as references; an XML parser, never a text
# search). Prints the bytes and sha256, every VOUCHER under the file with its attributes (REMOTEID among them), DATE,
# VOUCHERTYPENAME, NARRATION (untrimmed, with its length), each ledger entry's LEDGERNAME, ISDEEMEDPOSITIVE and AMOUNT, and
# every element name present in the file once (names only). A file that does not parse is stop (exit 1).
# Usage: python -I -X utf8 impfile.py <file>
import hashlib, re, sys
import xml.etree.ElementTree as ET
raw = open(sys.argv[1], 'rb').read()
if raw[:2] == b'\xff\xfe': text = raw[2:].decode('utf-16-le'); enc = 'UTF-16LE with BOM'
elif raw[:3] == b'\xef\xbb\xbf': text = raw[3:].decode('utf-8'); enc = 'UTF-8 with BOM'
else: text = raw.decode('utf-8'); enc = 'UTF-8, no BOM'
FORB = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));", re.I)
n = len(FORB.findall(text)); text = FORB.sub('', text)
text = re.sub(r'^\s*<\?xml[^>]*\?>', '', text)
try: root = ET.fromstring(text)
except ET.ParseError as e: print('stop: does not parse:', e); sys.exit(1)
def show(s): return 'None' if s is None else repr(s.replace('\r', '\\r').replace('\n', '\\n'))
print(f'{sys.argv[1]}: {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}, {enc}; forbidden characters dropped: {n}')
vs = list(root.iter('VOUCHER')); print('VOUCHER elements:', len(vs))
for v in vs:
    print('VOUCHER attributes:', dict(v.attrib))
    for t in ('DATE', 'VOUCHERTYPENAME', 'PARTYLEDGERNAME', 'VOUCHERNUMBER', 'EFFECTIVEDATE'):
        e = v.find(t); print(f'  {t}:', '<absent>' if e is None else repr((e.text or '').strip()))
    e = v.find('NARRATION'); print('  NARRATION:', '<absent>' if e is None else f'{show(e.text)} (len {len(e.text or "")})')
    ents = [x for x in v if x.tag.endswith('LEDGERENTRIES.LIST')]
    print('  ledger entries:', len(ents))
    for x in ents:
        g = lambda t: '<absent>' if x.find(t) is None else repr((x.find(t).text or '').strip())
        print(f'    {x.tag}: LEDGERNAME {show(x.findtext("LEDGERNAME"))}; ISDEEMEDPOSITIVE {g("ISDEEMEDPOSITIVE")}; AMOUNT {g("AMOUNT")}; children {sorted({c.tag for c in x})}')
print('element names:', sorted({e.tag for e in root.iter()}))
