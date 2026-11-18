# Lab helper (rows 1381-R1 and later, not product code): LAB-COMMON C2 a to c, with the window capture first. Writes
# evidence/<label>-tasklist.txt, evidence/<label>-netstat.txt (whole, kept on the lab PC) and appends one summary line
# to evidence/notes.txt. Sends nothing to 9001 but GET /status. The MARKS read of C2 d is the row's own send.
# Usage: python -I -X utf8 step0.py <label>
import hashlib, os, subprocess, sys, time
R = os.path.dirname(os.path.abspath(__file__)).replace(os.sep, '/'); label = sys.argv[1]
os.makedirs(R + '/evidence', exist_ok=True); os.makedirs(R + '/shots', exist_ok=True)
def now(): t = time.time(); return time.strftime('%Y-%m-%dT%H:%M:%S', time.gmtime(t)) + f'.{int(t * 1000) % 1000:03d}Z'
t0 = now()
h = subprocess.run(['powershell', '-NoProfile', '-Command', '[int64](Get-Process tally | Select-Object -First 1).MainWindowHandle'], capture_output=True, text=True).stdout.strip()
shot = R.replace('/', '\\') + '\\shots\\' + label + '-window.png'
subprocess.run(['powershell', '-NoProfile', '-File', '<lab folder>/rc070/tools/capwin.ps1', '-Handle', h, '-Out', shot], capture_output=True, text=True)
cap = hashlib.sha256(open(shot, 'rb').read()).hexdigest() if os.path.exists(shot) else 'none'
a = subprocess.run('cmd /c tasklist /FI "IMAGENAME eq tally.exe" /FO CSV /NH 2>&1', capture_output=True, text=True, shell=True)
open(f'{R}/evidence/{label}-tasklist.txt', 'w', encoding='utf-8', newline='\n').write(a.stdout + f'exit {a.returncode}\n')
lines = [l for l in a.stdout.splitlines() if 'tally.exe' in l.lower()]; pid = lines[0].split('","')[1] if len(lines) == 1 else None
b = subprocess.run('cmd /c netstat -ano 2>&1', capture_output=True, text=True, shell=True)
open(f'{R}/evidence/{label}-netstat.txt', 'w', encoding='utf-8', newline='\n').write(b.stdout + f'exit {b.returncode}\n')
lis = [l.split() for l in b.stdout.splitlines() if 'LISTENING' in l]
p9001 = sorted({x[-1] for x in lis if x[1].endswith(':9001')}); other = len([x for x in lis if not x[1].endswith(':9001')])
c = subprocess.run(['curl.exe', '-sS', '-w', '%{http_code} %{size_download}', '-o', os.devnull, 'http://127.0.0.1:9001/status'], capture_output=True, text=True)
ok = len(lines) == 1 and p9001 == [pid] and other > 0 and a.returncode == 0 and b.returncode == 0 and c.returncode == 0 and c.stdout.startswith('200 ')
line = (f'{t0} {label} step 0: capture {cap}; a. tasklist exit {a.returncode}, {len(lines)} tally.exe line(s), PID {pid}; '
        f'b. netstat exit {b.returncode}, 9001 LISTENING PIDs {p9001}, other LISTENING lines {other}; c. GET /status curl {c.returncode} "{c.stdout.strip()}"; ok {ok}')
open(R + '/evidence/notes.txt', 'a', encoding='utf-8', newline='\n').write(line + '\n'); print(line)
if not ok: sys.exit(1)
