# Build the public W12 push folder: answers, send log, report, notes (lane ids redacted), tools,
# and hash lists for the screenshots and the backup (which stay on the lab PC).
# The full answers carry the synthetic company's GSTIN in <CMPGSTIN>; the published copy replaces it with a
# same-length marker (UTF-16LE, so byte counts stay equal). SHA256-answers.txt lists the lab file's hash and the
# published copy's hash.
# Usage: python -I build_push.py <dst>
import hashlib, os, re, shutil, sys
SRC = 'C:/bridge-lab/w12'
GSTIN = re.compile(r'\b\d{2}[A-Z]{5}\d{4}[A-Z][1-9A-Z]Z[0-9A-Z]\b')
MARK = '<GSTIN-SYNTH15>'
dst = sys.argv[1]
if os.path.exists(dst):
    sys.exit('destination exists: ' + dst)
os.makedirs(os.path.join(dst, 'answers'))
def sha(p):
    return hashlib.sha256(open(p, 'rb').read()).hexdigest()
ans = sorted(n for n in os.listdir(SRC + '/answers') if n.startswith('aw12-') and n.endswith('.xml'))
with open(os.path.join(dst, 'SHA256-answers.txt'), 'w', encoding='utf-8', newline='\n') as f:
    f.write('# lab-file-sha256  published-copy-sha256  path  bytes  gstin-markers\n')
    for n in ans:
        p = SRC + '/answers/' + n
        b = open(p, 'rb').read()
        t, k = GSTIN.subn(MARK, b.decode('utf-16-le'))
        nb = t.encode('utf-16-le')
        assert len(nb) == len(b)
        q = os.path.join(dst, 'answers', n)
        open(q, 'wb').write(nb)
        f.write('%s  %s  answers/%s  %d  %d\n' % (sha(p), sha(q), n, len(b), k))
shutil.copyfile(SRC + '/answers/log.txt', os.path.join(dst, 'send-log.txt'))
shutil.copyfile(SRC + '/REPORT.md', os.path.join(dst, 'REPORT.md'))
notes = re.sub(r'\bD3\d\b', '<lane>', open(SRC + '/NOTES.txt', encoding='utf-8').read())
open(os.path.join(dst, 'notes.txt'), 'w', encoding='utf-8', newline='\n').write(notes)
for t in ['send.sh', 'check.py', 'vcmp.py', 'vfields.py', 'diffans.py', 'tools/build_push.py']:
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
print('answers', len(ans), 'shots', len(shots))
