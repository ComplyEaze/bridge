# Lab helper (#2119-H, not product code; the 29 Oct row's restart-desktop.ps1 with its folder changed and its bridge_mcp step
# dropped): asks the desktop app to close, ends any of its <desktop client exe> processes still running after the request, and (unless
# -CloseOnly) starts it again by its AppID. Run detached; it logs to restart-log.txt. It ends no other process: any
# bridge_mcp.exe or node.exe left after the quit is only recorded. The restart ends the lab's own session, which runs inside the app.
param([int]$DelaySeconds = 20, [string]$Tag = 'setup', [switch]$CloseOnly)
$E = '<lab folder>'; $log = "$E\restart-log.txt"
function L($s) { Add-Content -Encoding utf8 $log "$((Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ')) $Tag $s" }
function Others { "bridge_mcp.exe $((@(Get-CimInstance Win32_Process -Filter "Name='bridge_mcp.exe'") | ForEach-Object { "$($_.ProcessId) parent $($_.ParentProcessId)" }) -join ' | '); node sleep server $((@(Get-CimInstance Win32_Process -Filter "Name='node.exe'" | Where-Object { $_.CommandLine -match 'mcp-sleep-server' }) | ForEach-Object { $_.ProcessId }) -join ' ')" }
L "start; waiting $DelaySeconds s; close only: $([bool]$CloseOnly)"
Start-Sleep -Seconds $DelaySeconds
$c = @(Get-Process the desktop client -ErrorAction SilentlyContinue); L "<desktop client exe> processes before: $($c.Count); $(Others)"
$c | ForEach-Object { try { $_.CloseMainWindow() | Out-Null } catch {} }
L 'close requested (CloseMainWindow on each <desktop client exe>)'
Start-Sleep -Seconds 5
$c = @(Get-Process the desktop client -ErrorAction SilentlyContinue); if ($c.Count) { $c | Stop-Process -Force -ErrorAction SilentlyContinue; L "ended $($c.Count) <desktop client exe> still running after the close request" }
Start-Sleep -Seconds 8
L "<desktop client exe> left: $(@(Get-Process the desktop client -ErrorAction SilentlyContinue).Count); $(Others)"
if ($CloseOnly) { L 'close only: not started'; exit 0 }
Start-Process explorer.exe 'shell:AppsFolder\<desktop AppID>'; L 'the desktop client started by its AppID'
Start-Sleep -Seconds 30
L "after start: <desktop client exe> $(@(Get-Process the desktop client -ErrorAction SilentlyContinue).Count); $(Others)"
