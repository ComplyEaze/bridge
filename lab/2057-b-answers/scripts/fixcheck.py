# Lab helper (2057-B fixture, copied from the 2027 fixcheck.py with its book changed; the company number is read from the
# BK3 file name, never typed, not product code): checks answer files unedited before they are offered as fixtures.
# Prints size and sha256, then offsets and element names only (never a value): digit runs of 10 or more and their
# element, runs inside an IRN, the company number in UTF-8 and UTF-16LE, GSTIN and PAN shapes with their element,
# phone shapes, and the book GUID's occurrences.
# Usage: python -I -X utf8 fixcheck.py <file> [<file> ...]
import hashlib, re, sys
import os
GUID = '92a7ef97-dd28-4286-b02c-c059fcb8690e'; N = os.listdir('<LAB_DIR>/backups/bk3')[0].replace('TDBK1800_', '').replace('.001', '')
GSTIN = re.compile(r'(?<![0-9A-Za-z])[0-9]{2}[A-Z]{5}[0-9]{4}[A-Z][0-9A-Z]Z[0-9A-Z]')
GSTIN_COUNTPY = re.compile(r'\d{2}[A-Z]{5}\d{4}[A-Z][A-Z\d]Z[A-Z\d]')
PAN = re.compile(r'(?<![0-9A-Za-z])[A-Z]{5}[0-9]{4}[A-Z](?![0-9A-Za-z])')
PHONE = re.compile(r'(?<![\d.\-])(?:\+?91[\s-]?)?[6-9]\d{9}(?![\d.])')
def element(t, at):
    o = t.rfind('<', 0, at)
    m = re.match(r'<([A-Za-z0-9._]+)', t[o:])
    return m.group(1) if m else '?'
for f in sys.argv[1:]:
    b = open(f, 'rb').read()
    t = b[2:].decode('utf-16-le') if b[:2] == b'\xff\xfe' else b.decode('utf-16-le')
    print(f.rsplit('/', 1)[-1], len(b), 'B sha256', hashlib.sha256(b).hexdigest(), '| BOM' if b[:2] == b'\xff\xfe' else '| no BOM')
    runs = [(m.start(), len(m.group()), element(t, m.start())) for m in re.finditer(r'\d{10,}', t)]
    print('  digit runs >=10 outside VOUCHERKEY:', [r for r in runs if r[2] != 'VOUCHERKEY'], '| in VOUCHERKEY:', sum(1 for r in runs if r[2] == 'VOUCHERKEY'))
    irn = [(m.start(1), len(m.group(1))) for m in re.finditer(r'<IRN[^>]*>([^<]*)</IRN>', t)]
    print('  IRN elements (offset, length):', irn, '| digit runs inside an IRN:', sum(len(re.findall(r'\d{10,}', t[s:s + l])) for s, l in irn))
    print('  company number: utf8', b.count(N.encode()), 'utf16le', b.count(N.encode('utf-16-le')))
    print('  GSTIN shapes (char offset, element):', [(m.start(), element(t, m.start())) for m in GSTIN.finditer(t)], '| count.py rule:', len(GSTIN_COUNTPY.findall(t)))
    print('  PAN shapes (char offset, element):', [(m.start(), element(t, m.start())) for m in PAN.finditer(t)])
    print('  phone shapes:', [(m.start(), element(t, m.start())) for m in PHONE.finditer(t)])
    print('  GUID occurrences:', t.count(GUID))
