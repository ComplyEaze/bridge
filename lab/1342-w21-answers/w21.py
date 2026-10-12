# W21 expectations (b)-(d): per voucher, count each flag element and compare the fields of (b) across aw20-b, aw21-a, aw21-b, aw21-c.
import re, xml.etree.ElementTree as ET, collections
def load(p):
    t = open(p, 'rb').read().decode('utf-16')
    t = re.sub(r'&#(\d+);', lambda m: '?' if int(m.group(1)) < 32 and int(m.group(1)) not in (9, 10, 13) else m.group(0), t)
    return ET.fromstring(t.lstrip('\ufeff')).findall('BODY/DATA/COLLECTION/VOUCHER')
B = ['ALTERID', 'MASTERID', 'VCHGSTSTATUSISINCLUDED', 'VCHGSTSTATUSISUNCERTAIN', 'VCHGSTSTATUSISEXCLUDED', 'ISGSTOVERRIDDEN', 'ISCANCELLED', 'ISOPTIONAL']
FL = ['ISPOSTDATED', 'ISDELETED', 'ISDELETEDVCHRETAINED']
def rows(p):
    out = {}
    for v in load(p):
        k = (v.findtext('VOUCHERTYPENAME'), v.findtext('VOUCHERNUMBER'), v.findtext('GUID'))
        out[k] = {'b': {f: (v.findtext(f) or '').strip() for f in B}, 'counts': {f: [(e.text or '').strip() for e in v.findall(f)] for f in FL}}
    return out
F = {'aw20-b': 'C:/bridge-lab/w20/answers/aw20-b.xml', 'aw21-a': 'answers/aw21-a.xml', 'aw21-b': 'answers/aw21-b.xml', 'aw21-c': 'answers/aw21-c.xml'}
R = {n: rows(p) for n, p in F.items()}
ref = R['aw20-b']
for n in ['aw21-a', 'aw21-b', 'aw21-c']:
    r = R[n]
    print(f'{n}: {len(r)} vouchers; same keys as aw20-b: {set(r) == set(ref)}; vouchers with a (b)-field difference: {sum(1 for k in ref if k in r and r[k]["b"] != ref[k]["b"])}')
    dist = collections.Counter((f, tuple(r[k]['counts'][f])) for k in r for f in FL)
    print('   flag occurrences per voucher (flag, values) -> vouchers:', dict(sorted(dist.items())))
print('aw20-b flag occurrences:', dict(sorted(collections.Counter((f, tuple(ref[k]['counts'][f])) for k in ref for f in FL).items())))
