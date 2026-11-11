# Lab helper (row 2346-N134, new; not product code; written by mk-helpers.py): runs timed.py once with a pipe as its stdin, copies its
# output to feed-out.txt with a UTC stamp per line, and gives the one Enter only when enter.go appears in this folder, or closes the pipe
# with no Enter when abort.go appears. It sends nothing to Tally itself. Usage: python -I -X utf8 feed.py URL LABEL IMPORT READ
import os, subprocess, sys, threading, time
H = os.path.dirname(os.path.abspath(__file__))
def now(): t = time.time(); return time.strftime('%Y-%m-%dT%H:%M:%S', time.gmtime(t)) + f'.{int(t * 1000) % 1000:03d}Z'
for f in ('enter.go', 'abort.go'):
    if os.path.exists(os.path.join(H, f)): raise SystemExit(f + ' exists before the run: nothing started')
out = open(os.path.join(H, 'feed-out.txt'), 'a', encoding='utf-8', newline='\n')
def log(s): out.write(f'{now()} {s}\n'); out.flush()
p = subprocess.Popen([sys.executable, '-I', '-u', os.path.join(H, 'timed.py')] + sys.argv[1:5], cwd=H, stdin=subprocess.PIPE,
                     stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, encoding='utf-8', errors='replace', bufsize=0)
log('started timed.py pid ' + str(p.pid))
def pump():
    buf = ''
    while True:
        c = p.stdout.read(1)
        if not c: break
        buf += c
        if c == '\n' or buf.endswith(': '): log('timed.py: ' + buf.rstrip('\n')); buf = ''
    if buf: log('timed.py: ' + buf)
threading.Thread(target=pump, daemon=True).start()
given = False
while p.poll() is None:
    if not given and os.path.exists(os.path.join(H, 'enter.go')):
        p.stdin.write('\n'); p.stdin.flush(); given = True; log('Enter given (enter.go seen)')
    elif not given and os.path.exists(os.path.join(H, 'abort.go')):
        p.stdin.close(); given = True; log('stdin closed with no Enter (abort.go seen)')
    time.sleep(0.02)
time.sleep(0.3); log(f'timed.py exit {p.returncode}')
