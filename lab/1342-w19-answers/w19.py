# W19 analysis: parse GSTR-1 status reads (VOUCHER rows under BODY/DATA/COLLECTION), compare, print.
# Usage: python -I w19.py rows <answer>            -> one line per voucher
#        python -I w19.py diff <answerA> <answerB> -> vouchers added/removed/changed (field level)
#        python -I w19.py raw <answer> <number>     -> the voucher element's exact text (for byte comparison)
import re, sys, xml.etree.ElementTree as ET
F = ['DATE', 'VOUCHERTYPENAME', 'VOUCHERNUMBER', 'ALTERID', 'MASTERID', 'ISGSTOVERRIDDEN', 'VCHGSTSTATUSISINCLUDED',
     'VCHGSTSTATUSISUNCERTAIN', 'VCHGSTSTATUSISEXCLUDED', 'ISDELETED', 'PARTYLEDGERNAME', 'ISCANCELLED', 'ISOPTIONAL', 'AMOUNT']

def txt(p):
    b = open(p, 'rb').read()
    return b.decode('utf-16') if b[1:2] == b'\x00' or b[:2] == b'\xff\xfe' else b.decode('utf-8', 'replace')

def clean(t):
    t = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '?', t)
    return re.sub(r'&#(\d+);', lambda m: '?' if int(m.group(1)) < 32 and int(m.group(1)) not in (9, 10, 13) else m.group(0), t)

def rows(p):
    r = ET.fromstring(clean(txt(p)).lstrip('﻿'))
    c = r.find('BODY/DATA/COLLECTION')
    out = []
    for v in ([] if c is None else c.findall('VOUCHER')):
        d = {}
        for f in F:
            e = v.find(f)
            d[f] = 'ABSENT' if e is None else (e.text or '').strip()
        d['_GUID'] = (v.findtext('GUID') or '').strip()
        out.append(d)
    return out

def key(d): return (d['VOUCHERTYPENAME'], d['VOUCHERNUMBER'], d['_GUID'])

def raw(p, number):
    t = txt(p)
    for m in re.finditer(r'<VOUCHER [^>]*>.*?</VOUCHER>', t, re.S):
        if f'<VOUCHERNUMBER>{number}</VOUCHERNUMBER>' in m.group(0):
            yield m.group(0)

if __name__ == '__main__':
    mode = sys.argv[1]
    if mode == 'rows':
        rs = rows(sys.argv[2]); print(len(rs), 'VOUCHER rows')
        for d in rs:
            print(' | '.join(f'{k}={d[k]}' for k in F if d[k] != 'ABSENT' or k in ('PARTYLEDGERNAME', 'ISCANCELLED', 'ISOPTIONAL', 'AMOUNT')))
    elif mode == 'diff':
        a = {key(d): d for d in rows(sys.argv[2])}; b = {key(d): d for d in rows(sys.argv[3])}
        print('only in A:', sorted(k[1] for k in a.keys() - b.keys()))
        print('only in B:', sorted(k[1] for k in b.keys() - a.keys()))
        for k in sorted(a.keys() & b.keys(), key=lambda x: x[1]):
            ch = [f'{f}: {a[k][f]} -> {b[k][f]}' for f in F if a[k][f] != b[k][f]]
            if ch: print('changed', k[1], '; '.join(ch))
    elif mode == 'raw':
        for x in raw(sys.argv[2], sys.argv[3]): print(x)
