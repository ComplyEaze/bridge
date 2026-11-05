# Lab helper (852-C2, not product code): sends CTRL_C to the console of one relay process only, after checking that the
# console holds no process outside the allowed set (the relay, its PowerShell host and cmd). This helper ignores the event
# itself. Usage: python -I ctrlc.py <relay pid> <allowed pid,...>
import ctypes, sys
k = ctypes.windll.kernel32; pid = int(sys.argv[1]); allowed = {int(x) for x in sys.argv[2].split(',')} | {pid}
k.FreeConsole()
if not k.AttachConsole(pid): print('AttachConsole failed', ctypes.GetLastError()); sys.exit(1)
arr = (ctypes.c_uint32 * 64)(); n = k.GetConsoleProcessList(arr, 64); pids = set(arr[:n]) - {k.GetCurrentProcessId()}
if not pids <= allowed: k.FreeConsole(); print('console holds other processes:', sorted(pids - allowed), '- nothing sent'); sys.exit(2)
k.SetConsoleCtrlHandler(None, True)
ok = k.GenerateConsoleCtrlEvent(0, 0)
k.FreeConsole()
print(f'CTRL_C sent to the console of {pid} ({sorted(pids)}): {bool(ok)}')
