# Lab helper (row 2511-P, new; not product code): makes the row folder <LAB_DIR>, which must not exist.
# Copies the sheet's 17 kit files byte for byte from the worktree and checks each against the sheet's SHA256.txt
# (sha256, bytes, name); copies row 683-TN's m2 (the KIT LAST MARKS file) to kit-last/ and checks its sha256 and bytes
# against the sheet's line; copies the lab's stdio client helpers from row 2507-T (client.py, call.py, show.py, step0.py)
# byte for byte; makes data/ (new, empty), inbox/, answers/, args/, step0/, evidence/, shots/; writes launch-env.json.
# Deletes nothing. Usage: python -I -X utf8 mk-row.py <TN m2 copy>
import hashlib, json, os, shutil, sys
S = '<SHEET_DIR>/lab/2511-p'; R = '<LAB_DIR>'; T = '<ROW_2507T_DIR>'
if os.path.exists(R): raise SystemExit('row folder exists: nothing done')
def sha(p): return hashlib.sha256(open(p, 'rb').read()).hexdigest()
rows = [l.split('  ') for l in open(S + '/SHA256.txt', encoding='utf-8').read().splitlines() if l.strip()]
os.makedirs(R)
for d in ('data', 'inbox', 'answers', 'args', 'step0', 'evidence', 'shots', 'kit-last'): os.makedirs(R + '/' + d)
bad = 0
for h, n, name in rows:
    shutil.copyfile(S + '/' + name, R + '/' + name)
    ok = sha(R + '/' + name) == h and os.path.getsize(R + '/' + name) == int(n)
    bad += not ok; print(('equal ' if ok else 'DIFFERS ') + name)
shutil.copyfile(S + '/SHA256.txt', R + '/SHA256.txt')
k = R + '/kit-last/a-m2-M-marks.xml'; shutil.copyfile(sys.argv[1], k)
kok = sha(k) == '3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba' and os.path.getsize(k) == 3404
print(('equal ' if kok else 'DIFFERS ') + 'kit-last/a-m2-M-marks.xml', os.path.getsize(k), sha(k))
for f in ('client.py', 'call.py', 'show.py', 'step0.py'):
    shutil.copyfile(T + '/' + f, R + '/' + f); print('helper', f, sha(R + '/' + f))
launch = {'BRIDGE_TALLY_HOST': '127.0.0.1', 'BRIDGE_TALLY_PORT': '9001', 'BRIDGE_AGENT_DATA_DIR': '<LAB_DIR>\\data',
          'BRIDGE_TERMS_ACCEPTED': 'true', 'BRIDGE_AGENT_ENABLE_IMPORT': 'true'}
with open(R + '/launch-env.json', 'w', encoding='utf-8', newline='\n') as f: json.dump(launch, f, indent=1); f.write('\n')
print('launch-env.json', sha(R + '/launch-env.json'), sorted(launch))
print('kit files', len(rows), 'differ', bad, 'kit-last ok', kok, 'data entries', len(os.listdir(R + '/data')))
