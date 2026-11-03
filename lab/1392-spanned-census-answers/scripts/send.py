# Lab helper (#2164 spanned census, not product code): one send with the sheet's exact curl line, from the row folder,
# each name once; logs the ISO time, the exit code and the -w line to evidence\log.txt. Usage: python -I send.py <file> <name>
import os, subprocess, sys, time
R = '<lab folder>'; f, name = sys.argv[1], sys.argv[2]
if os.path.exists(f'{R}/evidence/{name}.response'): sys.exit(f'{name} exists; never sent again')
t = time.time(); iso = time.strftime('%Y-%m-%dT%H:%M:%S', time.gmtime(t)) + f'.{int(t * 1000) % 1000:03d}Z'
cmd = f'curl.exe -sS --data-binary "@sheet\\{f}" -H "Content-Type: text/xml; charset=utf-16" -o "evidence\\{name}.response" -D "evidence\\{name}.headers" -w "%{{http_code}} %{{size_download}} %{{time_total}}\\n" http://localhost:9001/'
r = subprocess.run(cmd, cwd=R, capture_output=True, text=True, shell=True)
line = f'{iso} {name} file {f} exit {r.returncode} w "{r.stdout.strip()}" {r.stderr.strip()}'
open(f'{R}/evidence/log.txt', 'a', encoding='utf-8', newline='\n').write(line + '\n'); print(line)
if r.returncode != 0 or not r.stdout.startswith('200 '): print('TRANSPORT FAILURE: stop'); sys.exit(2)
