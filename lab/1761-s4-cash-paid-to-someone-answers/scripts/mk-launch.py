# Lab helper (row 1761-S4, new; not product code): writes the row's launch files by json.dump. launch-env.base.json holds the
# sheet's environment without the posting setting; launch-env.json adds BRIDGE_AGENT_ENABLE_WRITES=true, set by the lab on the owner's
# word of 27 and 28 Oct (synthetic books only). The first pair, written by a shell heredoc, lost its backslashes and was moved aside.
# Usage: python -I -X utf8 mk-launch.py
import hashlib, json, os
R = '<LAB_DIR>\\'
base = {'BRIDGE_TALLY_HOST': '127.0.0.1', 'BRIDGE_TALLY_PORT': '9001',
        'BRIDGE_AGENT_DATA_DIR': '<LAB_DIR>\\data', 'BRIDGE_TERMS_ACCEPTED': 'true'}
full = dict(base, BRIDGE_AGENT_ENABLE_WRITES='true')
for name, d in (('launch-env.base.json', base), ('launch-env.json', full)):
    p = R + name
    assert not os.path.exists(p), p
    with open(p, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(d, f, indent=1); f.write('\n')
    assert json.load(open(p, encoding='utf-8')) == d
    print(name, os.path.getsize(p), 'B', hashlib.sha256(open(p, 'rb').read()).hexdigest(), sorted(d))
assert os.path.isdir(base['BRIDGE_AGENT_DATA_DIR']) and not os.listdir(base['BRIDGE_AGENT_DATA_DIR'])
print('data folder exists and is empty')
