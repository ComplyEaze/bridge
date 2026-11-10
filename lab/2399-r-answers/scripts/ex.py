# Lab helper (row 2399-R, not product code): writes the JSON-RPC message the client saved under "response" in run/answers/<file>
# to rpc/<file>, as json.dumps(ensure_ascii=False) in UTF-8, so answer.py reads the message it expects (the client wraps it with
# its own fields). The client's file is not changed. Usage: python -I -X utf8 ex.py <answer file name>
import json, sys, hashlib
n = sys.argv[1]; run = sys.argv[2] if len(sys.argv) > 2 else 'run'; src = f'<LAB_DIR>/{run}/answers/{n}'; dst = f'<LAB_DIR>/rpc/{run}-{n}' if run != 'run' else f'<LAB_DIR>/rpc/{n}'
j = json.load(open(src, encoding='utf-8')); d = json.dumps(j['response'], ensure_ascii=False).encode('utf-8')
open(dst, 'xb').write(d)
print(f'{run}/answers/{n}: client file sha256 {hashlib.sha256(open(src, "rb").read()).hexdigest()}; {dst[21:]} {len(d)} B sha256 {hashlib.sha256(d).hexdigest()}')
