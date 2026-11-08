powershell -NoProfile -NonInteractive -ExecutionPolicy Bypass -File ROOT\probe.ps1 -Side out -Step K1bc -Family <family> -Root ROOT -WriteNonce d9cb26f9
echo %ERRORLEVEL% > ROOT\evidence\K1bc-out.exit
