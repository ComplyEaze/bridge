# Lab runner (#1688, not product code): THE READS of one set (MARKS, CAT, CV2, IDX, in that order, each once) through
# rd.py, MARKS checked against the expected ALTMSTID, then each list's names per LEDGER row printed for the step table.
# Usage: python -I -X utf8 reads4.py <prefix> <expected ALTMSTID>
import subprocess, sys
W = '<LAB_DIR>'; E = W + '/evidence'
pre, mst = sys.argv[1], sys.argv[2]
for lab, req, exp in ((f'{pre}-marks', 'requests/marks.utf16le.xml', mst), (f'{pre}-cat', 'requests/ledger-catalogue.utf16le.xml', None),
                      (f'{pre}-cv2', 'requests/ledger-catalogue-v2.utf16le.xml', None), (f'{pre}-idx', 'requests/ledger-name-index.utf16le.xml', None)):
    r = subprocess.run(['python', '-I', '-X', 'utf8', W + '/rd.py', lab, req] + ([exp] if exp else []), capture_output=True, text=True)
    print(r.stdout.strip()[:600], flush=True)
    if r.returncode != 0: print(f'runner: stopped at {lab}'); sys.exit(2)
for lst in ('cat', 'cv2', 'idx'):
    print(f'--- {pre}-{lst}')
    for l in open(f'{E}/{pre}-{lst}.table.txt', encoding='utf-8'):
        if l.startswith('LEDGER') or l.startswith('    name'): print(l.rstrip()[:230])
