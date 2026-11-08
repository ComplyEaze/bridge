powershell -NoProfile -NonInteractive -ExecutionPolicy Bypass -File ROOT\probe.ps1 -Side out -Step K1bc2 -Family <family> -Root ROOT -WriteNonce f4c872bd
echo %ERRORLEVEL% > ROOT\evidence\K1bc2-out.exit
