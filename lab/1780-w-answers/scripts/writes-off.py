# Lab helper (row 1780-W WRITES OFF, new; not product code; written whole by an editor): removes BRIDGE_AGENT_ENABLE_WRITES from this
# row's launch-env.json (the file is rewritten in place, as mk-s.py wrote it, without that key; nothing is deleted) and prints the
# file's key names and sha256, never its values. Refuses while the launch runs (server.pid's process alive is not checked here; the
# client log's last line must read "session ended"). Usage: python -I -X utf8 writes-off.py
import hashlib, json
R = '<LAB_DIR>\\'
last = open(R + 'client-log.txt', encoding='utf-8').read().rstrip('\n').split('\n')[-1]
assert 'session ended' in last, last
env = json.load(open(R + 'launch-env.json', encoding='utf-8'))
print('before: keys', sorted(env), 'sha256', hashlib.sha256(open(R + 'launch-env.json', 'rb').read()).hexdigest())
env.pop('BRIDGE_AGENT_ENABLE_WRITES')
open(R + 'launch-env.json', 'w', encoding='utf-8', newline='\n').write(json.dumps(env, indent=1) + '\n')
print('after: keys', sorted(env), 'sha256', hashlib.sha256(open(R + 'launch-env.json', 'rb').read()).hexdigest())
print('base sha256', hashlib.sha256(open(R + 'launch-env.base.json', 'rb').read()).hexdigest())
