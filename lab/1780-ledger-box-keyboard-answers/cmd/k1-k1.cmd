powershell -NoProfile -Command Start-Sleep -Seconds 8
powershell -NoProfile -ExecutionPolicy Bypass -File <LAB_DIR>\requests\spawn-parent.ps1 -Target child -ChildExe <LAB_DIR>\build\bin\x86_64-pc-windows-msvc\bridge_mcp.exe -ChildMode masters -InputFile <LAB_DIR>\requests\confirm-masters-input.txt -Stdio files -Out <LAB_DIR>\evidence\k1-k1.txt -TimeoutSeconds 60
