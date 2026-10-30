powershell -NoProfile -NonInteractive -ExecutionPolicy Bypass -File C:\bridge-lab\1584\probe.ps1 -Side in -Step K1a -Family <family> -WriteNonce 107fe768 > C:\bridge-lab\1584\evidence\K1a-in.out 2>&1
echo %ERRORLEVEL% > C:\bridge-lab\1584\evidence\K1a-in.exit
