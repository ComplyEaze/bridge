# Build the public W13 push folder: answers and JSON exports (synthetic GSTIN replaced by a same-length marker),
# send log, report, notes (run ids and GSTIN masked), tools, and hash lists for screenshots and backup
# (which stay on the lab PC). Usage: python -I build_push.py <dst>
import hashlib, os, re, shutil, sys
SRC = 'C:/bridge-lab/w13'
GSTIN = re.compile(r'\b\d{2}[A-Z]{5}\d{4}[A-Z][1-9A-Z]Z[0-9A-Z]')
MARK = '<GSTIN-SYNTH15>'
dst = sys.argv[1]
if os.path.exists(dst):
    sys.exit('destination exists: ' + dst)
for d in ('answers', 'export'):
    os.makedirs(os.path.join(dst, d))
def sha(p): return hashlib.sha256(open(p, 'rb').read()).hexdigest()
def masked(src, out, enc):
    b = open(src, 'rb').read()
    t, k = GSTIN.subn(MARK, b.decode(enc))
    nb = t.encode(enc)
    assert len(nb) == len(b)
    open(out, 'wb').write(nb)
    return len(b), k
with open(os.path.join(dst, 'SHA256-answers.txt'), 'w', encoding='utf-8', newline='\n') as f:
    f.write('# lab-file-sha256  published-copy-sha256  path  bytes  gstin-markers\n')
    for n in sorted(os.listdir(SRC + '/answers')):
        if n.startswith('aw13-') and n.endswith('.xml'):
            p, q = SRC + '/answers/' + n, os.path.join(dst, 'answers', n)
            sz, k = masked(p, q, 'utf-16-le')
            f.write('%s  %s  answers/%s  %d  %d\n' % (sha(p), sha(q), n, sz, k))
with open(os.path.join(dst, 'SHA256-exports.txt'), 'w', encoding='utf-8', newline='\n') as f:
    f.write('# lab-file-sha256  published-copy-sha256  path  bytes  gstin-markers\n')
    for n in sorted(os.listdir(SRC + '/export')):
        p, q = SRC + '/export/' + n, os.path.join(dst, 'export', n)
        sz, k = masked(p, q, 'utf-8')
        f.write('%s  %s  export/%s  %d  %d\n' % (sha(p), sha(q), n, sz, k))
def text(src, out):
    t = open(src, encoding='utf-8').read()
    t = GSTIN.sub(MARK, re.sub(r'\bD3\d\b', '<run>', t))
    open(out, 'w', encoding='utf-8', newline='\n').write(t)
text(SRC + '/answers/log.txt', os.path.join(dst, 'send-log.txt'))
text(SRC + '/REPORT.md', os.path.join(dst, 'REPORT.md'))
text(SRC + '/NOTES.txt', os.path.join(dst, 'notes.txt'))
for t in ['send.sh', 'check.py', 'vcmp.py', 'diffans.py', 'wdiff.py', 'jcmp.py', 'tools/build_push.py']:
    shutil.copyfile(SRC + '/' + t, os.path.join(dst, os.path.basename(t)))
shots = sorted(n for n in os.listdir(SRC + '/shots') if n.endswith('.jpg'))
with open(os.path.join(dst, 'SHA256-screenshots.txt'), 'w', encoding='utf-8', newline='\n') as f:
    for n in shots:
        p = SRC + '/shots/' + n
        f.write('%s  %s  %d\n' % (sha(p), n, os.path.getsize(p)))
with open(os.path.join(dst, 'SHA256-backups.txt'), 'w', encoding='utf-8', newline='\n') as f:
    for n in sorted(os.listdir(SRC + '/backup-pre')):
        p = SRC + '/backup-pre/' + n
        f.write('%s  backup-pre/%s  %d\n' % (sha(p), n, os.path.getsize(p)))
print('shots', len(shots))
