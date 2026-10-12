# Quick structural look at a Tally answer: encoding, error markers, ledger NAME attribute vs stored name, voucher LEDGERNAMEs.
# Usage: python -I check.py <answer file>
import re
import sys

sys.stdout.reconfigure(encoding='utf-8')
raw = open(sys.argv[1], 'rb').read()
if raw[:2] in (b'\xff\xfe', b'\xfe\xff'):
    txt = raw.decode('utf-16')
elif raw[1:2] == b'\x00':
    txt = raw.decode('utf-16-le')
else:
    txt = raw.decode('utf-8', 'replace')
txt = txt.replace('&#4;', '\u0004')
err = re.findall(r'<LINEERROR>[^<]*</LINEERROR>|<EXCEPTIONS>[^<]*</EXCEPTIONS>|Could not find', txt)
print(sys.argv[1], 'bytes', len(raw), 'errors', err[:3])
for m in re.finditer(r'<LEDGER NAME="([^"]*)"[^>]*>(.*?)</LEDGER>', txt, re.S):
    stored = re.search(r'<LANGUAGENAME\.LIST>.*?<NAME\.LIST[^>]*>\s*<NAME>([^<]*)</NAME>', m.group(2), re.S)
    attr = m.group(1)
    if 'spell' in attr.lower() or (stored and 'spell' in stored.group(1).lower()):
        print('  LEDGER attr=%r stored=%r differ=%s' % (attr, stored.group(1) if stored else None, stored is not None and attr != stored.group(1)))
for v in re.finditer(r'<VOUCHER [^>]*>(.*?)</VOUCHER>', txt, re.S):
    body = v.group(1)
    date = re.search(r'<DATE[^>]*>([^<]*)</DATE>', body)
    num = re.search(r'<VOUCHERNUMBER[^>]*>([^<]*)<', body)
    party = re.search(r'<PARTYLEDGERNAME[^>]*>([^<]*)<', body)
    names = re.findall(r'<LEDGERNAME[^>]*>([^<]*)</LEDGERNAME>', body)
    print('  VOUCHER date=%s no=%s party=%r ledgernames=%r' % (date.group(1) if date else None, num.group(1) if num else None, party.group(1) if party else None, names))
