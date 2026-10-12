# W18b (b): compare the verification-window answer's vouchers with the 9 Oct pilot answers of the same request.
import re, sys, xml.etree.ElementTree as ET
def rows(p):
    t = open(p, 'rb').read().decode('utf-16')
    t = re.sub(r'&#(\d+);', lambda m: '?' if int(m.group(1)) < 32 and int(m.group(1)) not in (9, 10, 13) else m.group(0), t)
    c = ET.fromstring(t.lstrip('﻿')).find('BODY/DATA/COLLECTION')
    return {(v.findtext('VOUCHERTYPENAME'), v.findtext('VOUCHERNUMBER'), v.findtext('GUID')): {e.tag: (e.text or '').strip() for e in v if len(e) == 0} for v in (c.findall('VOUCHER') if c is not None else [])}
b = rows('answers/aw18b-verification-window.xml')
print('aw18b-verification-window:', len(b), 'vouchers', sorted(k[1] for k in b))
for ref in sys.argv[1:]:
    a = rows(ref)
    common = set(a) & set(b)
    diffs = {k[1]: sorted(f for f in set(a[k]) | set(b[k]) if a[k].get(f) != b[k].get(f)) for k in common}
    print(f'vs {ref.split("/")[-1]}: {len(a)} vouchers; only in 9 Oct: {sorted(k[1] for k in set(a)-set(b))}; only in W18b: {sorted(k[1] for k in set(b)-set(a))}; vouchers with field differences: { {k: v for k, v in diffs.items() if v} }')
