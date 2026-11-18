# Lab helper (2074-2214 W row, 18 Nov; not product code): makes this sitting's run folder from the kit and the 9 Nov W run's
# helpers. Copies the kit into sheet/ (the kit stays where it is), copies the helpers of <lab root>/2074-w with that folder
# name changed to this one (nothing else changed), the read arguments, and the client into run2/ with a new launch-env.json
# and a new empty data folder. Checks the a83ee02d bundle (artifact zip digest, mcpb archive and binary hashes from
# mcpb-smoke.json) and extracts the mcpb into build/. Refuses if any target exists. Deletes nothing.
# Usage: python -I -X utf8 setup-wb.py
import hashlib, json, os, shutil, subprocess
R = '<lab folder>'; W = '<lab root>/2074-w'; B = '<lab root>/item46-build'
def sha(p): return hashlib.sha256(open(p, 'rb').read()).hexdigest()
for d in ('sheet', 'args', 'run2', 'build', 'evidence', 'reads', 'shots', 'arm', 'debt', 'bk'):
    if os.path.exists(f'{R}/{d}'): raise SystemExit(f'{d} exists')
for n in ('call.py', 'raw.py', 'creates.py', 'a5.py', 'rd.py', 'xlsx.py'):
    if os.path.exists(f'{R}/{n}'): raise SystemExit(f'{n} exists')
os.makedirs(R + '/sheet')
for n in ('.gitattributes', 'LAB-COMMON.md', 'RUN-SHEET.md', 'SHA256.txt', 'proxy.py', 'requests-manifest.txt'): shutil.copy2(f'{R}/{n}', f'{R}/sheet/{n}')
shutil.copytree(R + '/requests', R + '/sheet/requests'); shutil.copytree(R + '/scripts', R + '/sheet/scripts')
bad = 0
for l in open(R + '/sheet/SHA256.txt', encoding='utf-8'):
    h, n, p = l.split('  ', 2); p = p.strip()
    if sha(f'{R}/sheet/{p}') != h or os.path.getsize(f'{R}/sheet/{p}') != int(n): print('MISMATCH', p); bad += 1
print('kit in sheet/: mismatched', bad)
if bad: raise SystemExit('kit mismatch')
for n in ('call.py', 'raw.py', 'creates.py', 'a5.py', 'rd.py', 'xlsx.py'):
    t = open(f'{W}/{n}', encoding='utf-8').read(); c = t.count(W) + t.count('2074-w\\\\shots')
    t2 = t.replace(W, R).replace('2074-w\\\\shots', '2074-2214\\\\shots')
    open(f'{R}/{n}', 'w', encoding='utf-8', newline='\n').write(t2); print(n, 'folder name replaced', c, 'x')
shutil.copytree(W + '/args', R + '/args')
os.makedirs(R + '/run2/inbox'); os.makedirs(R + '/run2/answers'); os.makedirs(R + '/run2/data'); os.makedirs(R + '/run2/evidence')
shutil.copy2(W + '/run2/client.py', R + '/run2/client.py')
env = json.load(open(W + '/run2/launch-env.json', encoding='utf-8')); env['BRIDGE_AGENT_DATA_DIR'] = '<lab folder>\\run2\\data'
open(R + '/run2/launch-env.json', 'w', encoding='utf-8', newline='\n').write(json.dumps(env))
print('launch-env keys', sorted(env), 'port', env['BRIDGE_TALLY_PORT'], 'max bytes', env['BRIDGE_AGENT_MAX_BYTES'])
for d in ('evidence', 'reads', 'shots', 'arm', 'debt', 'bk'): os.makedirs(f'{R}/{d}')
z = sha(B + '/artifact.zip'); print('artifact.zip', z, z == '4f6e91b082d5fde228aded36713e65dcb0f65f36a7fb7f13bb03730aa8a72734')
sm = json.load(open(B + '/zip/mcpb-smoke.json', encoding='utf-8'))
m = sha(B + '/zip/bridge-tally.mcpb'); print('mcpb', m, m == sm['archive_sha256'])
os.makedirs(R + '/build')
r = subprocess.run(['C:/Program Files/7-Zip/7z.exe', 'x', '-y', '-o' + R + '/build', B + '/zip/bridge-tally.mcpb'], capture_output=True, text=True); print('7z exit', r.returncode)
e = sha(R + '/build/bin/x86_64-pc-windows-msvc/bridge_mcp.exe'); print('bridge_mcp.exe', e, e == sm['binary_sha256'] == '7086f1d0d7430e7dc1fefd6f69c0d4a14d717a2e06477b32935a1d3aa4fee1b8')
