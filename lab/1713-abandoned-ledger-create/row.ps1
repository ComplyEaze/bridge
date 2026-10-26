# One row of the #1713 abandoned-create sheet, sent by script so its times are exact (README, "Rows L1 to L5").
# Windows PowerShell 5.1. Run from this folder:  powershell -NoProfile -File .\row.ps1 -Row L2 [-DryRun]
# -DryRun prints the plan and sends nothing. Every curl.exe runs in its own job; each job records its own
# start and end time (UTC, milliseconds) and exit code into times.txt, so the order of answers is read from the log.
param(
    [Parameter(Mandatory = $true)][ValidateSet('L1', 'L1B', 'L2', 'L3', 'L4', 'L5')][string]$Row,
    [string]$Evidence = 'C:\bridge-lab\1713\evidence',
    [switch]$DryRun
)
$ErrorActionPreference = 'Stop'
$here = Split-Path -Parent $MyInvocation.MyCommand.Path
$requests = Join-Path $here 'requests'
$url = 'http://127.0.0.1:9001'
$dir = Join-Path $Evidence $Row
$create = @{ L1 = 'l1-create.xml'; L1B = 'l1b-create.xml'; L2 = 'l2-create.xml'; L3 = 'l3-create.xml';
             L4 = 'l4-create.xml'; L5 = 'l5-create.xml' }[$Row]
# A is the busy read; it is abandoned at 20 s in L4 and L5 only.
$aLimit = if ($Row -in @('L4', 'L5')) { 20 } else { 0 }
# B is the create; it is abandoned at 20 s (Bridge's own limit) in L2 to L5.
$bLimit = if ($Row -in @('L1', 'L1B')) { 0 } else { 20 }

$plan = @(
    "row $Row; evidence $dir",
    "A: busy-year.xml, limit $aLimit (0 = none), at T0",
    $(if ($Row -eq 'L5') { "B: $create, limit $bLimit, when A has exited" } else { "B: $create, limit $bLimit, at T0 + 5 s" }),
    $(if ($Row -eq 'L1') { 'C: none' } elseif ($Row -eq 'L1B') { 'C: ledgers.xml, no limit, at T0 + 10 s' }
      else { 'C: ledgers.xml, no limit, the moment B has exited; re-sent every 10 s, up to 30 times, until one is answered' }),
    $(if ($Row -in @('L1', 'L1B')) { 'observation: none (LEDGERS and MARKS by hand after the row)' }
      else { 'observation: LEDGERS, MARKS and netstat at C answered + 1, + 5, + 15 and + 30 minutes' })
)
if ($DryRun) { $plan | ForEach-Object { Write-Output $_ }; exit 0 }

if (Test-Path $dir) { throw "STOP: the evidence folder $dir already exists" }
New-Item -ItemType Directory -Path $dir | Out-Null
$log = Join-Path $dir 'times.txt'
$plan | Set-Content -Encoding UTF8 $log

function Note([string]$what) {
    ('{0} {1}' -f [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ss.fffZ'), $what) | Add-Content -Encoding UTF8 $log
}

function Start-Send([string]$file, [string]$name, [int]$limit) {
    $curlArgs = @('-sS', '-w', '%{http_code} %{size_download} %{time_connect} %{time_starttransfer} %{time_total}',
        '--data-binary', ('@' + (Join-Path $requests $file)), '-H', 'Content-Type: text/xml; charset=utf-16',
        '-D', (Join-Path $dir "$name.headers.txt"), '-o', (Join-Path $dir "$name.xml"))
    if ($limit -gt 0) { $curlArgs += @('--max-time', "$limit") }
    $curlArgs += $url
    Note "queue $name ($file, limit $limit)"
    Start-Job -ScriptBlock {
        param($argv)
        $start = [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ss.fffZ')
        $printed = & curl.exe @argv 2>&1
        $code = $LASTEXITCODE
        $end = [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ss.fffZ')
        [pscustomobject]@{ Start = $start; End = $end; Code = $code; Printed = ($printed -join ' ') }
    } -ArgumentList (, $curlArgs)
}

function Finish($job, [string]$name) {
    $result = Receive-Job -Job $job -Wait
    Remove-Job -Job $job
    ('{0} start {1}' -f $result.Start, $name) | Add-Content -Encoding UTF8 $log
    ('{0} end {1} exit={2} curl: {3}' -f $result.End, $name, $result.Code, $result.Printed) | Add-Content -Encoding UTF8 $log
    $result
}

function Net([string]$tag) {
    Note "netstat $tag"
    netstat -ano | Out-File -Encoding UTF8 (Join-Path $dir "netstat-$tag.txt")
}

function Read-State([string]$tag) {
    Finish (Start-Send 'ledgers.xml' "ledgers-$tag" 0) "ledgers-$tag" | Out-Null
    Finish (Start-Send 'marks.xml' "marks-$tag" 0) "marks-$tag" | Out-Null
    Net $tag
}

Net 'before'
$a = Start-Send 'busy-year.xml' 'A' $aLimit
if ($Row -eq 'L5') {
    $aResult = Finish $a 'A'
    Net 'a-exit'
    $b = Start-Send $create 'B' $bLimit
} else {
    Start-Sleep -Seconds 5
    $b = Start-Send $create 'B' $bLimit
}
if ($Row -eq 'L1B') {
    Start-Sleep -Seconds 5
    $c = Start-Send 'ledgers.xml' 'C' 0
    Finish $b 'B' | Out-Null
    Finish $c 'C' | Out-Null
    Finish $a 'A' | Out-Null
    Note 'row done: read the answer order from the end lines above'
    exit 0
}
$bResult = Finish $b 'B'
Net 'b-exit'
if ($Row -eq 'L1') {
    Finish $a 'A' | Out-Null
    Note 'row done'
    exit 0
}
if ($bResult.Code -eq 0) {
    Note 'VOID: B was answered within its limit, so nothing was abandoned; no C is sent'
    if ($Row -ne 'L5') { Finish $a 'A' | Out-Null }
    exit 0
}
$answered = $false
for ($try = 1; $try -le 30 -and -not $answered; $try++) {
    $cResult = Finish (Start-Send 'ledgers.xml' "C$try" 0) "C$try"
    if ($cResult.Code -eq 0) { $answered = $true; $cAnswered = [DateTime]::UtcNow; Note "C is C$try" }
    else { Start-Sleep -Seconds 10 }
}
if ($Row -ne 'L5') {
    $aResult = Finish $a 'A'
    Net 'a-end'
}
if (-not $answered) { Note 'STOP: no C was answered in 30 tries'; exit 1 }
# Observation is timed from C's answer, and nothing is sent while C is unanswered.
foreach ($minutes in @(1, 5, 15, 30)) {
    $due = $cAnswered.AddMinutes($minutes)
    while ([DateTime]::UtcNow -lt $due) { Start-Sleep -Seconds 5 }
    Read-State "plus$minutes"
}
Note 'row done'
