# Lab helper (row 2119-P, not product code): one tool call through the running stdio client (run\), as call.py does,
# with the sheet's rule 3 screenshots for the calls named with --shots: Tally's whole window by PrintWindow (no click, no
# focus change, no key) at about 10 s and 30 s after the call is written, and every 30 s after that until its answer is
# read; then one after the answer (C3: a new shape). With --answer-shot only the screenshot after the answer is taken.
# Prints one line with the client's own write and read times. At 600 s with no answer it prints a line and keeps waiting
# (stop rules: stop nothing, send nothing).
# Usage: python -I -X utf8 pcall.py <label> <tool> <args file> [--shots | --answer-shot]
import json, os, subprocess, sys, time
R = '<lab root>/2119-p'; RUN = R + '/run'
label, tool, argf = sys.argv[1:4]; mode = sys.argv[4] if len(sys.argv) > 4 else ''
args = json.load(open(argf, encoding='utf-8')) if argf != '-' else {}
def shot(name):
    h = subprocess.run(['powershell', '-NoProfile', '-Command', '[int64](Get-Process tally | Select-Object -First 1).MainWindowHandle'], capture_output=True, text=True).stdout.strip()
    r = subprocess.run(['powershell', '-NoProfile', '-File', '<lab root>/rc070/tools/capwin.ps1', '-Handle', h, '-Out', '<lab root>\\2119-p\\shots\\' + name + '.png'], capture_output=True, text=True)
    print(f'  shot {name}: {r.stdout.strip().splitlines()[-1] if r.stdout.strip() else r.stderr.strip()[:120]}', flush=True)
seq = len([f for f in os.listdir(RUN + '/inbox') if f.endswith('.json')]); fn = f'{seq:03d}-{label}.json'
tmp = f'{RUN}/{fn}.tmp'
with open(tmp, 'w', encoding='utf-8', newline='\n') as f: json.dump({'tool': tool, 'arguments': args}, f)
m0 = time.monotonic(); os.replace(tmp, f'{RUN}/inbox/{fn}')
out = f'{RUN}/answers/{fn}'; marks = [10, 30] if mode == '--shots' else []; k = 0; warned = False
while not os.path.exists(out):
    time.sleep(0.1); el = time.monotonic() - m0
    if mode == '--shots':
        nxt = marks[k] if k < len(marks) else 30 * (k)
        if el >= nxt: shot(f'{label}-t{int(nxt):03d}'); k += 1
    if not warned and el > 600: print('NO ANSWER IN 600 s: stop nothing, send nothing, report', flush=True); warned = True
time.sleep(0.3)
while True:
    try: j = json.load(open(out, encoding='utf-8')); break
    except ValueError: time.sleep(0.2)
if mode in ('--shots', '--answer-shot'): shot(f'{label}-after')
r = j['response'].get('result') or {}
line = f'{fn}: {tool} written {j.get("call_sent_utc")} read {j.get("answer_read_utc")} secs {j.get("round_trip_seconds")} bytes {os.path.getsize(out)} isError {r.get("isError")} rpc {bool(j["response"].get("error"))}'
print(line, flush=True)
with open(RUN + '/evidence/tools-log.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(line + '\n')
