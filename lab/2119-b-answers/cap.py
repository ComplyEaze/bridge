# Lab helper (not product code): capture the whole Tally window (PrintWindow, capwin2.ps1: the lab's capwin.ps1 made DPI-aware from R2.4 on) as shots\<name>.png and
# print the file's size and sha256. Usage: python -I cap.py <name>
import hashlib, subprocess, sys
name = sys.argv[1]; out = '<LAB_DIR>\\shots\\' + name + '.png'
h = subprocess.run(['powershell', '-NoProfile', '-Command', '[int64](Get-Process tally | Select-Object -First 1).MainWindowHandle'], capture_output=True, text=True).stdout.strip()
r = subprocess.run(['powershell', '-NoProfile', '-File', '<LAB_DIR>\\capwin2.ps1', '-Handle', h, '-Out', out], capture_output=True, text=True)
d = open(out, 'rb').read()
print(f'capture {name}.png {len(d)} B sha256 {hashlib.sha256(d).hexdigest()}')
