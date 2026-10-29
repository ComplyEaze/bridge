# Lab helper (#1780-D B5/C6): lists every process whose command line holds the confirm-journal flag; reads only.
$flag = "--confirm-" + "journal"
$s = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ss.fffZ")
$c = @(Get-CimInstance Win32_Process | Where-Object { $_.CommandLine -and $_.CommandLine.Contains($flag) })
"$s processes with the flag: $($c.Count)"
$c | ForEach-Object { "pid $($_.ProcessId) parent $($_.ParentProcessId) start $($_.CreationDate) path $($_.ExecutablePath) cmd $($_.CommandLine)" }
$b = @(Get-CimInstance Win32_Process | Where-Object { $_.Name -eq "bridge_mcp.exe" })
$b | ForEach-Object { "bridge_mcp.exe pid $($_.ProcessId) parent $($_.ParentProcessId) path $($_.ExecutablePath)" }
