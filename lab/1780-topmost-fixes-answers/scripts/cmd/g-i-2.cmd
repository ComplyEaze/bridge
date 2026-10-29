powershell -NoProfile -Command Start-Sleep -Seconds 8
powershell -NoProfile -ExecutionPolicy Bypass -File <lab folder>\requests\spawn-parent.ps1 -Target control -ControlScript <lab folder>\requests\control-box-fixes.ps1 -ControlVariant i -Stdio tokio -Out <lab folder>\evidence\g-i-2.txt -TimeoutSeconds 60
