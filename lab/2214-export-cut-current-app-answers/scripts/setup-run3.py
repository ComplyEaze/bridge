# Lab helper (2074-2214 W row Close, 18 Nov; not product code): makes run3/ for Close step 4's new server on port 9001:
# run2's client.py copied unchanged, run2's launch-env.json with the port set to 9001 and a new empty data folder, and
# empty inbox, answers and evidence folders. Refuses if run3 exists. Deletes nothing.
# Usage: python -I -X utf8 setup-run3.py
import json, os, shutil
R = '<lab folder>'
if os.path.exists(R + '/run3'): raise SystemExit('run3 exists')
for d in ('inbox', 'answers', 'data', 'evidence'): os.makedirs(f'{R}/run3/{d}')
shutil.copy2(R + '/run2/client.py', R + '/run3/client.py')
env = json.load(open(R + '/run2/launch-env.json', encoding='utf-8'))
env['BRIDGE_TALLY_PORT'] = '9001'; env['BRIDGE_AGENT_DATA_DIR'] = '<lab folder>\\run3\\data'
open(R + '/run3/launch-env.json', 'w', encoding='utf-8', newline='\n').write(json.dumps(env))
print('run3 made; port', env['BRIDGE_TALLY_PORT'], 'data', env['BRIDGE_AGENT_DATA_DIR'])
