# Summarise a W11 read-back answer: voucher number, narration, date and the HSN source fields per sales line.
# Usage: python -I rb.py <answer file>
import re
import sys

sys.stdout.reconfigure(encoding='utf-8')
raw = open(sys.argv[1], 'rb').read()
t = raw.decode('utf-16-le').lstrip('﻿').replace('&#4;', '')
vouchers = re.findall(r'<VOUCHER [^>]*>(.*?)</VOUCHER>', t, re.S)
print('VOUCHER count', len(vouchers))
for v in vouchers:
    for tag in ['VOUCHERNUMBER', 'NARRATION', 'DATE']:
        print(' ', tag, re.findall(r'<%s[^>]*>([^<]*)</%s>' % (tag, tag), v)[:1])
    for tag in ['HSNSOURCETYPE', 'HSNLEDGERSOURCE', 'HSNGROUPSOURCE', 'GSTHSNNAME', 'GSTHSNINFERAPPLICABILITY', 'GSTOVRDNTAXABILITY']:
        vals = re.findall(r'<%s[^>]*>([^<]*)</%s>' % (tag, tag), v)
        print(' ', tag, vals[:6])
    print('  LEDGERNAME', re.findall(r'<LEDGERNAME[^>]*>([^<]*)</LEDGERNAME>', v)[:6])
