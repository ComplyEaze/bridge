#!/usr/bin/env python3 -I
"""Check a W15 answer positively: the objects altered in (BEFORE, AFTER] are identified only if they are all there.

usage: python3 -I check-w15-answer.py ANSWER BEFORE AFTER
Exit 0 IDENTIFIED: exactly AFTER-BEFORE vouchers, with the ALTERIDs BEFORE+1 .. AFTER each once; prints them.
Exit 1 NOT IDENTIFIED: a complete envelope with a COLLECTION and no LINEERROR, but not that set (prints what came back).
Exit 2 FAILED READ: not a complete envelope, no COLLECTION, a LINEERROR, or the file unreadable: nothing is claimed.
An empty collection is NOT 'no object was altered': the counter moved, so it means the objects are not in the Voucher
collection or not in the date window.
"""
import re, sys

def text_of(path):
    d = open(path, 'rb').read()
    if d[:2] in (b'\xff\xfe', b'\xfe\xff'):
        return d.decode('utf-16')
    if b'\x00' in d[:4]:
        return d.decode('utf-16-le')
    return d.decode('utf-8-sig')

def main(argv):
    if len(argv) != 4:
        print(__doc__); return 2
    try:
        t = text_of(argv[1]); b = int(argv[2]); a = int(argv[3])
    except Exception as e:
        print('FAILED READ:', e); return 2
    if '<ENVELOPE>' not in t or '</ENVELOPE>' not in t or '<COLLECTION' not in t or 'LINEERROR' in t:
        print('FAILED READ: not a complete envelope with a COLLECTION and no LINEERROR'); return 2
    # a collection row is `<VOUCHER REMOTEID=...>`; the header's `<VOUCHER>N</VOUCHER>` count has no attributes
    vouchers = re.findall(r'<VOUCHER [^>]*>.*?</VOUCHER>', t, re.S)
    rows = []
    for v in vouchers:
        def one(k):
            m = re.findall(r'<%s[^>]*>([^<]*)</%s>' % (k, k), v)
            return m[0].strip() if len(m) == 1 else None
        rows.append((one('ALTERID'), one('VOUCHERTYPENAME'), one('VOUCHERNUMBER'), one('DATE'), one('MASTERID'), one('NARRATION')))
    for r in rows:
        print('  ALTERID %s | type %s | no %s | date %s | master %s | narration %r' % r)
    ids = [r[0] for r in rows]
    want = [str(i) for i in range(b + 1, a + 1)]
    if sorted(ids, key=lambda x: (x is None, int(x) if x and x.isdigit() else 0)) == want:
        print('IDENTIFIED: %d vouchers, ALTERIDs %d..%d' % (len(rows), b + 1, a)); return 0
    print('NOT IDENTIFIED: found %d voucher(s) with ALTERIDs %s; the counter moved %d..%d (%d step(s))'
          % (len(rows), ids, b, a, a - b)); return 1

sys.exit(main(sys.argv))
