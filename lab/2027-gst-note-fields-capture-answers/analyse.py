# Lab helper (#2027 note fields, not product code): reads the raw answers by structure (LAB-COMMON C4: UTF-16LE,
# forbidden characters dropped, parsed as XML, never a text search on the bytes) and prints, per VOUCHER row of
# ENVELOPE/BODY/DATA/COLLECTION: its header, the top-level GST and note elements the sheet names, every element anywhere
# under it whose trimmed text equals a value typed in step 2 (with its full path), and, for the fifteen lists the
# candidate request names, how many children each holds. For the a07 answers it also prints the rate, taxability and
# type-of-supply elements on each ledger entry. Read only. Usage: python -I analyse.py <answer> [<answer> ...]
import re
import sys
import xml.etree.ElementTree as ET

TYPED = ['SYN-ORIG-1', 'SYN-ORIG-2', 'SYN-NOTE-1', 'SYN-NOTE-2', 'SYN-SUP-1', 'SYN-BILL-S2', 'SYN-BILL-C2',
         'SYN-BILL-D2', 'SYN-BILL-P2', '20250512', '20250513', '20250514', '20250515', '20250516', '12-May-2025',
         '13-May-2025', '14-May-2025', '15-May-2025', '16-May-2025', '01-Sales Return', '04-Correction in Invoice',
         'Sales Return', 'Correction in Invoice']
CODES = ['01', '04']
TOP = ['DATE', 'VOUCHERTYPENAME', 'VOUCHERNUMBER', 'PARTYLEDGERNAME', 'MASTERID', 'ALTERID', 'PLACEOFSUPPLY',
       'STATENAME', 'GSTREGISTRATIONTYPE', 'PARTYGSTIN', 'COUNTRYOFRESIDENCE', 'CMPGSTSTATE', 'CONSIGNEESTATENAME',
       'ORIGINALVCHNUMBER', 'ORIGINALVCHDATE', 'REFERENCE', 'REFERENCEDATE', 'VCHGSTCLASS', 'GSTNATUREOFRETURN',
       'ISGSTOVERRIDDEN', 'NARRATION']
LISTS = ['ORIGINVOICEDETAILS.LIST', 'INVOICEDELNOTES.LIST', 'INVOICEORDERLIST.LIST', 'INVOICEINDENTLIST.LIST',
         'INVOICEEXPORTLIST.LIST', 'EWAYBILLDETAILS.LIST', 'EWAYBILLERRORLIST.LIST', 'IRNERRORLIST.LIST',
         'GSTADVADJDETAILS.LIST', 'ADVADJUSTMENTREFERENCEDETAILS.LIST', 'DUTYHEADDETAILS.LIST',
         'SUPPLEMENTARYDUTYHEADDETAILS.LIST', 'EXCLUDEDTAXATIONS.LIST', 'CONTRITRANS.LIST', 'ALLINVENTORYENTRIES.LIST']


def load(path):
    text = open(path, 'rb').read().decode('utf-16-le')
    if text.startswith('﻿'):
        text = text[1:]
    text = re.sub(r'&#(0*([0-8]|1[124]|1[4-9]|2[0-9]|3[01]));', '', text)
    text = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', text)
    root = ET.fromstring(text)
    coll = root.find('./BODY/DATA/COLLECTION')
    return [] if coll is None else [e for e in coll if e.tag == 'VOUCHER']


def walk(e, path):
    for c in e:
        p = f'{path}/{c.tag}'
        yield p, c
        yield from walk(c, p)


def show(v):
    t = v.find('DATE')
    return {k: ((v.find(k).text or '').strip() if v.find(k) is not None else None) for k in TOP}


for path in sys.argv[1:]:
    rows = load(path)
    print(f'== {path}: VOUCHER rows {len(rows)}')
    for v in rows:
        head = show(v)
        print('VOUCHER ' + ' '.join(f'{k}={head[k]!r}' for k in TOP if head[k] is not None))
        missing = [k for k in TOP if head[k] is None]
        print('  top-level elements absent: ' + ', '.join(missing))
        for p, e in walk(v, ''):
            txt = (e.text or '').strip()
            if txt in TYPED:
                print(f'  typed value {txt!r} at {p}')
            elif txt in CODES and ('REASON' in e.tag or 'NATURE' in e.tag):
                print(f'  code {txt!r} at {p}')
        for name in LISTS:
            found = v.findall(name)
            if found:
                print(f'  {name}: x{len(found)}, children {[len(list(f)) for f in found]}')
            else:
                print(f'  {name}: absent')
        if '/a07-' in path.replace('\\', '/'):
            for i, le in enumerate(v.findall('ALLLEDGERENTRIES.LIST')):
                name = (le.findtext('LEDGERNAME') or '').strip()
                bits = {}
                for tag in ('GSTCLASS', 'GSTOVRDNNATURE', 'GSTOVRDNTAXABILITY', 'GSTOVRDNTYPEOFSUPPLY', 'GSTHSNNAME',
                            'GSTOVRDNISREVCHARGEAPPL', 'AMOUNT', 'ISDEEMEDPOSITIVE'):
                    x = le.find(tag)
                    if x is not None:
                        bits[tag] = (x.text or '').strip() or '<empty>'
                rd = le.findall('RATEDETAILS.LIST')
                rds = [{c.tag: (c.text or '').strip() or '<empty>' for c in r} for r in rd]
                print(f'  ledger {i} {name!r} {bits} RATEDETAILS x{len(rd)} {rds}')
