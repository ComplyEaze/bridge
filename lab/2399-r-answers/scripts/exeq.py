# Lab helper (row 2399-R, not product code): parses run/answers/<file> and rpc/<file> as JSON and prints whether the rpc/ object
# equals the client file's "response" value (parsed values compared, not text). Usage: python -I -X utf8 exeq.py <file>...
import json, sys
run = 'run'
for n in sys.argv[1:]:
    if n.startswith('--run='): run = n[6:]; continue
    a = json.load(open(f'<LAB_DIR>/{run}/answers/{n}', encoding='utf-8'))['response']
    b = json.load(open(f'<LAB_DIR>/rpc/{n}' if run == 'run' else f'<LAB_DIR>/rpc/{run}-{n}', encoding='utf-8'))
    print(f'{n}: rpc/{n} JSON-equal to the client file\'s "response" value: {a == b}')
