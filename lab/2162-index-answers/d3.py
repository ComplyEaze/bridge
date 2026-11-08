# Lab helper (#2162 index rows, not product code): D3 of the sheet, L5. Twelve times, one at a time: B-Y0-month.xml by the
# kit's timed.py (labels l501 to l512), then M-dense.xml (labels dm2 to dm13). It stops at the first send timed.py refuses
# or fails, and at the first marks answer that is not byte-equal to dm1's (a stop of the sheet: the marks moved or the
# read failed). Sends nothing else and presses no key. Prints one line per send. Usage: python -I -X utf8 d3.py
import os, subprocess, sys
RUN = '<LAB_DIR>/run'
os.chdir(RUN)
dm1 = open('a-dm1-M-dense.xml', 'rb').read()
def send(label, req):
    r = subprocess.run([sys.executable, '-I', '-X', 'utf8', 'timed.py', 'http://localhost:9001', label, req], capture_output=True, text=True, encoding='utf-8')
    line = open(f'log-{label}.txt', encoding='utf-8').read().strip() if os.path.exists(f'log-{label}.txt') else '(no log line)'
    print(label, 'exit', r.returncode, line, flush=True)
    if r.returncode != 0 or os.path.exists(f'a-{label}-{req}.pending'):
        print(label, 'FAILED:', (r.stdout + r.stderr).strip()[-400:]); sys.exit(2)
for i in range(1, 13):
    send(f'l5{i:02d}', 'B-Y0-month.xml')
    lab = f'dm{i + 1}'
    send(lab, 'M-dense.xml')
    if open(f'a-{lab}-M-dense.xml', 'rb').read() != dm1:
        print(lab, 'differs from dm1: stop'); sys.exit(3)
    print(lab, 'byte-equal to dm1', flush=True)
print('D3 done: 12 month reads, 12 marks reads byte-equal to dm1')
