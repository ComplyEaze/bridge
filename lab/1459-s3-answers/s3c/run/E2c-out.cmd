powershell -NoProfile -NonInteractive -ExecutionPolicy Bypass -File ROOT\probe.ps1 -Side out -Step E2c -Family <family> -Root ROOT -ExtraFile ROOT\run\e2-paths.txt
echo %ERRORLEVEL% > ROOT\evidence\E2c-out.exit
