# Lab helper (#2057 item purchase class GST, not product code): the step 12-16 record of one raw a07 answer, read by
# structure (LAB-COMMON C4: UTF-16LE, forbidden characters dropped for parsing only, parsed as XML, never a text search).
# ROWS are the VOUCHER children of ENVELOPE/BODY/DATA/COLLECTION. Per VOUCHER: DATE, VOUCHERTYPENAME, VOUCHERNUMBER,
# ISINVOICE, PARTYGSTIN, top-level ALLINVENTORYENTRIES.LIST presence; per ALLLEDGERENTRIES.LIST: LEDGERNAME, AMOUNT,
# ISDEEMEDPOSITIVE, the names of its child lists, every element whose name begins GST, INVENTORYALLOCATIONS.LIST (item,
# quantity, rate, amount and its ACCOUNTINGALLOCATIONS.LIST) and RATEDETAILS.LIST (head, valuation type, rate).
# Usage: python -I rec2057.py <answer file>
import re
import sys
import xml.etree.ElementTree as ET

raw = open(sys.argv[1], 'rb').read()
text = raw.decode('utf-16-le')
if text.startswith('﻿'):
    text = text[1:]
text = re.sub(r'&#(0*([0-8]|1[124]|1[4-9]|2[0-9]|3[01]));', '', text)
text = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', text)
root = ET.fromstring(text)
coll = root.find('./BODY/DATA/COLLECTION')
rows = [] if coll is None else [e for e in coll if e.tag == 'VOUCHER']
print(f'{sys.argv[1].replace(chr(92), "/").split("/")[-1]}: {len(raw)} B; VOUCHER rows {len(rows)}')


def val(e, tag):
    c = e.find(tag)
    if c is None:
        return '<absent>'
    t = (c.text or '').strip()
    return t if t else '<empty>'


for v in rows:
    print('VOUCHER ' + ' '.join(f'{h}={val(v, h)!r}' for h in ('DATE', 'VOUCHERTYPENAME', 'VOUCHERNUMBER', 'ISINVOICE', 'PARTYGSTIN', 'PARTYLEDGERNAME', 'ALTERID')))
    print(f'  top-level ALLINVENTORYENTRIES.LIST: x{len(v.findall("ALLINVENTORYENTRIES.LIST"))}')
    for le in v.findall('ALLLEDGERENTRIES.LIST'):
        kids = sorted({c.tag for c in le if c.tag.endswith('.LIST')})
        gst = [(c.tag, (c.text or '').strip()) for c in le if c.tag.startswith('GST') and not c.tag.endswith('.LIST')]
        print(f'  ALLLEDGERENTRIES.LIST LEDGERNAME={val(le, "LEDGERNAME")!r} AMOUNT={val(le, "AMOUNT")!r} ISDEEMEDPOSITIVE={val(le, "ISDEEMEDPOSITIVE")!r}')
        print(f'    child lists: {kids}')
        print(f'    GST elements: ' + ' '.join(f'{t}={x!r}' for t, x in gst))
        for ia in le.findall('INVENTORYALLOCATIONS.LIST'):
            print('    INVENTORYALLOCATIONS.LIST ' + ' '.join(f'{k}={val(ia, k)!r}' for k in ('STOCKITEMNAME', 'ACTUALQTY', 'BILLEDQTY', 'RATE', 'AMOUNT')))
            for aa in ia.findall('ACCOUNTINGALLOCATIONS.LIST'):
                print('      ACCOUNTINGALLOCATIONS.LIST ' + ' '.join(f'{k}={val(aa, k)!r}' for k in ('LEDGERNAME', 'AMOUNT', 'ISDEEMEDPOSITIVE')))
        for rd in le.iter('RATEDETAILS.LIST'):
            print('    RATEDETAILS.LIST ' + ' '.join(f'{k}={val(rd, k)!r}' for k in ('GSTRATEDUTYHEAD', 'GSTRATEVALUATIONTYPE', 'GSTRATE')))
