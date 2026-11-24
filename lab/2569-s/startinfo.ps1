# Lab helper (row 2569-S; not product code). ASCII. Read-only: loads startinfo.cs (beside this file) with Add-Type
# and writes what it reads to a NEW file (never overwrites). Clicks nothing, sends no key, ends no process.
# Usage (64-bit Windows PowerShell 5.1):
#   powershell -NoProfile -File startinfo.ps1 -Mode process -ProcId <pid> -Out <new file>
#       the start-up block of <pid> (flags, show state, console and standard handles, image, command line),
#       its PE subsystem, its parent pid, and its top-level windows (hidden ones included, at most 20)
#   powershell -NoProfile -File startinfo.ps1 -Mode chain -ProcId <pid> -Out <new file>
#       the same block for <pid> and up to -Depth (default 6) ancestors, each checked for pid reuse
#   powershell -NoProfile -File startinfo.ps1 -Mode console -ProcId <pid> -Out <new file>
#       attaches to <pid>'s console, reads its console window (visible, iconic), detaches; writes nothing to it
#   powershell -NoProfile -File startinfo.ps1 -Mode selftest -Out <new file>
#       ten parse cases on synthetic blocks, then this process against its own start-up information and two
#       ping.exe children started with known values (each left to exit by itself; never ended)
# Exit: 0 read (or self-test passed); 2 the read failed (the file's RESULT line names how); 1 a self-test case wrong.
param([ValidateSet('process', 'chain', 'console', 'selftest')][string]$Mode = 'process', [int]$ProcId = 0,
      [string]$Out, [int]$Depth = 6)
$ErrorActionPreference = 'Stop'
$Out = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($Out)
if (Test-Path -LiteralPath $Out) { throw "exists: $Out" }
$source = [System.IO.File]::ReadAllText((Join-Path $PSScriptRoot 'startinfo.cs'), [System.Text.Encoding]::ASCII)
Add-Type -TypeDefinition $source -Language CSharp -IgnoreWarnings
$lines = New-Object 'System.Collections.Generic.List[string]'
$lines.Add('startinfo mode ' + $Mode + ' read at ' + [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ss.fffZ') + ' pid ' + $ProcId +
  ' host_64bit ' + [Environment]::Is64BitProcess)
$rc = 0
if ($Mode -eq 'process') {
  $r = [StartInfo]::ReadProcess($ProcId)
  $lines.AddRange($r.Lines)
  $lines.AddRange([StartInfo]::Windows($ProcId, 20))
  if (-not $r.Ok) { $rc = 2 }
} elseif ($Mode -eq 'chain') {
  $c = [StartInfo]::ReadChain($ProcId, $Depth)
  $lines.AddRange($c)
  $first = @($c | Where-Object { $_.StartsWith('RESULT') } | Select-Object -First 1)
  if ($first.Count -ne 1 -or $first[0] -ne 'RESULT OK') { $rc = 2 }
} elseif ($Mode -eq 'console') {
  $c = [StartInfo]::ConsoleOf($ProcId)
  $lines.AddRange($c)
  if (@($c | Where-Object { $_.Contains('ATTACH_FAILED') }).Count -gt 0) { $rc = 2 }
} else {
  $f1 = 0
  $f2 = 0
  $lines.AddRange([StartInfo]::ParseSelfTest([ref]$f1))
  $lines.AddRange([StartInfo]::LiveSelfTest([ref]$f2))
  $lines.Add('selftest wrong ' + ($f1 + $f2))
  if (($f1 + $f2) -ne 0) { $rc = 1 }
}
$lines.Add('exit ' + $rc)
[System.IO.File]::WriteAllLines($Out, $lines, (New-Object System.Text.UTF8Encoding $false))
Write-Output ('wrote ' + $Out + ' lines ' + $lines.Count + ' exit ' + $rc)
exit $rc
