# Lab helper (1038-R1, not product code): waits for box.ps1's log of a ComplyEaze box, reads the box's text, and writes
# Yes to <tag>.answer only if every required phrase is in it; otherwise writes nothing (box.ps1 then clicks No at its
# answer-by limit). Usage: python -I -X utf8 boxcheck.py <tag> <phrase> [<phrase> ...]
import sys, time
from datetime import datetime, timezone
tag, need = sys.argv[1], sys.argv[2:]; B = '<lab folder>/box/'
log = B + tag + '-box.txt'; t0 = time.time()
while time.time() - t0 < 110:
    try: t = open(log, encoding='utf-8-sig').read()
    except OSError: t = ''
    if 'WAITING' in t: break
    time.sleep(0.2)
else: sys.exit('no box seen in 110 s')
miss = [n for n in need if n not in t]
now = datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
if not miss: open(B + tag + '.answer', 'w').write('Yes'); print(now, 'every required phrase is in the box text; wrote Yes:', need)
else: print(now, 'MISSING from the box text; no answer written:', miss)
