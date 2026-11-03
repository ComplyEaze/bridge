# Lab helper (852-C1, not product code): waits for box.ps1's log of the approval box, reads the box's text, and writes
# Yes to <tag>.answer only if the text names this company, this batch and 50 vouchers; otherwise writes nothing.
# Usage: python -I -X utf8 boxcheck.py <tag> <batch_id>
import os, sys, time
from datetime import datetime, timezone
tag, batch = sys.argv[1], sys.argv[2]; B = '<lab folder>/box/'
log = B + tag + '-box.txt'; t0 = time.time()
while time.time() - t0 < 80:
    try: t = open(log, encoding='utf-8-sig').read()
    except OSError: t = ''
    if 'WAITING' in t: break
    time.sleep(0.2)
else: sys.exit('no box seen in 80 s')
ok = 'BRIDGE LARGE LAB' in t and batch in t and 'post 50 vouchers' in t and 'Company GUID: 2ba6df28-d1e3-4e81-b6d2-eb50f0a693ab' in t
now = datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
if ok: open(B + tag + '.answer', 'w').write('Yes'); print(now, 'box text checked: company, GUID, batch and count match; wrote Yes')
else: print(now, 'box text did NOT match; no answer written')
