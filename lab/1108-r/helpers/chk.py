# Lab helper (1108-R, from 1108-A2's chk.py with its folders changed; that from 1108-A1, from 1381-M, not product code): runs the sheet's check_1108.py (from run/, unchanged) and keeps its whole output as
# evidence/chk-<name>.txt, with the command and exit code. Prints the output with THE COMPANY's GUID shown as <company guid> and
# the company number as <company number> (the kept file holds them as printed). Once guid.txt exists it also checks that every
# GUID the output names begins with THE COMPANY's GUID: any other = STOP. 'marks' mode with --set-guid writes guid.txt from the
# one COMPANY row (step 2). Usage: python -I -X utf8 chk.py <name> <mode> <args...> [--set-guid]
import os, re, subprocess, sys
R = 'C:/bridge-lab/1108r'; A = R + '/answers/'
args = [a for a in sys.argv[1:] if a != '--set-guid']; setg = '--set-guid' in sys.argv
name, mode, rest = args[0], args[1], args[2:]
cargs = [x if (x.startswith('SC ') or not x.endswith('.xml')) else A + x for x in rest]
r = subprocess.run([sys.executable, '-I', 'check_1108.py', mode] + cargs, cwd=R + '/lab/1108-r', capture_output=True, text=True, encoding='utf-8')
out = r.stdout + r.stderr
open(f'{R}/evidence/chk-{name}.txt', 'w', encoding='utf-8', newline='\n').write(
    f'$ python -I check_1108.py {mode} ' + ' '.join(f'"{x}"' if ' ' in x else x for x in rest) + f'\nexit {r.returncode}\n{out}')
if setg:
    rows = re.findall(r'^\s+GUID (\S+) ALTVCHID', out, re.M)
    if len(rows) != 1: raise SystemExit(f'STOP: {len(rows)} COMPANY rows')
    open(R + '/guid.txt', 'w').write(rows[0])
g = open(R + '/guid.txt').read().strip() if os.path.exists(R + '/guid.txt') else None
num = open(R + '/company-number.txt').read().strip() if os.path.exists(R + '/company-number.txt') else None
show = out
if g: show = show.replace(g, '<company guid>')
if num: show = re.sub(r'(?<!\d)' + num + r'(?!\d)', '<company number>', show)
print(f'exit {r.returncode}'); print(show, end='')
if g:
    guids = re.findall(r'\bGUID (\S+)', out) + re.findall(r'(?:ADDED|REMOVED|CHANGED) .* (\S+-\S+-\S+-\S+-\S+\S*)(?: :|$)', out, re.M)
    bad = [x for x in guids if not x.startswith(g)]
    print(f'GUID check: {len(guids)} GUID(s) named, {len(bad)} not beginning with THE COMPANY\'s')
    if bad: raise SystemExit('STOP: a GUID that does not begin with THE COMPANY\'s')
if r.returncode: raise SystemExit(f'STOP: check_1108.py exit {r.returncode} ({"NOT READ" if r.returncode == 2 else "FAIL"})')
