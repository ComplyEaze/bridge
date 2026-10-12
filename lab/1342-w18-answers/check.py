# check W18 answers: decoded UTF-16/8, complete ENVELOPE, COLLECTION (or BODY for company object), no LINEERROR, size
import sys, re, os
for p in sys.argv[1:]:
    b = open(p, 'rb').read()
    t = b.decode('utf-16') if b[1:2] == b'\x00' or b[:2] == b'\xff\xfe' else b.decode('utf-8', 'replace')
    env = t.lstrip('\ufeff').strip().startswith('<ENVELOPE') and t.strip().endswith('</ENVELOPE>')
    coll = '<COLLECTION' in t; body = '<BODY' in t
    le = 'LINEERROR' in t
    print(f"{os.path.basename(p)}: {len(b)} B envelope={env} collection={coll} body={body} lineerror={le} over500KB={len(b) > 512000}")
