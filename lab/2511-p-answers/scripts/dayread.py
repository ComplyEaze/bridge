# Lab helper (row 2511-P, new; not product code): reads a movement answer as LAB-COMMON C4 says (UTF-16LE with or without
# a BOM; drops only XML's forbidden characters, raw or as references; an XML parser, never a text search) and prints each
# VOUCHER row (children of ENVELOPE/BODY/DATA/COLLECTION), sorted by MASTERID as a number: DATE, VOUCHERTYPENAME,
# VOUCHERNUMBER, MASTERID, ALTERID, GUID, NARRATION (untrimmed, line breaks shown, with its length), each ledger entry
# (every child list whose tag ends in LEDGERENTRIES.LIST) with LEDGERNAME, AMOUNT and ISDEEMEDPOSITIVE, and under it every
# BILLALLOCATIONS.LIST with NAME, BILLTYPE and AMOUNT; then the row's element names. stop (exit 1) when it does not parse
# or has no COLLECTION, or a GUID does not begin with the book's GUID. Usage: python -I -X utf8 dayread.py <answer>
import hashlib, re, sys
import xml.etree.ElementTree as ET
BOOK = '1bb48be5-baba-45b2-90d8-7593244915a9'
raw = open(sys.argv[1], 'rb').read()
body = raw[2:] if raw[:2] == b'\xff\xfe' else raw
text = body.decode('utf-16-le')
FORB = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));", re.I)
n = len(FORB.findall(text)); text = FORB.sub('', text)
print(f'{sys.argv[1]}: {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}; forbidden characters dropped: {n}')
try: root = ET.fromstring(text)
except ET.ParseError as e: print('stop: does not parse:', e); sys.exit(1)
def show(s): return 'None' if s is None else repr(s.replace('\r', '\\r').replace('\n', '\\n'))
def tx(e, t):
    x = e.find(t); return '<absent>' if x is None else repr((x.text or '').strip())
st = root.find('HEADER/STATUS'); print('HEADER/STATUS', '<absent>' if st is None else (st.text or '').strip())
for le in root.iter('LINEERROR'): print('LINEERROR', show(le.text))
col = root.find('BODY/DATA/COLLECTION')
if col is None: print('stop: no BODY/DATA/COLLECTION'); sys.exit(1)
rows = [c for c in col if c.tag == 'VOUCHER']; print('VOUCHER rows:', len(rows), '; other children:', [c.tag for c in col if c.tag != 'VOUCHER'])
def mid(v):
    try: return int((v.findtext('MASTERID') or '').strip())
    except ValueError: return 10 ** 12
bad = 0
for v in sorted(rows, key=mid):
    g = (v.findtext('GUID') or '').strip()
    if not g.startswith(BOOK + '-'): bad += 1
    nar = v.find('NARRATION')
    print(f'VOUCHER DATE {tx(v, "DATE")} TYPE {tx(v, "VOUCHERTYPENAME")} NUMBER {tx(v, "VOUCHERNUMBER")} MASTERID {tx(v, "MASTERID")} '
          f'ALTERID {tx(v, "ALTERID")} GUID {g!r} attributes {dict(v.attrib)}')
    print('  NARRATION', '<absent>' if nar is None else f'{show(nar.text)} (len {len(nar.text or "")})')
    print('  PARTYLEDGERNAME', show(v.findtext('PARTYLEDGERNAME')))
    for x in v:
        if not x.tag.endswith('LEDGERENTRIES.LIST'): continue
        print(f'  {x.tag}: LEDGERNAME {show(x.findtext("LEDGERNAME"))}; AMOUNT {tx(x, "AMOUNT")}; ISDEEMEDPOSITIVE {tx(x, "ISDEEMEDPOSITIVE")}')
        for b in x.iter('BILLALLOCATIONS.LIST'):
            print(f'    BILLALLOCATIONS.LIST: NAME {show(b.findtext("NAME"))}; BILLTYPE {tx(b, "BILLTYPE")}; AMOUNT {tx(b, "AMOUNT")}')
    print('  element names:', sorted({e.tag for e in v.iter()}))
if bad: print('stop:', bad, 'GUID(s) not the book\'s'); sys.exit(1)
