# Lab runner (#1331 B1/B3, not product code): for each import in order, m-marks first (send.py: exactly THE COMPANY's row),
# then the import (send.py: sent once, counters CREATED 500 and every other 0, no LINEERROR) and a capture of the whole Tally
# window. The capture must be byte-equal to the Gateway reference (a0-gateway.png): anything else may be a dialog, so the
# runner stops and nothing further is sent until the capture is looked at. Any helper stop ends the run.
# Usage: python -I -X utf8 imports.py <first> <last>
import hashlib, subprocess, sys
W = '<LAB_DIR>'
REF = {hashlib.sha256(open(W + '/shots/' + n, 'rb').read()).hexdigest() for n in ('a0-gateway.png', 'B-i003-after.png')}  # the Gateway, focused and unfocused (B-i003-after looked at: the Gateway, no dialog)
a, b = int(sys.argv[1]), int(sys.argv[2])
for i in range(a, b + 1):
    for args in (['m-marks.xml', '8192', f'B-i{i:03d}-marks'], [f'imports/i{i:03d}-debtors.xml', '65536', f'B-i{i:03d}']):
        r = subprocess.run(['python', '-I', '-X', 'utf8', W + '/send.py', args[2], args[0], args[1]], capture_output=True, text=True)
        print(r.stdout.strip()[:700], flush=True)
        if r.returncode != 0: print(f'runner: stopped at {args[2]}'); sys.exit(2)
    c = subprocess.run(['python', '-I', W + '/cap.py', f'B-i{i:03d}-after'], capture_output=True, text=True).stdout.strip()
    d = hashlib.sha256(open(f'{W}/shots/B-i{i:03d}-after.png', 'rb').read()).hexdigest()
    with open(W + '/evidence/send-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(f'  {c}; equal to the Gateway reference: {d in REF}\n')
    print(f'  {c}; equal to the Gateway reference: {d in REF}', flush=True)
    if d not in REF: print(f'runner: the capture after i{i:03d} is not the Gateway reference; stopped, look at it'); sys.exit(3)
print('runner: done')
