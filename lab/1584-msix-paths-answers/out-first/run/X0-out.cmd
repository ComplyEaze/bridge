powershell -NoProfile -NonInteractive -ExecutionPolicy Bypass -File C:\bridge-lab\1584\probe.ps1 -Side out -Step X0 -Family <family> -Key 45354b5d45e8c1b4f7889a4222bf8210044f23f98a2206262f922ae6a8968313 > C:\bridge-lab\1584\evidence\X0-out.out 2>&1
echo %ERRORLEVEL% > C:\bridge-lab\1584\evidence\X0-out.exit
