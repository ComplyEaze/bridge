# Lab helper (#2119 B row setup, not product code): quits the desktop client fully and starts it again, on the owner's word of
# 29 Oct 2026 ("you do it on my behalf"). Run detached; it logs to restart-log.txt. After the app's processes are gone it ends
# only orphaned bridge_mcp.exe processes of the two entries the row turns off (the installed extension and the master-fd6f7c0d
# build), which the quit leaves without a parent; nothing else is ended. Then it starts the app by its AppID.
param([int]$DelaySeconds = 20, [string]$Tag = 'setup')
$E = '<lab folder>'; $log = "$E\restart-log.txt"
function L($s) { Add-Content -Encoding utf8 $log "$((Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ')) $Tag $s" }
L "start; waiting $DelaySeconds s"
Start-Sleep -Seconds $DelaySeconds
$c = @(Get-Process <desktop client> -ErrorAction SilentlyContinue); L "<desktop client> processes before: $($c.Count)"
$c | ForEach-Object { try { $_.CloseMainWindow() | Out-Null } catch {} }
Start-Sleep -Seconds 5
$c = @(Get-Process <desktop client> -ErrorAction SilentlyContinue); if ($c.Count) { $c | Stop-Process -Force -ErrorAction SilentlyContinue; L "ended $($c.Count) <desktop client> still running after the close request" }
Start-Sleep -Seconds 8
L "<desktop client> left: $(@(Get-Process <desktop client> -ErrorAction SilentlyContinue).Count)"
$orph = @(Get-CimInstance Win32_Process -Filter "Name='bridge_mcp.exe'" | Where-Object { $_.ExecutablePath -match 'complyeaze-contributors\.bridge-tally|master-fd6f7c0d' })
foreach ($p in $orph) { Stop-Process -Id $p.ProcessId -Force -ErrorAction SilentlyContinue; L "ended orphaned bridge_mcp.exe pid $($p.ProcessId) $($p.ExecutablePath)" }
L "bridge_mcp.exe left: $((@(Get-CimInstance Win32_Process -Filter "Name='bridge_mcp.exe'") | ForEach-Object { "$($_.ProcessId) $($_.ExecutablePath)" }) -join ' | ')"
Start-Process explorer.exe 'shell:AppsFolder\<desktop client AppID>'; L '<desktop client> started by its AppID'
Start-Sleep -Seconds 30
L "after start: <desktop client> $(@(Get-Process <desktop client> -ErrorAction SilentlyContinue).Count); bridge_mcp.exe $((@(Get-CimInstance Win32_Process -Filter "Name='bridge_mcp.exe'") | ForEach-Object { "$($_.ProcessId) $($_.ExecutablePath)" }) -join ' | '); node sleep server $(@(Get-CimInstance Win32_Process -Filter "Name='node.exe'" | Where-Object { $_.CommandLine -match 'mcp-sleep-server' }).Count)"
