# Build the public 1050-B push folder. Answers and filled requests byte-exact; text files with the Tally data folder
# path (under the Windows Users tree) written as <tally-data-folder>; screenshots and backups listed by hash only.
# Usage: python -I build_push.py <dst>
import hashlib
import os
import shutil
import sys

SRC = os.path.join('C:' + os.sep, 'bridge-lab', '1050-B')
dst = sys.argv[1]
if os.path.exists(dst):
    sys.exit('destination exists: ' + dst)
BS = chr(92)
DATA = BS.join(['C:', 'Users', 'Public', 'TallyPrimeEditLog', 'data'])
red_log = []


def sha(p):
    return hashlib.sha256(open(p, 'rb').read()).hexdigest()


def copy_text(src, out):
    t = open(src, encoding='utf-8').read()
    n = t.count(DATA) + t.count(DATA.replace(BS, BS * 2))
    t = t.replace(DATA.replace(BS, BS * 2), '<tally-data-folder>').replace(DATA, '<tally-data-folder>')
    os.makedirs(os.path.dirname(out), exist_ok=True)
    open(out, 'w', encoding='utf-8', newline='\n').write(t)
    if n:
        red_log.append('%s: Tally data folder path x%d -> <tally-data-folder>' % (os.path.relpath(out, dst), n))


for sub in ['answers', 'filled']:
    os.makedirs(os.path.join(dst, sub))
    for n in sorted(os.listdir(os.path.join(SRC, sub))):
        p = os.path.join(SRC, sub, n)
        if n.endswith('.xml'):
            shutil.copyfile(p, os.path.join(dst, sub, n))
        else:
            copy_text(p, os.path.join(dst, sub, n))
for n in ['sent-remoteid.txt', 'restore-plan.txt', 'send.sh', 'fill.ps1', 'checkfill.py', 'analyse.py', 'compare.py', 'build_push.py']:
    copy_text(os.path.join(SRC, n), os.path.join(dst, n))
shots = []
for n in sorted(os.listdir(os.path.join(SRC, 'evidence'))):
    p = os.path.join(SRC, 'evidence', n)
    if n.endswith('.jpg'):
        shots.append('%s  %s  %d' % (sha(p), n, os.path.getsize(p)))
    elif n.endswith('.log'):
        continue  # window-watch process log stays on the lab PC
    else:
        copy_text(p, os.path.join(dst, 'evidence', n))
open(os.path.join(dst, 'evidence', 'SHA256-screenshots.txt'), 'w', encoding='utf-8', newline='\n').write('\n'.join(shots) + '\n')
bk = []
for b in ['I0', 'I1', 'I2']:
    for n in sorted(os.listdir(os.path.join(SRC, 'backups', b))):
        p = os.path.join(SRC, 'backups', b, n)
        bk.append('%s  backups/%s/%s  %d' % (sha(p), b, n, os.path.getsize(p)))
open(os.path.join(dst, 'evidence', 'SHA256-backups.txt'), 'w', encoding='utf-8', newline='\n').write('\n'.join(bk) + '\n')
open(os.path.join(dst, 'REDACTIONS.md'), 'w', encoding='utf-8', newline='\n').write(
    '# REDACTIONS\n\nAnswers (.xml) and filled requests are byte-exact. In text files the Tally data folder path is written as `<tally-data-folder>`:\n\n'
    + '\n'.join('- ' + r for r in red_log) + '\n\nScreenshots and Tally backups stay on the lab PC (hashes in evidence/SHA256-*.txt); the window-watch log stays on the lab PC.\n')
open(os.path.join(dst, '.gitattributes'), 'w', encoding='utf-8', newline='\n').write('# Byte-exact evidence: no line-ending normalisation.\n* -text\n')
print('shots', len(shots), 'backups', len(bk), 'redacted files', len(red_log))
