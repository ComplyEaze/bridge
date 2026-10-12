# Read 1050-B answers by structure (ElementTree), never by text search.
# Usage: python -I analyse.py <answer> [<answer> ...]
# Prints: envelope check, the number of VOUCHER elements under COLLECTION, each voucher's attributes,
# and (with --fields) every child element name with its text (direct children; lists summarised by tag count).
import re
import sys
import xml.etree.ElementTree as ET

sys.stdout.reconfigure(encoding='utf-8')
args = [a for a in sys.argv[1:] if not a.startswith('--')]
fields = '--fields' in sys.argv


def load(p):
    raw = open(p, 'rb').read()
    enc = 'utf-16' if raw[:2] in (b'\xff\xfe', b'\xfe\xff') else 'utf-16-le'
    t = raw.decode(enc).lstrip('\ufeff').replace('&#4;', '')
    t = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t)
    return ET.fromstring(t)


for p in args:
    root = load(p)
    cols = root.findall('.//COLLECTION')
    lineerr = root.findall('.//LINEERROR')
    vs = [v for c in cols for v in c.findall('VOUCHER')]
    print('==', p.split('\\')[-1].split('/')[-1], '| root', root.tag, '| COLLECTION', len(cols), '| LINEERROR', len(lineerr), '| VOUCHER', len(vs))
    for i, v in enumerate(vs):
        print('  voucher', i, 'attrs', dict(v.attrib))
        if fields:
            kids = list(v)
            counts = {}
            for k in kids:
                counts[k.tag] = counts.get(k.tag, 0) + 1
            for k in kids:
                if len(list(k)) == 0:
                    print('    %s = %r' % (k.tag, (k.text or '')))
            print('    list children:', {t: n for t, n in counts.items() if any(len(list(k)) for k in kids if k.tag == t)})
