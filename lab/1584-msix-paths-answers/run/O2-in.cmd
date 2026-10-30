powershell -NoProfile -NonInteractive -ExecutionPolicy Bypass -File C:\bridge-lab\1584\probe.ps1 -Side in -Step O2 -Family <family> -ExtraFile C:\bridge-lab\1584\o1\paths.txt > C:\bridge-lab\1584\evidence\O2-in.out 2>&1
echo %ERRORLEVEL% > C:\bridge-lab\1584\evidence\O2-in.exit
