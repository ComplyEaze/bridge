# Lab helper (#1584, not product code): writes run\<Name>.cmd as ASCII (the step's lines, then the exit line the sheet
# names), records its text in notes-1584.txt, and runs it: -Side in = cmd /c from this (the lab) shell; -Side out = the
# WMI method (Invoke-CimMethod Win32_Process Create 'cmd.exe /c <path>'), the Run box being out of reach for this session.
# -Wait waits for evidence\<Name>.exit (up to -Timeout seconds) and prints it; without -Wait (a client or a holder that
# keeps running) it returns at once.
# Usage: powershell -NoProfile -File step.ps1 -Name <step-side> -Side in|out -Lines @('...','...') [-Wait] [-Timeout 600]
param(
    [Parameter(Mandatory = $true)][string]$Name,
    [Parameter(Mandatory = $true)][ValidateSet('in', 'out')][string]$Side,
    [Parameter(Mandatory = $true)][string[]]$Lines,
    [switch]$Wait,
    [int]$Timeout = 600
)
$W = 'C:\bridge-lab\1584'
$cmd = Join-Path $W ('run\{0}.cmd' -f $Name)
$exit = Join-Path $W ('evidence\{0}.exit' -f $Name)
if ([IO.File]::Exists($cmd)) { Write-Output "stop: $cmd exists; nothing run"; exit 2 }
$text = ($Lines + ('echo %ERRORLEVEL% > {0}' -f $exit)) -join "`r`n"
[IO.File]::WriteAllText($cmd, $text + "`r`n", [Text.Encoding]::ASCII)
$t0 = [DateTime]::UtcNow.ToString('o')
$how = ''
if ($Side -eq 'in') {
    $how = 'cmd /c from the lab shell'
    if ($Wait) { & cmd.exe /c $cmd } else { Start-Process -FilePath cmd.exe -ArgumentList '/c', $cmd -WindowStyle Hidden | Out-Null }
} else {
    $r = Invoke-CimMethod -ClassName Win32_Process -MethodName Create -Arguments @{ CommandLine = ('cmd.exe /c {0}' -f $cmd) }
    $how = ('WMI Win32_Process Create, ReturnValue {0}, ProcessId {1}' -f $r.ReturnValue, $r.ProcessId)
}
Add-Content -Path (Join-Path $W 'notes-1584.txt') -Encoding UTF8 -Value ("{0} {1} ({2}): run\{1}.cmd =`n  {3}" -f $t0, $Name, $how, ($text -replace "`r`n", "`n  "))
Write-Output "$t0 $Name started ($how)"
if ($Wait) {
    $deadline = [DateTime]::UtcNow.AddSeconds($Timeout)
    while (-not [IO.File]::Exists($exit) -and [DateTime]::UtcNow -lt $deadline) { Start-Sleep -Milliseconds 300 }
    Start-Sleep -Milliseconds 300
    if ([IO.File]::Exists($exit)) { $e = (Get-Content $exit -Raw).Trim(); Write-Output "exit $e"; Add-Content -Path (Join-Path $W 'notes-1584.txt') -Encoding UTF8 -Value ("  exit {0} at {1}" -f $e, [DateTime]::UtcNow.ToString('o')) }
    else { Write-Output "no exit file within $Timeout s"; Add-Content -Path (Join-Path $W 'notes-1584.txt') -Encoding UTF8 -Value "  no exit file within $Timeout s" }
}
