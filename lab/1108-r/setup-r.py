"""#1108-R: makes the item's row folder. Lab helper, from 1108-A2 sitting 2's setup2.py; not for merge.

python -I setup-r.py

Reads the sheet folder it sits in. First it checks every file SHA256.txt lists, by bytes and sha256,
and that the folder holds no file SHA256.txt does not list. Any miss prints STOP, and nothing is
made. Then it makes C:/bridge-lab/1108r, which must not exist yet:
- the sheet folder copied whole as lab/1108-r (send.py, chk.py and args1108.py read it there);
- every file of helpers/ copied byte for byte into the row folder;
- guid.txt and company-number.txt (THE COMPANY's) from A2 sitting 2's folder, C:/bridge-lab/1108a2s2;
- for each row, its own client folder agent-run-r0, -r1 or -r2, with the lab's stdio client
  (1392-E's client.py, sha256 0ed0bb03...) and empty inbox, answers, step0 and evidence folders;
- empty evidence, answers, shots, box and calls folders.

It makes no data folder (agent-r0 to agent-r2), no relay folder and no backup folder: the step that
uses each makes it. Every copy is checked again after it is written. A miss once the row folder
exists prints STOP and leaves the folder as it is.
"""
import hashlib
import os
import shutil
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
R = 'C:/bridge-lab/1108r'
A2 = 'C:/bridge-lab/1108a2s2'
CLIENT = 'C:/bridge-lab/1392e/agent-run/client.py'
ROWS = ('r0', 'r1', 'r2')


def sha(path):
    with open(path, 'rb') as source:
        return hashlib.sha256(source.read()).hexdigest()


def stop(why):
    print('STOP: ' + why)
    sys.exit(1)


def files_under(top):
    out = []
    for folder, _, names in os.walk(top):
        out += [os.path.relpath(os.path.join(folder, n), top).replace(os.sep, '/') for n in names]
    return sorted(out)


def misses(top, listed):
    """Each listed file absent or different under top, and each file under top that is not listed."""
    found = files_under(top)
    bad = []
    for path, (digest, size) in sorted(listed.items()):
        full = os.path.join(top, path)
        if path not in found:
            bad.append(path + ': missing')
        elif (sha(full), os.path.getsize(full)) != (digest, size):
            bad.append(path + ': differs from SHA256.txt')
    bad += [p + ': not in SHA256.txt' for p in found if p not in listed and p != 'SHA256.txt']
    return bad


listed = {}
with open(os.path.join(HERE, 'SHA256.txt'), encoding='utf-8') as source:
    for line in source:
        if line.strip() and not line.startswith('#'):
            digest, size, path = line.rstrip('\r\n').split('  ')
            listed[path] = (digest, int(size))
bad = misses(HERE, listed)
for miss in bad:
    print('FAIL ' + miss)
print('files: ok %d, FAIL %d' % (len(listed) - len([b for b in bad if 'not in SHA256' not in b]), len(bad)))
if bad:
    stop('the sheet folder is not as SHA256.txt lists it; nothing made')
if os.path.exists(R):
    stop(R + ' exists; nothing made')
for path in (A2 + '/guid.txt', A2 + '/company-number.txt', CLIENT):
    if not os.path.isfile(path):
        stop(path + ' is not a file; nothing made')
if not sha(CLIENT).startswith('0ed0bb03'):
    stop('client.py sha256 %s is not 0ed0bb03...; nothing made' % sha(CLIENT))

os.makedirs(R)
shutil.copytree(HERE, R + '/lab/1108-r')
bad = misses(R + '/lab/1108-r', listed)
if bad:
    stop('the copy differs: %s; %s left as it is' % (bad, R))
helpers = sorted(os.listdir(os.path.join(HERE, 'helpers')))
for name in helpers:
    shutil.copyfile(os.path.join(HERE, 'helpers', name), R + '/' + name)
    if (sha(R + '/' + name), os.path.getsize(R + '/' + name)) != listed['helpers/' + name]:
        stop('%s differs after the copy; %s left as it is' % (name, R))
    print('%s sha256 %s' % (name, sha(R + '/' + name)))
for name in ('guid.txt', 'company-number.txt'):
    shutil.copyfile(A2 + '/' + name, R + '/' + name)
    if sha(R + '/' + name) != sha(A2 + '/' + name):
        stop('%s differs after the copy; %s left as it is' % (name, R))
    print('%s copied from A2 sitting 2' % name)
for folder in ('evidence', 'answers', 'shots', 'box', 'calls'):
    os.makedirs(R + '/' + folder)
for row in ROWS:
    for folder in ('inbox', 'answers', 'step0', 'evidence'):
        os.makedirs('%s/agent-run-%s/%s' % (R, row, folder))
    shutil.copyfile(CLIENT, '%s/agent-run-%s/client.py' % (R, row))
    print('agent-run-%s/client.py sha256 %s' % (row, sha('%s/agent-run-%s/client.py' % (R, row))))
    if os.path.exists('%s/agent-%s' % (R, row)):
        stop('agent-%s exists; %s left as it is' % (row, R))
print('setup: ok, %d helpers, %s made' % (len(helpers), R))
