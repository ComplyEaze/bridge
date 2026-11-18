@echo off
rem Lab helper (2074-2214 W row, 18 Nov; not product code): starts the installed desktop app from its real path (outside
rem the package view), then writes the running bridge.exe process path and its file sha256 to app-start-out.txt.
set O=<lab folder>\evidence\app-start-out.txt
echo app start at %DATE% %TIME% > %O%
start "" "<local app data>\Bridge\bridge.exe"
ping -n 9 127.0.0.1 > nul
powershell -NoProfile -Command "Get-CimInstance Win32_Process -Filter \"Name='bridge.exe'\" | ForEach-Object { 'pid ' + $_.ProcessId + ' parent ' + $_.ParentProcessId + ' path ' + $_.ExecutablePath; (Get-FileHash -Algorithm SHA256 -LiteralPath $_.ExecutablePath).Hash.ToLower() }" >> %O%
echo done >> %O%
