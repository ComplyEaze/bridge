# Lab helper (row 1761-S4 v3, row 1761-S4 v2's mk-row.py by mk-mkrow.py with the commit and folders changed; not product code): makes the row folder <LAB_DIR>, which must not exist
# (the sheet's "Before the sitting" item 3). Takes the sheet's files from the commit c00dc92d by git (blob bytes, no
# filters) and checks each against the sheet's SHA256.txt; copies statement.csv to the row folder and checks that copy;
# copies the lab's stdio client helpers from row 2511-P byte for byte (client.py, call.py, show.py, step0.py; note.py and
# shot.sh are written new for this folder); makes data/ (new, empty), inbox/, answers/, args/, evidence/, shots/, backups/
# (BK1 is not made: C5 asks for a new empty folder, made just before the backup); writes launch-env.base.json (the sheet's
# environment without the posting setting) and launch-env.json (adds BRIDGE_AGENT_ENABLE_WRITES=true, set by the lab on the
# owner's word of 27 and 28 Oct, synthetic books only). Deletes nothing. Usage: python -I -X utf8 mk-row.py
import hashlib, json, os, shutil, subprocess
G = '<WORKTREE>'; C = 'c00dc92d1c1395e1386704fd82a367a53b511f9d'; P = 'lab/1761-s4v3-cash-paid-to-someone'
R = '<LAB_DIR>'; T = '<ROW_2511P_DIR>'
if os.path.exists(R): raise SystemExit('row folder exists: nothing done')
def sha(b): return hashlib.sha256(b).hexdigest()
def blob(n): return subprocess.run(['git', '-C', G, 'cat-file', 'blob', f'{C}:{P}/{n}'], capture_output=True, check=True).stdout
rows = [l.split('  ') for l in blob('SHA256.txt').decode('utf-8').splitlines() if l.strip()]
os.makedirs(R)
for d in ('data', 'inbox', 'answers', 'args', 'evidence', 'shots', 'backups', 'sheet', 'step0'): os.makedirs(R + '/' + d)
bad = 0
for h, name in rows:
    b = blob(name); open(f'{R}/sheet/{name}', 'wb').write(b)
    ok = sha(b) == h; bad += not ok; print(('equal ' if ok else 'DIFFERS ') + name, len(b), 'B')
for name in ('SHA256.txt', '.gitattributes'):
    b = blob(name); open(f'{R}/sheet/{name}', 'wb').write(b); print('copied', name, len(b), 'B', sha(b))
shutil.copyfile(R + '/sheet/statement.csv', R + '/statement.csv')
cs = sha(open(R + '/statement.csv', 'rb').read()); print('statement.csv copy', cs, 'equal' if cs == dict((n, h) for h, n in rows)['statement.csv'] else 'DIFFERS')
for f in ('client.py', 'call.py', 'show.py', 'step0.py'):
    shutil.copyfile(T + '/' + f, R + '/' + f); print('helper', f, sha(open(R + '/' + f, 'rb').read()))
open(R + '/note.py', 'w', encoding='utf-8', newline='\n').write(
    '# Lab helper (row 1761-S4 v3, row 2511-P\'s note.py with the folder changed; not product code): appends one line to\n'
    '# evidence/notes.txt, the UTC time first, then the text given as one argument (so no shell reads its backslashes).\n'
    '# Usage: python -I -X utf8 note.py "<text>"\n'
    'import sys\nfrom datetime import datetime, timezone\n'
    "t = datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')\n"
    "open('<LAB_DIR>/evidence/notes.txt', 'a', encoding='utf-8', newline='\\n').write(t + ' ' + sys.argv[1] + '\\n')\n"
    'print(t, sys.argv[1][:200])\n')
open(R + '/shot.sh', 'w', encoding='utf-8', newline='\n').write(
    "# Lab helper (row 1761-S4 v3, row 2511-P's shot.sh with the folder changed; not product code): one PrintWindow capture of the Tally window to shots/<name>.png, then the UTC time.\n"
    "H=$(powershell -NoProfile -Command '[int64](Get-Process tally | Select-Object -First 1).MainWindowHandle'); powershell -NoProfile -File <RC070_DIR>/tools/capwin.ps1 -Handle $H -Out \"<LAB_DIR>/shots/$1.png\" | tail -1; date -u +%FT%T.%3NZ\n")
base = {'BRIDGE_TALLY_HOST': '127.0.0.1', 'BRIDGE_TALLY_PORT': '9001',
        'BRIDGE_AGENT_DATA_DIR': '<LAB_DIR>\\data', 'BRIDGE_TERMS_ACCEPTED': 'true'}
full = dict(base, BRIDGE_AGENT_ENABLE_WRITES='true')
for name, d in (('launch-env.base.json', base), ('launch-env.json', full)):
    with open(R + '/' + name, 'w', encoding='utf-8', newline='\n') as f: json.dump(d, f, indent=1); f.write('\n')
    assert json.load(open(R + '/' + name, encoding='utf-8')) == d
    print(name, os.path.getsize(R + '/' + name), 'B', sha(open(R + '/' + name, 'rb').read()), sorted(d))
print('sheet files', len(rows), 'differ', bad, 'data entries', len(os.listdir(R + '/data')))
