# Lab helper (2569-R; not product code): one Part H run up to the box reads. Hashes the exe, starts starth.ps1 in
# the background, reads the pid, then boxstate.ps1 (again every 2 s up to 10 s while it reads windows 0),
# readwin.ps1 and one whole-screen capture. Answers nothing.
# Usage: powershell -NoProfile -File hrun.ps1 -N <h1|h2|h3> -Build <old|new> -Show <1|7>
param([string]$N, [string]$Build, [int]$Show)
$r = '<lab folder>'
$exe = "$r\exe-$Build\mcpb\bin\x86_64-pc-windows-msvc\bridge_mcp.exe"
"exe sha256 " + (Get-FileHash -LiteralPath $exe -Algorithm SHA256).Hash.ToLower()
$st = "$r\h\$N-starth.txt"
foreach ($f in $st, "$r\h\$N-starth-err.txt", "$r\h\$N-out.txt", "$r\h\$N-err.txt") { if (Test-Path -LiteralPath $f) { throw "exists: $f" } }
Start-Process powershell -WorkingDirectory $r -WindowStyle Hidden `
  -ArgumentList '-NoProfile','-File',"$r\kit\starth.ps1",'-Exe',$exe,'-Show',"$Show",'-In',"$r\kit\H-input.txt",'-OutFile',"$r\h\$N-out.txt",'-ErrFile',"$r\h\$N-err.txt" `
  -RedirectStandardOutput $st -RedirectStandardError "$r\h\$N-starth-err.txt"
$i = 0
while ($i -lt 40 -and -not ((Test-Path -LiteralPath $st) -and (Get-Item -LiteralPath $st).Length -gt 0)) { Start-Sleep -Milliseconds 250; $i++ }
$s = (Get-Content -LiteralPath $st) -join ' '
$s
if ($s -notmatch 'pid=(\d+)') { "no STARTED line"; Get-Content -LiteralPath "$r\h\$N-starth-err.txt"; exit 1 }
$procId = [int]$Matches[1]
Start-Sleep -Seconds 2
for ($k = 1; $k -le 6; $k++) {
  $o = & powershell -NoProfile -File "$r\kit\boxstate.ps1" -ProcId $procId -Out "$r\h\$N-boxstate-$k.txt"
  $o
  if (($o -join "`n") -notmatch '(?m)^windows 0$') { break }
  Start-Sleep -Seconds 2
}
& powershell -NoProfile -File "$r\kit\readwin.ps1" -ProcId $procId -Out "$r\h\$N-readwin.txt"
& powershell -NoProfile -File "$r\capscreen.ps1" -Name "$N-screen"
