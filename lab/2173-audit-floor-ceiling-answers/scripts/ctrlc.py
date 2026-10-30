# Lab helper (not product code): send Ctrl+C to the console of one process (the relay's python), with no key typed:
# detach from our own console, attach to the target's, ignore Ctrl+C ourselves, raise CTRL_C_EVENT for that console.
# Usage: python -I ctrlc.py <pid> [event: 0 Ctrl+C (default), 1 Ctrl+Break]
import ctypes, sys
k = ctypes.windll.kernel32
pid = int(sys.argv[1])
k.FreeConsole()
if not k.AttachConsole(pid): print('attach failed', ctypes.GetLastError()); sys.exit(1)
k.SetConsoleCtrlHandler(None, True)
ok = k.GenerateConsoleCtrlEvent(int(sys.argv[2]) if len(sys.argv) > 2 else 0, 0)
k.FreeConsole()
sys.exit(0 if ok else 2)
