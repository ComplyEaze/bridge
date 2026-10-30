# Lab helper (#1584, not product code): writes one request or settings file the way the sheet says (UTF-8 without a
# BOM, to <name>.tmp, then renamed to <name>.json), and for a request waits for its answer in <row>\answers and prints
# the answer's main fields. Usage: python -I -X utf8 req.py env <row folder> <port>
#                                  python -I -X utf8 req.py call <row folder> <seq-name> <tool> <json args>
import json, os, sys, time
def put(path, obj):
    tmp = path[:-5] + '.tmp'
    with open(tmp, 'w', encoding='utf-8', newline='\n') as f: json.dump(obj, f)
    os.replace(tmp, path)
if sys.argv[1] == 'env':
    row, port = sys.argv[2], sys.argv[3]
    put(os.path.join(row, 'launch-env.json'), {'BRIDGE_TERMS_ACCEPTED': 'true', 'BRIDGE_TALLY_HOST': '127.0.0.1', 'BRIDGE_TALLY_PORT': port, 'BRIDGE_AGENT_ENABLE_IMPORT': 'true'})
    print('launch-env.json written'); sys.exit(0)
row, name, tool, args = sys.argv[2], sys.argv[3], sys.argv[4], json.loads(sys.argv[5]) if len(sys.argv) > 5 else {}
req = {'tool': tool} if tool == 'list' else {'tool': tool, 'arguments': args}
put(os.path.join(row, 'inbox', name + '.json'), req)
out = os.path.join(row, 'answers', name + '.json'); t0 = time.monotonic(); warned = False
while not os.path.exists(out):
    time.sleep(0.2)
    if not warned and time.monotonic() - t0 > 600: print('no answer in 10 minutes; left running'); warned = True
j = json.load(open(out, encoding='utf-8'))
r = j['response'].get('result') or {}
sc = r.get('structuredContent')
summary = {'call_sent_utc': j['call_sent_utc'], 'round_trip_seconds': j['round_trip_seconds'], 'isError': r.get('isError'), 'rpc_error': j['response'].get('error')}
if tool == 'list':
    names = [t['name'] for t in r.get('tools', [])]
    summary.update({'tools': len(names), 'build_import_xml': 'build_import_xml' in names, 'local_data_report': 'local_data_report' in names, 'post_import': 'post_import' in names})
    print(json.dumps(summary)); sys.exit(0)
print(json.dumps(summary)); print(json.dumps(sc if sc is not None else r, ensure_ascii=False)[:4000])
