# Lab helper (2569-R; not product code): read-only. Waits up to -Wait seconds for the server's child whose command
# line holds --confirm-journal, records it, then boxstate.ps1 (again every 2 s up to 10 s while windows 0),
# readwin.ps1 and one whole-screen capture. Answers nothing.
# Usage: powershell -NoProfile -File bwatch.ps1 -ServerPid <pid> -N <label> -Wait <s>
param([int]$ServerPid, [string]$N, [int]$Wait = 90)
$r = '<lab folder>'
$t0 = Get-Date
$child = $null
while (((Get-Date) - $t0).TotalSeconds -lt $Wait) {
  $child = Get-CimInstance Win32_Process -Filter "ParentProcessId=$ServerPid" | Where-Object { $_.CommandLine -like '*--confirm-journal*' } | Select-Object -First 1
  if ($child) { break }
  Start-Sleep -Milliseconds 300
}
if (-not $child) { "no helper within $Wait s"; exit 1 }
$seen = [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ss.fffZ')
$line = "helper seen at $seen pid " + $child.ProcessId + " cmd [" + $child.CommandLine + "]"
$line | Out-File -FilePath "$r\evidence\$N-helper.txt" -Encoding utf8
$line
Start-Sleep -Milliseconds 1500
for ($k = 1; $k -le 6; $k++) {
  $o = & powershell -NoProfile -File "$r\kit\boxstate.ps1" -ProcId $child.ProcessId -Out "$r\evidence\$N-boxstate-$k.txt"
  $o
  if (($o -join "`n") -notmatch '(?m)^windows 0$') { break }
  Start-Sleep -Seconds 2
}
& powershell -NoProfile -File "$r\kit\readwin.ps1" -ProcId $child.ProcessId -Out "$r\evidence\$N-readwin.txt"
& powershell -NoProfile -File "$r\capscreen.ps1" -Name "$N-screen"
