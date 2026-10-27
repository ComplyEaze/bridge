# Lab helper (#1231, read only): per voucher and ledger entry, the non-empty children of BANKALLOCATIONS.LIST and the
# values of BANKERSDATE, STATUS and PAYMENTMODE. Usage: python -I -X utf8 bankalloc.py <response file>
import re, sys
import xml.etree.ElementTree as ET
t = open(sys.argv[1], 'rb').read().decode('utf-16')
t = re.sub(r'&#(\d+);', lambda m: '' if int(m.group(1)) < 32 and int(m.group(1)) not in (9, 10, 13) else m.group(0), t)
root = ET.fromstring(t)
for v in root.iter('VOUCHER'):
    if not len(v):
        continue
    print('VOUCHER', v.findtext('VOUCHERNUMBER'), v.findtext('DATE'), v.findtext('GUID'), 'ALTERID', v.findtext('ALTERID'), v.findtext('VOUCHERTYPENAME'))
    for le in v.findall('ALLLEDGERENTRIES.LIST'):
        print('  ', le.findtext('LEDGERNAME'), le.findtext('AMOUNT'))
        for b in le.findall('BANKALLOCATIONS.LIST'):
            ne = [(c.tag, c.text.strip()) for c in b if c.text and c.text.strip()]
            print('     BANKALLOC nonempty', ne)
            print('     BANKERSDATE=%r STATUS=%r PAYMENTMODE=%r' % (b.findtext('BANKERSDATE'), b.findtext('STATUS'), b.findtext('PAYMENTMODE')))
