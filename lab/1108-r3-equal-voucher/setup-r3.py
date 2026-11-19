"""#1108-R3: makes the row's folder. Lab helper, from 1108-R's setup-r.py; new, not run on Windows; not for merge.

python -I setup-r3.py

Reads the sheet folder it sits in and the folder rows 1108-R0 to R2 left on the lab PC (PRIOR). Writes only the new
row folder R, which must not exist yet. Every miss prints STOP and ends with exit 1, and a miss before R is made
makes nothing:
- every file SHA256.txt lists must be here with its bytes and sha256, and no other file may be here;
- from PRIOR, read only: guid.txt and company-number.txt (size and sha256 as the rows' record lists them), the lab's
  stdio client (agent-run-r0/client.py, sha256 0ed0bb03...), and the row backup BK-R, backups/bkr/ holding exactly one
  file, TDBK1800_<company number>.001, of 26485 bytes and the sha256 below;
- R is made: this folder copied whole to R/lab/1108-r3; helpers/ copied byte for byte into R (placeholder-helpers/
  is not: the lab makes its run copies); guid.txt and company-number.txt copied; BK-R copied to R/backups/bkr, so
  that every restore source of the row is under R; agent-run-r3 with the client and empty inbox, answers, step0 and
  evidence folders;
  empty evidence, answers, shots, box and calls folders; and agent-run-r3/launch-env.json (the row's launch
  configuration: writes on, the one setting the lab adds on its standing word for synthetic books; the batch
  and redaction settings absent; the relay's port; the row's own data folder, which is not made here and must be absent).
It makes no data, relay or backup folder: the step that uses each makes it. Every copy is checked again after it is
written. It never writes under PRIOR.
"""
import hashlib
import json
import os
import shutil
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
R = 'C:/bridge-lab/1108r3'
PRIOR = 'C:/bridge-lab/1108r'
CLIENT = PRIOR + '/agent-run-r0/client.py'
WANT = {
    PRIOR + '/guid.txt': (36, 'd49ecd16b69118dfafd544af8d9fd62f835d93cdcd1a0624140562166f95e249'),
    PRIOR + '/company-number.txt': (6, '7b501a4208b6b1a3afe9249a145f73df675eb13a0888b44d6e746f285320fd08'),
    CLIENT: (4687, '0ed0bb03bafb4f7999fa69756ef1e529d0157d826dbec58110bb96e9a4075d63'),
}
BKR_SIZE = 26485
BKR_SHA = 'f02e31d171d92e41cf2120bdcbdf50375383e461da64d8c273df16d8b9179e59'
BKR_DIR = PRIOR + '/backups/bkr'


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
if not listed:
    stop('SHA256.txt lists no file; nothing made')
bad = misses(HERE, listed)
for miss in bad:
    print('FAIL ' + miss)
print('files: ok %d, FAIL %d' % (len(listed) - len([b for b in bad if 'not in SHA256' not in b]), len(bad)))
if bad:
    stop('the sheet folder is not as SHA256.txt lists it; nothing made')
if os.path.exists(R):
    stop(R + ' exists; nothing made')
for path, (size, digest) in WANT.items():
    if not os.path.isfile(path):
        stop(path + ' is not a file; nothing made')
    if (os.path.getsize(path), sha(path)) != (size, digest):
        stop(path + ' is not the file the rows 1108-R0 to R2 record; nothing made')
number = open(PRIOR + '/company-number.txt', encoding='utf-8').read().strip()
if not number.isdigit():
    stop('company-number.txt does not hold a number; nothing made')
if not os.path.isdir(BKR_DIR):
    stop(BKR_DIR + ' is not a folder; nothing made')
held = sorted(os.listdir(BKR_DIR))
if held != ['TDBK1800_%s.001' % number]:
    stop('%s holds %s, not the one file TDBK1800_<company number>.001; nothing made' % (BKR_DIR, held))
bkr = BKR_DIR + '/' + held[0]
if (os.path.getsize(bkr), sha(bkr)) != (BKR_SIZE, BKR_SHA):
    stop('BK-R is not %d bytes with sha256 %s; nothing made' % (BKR_SIZE, BKR_SHA))
print('BK-R: %s %d B sha256 %s' % (held[0].replace(number, '<company number>'), BKR_SIZE, BKR_SHA))

data = R + '/agent-r3'
os.makedirs(R)
shutil.copytree(HERE, R + '/lab/1108-r3')
bad = misses(R + '/lab/1108-r3', listed)
if bad:
    stop('the copy differs: %s; %s left as it is' % (bad, R))
helpers = sorted(os.listdir(os.path.join(HERE, 'helpers')))
for name in helpers:
    shutil.copyfile(os.path.join(HERE, 'helpers', name), R + '/' + name)
    if (sha(R + '/' + name), os.path.getsize(R + '/' + name)) != listed['helpers/' + name]:
        stop('%s differs after the copy; %s left as it is' % (name, R))
    print('%s sha256 %s' % (name, sha(R + '/' + name)))
for name in ('guid.txt', 'company-number.txt'):
    shutil.copyfile(PRIOR + '/' + name, R + '/' + name)
    if sha(R + '/' + name) != sha(PRIOR + '/' + name):
        stop('%s differs after the copy; %s left as it is' % (name, R))
    print('%s copied from the rows 1108-R0 to R2' % name)
os.makedirs(R + '/backups/bkr')
shutil.copyfile(bkr, R + '/backups/bkr/' + held[0])
if (os.path.getsize(R + '/backups/bkr/' + held[0]), sha(R + '/backups/bkr/' + held[0])) != (BKR_SIZE, BKR_SHA):
    stop('BK-R differs after the copy; %s left as it is' % R)
print('BK-R copied to backups/bkr')
for folder in ('evidence', 'answers', 'shots', 'box', 'calls'):
    os.makedirs(R + '/' + folder)
run = R + '/agent-run-r3'
for folder in ('inbox', 'answers', 'step0', 'evidence'):
    os.makedirs(run + '/' + folder)
shutil.copyfile(CLIENT, run + '/client.py')
if sha(run + '/client.py') != WANT[CLIENT][1]:
    stop('client.py differs after the copy; %s left as it is' % R)
print('agent-run-r3/client.py sha256 %s' % sha(run + '/client.py'))
if os.path.exists(data):
    stop('agent-r3 exists; %s left as it is' % R)
launch = {'BRIDGE_AGENT_ENABLE_WRITES': 'true', 'BRIDGE_TERMS_ACCEPTED': 'true', 'BRIDGE_TALLY_HOST': '127.0.0.1',
          'BRIDGE_TALLY_PORT': '9003', 'BRIDGE_AGENT_DATA_DIR': data}
with open(run + '/launch-env.json', 'w', encoding='utf-8', newline='') as out:
    json.dump(launch, out)
with open(run + '/launch-env.json', encoding='utf-8') as back:
    if json.load(back) != launch:
        stop('launch-env.json does not read back as written; %s left as it is' % R)
print('launch-env.json sha256 %s, keys %s' % (sha(run + '/launch-env.json'), sorted(launch)))
print('setup: ok, %d helpers, %s made' % (len(helpers), R))
