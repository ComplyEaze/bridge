# Build the published Masters M1 answers folder from the lab evidence folder.
# The run sheet's synthetic GSTINs become the same-length mask ZZZZZZZZZZZZZZZ (the XML stays well-formed).
# Screenshots and the backup stay on the lab PC and are listed by hash. Usage: python -I build_push.py <dst>
import hashlib, os, re, shutil, sys
SRC = 'C:/bridge-lab/masters-m1/evidence'
GSTIN = re.compile(r'\b\d{2}[A-Z]{5}\d{4}[A-Z][1-9A-Z]Z[0-9A-Z]\b')
MASK = 'Z' * 15
dst = sys.argv[1]
if os.path.exists(dst):
    sys.exit('destination exists: ' + dst)
os.makedirs(dst)
def sha(p): return hashlib.sha256(open(p, 'rb').read()).hexdigest()
rows = []
for n in sorted(os.listdir(SRC)):
    p = os.path.join(SRC, n)
    if not (n.endswith('.response') or n.endswith('.headers')):
        continue
    b = open(p, 'rb').read()
    k = 0
    if n.endswith('.response'):
        t, k = GSTIN.subn(MASK, b.decode('utf-16-le'))
        nb = t.encode('utf-16-le')
        assert len(nb) == len(b)
    else:
        nb = b
    q = os.path.join(dst, n)
    open(q, 'wb').write(nb)
    rows.append('%s  %s  %s  %d  %d' % (sha(p), sha(q), n, len(b), k))
def text(n, extra=None, drop=None):
    t = open(os.path.join(SRC, n), encoding='utf-8').read()
    if drop:
        t = re.sub(drop, '', t, count=1)
    t, k = GSTIN.subn(MASK, t)
    if extra:
        t += extra
    q = os.path.join(dst, n)
    open(q, 'w', encoding='utf-8', newline='\n').write(t)
    return k
k_log = text('log.txt', drop=r' \(item [^)]*\)')
k_p3 = text('phase3-parse.txt')
k_p4 = text('phase4-parse.txt') if os.path.exists(os.path.join(SRC, 'phase4-parse.txt')) else 0
for n in ('phase2-parse.txt', 'phase2-parse.py', 'phase3-parse.py', 'send-read.sh'):
    shutil.copyfile(os.path.join(SRC, n), os.path.join(dst, n))
shutil.copyfile(__file__, os.path.join(dst, 'build_push.py'))
with open(os.path.join(dst, 'SHA256-evidence.txt'), 'w', encoding='utf-8', newline='\n') as f:
    f.write('# lab-file-sha256  published-copy-sha256  file  bytes  gstin-masks\n')
    f.write('\n'.join(rows) + '\n')
    for n in [x for x in ('log.txt', 'phase3-parse.txt', 'phase4-parse.txt') if os.path.exists(os.path.join(SRC, x))]:
        f.write('%s  %s  %s\n' % (sha(os.path.join(SRC, n)), sha(os.path.join(dst, n)), n))
shots = sorted(n for n in os.listdir(SRC + '/screenshots') if n.endswith('.jpg'))
with open(os.path.join(dst, 'SHA256-screenshots.txt'), 'w', encoding='utf-8', newline='\n') as f:
    for n in shots:
        p = SRC + '/screenshots/' + n
        f.write('%s  %s  %d\n' % (sha(p), n, os.path.getsize(p)))
open(os.path.join(dst, 'SHA256-backup.txt'), 'w', encoding='utf-8', newline='\n').write(
    '9ca37bd80d695f2fd93b8055a59566bfdf1f2b4a3b581a5b16c3092eb792994d  TDBK1800_100060.001  24735  B0, BRIDGE MASTERS LAB, 2026-10-12T14:16:25Z\n')
rep = open(os.path.join(SRC, 'REPORT.md'), encoding='utf-8').read()
rep = re.sub(r'GST Regular, GSTIN \S+, Monthly\.', "GST Regular, Monthly, with the run sheet's company GSTIN (shown here as the same-length mask ZZZZZZZZZZZZZZZ).", rep)
rep = re.sub(r'Regular, GSTIN \S+\.', "Regular, with the run sheet's party GSTIN (mask ZZZZZZZZZZZZZZZ).", rep)
rep, k_rep = GSTIN.subn(MASK, rep)
rep += ('\n## Masking\nThe run sheet\'s synthetic GSTINs (company, party and M1 party) are written as the same-length mask ZZZZZZZZZZZZZZZ.\n'
        '- **Text:** the mask is used in log.txt, the phase parse files and this report. The published log.txt also drops one scheduling note from its first line.\n'
        '- **XML answers:** the mask is used inside the R-PARTY answers (n-0b-party-before/-after, m1-before, m1-end). It keeps their byte length and adds no markup; the answers keep Tally\'s own &#4; references (see the parsing note). No other answer contains a GSTIN, and they are published byte for byte.\n'
        '- **Hashes:** SHA256-evidence.txt lists the lab hash and the published hash of every file.\n')
open(os.path.join(dst, 'REPORT.md'), 'w', encoding='utf-8', newline='\n').write(rep)
print('files', len(os.listdir(dst)), 'masks log', k_log, 'p3', k_p3, 'report', k_rep, 'answers', sum(int(r.split()[-1]) for r in rows))
