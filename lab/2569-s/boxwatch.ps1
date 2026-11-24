# Lab helper (row 2569-S; not product code). ASCII. Read-only: waits for the server's approval helper (its child
# whose command line holds --confirm-journal), then takes the box's first reads with no act in between:
#   R1  boxstate.ps1 on the helper, again every 2 s up to 10 s while it reads "windows 0" and the helper runs
#   Q2h startinfo.ps1 -Mode process on the helper (its start-up block and every top-level window)
#   R1b boxstate.ps1 on the helper again (a control: the same box, no screen act since R1)
# It clicks nothing, sends no key, moves no window and ends no process.
# Usage: powershell -NoProfile -File boxwatch.ps1 -ServerPid <pid> -Dir <new empty folder> -Tag <launch> [-WaitS 90]
# Writes <Dir>\<Tag>-watch.txt (the log) and the read files named there; never overwrites a file.
# Exit: 0 first reads taken; 1 no helper appeared within -WaitS (a result); 2 the watch or a read failed, the
# server ended, or the helper ended before its box was read (each named in the log; never reported as "no box").
param([int]$ServerPid, [string]$Dir, [string]$Tag, [int]$WaitS = 90)
$ErrorActionPreference = 'Stop'
$Dir = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($Dir)
$log = Join-Path $Dir ($Tag + '-watch.txt')
if (Test-Path -LiteralPath $log) { throw "exists: $log" }
$shell = Join-Path $PSHOME 'powershell.exe'
function Now { [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
function Log([string]$line) { Add-Content -LiteralPath $log -Value ((Now) + ' ' + $line) -Encoding Ascii }
function Alive([int]$id) { return $null -ne (Get-Process -Id $id -ErrorAction SilentlyContinue) }
# A child's error output must not end this watch (Windows PowerShell 5.1 turns it into error records): Continue here.
function Run-Child([string]$script, [string[]]$rest) {
  $ErrorActionPreference = 'Continue'
  & $shell -NoProfile -File (Join-Path $PSScriptRoot $script) @rest 2>&1 | Out-Null
  return $LASTEXITCODE
}
function Read-Box([string]$name) {
  $file = Join-Path $Dir $name
  $code = Run-Child 'boxstate.ps1' @('-ProcId', [string]$script:helper, '-Out', $file)
  if ($code -ne 0 -or -not (Test-Path -LiteralPath $file)) { Log ($name + ' BOXSTATE_FAILED exit ' + $code); return -1 }
  $count = @(Get-Content -LiteralPath $file | Where-Object { $_ -match '^windows [0-9]+$' })
  if ($count.Count -ne 1) { Log ($name + ' BOXSTATE_FAILED no windows line'); return -1 }
  $n = [int]($count[0].Substring(8))
  $box = @(Get-Content -LiteralPath $file | Where-Object { $_.StartsWith('WINDOW ') } | ForEach-Object {
    if ($_ -match ' iconic=(\S+) visible=(\S+) ') { 'iconic=' + $matches[1] + ' visible=' + $matches[2] } })
  Log ($name + ' windows ' + $n + ' ' + ($box -join '; '))
  return $n
}
Log ('watch start server pid ' + $ServerPid + ' wait ' + $WaitS + ' s')
$deadline = [DateTime]::UtcNow.AddSeconds($WaitS)
$failures = 0
$polls = 0
$script:helper = 0
while ([DateTime]::UtcNow -lt $deadline) {
  if (-not (Alive $ServerPid)) { Log 'SERVER_GONE before a helper appeared'; exit 2 }
  try {
    $kids = @(Get-CimInstance -ClassName Win32_Process -Filter ('ParentProcessId = ' + $ServerPid) -ErrorAction Stop)
  } catch {
    $failures++
    Log ('CIM_FAILED ' + $failures + ': ' + $_.Exception.GetType().Name)
    if ($failures -ge 3) { Log 'WATCH_FAILED: three process queries in a row failed'; exit 2 }
    Start-Sleep -Milliseconds 500
    continue
  }
  $failures = 0
  $polls++
  $hit = @($kids | Where-Object { $_.CommandLine -like '*--confirm-journal*' })
  if ($hit.Count -eq 0) { Start-Sleep -Milliseconds 250; continue }
  $script:helper = [int]$hit[0].ProcessId
  $cmd = [string]$hit[0].CommandLine
  if ($cmd.Length -gt 300) { $cmd = $cmd.Substring(0, 300) + '...' }
  Log ('HELPER pid ' + $script:helper + ' created ' + $hit[0].CreationDate.ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ') +
    ' polls ' + $polls + ' children ' + $kids.Count + ' helpers ' + $hit.Count + ' cmdline [' + $cmd + ']')
  break
}
if ($script:helper -eq 0) { Log ('NO_HELPER within ' + $WaitS + ' s, polls ' + $polls + ' (the queries succeeded)'); exit 1 }
$graded = ''
for ($i = 1; $i -le 6; $i++) {
  $n = Read-Box ($Tag + '-R1-' + $i + '.txt')
  if ($n -lt 0) { exit 2 }
  if ($n -gt 0) { $graded = $Tag + '-R1-' + $i + '.txt'; break }
  if (-not (Alive $script:helper)) { Log 'HELPER_GONE before its box was read'; exit 2 }
  if ($i -lt 6) { Start-Sleep -Seconds 2 }
}
if ($graded -eq '') { Log 'R1 NO_WINDOW within 10 s while the helper ran' } else { Log ('R1 graded ' + $graded) }
$q = Join-Path $Dir ($Tag + '-Q2h.txt')
$code = Run-Child 'startinfo.ps1' @('-Mode', 'process', '-ProcId', [string]$script:helper, '-Out', $q)
Log ('Q2h startinfo exit ' + $code + ' file ' + (Split-Path -Leaf $q))
$n = Read-Box ($Tag + '-R1b.txt')
if ($n -lt 0) { exit 2 }
Log ('FIRST READS DONE helper pid ' + $script:helper)
exit 0
