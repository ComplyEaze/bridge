# Lab helper (#2214, not product code): ONE raw MARKS read by LAB-COMMON C3's line (the creates runner's MARKS step on its
# own), the answer and headers kept under raw\, one line in evidence/raw-log.txt. Usage: python -I marks.py <label>
import importlib.util, os, shutil, sys, time
spec = importlib.util.spec_from_file_location('creates_lib', '<LAB_DIR>/creates_lib.py'); lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
R = '<LAB_DIR>'; os.makedirs(R + '/raw', exist_ok=True)
label = sys.argv[1]; ts = lib.stamp()
mk, info, body = lib.marks()
seq = len([f for f in os.listdir(R + '/raw') if f.endswith('.xml')])
open(f'{R}/raw/{seq:03d}-marks-{label}.xml', 'wb').write(body)
if os.path.exists(f'{lib.out}/_last.headers'): shutil.copy(f'{lib.out}/_last.headers', f'{R}/raw/{seq:03d}-marks-{label}.headers.txt')
line = f'{label}: MARKS sent {ts}: {info}; answer sha256 {lib.sha(body)}; GUID, V, M = {mk}'
with open(R + '/evidence/raw-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(line + '\n')
print(line)
