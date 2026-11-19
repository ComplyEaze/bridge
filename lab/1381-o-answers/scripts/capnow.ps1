# Lab helper (1381-O, not product code): captures the one tally.exe window with capwin2.ps1 (DPI-aware PrintWindow)
# to shots\<name>.png, never overwriting. Usage: powershell -NoProfile -File capnow.ps1 -Name <name>
param([string]$Name)
$out = '<LAB_DIR>\shots\' + $Name + '.png'
if (Test-Path $out) { throw "exists: $out" }
$tp = Get-Process tally | Where-Object { $_.MainWindowHandle -ne 0 } | Select-Object -First 1
powershell -NoProfile -File <lab folder>\1211-evidence\capwin2.ps1 -Handle ([int64]$tp.MainWindowHandle) -Out $out | Out-Null
"$out " + (Get-FileHash -Algorithm SHA256 $out).Hash.ToLower()
