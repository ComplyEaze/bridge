# W21b: per answer, the voucher set and, for BP/26-27/0045 and every other voucher, the flags; compares the others with aw21b-a.
import re, sys, xml.etree.ElementTree as ET, collections
def load(p):
    t = open(p, 'rb').read().decode('utf-16')
    t = re.sub(r'&#(\d+);', lambda m: '?' if int(m.group(1)) < 32 and int(m.group(1)) not in (9, 10, 13) else m.group(0), t)
    return ET.fromstring(t.lstrip('﻿')).findall('BODY/DATA/COLLECTION/VOUCHER')
F = ['DATE', 'VOUCHERTYPENAME', 'VOUCHERNUMBER', 'ALTERID', 'MASTERID', 'VCHGSTSTATUSISINCLUDED', 'VCHGSTSTATUSISUNCERTAIN', 'VCHGSTSTATUSISEXCLUDED', 'ISGSTOVERRIDDEN', 'ISCANCELLED', 'ISOPTIONAL', 'ISPOSTDATED', 'ISDELETED', 'ISDELETEDVCHRETAINED']
def rows(p):
    out = {}
    for v in load(p):
        k = (v.findtext('VOUCHERTYPENAME'), v.findtext('VOUCHERNUMBER'), v.findtext('GUID'))
        out[k] = {f: [(e.text or '').strip() for e in v.findall(f)] for f in F}
    return out
a = rows('answers/aw21b-a.xml')
for name in sys.argv[1:]:
    r = rows(f'answers/{name}.xml')
    new = sorted(set(r) - set(a), key=lambda k: k[1]); gone = sorted(set(a) - set(r), key=lambda k: k[1])
    changed = [k[1] for k in a if k in r and r[k] != a[k]]
    print(f'{name}: {len(r)} vouchers; new vs aw21b-a: {[k[1] for k in new]}; gone: {[k[1] for k in gone]}; others changed: {changed}')
    for k in new:
        print('   ', k[1], 'GUID', k[2], {f: r[k][f] for f in F})
