powershell -NoProfile -NonInteractive -ExecutionPolicy Bypass -File C:\bridge-lab\1584\probe.ps1 -Side out -Step K1b -Family <family> -WriteNonce 21f2fd7b -ExtraFile C:\bridge-lab\1584\run\python.txt > C:\bridge-lab\1584\evidence\K1b-out.out 2>&1
echo %ERRORLEVEL% > C:\bridge-lab\1584\evidence\K1b-out.exit
