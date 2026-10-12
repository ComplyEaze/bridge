# Build the public W19 push folder: answers (synthetic GSTIN replaced by same-length Z's, XML stays well-formed),
# send log, scripts, analysis output, and hash lists for screenshots and backups (which stay on the lab PC).
# Usage: python -I build_push.py <dst>
import hashlib, os, re, shutil, sys
SRC = 'C:/bridge-lab/w19'
GSTIN = re.compile(r'\b\d{2}[A-Z]{5}\d{4}[A-Z][1-9A-Z]Z[0-9A-Z]')
MARK = 'Z' * 15
dst = sys.argv[1]
if os.path.exists(dst):
    sys.exit('destination exists: ' + dst)
os.makedirs(os.path.join(dst, 'answers'))
def sha(p): return hashlib.sha256(open(p, 'rb').read()).hexdigest()
def masked(src, out, enc):
    b = open(src, 'rb').read()
    t, k = GSTIN.subn(MARK, b.decode(enc))
    nb = t.encode(enc)
    assert len(nb) == len(b)
    open(out, 'wb').write(nb)
    return len(b), k
with open(os.path.join(dst, 'SHA256.txt'), 'w', encoding='utf-8', newline='\n') as f:
    f.write('# lab-answer-sha256  published-copy-sha256  file  bytes  gstin-masks (same-length ZZZZZZZZZZZZZZZ)\n')
    for n in sorted(os.listdir(SRC + '/answers')):
        if n.startswith('aw19-') and n.endswith('.xml'):
            p, q = SRC + '/answers/' + n, os.path.join(dst, 'answers', n)
            sz, k = masked(p, q, 'utf-16-le')
            f.write('%s  %s  answers/%s  %d  %d\n' % (sha(p), sha(q), n, sz, k))
t = open(SRC + '/answers/log.txt', encoding='utf-8').read()
open(os.path.join(dst, 'send-log.txt'), 'w', encoding='utf-8', newline='\n').write(GSTIN.sub(MARK, t))
t = open(SRC + '/analyse-output.txt', encoding='utf-8').read()
open(os.path.join(dst, 'analyse-output.txt'), 'w', encoding='utf-8', newline='\n').write(GSTIN.sub(MARK, t))
for n in ['send.sh', 'check.py', 'w19.py', 'tools/build_push.py']:
    shutil.copyfile(SRC + '/' + n, os.path.join(dst, os.path.basename(n)))
with open(os.path.join(dst, 'SHA256-kept-on-lab-pc.txt'), 'w', encoding='utf-8', newline='\n') as f:
    f.write('# screenshots and backups stay on the lab PC: sha256  file  bytes\n')
    for n in sorted(os.listdir(SRC + '/shots')):
        p = SRC + '/shots/' + n
        f.write('%s  shots/%s  %d\n' % (sha(p), n, os.path.getsize(p)))
    for b in ['W19-C', 'W19-D', 'W19-E']:
        for n in sorted(os.listdir(SRC + '/backups/' + b)):
            p = SRC + '/backups/' + b + '/' + n
            f.write('%s  backups/%s/%s  %d\n' % (sha(p), b, n, os.path.getsize(p)))
print('ok')
