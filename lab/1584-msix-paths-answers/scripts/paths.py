# Lab helper (#1584 I2/O2/E3, not product code): writes <row>\paths.txt (UTF-8, no BOM) with three lines: result.path as
# decoded by a JSON parser from the build answer; R\imports\<batch_id>.xml; Q\imports\<batch_id>.xml (LA expanded).
# Usage: python -I -X utf8 paths.py <row folder> <answer file name>
import json, os, sys
row, ans = sys.argv[1], sys.argv[2]
j = json.load(open(os.path.join(row, 'answers', ans), encoding='utf-8'))
res = j['response']['result']['structuredContent']['result']
la, b = os.environ['LOCALAPPDATA'], res['batch_id']
lines = [res['path'], os.path.join(la, 'Bridge', 'agent', 'imports', b + '.xml'),
         os.path.join(la, 'Packages', '<family>', 'LocalCache', 'Local', 'Bridge', 'agent', 'imports', b + '.xml')]
open(os.path.join(row, 'paths.txt'), 'w', encoding='utf-8', newline='\n').write('\n'.join(lines) + '\n')
print(f"batch_id {b}; path_state {res.get('path_state')}; sha256 {res.get('sha256')}")
for l in lines: print(l.replace(os.environ.get('USERNAME', '?'), '<u>'))
