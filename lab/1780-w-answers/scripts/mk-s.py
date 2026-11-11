# Lab helper (row 1780-W part S, new; not product code; written whole by an editor): writes client.py, call.py and one.py from row
# 1780-H's by exact replacements (the folder, the build's path and sha256), and launch-env.json from 1780-H's launch-env.base.json
# with BRIDGE_AGENT_DATA_DIR turned to this row's data folder and BRIDGE_AGENT_ENABLE_WRITES set to true (S1, on the owner's word of
# 28 Oct); makes inbox\, answers\ and data\ new and empty. Prints the launch file's key names and sha256, never its values.
# Usage: python -I -X utf8 mk-s.py
import hashlib, json, os
R = '<LAB_DIR>'; H = '<ROW_1780H_DIR>'
EXE_SHA = '189cb22520447c9b0d91da3b773cbcf239ed5fdd2e977cd02ffbbdf91ba32fa6'
def mk(name, reps):
    dst = os.path.join(R, name); assert not os.path.exists(dst), dst
    s = open(os.path.join(H, name), encoding='utf-8').read()
    for a, b in reps:
        assert s.count(a) >= 1, (name, a)
        s = s.replace(a, b)
    assert '1780h' not in s, name
    open(dst, 'w', encoding='utf-8', newline='\n').write(s); print('written', name)
mk('client.py', [("(#1780-H part S and L, not product code; row 1780-F's client.py with the folder and build changed)",
                  "(row 1780-W part S, not product code; row 1780-H's client.py with the folder and build changed by mk-s.py)"),
                 ("<ROW_1780H_DIR>", R),
                 ("a946979561b61f51e6dab3539d3de98fd48c8f103f61d2e3abe787acf3240c23", EXE_SHA)])
mk('call.py', [("(#1780-H part S and L, not product code; row 1780-F's call.py with the folder changed)",
                "(row 1780-W part S, not product code; row 1780-H's call.py with the folder changed by mk-s.py)"),
               ("<ROW_1780H_DIR>", R)])
mk('one.py', [("(#1780-H part S and L, not product code; row 1780-F's one.py with the folder changed)",
               "(row 1780-W part S, not product code; row 1780-H's one.py with the folder changed by mk-s.py)"),
              ("'1780h'", "'1780w'")])
base = json.load(open(os.path.join(H, 'launch-env.base.json'), encoding='utf-8'))
assert sorted(base) == ['BRIDGE_AGENT_DATA_DIR', 'BRIDGE_TALLY_HOST', 'BRIDGE_TALLY_PORT', 'BRIDGE_TERMS_ACCEPTED'], sorted(base)
base['BRIDGE_AGENT_DATA_DIR'] = os.path.join(R, 'data')
for d in ('inbox', 'answers', 'data'):
    os.makedirs(os.path.join(R, d), exist_ok=False)
for name, env in (('launch-env.base.json', dict(base)), ('launch-env.json', dict(base, BRIDGE_AGENT_ENABLE_WRITES='true'))):
    p = os.path.join(R, name); assert not os.path.exists(p), p
    open(p, 'w', encoding='utf-8', newline='\n').write(json.dumps(env, indent=1) + '\n')
    print(name, 'keys', sorted(env), 'sha256', hashlib.sha256(open(p, 'rb').read()).hexdigest())
