# Lab helper (#1688, not product code): LAB-COMMON C2 a to c, each kept as a file under evidence/<tag>-c2-*.txt:
# tasklist for tally.exe, netstat -ano (the full output kept on the lab PC; the pushed copy keeps only the 9001 lines) and GET /status.
import subprocess, sys
from datetime import datetime, timezone
tag = sys.argv[1]; E = '<LAB_DIR>/evidence'
def now(): return datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
for name, cmd in (('tasklist', 'tasklist /FI "IMAGENAME eq tally.exe" /FO CSV /NH 2>&1'), ('netstat', 'netstat -ano 2>&1'),
                  ('status', 'curl.exe -sS -w " | %{http_code} %{size_download}" http://127.0.0.1:9001/status 2>&1')):
    t0 = now(); r = subprocess.run('cmd.exe /c ' + cmd, capture_output=True, text=True)
    open(f'{E}/{tag}-c2-{name}.txt', 'w', encoding='utf-8', newline='\n').write(f'command: {cmd}\ntime: {t0}\nexit: {r.returncode}\n{r.stdout}')
    lines = r.stdout.strip().splitlines()
    show = [l for l in lines if ':9001 ' in l and 'LISTENING' in l] + [f'({sum("LISTENING" in l for l in lines)} LISTENING lines in all)'] if name == 'netstat' else lines
    print(f'{name} exit {r.returncode}: ' + ' | '.join(s.strip() for s in show))
