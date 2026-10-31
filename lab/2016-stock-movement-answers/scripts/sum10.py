# Lab helper (#2016 stock movement, not product code): step 10's summary over the r-m answers given, read by structure
# (as rec10.py): per VOUCHER its type, number, flags, BRIDGEVCHRESERVEDNAME and the BRIDGEVCHIS* elements that are Yes;
# then, from the lines read, P's in and out quantity per day and in total, excluding the optional and the cancelled
# vouchers, from ALLINVENTORYENTRIES.LIST (ISDEEMEDPOSITIVE Yes = in), with godown and batch.
import re
import sys
import xml.etree.ElementTree as ET
from collections import defaultdict

ITEM = sys.argv[1]
tot = defaultdict(lambda: [0.0, 0.0])
for path in sys.argv[2:]:
    text = open(path, 'rb').read().decode('utf-16-le')
    text = re.sub(r'&#(0*([0-8]|1[124]|1[4-9]|2[0-9]|3[01]));', '', text)
    text = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', text)
    coll = ET.fromstring(text).find('./BODY/DATA/COLLECTION')
    for v in ([] if coll is None else [e for e in coll if e.tag == 'VOUCHER']):
        g = lambda t: (v.findtext(t) or '').strip()
        yes = [c.tag for c in v if c.tag.startswith('BRIDGEVCHIS') and (c.text or '').strip() == 'Yes']
        print(f'{g("DATE")} {g("VOUCHERTYPENAME")} {g("VOUCHERNUMBER")} ALTERID {g("ALTERID")} optional {g("ISOPTIONAL")} '
              f'cancelled {g("ISCANCELLED")} postdated {g("ISPOSTDATED")} reserved {g("BRIDGEVCHRESERVEDNAME")!r} Yes: {yes or "none"}')
        skip = g('ISOPTIONAL') == 'Yes' or g('ISCANCELLED') == 'Yes'
        for e in v.findall('ALLINVENTORYENTRIES.LIST'):
            if (e.findtext('STOCKITEMNAME') or '').strip() != ITEM:
                continue
            q = float((e.findtext('ACTUALQTY') or '0').split()[0])
            inward = (e.findtext('ISDEEMEDPOSITIVE') or '').strip() == 'Yes'
            for b in e.findall('BATCHALLOCATIONS.LIST'):
                print(f'    {ITEM}: {"in " if inward else "out"} {(b.findtext("ACTUALQTY") or "").strip()} '
                      f'godown {(b.findtext("GODOWNNAME") or "").strip()!r} batch {(b.findtext("BATCHNAME") or "").strip()!r}'
                      + ('  (not counted: optional or cancelled)' if skip else ''))
            if not skip:
                tot[g('DATE')][0 if inward else 1] += q
ins = sum(a for a, _ in tot.values()); outs = sum(b for _, b in tot.values())
for d in sorted(tot):
    print(f'{d}: in {tot[d][0]:g} out {tot[d][1]:g}')
print(f'total in {ins:g} out {outs:g}; net {ins - outs:g}')
