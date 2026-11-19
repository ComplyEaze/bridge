@echo off
rem Lab helper (2074-2214 W row Close, 18 Nov; not product code): hashes the installed app's two files from the real path
rem (outside the package view) and writes the result to evidence/close-hash-out.txt.
set O=<lab folder>\evidence\close-hash-out.txt
echo close re-hash at %DATE% %TIME% > %O%
certutil -hashfile "<local app data>\Bridge\bridge.exe" SHA256 >> %O%
certutil -hashfile "<local app data>\Bridge\bridge_mcp.exe" SHA256 >> %O%
echo done >> %O%
