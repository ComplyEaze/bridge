powershell -NoProfile -Command Start-Sleep -Seconds 8
powershell -NoProfile -ExecutionPolicy Bypass -File <LAB_DIR>\requests\spawn-parent.ps1 -Target child -ChildExe <BUILD_DIR>\bin\x86_64-pc-windows-msvc\bridge_mcp.exe -InputFile <LAB_DIR>\requests\confirm-input.txt -Stdio files -Out <LAB_DIR>\evidence\k.txt -TimeoutSeconds 60
