#!/usr/bin/env python3 -I
"""Census of the request files: a Collection export must define the collection its header ID names.

Tally raises a modal 'Error in TDL ... Could not find description!' (and blocks the gateway) when the
<ID> of a Collection request is not the NAME of a COLLECTION the request defines (or a built-in
collection it is a request for). Run: python3 -I check-request-ids.py DIR [DIR ...]
Exit 0 when every file passes; 1 when any file does not; 2 when a file could not be read (a read
that failed is never reported as a pass).

The header TYPE is compared without regard to case or surrounding space, so `COLLECTION` is checked
like `Collection`; a request that defines a collection but whose header has no readable TYPE or ID
is a FAIL, never a skip. Files under a `census-controls` folder are not part of a census: run
`check-request-ids.py --controls` to prove each verdict fires on them (exit 0 only when every control
gets its expected verdict).
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
    defined = re.findall(r'<COLLECTION\b[^>]*\bNAME="([^"]*)"', t, re.I)
    kind = re.search(r'<TYPE>([^<]*)</TYPE>', t, re.I)
    ident = re.search(r'<ID\b[^>]*>([^<]*)</ID>', t, re.I)
    if not kind:
        if defined:
            return 'FAIL', 'defines %r but the header has no readable TYPE' % defined
        return 'skip', 'no TYPE in the header'
    if kind.group(1).strip().lower() != 'collection':
        if defined:
            return 'FAIL', 'TYPE is %r but the request defines collections %r' % (kind.group(1), defined)
        return 'skip', 'TYPE is %s, not Collection' % kind.group(1)
    if not ident:
        return 'FAIL', 'a Collection request with no readable ID'
    if not defined:
        return 'skip', 'no defined collection (built-in %r)' % ident.group(1)
    if ident.group(1) in defined:
        return 'ok', ident.group(1)
    return 'FAIL', 'ID %r names none of the defined collections %r' % (ident.group(1), defined)

# Each control file's name says the verdict it must get: ok-*, fail-*, skip-*.
def controls():
    here = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'census-controls')
    files = sorted(f for f in os.listdir(here) if f.endswith('.xml'))
    if not files:
        print('no control files in %s' % here)
        return 2
    bad = 0
    for f in files:
        want = f.split('-', 1)[0].upper().replace('OK', 'ok').replace('SKIP', 'skip')
        got, why = check(os.path.join(here, f))
        if got != want:
            bad += 1
            print('CONTROL DID NOT FIRE %s: expected %s, got %s (%s)' % (f, want, got, why))
    print('controls: %d files, %d did not fire' % (len(files), bad))
    return 1 if bad else 0

def main(dirs):
    counts = {'ok': 0, 'skip': 0, 'FAIL': 0}
    bad = []
    for d in dirs:
        for root, subdirs, files in os.walk(d):
            subdirs[:] = [sub for sub in subdirs if sub != 'census-controls']
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
    if sys.argv[1:] == ['--controls']:
        sys.exit(controls())
    sys.exit(main(sys.argv[1:]) if len(sys.argv) > 1 else 2)
