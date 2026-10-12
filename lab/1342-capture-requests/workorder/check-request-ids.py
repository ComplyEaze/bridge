#!/usr/bin/env python3 -I
"""Census of the request files: a Collection export must define the collection its header ID names.

Tally raises a modal 'Error in TDL ... Could not find description!' (and blocks the gateway) when the
<ID> of a Collection request is not the NAME of a COLLECTION the request defines (or a built-in
collection it is a request for). Run: python3 -I check-request-ids.py DIR [DIR ...]
Exit 0 when every file passes; 1 when any file does not; 2 when a file could not be read (a read
that failed is never reported as a pass).
"""
import os, re, sys

def text_of(path):
    data = open(path, 'rb').read()
    if data[:2] == b'\xff\xfe':
        return data[2:].decode('utf-16-le')
    if data[:2] == b'\xfe\xff':
        return data[2:].decode('utf-16-be')
    if data[:3] == b'\xef\xbb\xbf':
        return data[3:].decode('utf-8')
    return data.decode('utf-8')

def check(path):
    t = text_of(path)
    if '<ENVELOPE>' not in t:
        return 'skip', 'not a request envelope'
    kind = re.search(r'<TYPE>([^<]*)</TYPE>', t)
    ident = re.search(r'<ID>([^<]*)</ID>', t)
    if not kind or not ident:
        return 'skip', 'no TYPE or ID in the header'
    if kind.group(1) != 'Collection':
        return 'skip', 'TYPE is %s, not Collection' % kind.group(1)
    names = re.findall(r'<COLLECTION\b[^>]*\bNAME="([^"]*)"', t)
    if not names:
        return 'skip', 'no defined collection (built-in %r)' % ident.group(1)
    if ident.group(1) in names:
        return 'ok', ident.group(1)
    return 'FAIL', 'ID %r names none of the defined collections %r' % (ident.group(1), names)

def main(dirs):
    counts = {'ok': 0, 'skip': 0, 'FAIL': 0}
    bad = []
    for d in dirs:
        for root, _, files in os.walk(d):
            for f in sorted(files):
                if not f.endswith('.xml'):
                    continue
                p = os.path.join(root, f)
                try:
                    verdict, why = check(p)
                except Exception as e:  # a file that cannot be read is a failure of the census
                    print('UNREADABLE %s: %s' % (p, e))
                    return 2
                counts[verdict] += 1
                if verdict == 'FAIL':
                    bad.append((p, why))
    for p, why in bad:
        print('FAIL %s: %s' % (p, why))
    print('census: ok %d, skipped %d, FAIL %d' % (counts['ok'], counts['skip'], counts['FAIL']))
    return 1 if bad else 0

if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]) if len(sys.argv) > 1 else 2)
