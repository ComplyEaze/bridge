# Lab helper (rows 1108-R0 to R2, new; not product code): writes one row's launch-env.json into its client folder
# <LAB_DIR>/agent-run-<row>, as A2 sitting 2's agent-run/launch-env.json with the data folder the sheet names for
# the row (BRIDGE_AGENT_ENABLE_WRITES=true set by the lab on the owner's word of 27 and 28 Oct, synthetic books only;
# BRIDGE_AGENT_REDACTION and BRIDGE_AGENT_ENABLE_BATCH_POST absent). Stops if the file exists or if the row's data folder
# <LAB_DIR>/agent-<row> exists (the sheet's 7.2 listing). Usage: python -I -X utf8 mk-launch.py r0
import hashlib, json, os, sys
row = sys.argv[1]
if row not in ('r0', 'r1', 'r2'): raise SystemExit('row must be r0, r1 or r2')
R = '<LAB_DIR>'; out = f'{R}/agent-run-{row}/launch-env.json'; data = f'{R}/agent-{row}'
if os.path.exists(out): raise SystemExit(out + ' exists')
print('listing of', data + ':', 'absent' if not os.path.exists(data) else 'PRESENT')
if os.path.exists(data): raise SystemExit('the row data folder exists: stop')
d = {'BRIDGE_AGENT_ENABLE_WRITES': 'true', 'BRIDGE_TERMS_ACCEPTED': 'true', 'BRIDGE_TALLY_HOST': '127.0.0.1',
     'BRIDGE_TALLY_PORT': '9003', 'BRIDGE_AGENT_DATA_DIR': data}
with open(out, 'w', encoding='utf-8', newline='') as f: json.dump(d, f)
assert json.load(open(out, encoding='utf-8')) == d
print('written', out, os.path.getsize(out), 'B', hashlib.sha256(open(out, 'rb').read()).hexdigest(), sorted(d))
