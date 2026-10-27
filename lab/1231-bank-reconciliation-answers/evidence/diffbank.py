# Lab helper (#1231, read only): element-by-element differences between two V-CONTRA answers, per voucher and per
# ledger entry, and whether each voucher's whole element is byte-identical. Usage: diffbank.py <a> <b>
import re, sys, hashlib
import xml.etree.ElementTree as ET
def load(p):
    raw = open(p, 'rb').read().decode('utf-16')
    vs = re.findall(r'<VOUCHER [^>]*>.*?</VOUCHER>', raw, re.S)
    t = re.sub(r'&#(\d+);', lambda m: '' if int(m.group(1)) < 32 and int(m.group(1)) not in (9, 10, 13) else m.group(0), raw)
    root = ET.fromstring(t)
    return {v.findtext('VOUCHERNUMBER'): v for v in root.iter('VOUCHER') if len(v)}, {re.search(r'<VOUCHERNUMBER>([^<]*)<', x).group(1): hashlib.sha256(x.encode()).hexdigest()[:12] for x in vs}
def flat(e, path=''):
    out = {}
    counts = {}
    for c in e:
        counts[c.tag] = counts.get(c.tag, 0) + 1
        p = '%s/%s[%d]' % (path, c.tag, counts[c.tag])
        if len(c):
            out.update(flat(c, p))
        else:
            out[p] = (c.text or '').strip()
        for k, v in c.attrib.items():
            out[p + '@' + k] = v
    return out
a, ah = load(sys.argv[1]); b, bh = load(sys.argv[2])
for n in sorted(set(a) | set(b), key=lambda x: int(x)):
    print('VOUCHER', n, 'bytes identical' if ah.get(n) == bh.get(n) else 'bytes DIFFER', ah.get(n), bh.get(n))
    if n in a and n in b:
        fa, fb = flat(a[n]), flat(b[n])
        for k in sorted(set(fa) | set(fb)):
            if fa.get(k) != fb.get(k):
                print('   ', k, repr(fa.get(k)), '->', repr(fb.get(k)))
