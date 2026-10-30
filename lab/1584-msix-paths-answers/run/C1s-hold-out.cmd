powershell -NoProfile -NonInteractive -ExecutionPolicy Bypass -File C:\bridge-lab\1584\hold-lock.ps1 -Path "<home>\.complyeaze-bridge\coordination\native-dispatch-leases$K.wire.lock" -Mode Exclusive -Seconds 900 -Log C:\bridge-lab\1584\evidence\C1s-out-hold.log -StopFile C:\bridge-lab\1584\stop-C1s-out.txt
echo %ERRORLEVEL% > C:\bridge-lab\1584\evidence\C1s-hold-out.exit
