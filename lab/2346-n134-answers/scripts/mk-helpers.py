# Lab helper (row 2346-N134, new; not product code): writes this row's step0.py, shot.sh and prep-c6.sh into the run folder from row
# 2346-MN's by exact replacements of the folder names, and feed.py (new): it starts timed.py with a pipe as its stdin and gives it the one
# Enter only when the file enter.go appears in the run folder (written after the window's screenshot shows no dialog), or closes the pipe
# with no Enter when abort.go appears (timed.py then records "interrupted (Ctrl+C, or no Enter to read)" and sends no +60 s read).
# Usage: python -I -X utf8 mk-helpers.py
import os
M = '<ROW_2346MN_DIR>\\'; R = '<LAB_DIR>\\'
def mk(name, reps):
    dst = R + name; assert not os.path.exists(dst), dst
    s = open(M + name, encoding='utf-8').read()
    for a, b in reps:
        assert s.count(a) >= 1, (name, a)
        s = s.replace(a, b)
    assert '2346-mn' not in s.replace('row 2346-MN', ''), name
    open(dst, 'w', encoding='utf-8', newline='\n').write(s); print('written', name)
mk('step0.py', [('# Lab helper (rows 1381-R1 and later, not product code)', '# Lab helper (rows 1381-R1 and later, row 2346-N134 copy by mk-helpers.py; not product code)')])
mk('shot.sh', [('# Lab helper (row 2346-MN, from row 2346-AR\'s)', '# Lab helper (row 2346-N134, row 2346-MN\'s by mk-helpers.py)'),
               ('<ROW_2346MN_DIR>/shots/', '<LAB_DIR>/shots/')])
mk('prep-c6.sh', [('# Lab helper (row 2346-MN, from row 2346-AR\'s)', '# Lab helper (row 2346-N134, row 2346-MN\'s by mk-helpers.py)'),
                  ('(13: the restore of step 13)', '(6: the restore of step 6)'),
                  ('cd <ROW_2346MN_DIR>', 'cd <LAB_DIR>'),
                  ('D=C:/lab/2346-mn/backup-1', 'D=C:/lab/2346-n134/backup-1'),
                  ('C:\\\\lab\\\\2346-mn\\\\backup-1', 'C:\\\\lab\\\\2346-n134\\\\backup-1')])
FEED = r'''# Lab helper (row 2346-N134, new; not product code; written by mk-helpers.py): runs timed.py once with a pipe as its stdin, copies its
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
'''
assert not os.path.exists(R + 'feed.py')
open(R + 'feed.py', 'w', encoding='utf-8', newline='\n').write(FEED); print('written feed.py')
