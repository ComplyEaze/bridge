# Summarise a proxy log: one line per request structure (by request sha), with ID, dates and counts.
# Usage: python -I classify.py <log_dir>
import collections
import os
import re
import sys

d = sys.argv[1]
rows = [l.rstrip('\n').split('\t') for l in open(os.path.join(d, 'index.tsv'), encoding='utf-8')]
groups = collections.OrderedDict()
for r in rows:
    seq, t0, t1, meth, path, qlen, qsha, st, plen, psha = r
    key = (meth, qsha)
    if key not in groups:
        raw = open(os.path.join(d, '%04d-req.bin' % int(seq)), 'rb').read()
        txt = ''
        for enc in ('utf-16', 'utf-8'):
            try:
                txt = raw.decode(enc)
                if '<' in txt or not raw:
                    break
            except Exception:
                pass
        ident = re.findall(r'<ID>([^<]*)</ID>', txt)
        typ = re.findall(r'<TYPE>([^<]*)</TYPE>', txt)
        dates = re.findall(r'<(SVFROMDATE|SVTODATE|SVCURRENTDATE)[^>]*>([^<]*)<', txt)
        coll = re.findall(r'<COLLECTION NAME="([^"]*)"', txt)
        groups[key] = dict(first=int(seq), seqs=[], qlen=qlen, typ=typ, ident=ident, dates=dates, coll=coll, statuses=set(), plens=set())
    g = groups[key]
    g['seqs'].append(int(seq)); g['statuses'].add(st); g['plens'].add(plen)
print('requests', len(rows))
for (meth, qsha), g in groups.items():
    print(f"{meth} x{len(g['seqs'])} first#{g['first']} req {g['qlen']}B sha {qsha[:12]} type={g['typ']} id={g['ident']} coll={g['coll'][:4]} dates={g['dates']} status={sorted(g['statuses'])} resp={sorted(g['plens'])}")
