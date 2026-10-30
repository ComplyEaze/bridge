powershell -NoProfile -NonInteractive -ExecutionPolicy Bypass -File C:\bridge-lab\1584\probe.ps1 -Side in -Step K1c -Family <family> > C:\bridge-lab\1584\evidence\K1c-in.out 2>&1
echo %ERRORLEVEL% > C:\bridge-lab\1584\evidence\K1c-in.exit
