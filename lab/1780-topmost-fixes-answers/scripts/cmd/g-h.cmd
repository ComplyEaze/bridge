powershell -NoProfile -Command Start-Sleep -Seconds 8
powershell -NoProfile -ExecutionPolicy Bypass -File <lab folder>\requests\spawn-parent.ps1 -Target control -ControlScript <lab folder>\requests\control-box-fixes.ps1 -ControlVariant h -Stdio tokio -Out <lab folder>\evidence\g-h.txt -TimeoutSeconds 60
