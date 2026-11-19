@echo off
rem Lab helper (2214-cut P0, not product code): reads the installed app's version strings from the real install path
rem (started through WMI, outside the package view) and writes them to evidence\p0-version-out.txt. Reads only.
set O=<lab folder>\evidence\p0-version-out.txt
echo version read at %DATE% %TIME% > %O%
powershell -NoProfile -Command "foreach ($n in 'bridge.exe','bridge_mcp.exe') { $i = Get-Item (Join-Path $env:LOCAL<app data> ('Bridge\' + $n)); $n + ' ProductVersion ' + $i.VersionInfo.ProductVersion + ' FileVersion ' + $i.VersionInfo.FileVersion + ' bytes ' + $i.Length + ' written ' + $i.LastWriteTimeUtc.ToString('s') + 'Z' }" >> %O%
echo done >> %O%
