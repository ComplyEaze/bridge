powershell -NoProfile -NonInteractive -ExecutionPolicy Bypass -File C:\bridge-lab\1584\hold-lock.ps1 -Path C:\bridge-lab\1584\scratch.lock -Mode Exclusive -Seconds 900 -Log C:\bridge-lab\1584\evidence\KH-out-hold.log -StopFile C:\bridge-lab\1584\stop-KH-out.txt
echo %ERRORLEVEL% > C:\bridge-lab\1584\evidence\KH-hold-out.exit
