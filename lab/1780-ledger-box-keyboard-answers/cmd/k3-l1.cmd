powershell -NoProfile -Command Start-Sleep -Seconds 8
powershell -NoProfile -ExecutionPolicy Bypass -File <LAB_DIR>\requests\spawn-parent.ps1 -Target child -ChildExe <LAB_DIR>\build\bin\x86_64-pc-windows-msvc\bridge_mcp.exe -ChildMode journal -InputFile <LAB_DIR>\requests\confirm-input.txt -Stdio files -Out <LAB_DIR>\evidence\k3-l1.txt -TimeoutSeconds 60
