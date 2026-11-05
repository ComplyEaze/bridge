# Lab helper (852-C2, not product code): lists the processes attached to the console of one process, without sending
# anything to them: FreeConsole, AttachConsole(pid), GetConsoleProcessList, FreeConsole. Prints pid and image name.
# Usage: python -I console-of.py <pid>
import ctypes, subprocess, sys
k = ctypes.windll.kernel32; pid = int(sys.argv[1])
k.FreeConsole()
if not k.AttachConsole(pid): print('AttachConsole failed', ctypes.GetLastError()); sys.exit(1)
arr = (ctypes.c_uint32 * 64)(); n = k.GetConsoleProcessList(arr, 64); pids = list(arr[:n]); k.FreeConsole()
names = {}
out = subprocess.run(['tasklist', '/FO', 'CSV', '/NH'], capture_output=True, text=True).stdout
for l in out.splitlines():
    p = l.strip('"').split('","')
    if len(p) > 1 and p[1].isdigit(): names[int(p[1])] = p[0]
print(f'console of {pid}: {n} process(es):', [(p, names.get(p, '?')) for p in pids])
