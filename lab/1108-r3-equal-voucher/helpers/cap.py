# Lab helper (1108-R3, this file of 1108-R (sha256 dce49834...) with its row folder C:/bridge-lab/1108r3 and sheet folder lab/1108-r3 in place of C:/bridge-lab/1108r and lab/1108-r; that file: 1108-R, from 1108-A2's cap.py with its folder changed; that from 1108-A1, from 1381-M, copied from 692's with its folder changed; that one copied from 2162-S13a2's with its folder changed; that one copied from 2162-S13's with its folder changed; copied from 2162-S5's with its folder changed; not product code): capture the whole Tally window (PrintWindow, capwin2.ps1: the lab's capwin.ps1 made DPI-aware from R2.4 on) as shots\<name>.png and
# print the file's size and sha256. Usage: python -I cap.py <name>
import hashlib, subprocess, sys
name = sys.argv[1]; out = 'C:\\bridge-lab\\1108r3\\shots\\' + name + '.png'
h = subprocess.run(['powershell', '-NoProfile', '-Command', '[int64](Get-Process tally | Select-Object -First 1).MainWindowHandle'], capture_output=True, text=True).stdout.strip()
r = subprocess.run(['powershell', '-NoProfile', '-File', 'C:\\bridge-lab\\1108r3\\capwin2.ps1', '-Handle', h, '-Out', out], capture_output=True, text=True)
d = open(out, 'rb').read()
print(f'capture {name}.png {len(d)} B sha256 {hashlib.sha256(d).hexdigest()}')
