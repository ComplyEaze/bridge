# Lab helper (#1780-F launcher cells, not product code): the lab's launcher (client.py of rows 1780-C2, D and E) with its command
# replaced: the same working folder, the same environment built from launch-env.json (every BRIDGE_ variable of the shell dropped,
# the file's keys put in), the same pipes (stdin and stdout piped, stderr to a file), but the command is
# powershell -NoProfile -ExecutionPolicy Bypass -File spawn-parent.ps1 <the cell's arguments>. It closes stdin, reads stdout to its
# end and waits. Usage: python -I launcher.py <cell> <spawn-parent arguments ...>
import json
import os
import subprocess
import sys
import time

ROOT = r'<lab folder>'
cell, args = sys.argv[1], sys.argv[2:]
launch = json.load(open(os.path.join(ROOT, 'launch-env.json'), encoding='utf-8'))
env = {k: v for k, v in os.environ.items() if not k.startswith('BRIDGE_')}
env.update({k: str(v) for k, v in launch.items()})
cmd = ['powershell', '-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', os.path.join(ROOT, 'requests', 'spawn-parent.ps1')] + args
log = open(os.path.join(ROOT, 'evidence', cell + '.launcher.txt'), 'w', encoding='utf-8', newline='\n')
log.write(time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()) + f' launcher pid {os.getpid()} parent {os.getppid()} cwd {ROOT} keys {sorted(launch)}\n')
log.write('command: ' + subprocess.list2cmdline(cmd) + '\n')
log.flush()
stderr_file = open(os.path.join(ROOT, 'evidence', cell + '.launcher-stderr.txt'), 'wb')
p = subprocess.Popen(cmd, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=stderr_file, env=env, cwd=ROOT)
log.write(time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()) + f' started pid {p.pid}\n')
log.flush()
p.stdin.close()
out = p.stdout.read()
p.wait()
log.write(time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()) + f' ended exit {p.returncode} stdout {len(out)} B\n')
log.close()
stderr_file.close()
