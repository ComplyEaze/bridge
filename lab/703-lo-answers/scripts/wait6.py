# Lab helper (row 703-LO step 6, not product code): 60 s wait, a window capture at 30 s and 60 s, then one GET /status.
import os, subprocess, time
OUT = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), 'shots') + os.sep  # the row's shots folder
def shot(n):
    h = subprocess.run(['powershell', '-NoProfile', '-Command', '[int64](Get-Process tally | Select-Object -First 1).MainWindowHandle'], capture_output=True, text=True).stdout.strip()
    r = subprocess.run(['powershell', '-NoProfile', '-File', '<lab root>/rc070/tools/capwin.ps1', '-Handle', h, '-Out', OUT + n + '.png'], capture_output=True, text=True)
    print(n, (r.stdout.strip().splitlines() or ['?'])[-1][:40], flush=True)
t0 = time.time()
time.sleep(30); shot('s6-t030'); time.sleep(max(0, 60 - (time.time() - t0))); shot('s6-t060')
a = time.time(); r = subprocess.run(['curl', '-s', '-o', 'NUL', '-w', '%{http_code} %{size_download}', 'http://127.0.0.1:9001/status'], capture_output=True, text=True); b = time.time()
line = (f"{time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime(b))} step 6: waited 60 s from {time.strftime('%H:%M:%SZ', time.gmtime(t0))}, captures "
        f"shots/s6-t030.png and shots/s6-t060.png (no click, no key); GET /status: {r.stdout} in {b - a:.3f} s.")
print(line); open('evidence/notes.txt', 'a', encoding='utf-8', newline='\n').write(line + '\n')
