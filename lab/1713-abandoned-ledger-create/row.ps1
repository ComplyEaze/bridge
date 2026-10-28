# One row of the #1713 abandoned-create sheet (README, "Rows"), sent by script so its times are exact.
# Windows PowerShell 5.1, under the PC's own execution policy (never -ExecutionPolicy Bypass). From this folder:
#   powershell -NoProfile -File .\row.ps1 -Row L2 -DryRun
# A live row runs in its own console window, started as README "Running the scripts" says, never as a foreground call:
#   Start-Process powershell.exe -WorkingDirectory . -ArgumentList '-NoProfile','-File','.\row.ps1','-Row','L2',
#     '-CompanyGuid','<S1''s GUID>','-BusySeconds','<D>','-BusyBytes','<M1''s size>','-BusyRows','<M1''s rows>'
# -DryRun prints the row's five plan lines and sends nothing. Every curl.exe runs in its own job; each job records its
# own start and end time (UTC, milliseconds) and exit code into times.txt, so the order of answers is read from the log.
# Every line written to times.txt is also printed. The script sends nothing more once a file named STOP exists in this
# folder (made by whoever sees a dialog on Tally's screen): it checks before every send and at least once a second
# while it waits, then waits for any curl still running to end by itself and exits. It never ends a curl or a process.
# Exit codes: 0 = row done or VOID; 1 = STOP, a check failed; 3 = STOP, a request's sha256 differs; 4 = STOP, the
# D + 10 minute watchdog; 5 = STOP, the STOP file.
param(
    [Parameter(Mandatory = $true)][ValidateSet('L1', 'L1B', 'L2', 'L3', 'L4', 'L5')][string]$Row,
    [string]$CompanyGuid = '',
    [int]$BusySeconds = 0,
    [long]$BusyBytes = 0,
    [int]$BusyRows = 0,
    [string]$Evidence = 'C:\bridge-lab\1713\evidence',
    [switch]$DryRun
)
$ErrorActionPreference = 'Stop'
$here = Split-Path -Parent $MyInvocation.MyCommand.Path
$requests = Join-Path $here 'requests'
$stopFile = Join-Path $here 'STOP'
$readPy = Join-Path $here 'read.py'
$url = 'http://127.0.0.1:9001'
$dir = Join-Path $Evidence $Row
$create = @{ L1 = 'l1-create.xml'; L1B = 'l1b-create.xml'; L2 = 'l2-create.xml'; L3 = 'l3-create.xml';
             L4 = 'l4-create.xml'; L5 = 'l5-create.xml' }[$Row]
$probe = @{ L1 = 'Lab Probe L1'; L1B = 'Lab Probe L1B'; L2 = 'Lab Probe L2'; L3 = 'Lab Probe L3';
            L4 = 'Lab Probe L4'; L5 = 'Lab Probe L5' }[$Row]
# A is the busy read; it is abandoned at 20 s (LIMIT 20) in L4 and L5 only.
$aLimit = if ($Row -in @('L4', 'L5')) { 20 } else { 0 }
# B is the create; it is abandoned at 20 s (LIMIT 20, Bridge's own limit) in L2 to L5.
$bLimit = if ($Row -in @('L1', 'L1B')) { 0 } else { 20 }
$readBound = 2000000
$busyBound = 400000000

$who = "row.ps1 -Row $Row"
# Every start, and every refusal before the row's own log exists, is also written to <Evidence>\starts.txt, so a
# script started in its own window leaves a record the watcher can read (README, "Running the scripts").
function Starts([string]$what) {
    if ($DryRun) { return }
    try {
        if (-not (Test-Path $Evidence)) { New-Item -ItemType Directory -Path $Evidence | Out-Null }
        $stamp = [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ss.fffZ', [Globalization.CultureInfo]::InvariantCulture)
        Add-Content -Encoding UTF8 -Path (Join-Path $Evidence 'starts.txt') -Value ('{0} {1}: {2}' -f $stamp, $who, $what)
    } catch { }
}

function Refuse([int]$code, [string]$why) {
    Write-Output $why
    Starts $why
    exit $code
}

$plan = @(
    "row $Row; evidence $dir; STOP file: STOP in this script's folder",
    "A: busy-year.xml, limit $aLimit (0 = none), at T0",
    $(if ($Row -eq 'L5') { "B: $create, limit $bLimit, when A's curl has exited; netstat 10 s after B is sent" }
      else { "B: $create, limit $bLimit (0 = none), at T0 + 5 s; netstat 10 s after B is sent" }),
    $(if ($Row -eq 'L1') { 'C: none' }
      elseif ($Row -eq 'L1B') { 'C: ledgers.xml, no limit, at T0 + 26 s; sent again 30 s apart after curl exit 7 until D + 10 minutes after the first C, and at most 5 times after exit 52 or 56' }
      else { 'C: ledgers.xml, no limit, at T0 + 26 s or 1 s after B''s curl has exited, whichever is later; sent again 30 s apart after curl exit 7 until D + 10 minutes after the first C, and at most 5 times after exit 52 or 56' }),
    $(if ($Row -eq 'L1') { 'observation: none (LEDGERS and MARKS by hand after the row)' }
      else { 'observation: LEDGERS, MARKS and netstat at C answered + 1, + 5, + 15 and + 30 minutes' })
)
if ($DryRun) { $plan | ForEach-Object { Write-Output $_ }; exit 0 }

# Nothing is sent, and no folder is made, unless every one of these holds.
if (Test-Path $stopFile) { Refuse 5 "STOP: the STOP file $stopFile is present; nothing sent" }
if ($CompanyGuid -notmatch '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$') {
    Refuse 1 'STOP: -CompanyGuid must be THE COMPANY''s GUID as MARKS recorded it at S1; nothing sent'
}
if ($BusySeconds -lt 40 -or $BusySeconds -gt 300) { Refuse 1 'STOP: -BusySeconds (D, from M1) must be 40 to 300; nothing sent' }
if ($Row -eq 'L5' -and $BusySeconds -lt 55) {
    Refuse 1 'STOP: L5 needs D of at least 55 s, because B is sent only when A has exited; record L5 as not run; nothing sent'
}
if ($BusyBytes -le 0 -or $BusyBytes -gt $busyBound) { Refuse 1 'STOP: -BusyBytes (M1''s size) must be 1 to 400,000,000; nothing sent' }
if ($BusyRows -le 0) { Refuse 1 'STOP: -BusyRows (M1''s VOUCHER rows) must be given; nothing sent' }
$drive = New-Object System.IO.DriveInfo ((Split-Path -Qualifier $Evidence))
$needed = 2 * $BusyBytes + 1073741824
if ($drive.AvailableFreeSpace -lt $needed) {
    Refuse 1 "STOP: $($drive.AvailableFreeSpace) bytes available on the evidence drive, below 2 x M1's size + 1 GB ($needed); nothing sent"
}
if (Test-Path $dir) { Refuse 1 "STOP: the evidence folder $dir already exists; nothing sent" }
if (-not (Get-Command python -ErrorAction SilentlyContinue)) { Refuse 1 'STOP: python is not on PATH, so no answer can be read; nothing sent' }

$expected = @{}
foreach ($line in Get-Content -Path (Join-Path $here 'SHA256.txt')) {
    $parts = $line -split '  ', 2
    if ($parts.Count -eq 2) { $expected[$parts[1]] = $parts[0] }
}

New-Item -ItemType Directory -Path $dir | Out-Null
$log = Join-Path $dir 'times.txt'
$plan | Set-Content -Encoding UTF8 $log
Starts "started; log $log"
$script:running = @{}
$script:queued = @{}
$script:bytes = @{}
$script:halting = $false
$invariant = [Globalization.CultureInfo]::InvariantCulture

function Stamp([DateTime]$when) { $when.ToString('yyyy-MM-ddTHH:mm:ss.fffZ', $invariant) }

function Note([string]$what) {
    $line = '{0} {1}' -f (Stamp ([DateTime]::UtcNow)), $what
    Add-Content -Encoding UTF8 -Path $log -Value $line
    Write-Output $line | Out-Host
}

# Ends the row: logs why, sends nothing more, waits for every curl still running to end by itself, then exits.
function Halt([int]$code, [string]$why) {
    $script:halting = $true
    Note $why
    foreach ($name in @($script:running.Keys)) {
        Note "waiting for $name to end by itself; nothing more is sent"
        Finish $name | Out-Null
    }
    Note 'row ended by STOP: nothing more was sent'
    exit $code
}

function Wait-Until([DateTime]$when) {
    while ([DateTime]::UtcNow -lt $when) {
        if (Test-Path $stopFile) { Halt 5 'STOP: the STOP file is present; nothing more is sent' }
        Start-Sleep -Milliseconds 200
    }
}

function Start-Send([string]$file, [string]$name, [int]$limit) {
    if (Test-Path $stopFile) { Halt 5 "STOP before ${name}: the STOP file is present; $name not sent" }
    $path = Join-Path $requests $file
    $hash = (Get-FileHash -Algorithm SHA256 -Path $path).Hash.ToLower()
    if ($hash -ne $expected["requests/$file"]) { Halt 3 "STOP before ${name}: the sha256 of $file differs from SHA256.txt; $name not sent" }
    $script:bytes[$name] = (Get-Item -Path $path).Length
    $curlArgs = @('-sS', '--noproxy', '*', '-w',
        '%{http_code} %{size_download} %{size_upload} %{time_connect} %{time_starttransfer} %{time_total}',
        '--data-binary', ('@' + $path), '-H', 'Content-Type: text/xml; charset=utf-16',
        '-D', (Join-Path $dir "$name.headers.txt"), '-o', (Join-Path $dir "$name.xml"),
        '--stderr', (Join-Path $dir "$name.curl.err"))
    if ($limit -gt 0) { $curlArgs += @('--max-time', "$limit") }
    $curlArgs += $url
    Note "queue $name ($file, sha256 ok, limit $limit)"
    $script:queued[$name] = [DateTime]::UtcNow
    $script:running[$name] = Start-Job -ScriptBlock {
        param($argv)
        $start = [DateTime]::UtcNow
        $printed = & curl.exe @argv
        $code = $LASTEXITCODE
        $end = [DateTime]::UtcNow
        [pscustomobject]@{ Start = $start.Ticks; End = $end.Ticks; Code = $code; Printed = ("$printed").Trim() }
    } -ArgumentList (, $curlArgs)
}

# Waits for a send to end. At D + 10 minutes after it was queued it logs the watchdog STOP once and keeps waiting:
# the window stays open, nothing is ended, nothing more is sent.
function Finish([string]$name) {
    $job = $script:running[$name]
    $deadline = $script:queued[$name].AddSeconds($BusySeconds + 600)
    $late = $false
    $stopSeen = $false
    while (-not (Wait-Job -Job $job -Timeout 1)) {
        if (-not $late -and [DateTime]::UtcNow -gt $deadline) {
            Note "STOP: $name has not ended D + 10 minutes after it was sent. Leave this window open, press no Ctrl+C and no key in Tally, and report (C9). Nothing more is sent."
            $late = $true
        }
        if (-not $stopSeen -and (Test-Path $stopFile)) {
            Note "the STOP file is present while $name runs; $name is left to end by itself and nothing more is sent"
            $stopSeen = $true
        }
    }
    $result = Receive-Job -Job $job
    Remove-Job -Job $job
    $script:running.Remove($name)
    $fields = "$($result.Printed)" -split ' '
    $r = [pscustomobject]@{ Name = $name; Code = [int]$result.Code; Start = [DateTime]::new([long]$result.Start, 'Utc');
        End = [DateTime]::new([long]$result.End, 'Utc'); Status = ''; Size = -1L; Upload = -1L; Connect = -1.0; Total = -1.0; Length = '';
        StartTransfer = -1.0; FirstByte = $null; File = (Join-Path $dir "$name.xml") }
    try {
        $r.Status = $fields[0]
        $r.Size = [long]$fields[1]
        $r.Upload = [long]$fields[2]
        $r.Connect = [double]::Parse($fields[3], $invariant)
        $r.StartTransfer = [double]::Parse($fields[4], $invariant)
        $r.Total = [double]::Parse($fields[5], $invariant)
    } catch { }
    # The first byte of the answer: the send's start plus curl's time_starttransfer. The start is taken just before curl.exe is
    # launched and the end just after it exits, so the first byte reads early and an end reads late, each by at most
    # (end - start) - time_total (0.128 s for A and 0.089 s for B in L1). The order gates compare an end against the first
    # byte, never against a curl end, because a long body streams between the first byte and the curl end.
    if ($r.StartTransfer -gt 0) { $r.FirstByte = $r.Start.AddSeconds($r.StartTransfer) }
    $headers = Join-Path $dir "$name.headers.txt"
    if (Test-Path $headers) {
        $m = @(Select-String -Path $headers -Pattern '^CONTENT-LENGTH:\s*(\d+)')
        if ($m.Count -gt 0) { $r.Length = $m[$m.Count - 1].Matches[0].Groups[1].Value }
    }
    Add-Content -Encoding UTF8 -Path $log -Value ('{0} start {1}' -f (Stamp $r.Start), $name)
    Note ('end {0} at {1} exit={2} curl: {3} content-length={4}' -f $name, (Stamp $r.End), $r.Code, $result.Printed, $r.Length)
    if ($r.FirstByte) { Note ('first byte {0} at {1} (start + time_starttransfer {2} s)' -f $name, (Stamp $r.FirstByte), $r.StartTransfer.ToString('0.000', $invariant)) }
    if ($late -and -not $script:halting) { Halt 4 "STOP: $name ended only after the D + 10 minute watchdog; nothing more is sent" }
    if ($stopSeen -and -not $script:halting) { Halt 5 'STOP: the STOP file is present; nothing more is sent' }
    $r
}

# C3's checks on an answer that must be complete. Returns '' when they hold, else what failed.
function Test-Answer($r, [long]$bound) {
    if ($r.Code -ne 0) { return "curl exit $($r.Code)" }
    if ($r.Status -ne '200') { return "status $($r.Status)" }
    if ($r.Size -le 0) { return 'an empty body' }
    if ("$($r.Size)" -ne $r.Length) { return "size $($r.Size) differs from CONTENT-LENGTH '$($r.Length)'" }
    if ($r.Size -gt $bound) { return "size $($r.Size) is above its bound $bound" }
    return ''
}

function Read-Check([string[]]$readArgs) {
    try {
        $printed = & python -I $readPy @readArgs
        [pscustomobject]@{ Code = $LASTEXITCODE; Out = ("$printed").Trim() }
    } catch {
        [pscustomobject]@{ Code = 99; Out = "read.py could not be run: $($_.Exception.Message)" }
    }
}

# A complete answer that must pass C3 and read.py, else the row STOPs as a failed request (never "nothing found").
function Require([string]$name, [long]$bound, [string[]]$readArgs) {
    $r = Finish $name
    $why = Test-Answer $r $bound
    if ($why) { Halt 1 "STOP: $name failed ($why): the request failed" }
    if ($readArgs) {
        $check = Read-Check (@($readArgs[0], $r.File) + $readArgs[1..($readArgs.Count - 1)])
        Note "read ${name}: $($check.Out)"
        if ($check.Code -ne 0) { Halt 1 "STOP: $name's answer failed its read: the request failed" }
        $r | Add-Member -NotePropertyName Read -NotePropertyValue $check.Out
    }
    $r
}

function Net([string]$tag) {
    $file = Join-Path $dir "netstat-$tag.txt"
    $out = & netstat.exe -ano
    $code = $LASTEXITCODE
    $out | Out-File -Encoding UTF8 $file
    if ($code -eq 0) { Note "netstat $tag exit=0" } else { Note "netstat $tag exit=${code}: the command failed" }
}

# The abandoned A of L4 and L5: its exit 28 at LIMIT 20, with a connection made, is its result; its partial answer is
# kept and not read (README, difference 9).
function Require-Abandoned($r, [string]$what) {
    if ($r.Code -ne 28 -or $r.Connect -le 0) {
        Halt 1 "STOP: $what ended with curl exit $($r.Code), time_connect $($r.Connect); this row needs exit 28 with a connection made"
    }
}

function Read-State([string]$tag) {
    Start-Send 'ledgers.xml' "ledgers-$tag" 0 | Out-Null
    $l = Require "ledgers-$tag" $readBound @('ledgers', $CompanyGuid, $probe)
    Start-Send 'marks.xml' "marks-$tag" 0 | Out-Null
    Require "marks-$tag" $readBound @('marks', $CompanyGuid) | Out-Null
    Net $tag
    $l
}

# Any error the script did not expect ends the row the same way: nothing more is sent, and a running curl is left to
# end by itself rather than ended with the script.
$script:ready = $true
trap {
    if (-not $script:ready) { Refuse 1 "STOP: the script failed before anything was sent: $($_.Exception.Message)" }
    if ($script:halting) { exit 1 }
    Halt 1 "STOP: the script failed: $($_.Exception.Message)"
}

function Probe-State([string]$read) {
    if ($read -match 'probe=present') { 'present' } elseif ($read -match 'probe=absent') { 'absent' } else { 'unknown' }
}

Note "D $BusySeconds s; M1 $BusyBytes bytes, $BusyRows rows; company $CompanyGuid"
Net 'before'
Start-Send 'busy-year.xml' 'A' $aLimit | Out-Null
$t0 = $script:queued['A']
$aResult = $null
if ($Row -eq 'L5') {
    $aResult = Finish 'A'
    Require-Abandoned $aResult 'A'
    Net 'a-exit'
} else {
    Wait-Until $t0.AddSeconds(5)
}
Start-Send $create 'B' $bLimit | Out-Null
Wait-Until $script:queued['B'].AddSeconds(10)
Net 'b-plus10'

if ($Row -eq 'L1') {
    $bResult = Require 'B' $readBound @('import', '1')
    $aResult = Require 'A' $busyBound $null
    if (-not $aResult.FirstByte) { Halt 1 'STOP: A''s time_starttransfer is missing or zero, so its first byte is unknown; the run coordinator decides on L1B' }
    Note ('ORDER A first byte {0}, A ended {1}, B ended {2}' -f (Stamp $aResult.FirstByte), (Stamp $aResult.End), (Stamp $bResult.End))
    if ($bResult.End -le $aResult.FirstByte) { Halt 1 'STOP: B ended before A''s first byte; the run coordinator decides on L1B' }
    if ($bResult.End -lt $aResult.End) { Note 'NOTE: B ended during A''s stream (after A''s first byte, before A''s curl end)' }
    Note 'ORDER ok: B ended after A''s first byte'
    $busy = Read-Check @('busy', $aResult.File, $CompanyGuid, "$BusyRows")
    Note "read A: $($busy.Out)"
    if ($busy.Code -ne 0) { Halt 1 'STOP: A''s answer failed its read' }
    Note 'row done'
    exit 0
}

$bResult = $null
if ($Row -ne 'L1B') {
    $bResult = Finish 'B'
    Net 'b-exit'
    if ($Row -eq 'L4') {
        $aResult = Finish 'A'
        Require-Abandoned $aResult 'A'
    }
    $void = ''
    $bBytes = $script:bytes['B']
    if ($bResult.Code -eq 28 -and $bResult.Connect -gt 0 -and $bResult.Upload -eq $bBytes) {
        Note "B abandoned at its limit with a connection made and all $bBytes bytes uploaded: the row counts"
    } elseif ($bResult.Code -eq 28 -and $bResult.Connect -gt 0) {
        $void = "VOID-not-sent: B's connection was made but only $($bResult.Upload) of its $bBytes bytes were uploaded"
    } elseif ($bResult.Code -eq 0) {
        $void = 'VOID-answered: B was answered within its limit, so nothing was abandoned'
        $check = Read-Check @('import', $bResult.File, '1')
        Note "read B: $($check.Out)"
    } elseif ($bResult.Code -eq 7 -or ($bResult.Code -eq 28 -and $bResult.Connect -le 0)) {
        $void = 'VOID-not-reached: B never had a connection to Tally (curl exit 7, or 28 with time_connect 0)'
    } elseif ($bResult.Code -in @(52, 56)) {
        $void = "VOID-closed: B's connection was closed with no answer (curl exit $($bResult.Code)), not abandoned at its limit"
    } else {
        Halt 1 "STOP: B ended with curl exit $($bResult.Code), which is not a result of this row"
    }
    if ($void) {
        Note "$void; no C is sent"
        if ($Row -in @('L2', 'L3')) {
            $aResult = Require 'A' $busyBound $null
            $busy = Read-Check @('busy', $aResult.File, $CompanyGuid, "$BusyRows")
            Note "read A: $($busy.Out)"
            if ($busy.Code -ne 0) { Halt 1 'STOP: A''s answer failed its read' }
        }
        Note 'row done (void)'
        exit 0
    }
}

# C's rule is the same in L2 to L5, so L5 differs from L4 only in when B is sent; L1B sends C when L2 would.
$cAt = $t0.AddSeconds(26)
if ($Row -ne 'L1B' -and $bResult.End.AddSeconds(1) -gt $cAt) { $cAt = $bResult.End.AddSeconds(1) }
Wait-Until $cAt
$cResult = $null
# C is sent again only while Tally refuses it (exit 7), 30 s apart, until D + 10 minutes after the first C: well past
# the 523 s of refusals reference 11b.2 measured after an abandoned read. A send that ended 52 or 56 may have reached
# Tally, so those are sent again at most 5 times.
$cDeadline = $null
$closed = 0
$try = 0
while (-not $cResult) {
    $try++
    $name = "C$try"
    Start-Send 'ledgers.xml' $name 0 | Out-Null
    if (-not $cDeadline) { $cDeadline = $script:queued[$name].AddSeconds($BusySeconds + 600) }
    $r = Finish $name
    if ($r.Code -eq 0) {
        $why = Test-Answer $r $readBound
        if ($why) { Halt 1 "STOP: $name failed ($why): the request failed, never 'probe absent'" }
        $check = Read-Check @('ledgers', $r.File, $CompanyGuid, $probe)
        Note "read ${name}: $($check.Out)"
        if ($check.Code -ne 0) { Halt 1 "STOP: $name's answer failed its read: the request failed, never 'probe absent'" }
        $r | Add-Member -NotePropertyName Read -NotePropertyValue $check.Out
        $cResult = $r
        Note "C is $name"
    } elseif ($r.Code -in @(7, 52, 56)) {
        if ($r.Code -ne 7) {
            $closed++
            if ($closed -gt 5) { Halt 1 "STOP: $name is the sixth C to end with exit 52 or 56; no C is sent again" }
        }
        $next = [DateTime]::UtcNow.AddSeconds(30)
        if ($next -gt $cDeadline) {
            Halt 1 'STOP: no C was answered by D + 10 minutes after the first C. C9: do nothing more, report and wait'
        }
        Note "$name not answered (curl exit $($r.Code)); sent again in 30 s"
        Wait-Until $next
    } else {
        Halt 1 "STOP: $name ended with curl exit $($r.Code): the request failed"
    }
}
$cState = Probe-State $cResult.Read

if ($Row -eq 'L1B') {
    $bResult = Require 'B' $readBound @('import', '1')
}
if ($Row -in @('L1B', 'L2', 'L3')) {
    $aResult = Require 'A' $busyBound $null
    Net 'a-end'
    if (-not $aResult.FirstByte) { Halt 1 'STOP: A''s time_starttransfer is missing or zero, so its first byte is unknown; the run coordinator decides' }
    Note ('ORDER A first byte {0}, A ended {1}, C ended {2}' -f (Stamp $aResult.FirstByte), (Stamp $aResult.End), (Stamp $cResult.End))
}
if ($Row -eq 'L1B') {
    Note ('ORDER B ended {0}' -f (Stamp $bResult.End))
    if (-not ($aResult.FirstByte -lt $bResult.End -and $bResult.End -lt $cResult.End)) {
        Halt 1 'STOP: A''s first byte, B''s end and C''s end are not in that order; L2 is not run; the run coordinator decides'
    }
    Note 'ORDER ok: A first byte < B end < C end'
    if ($bResult.End -lt $aResult.End) { Note 'NOTE: B ended during A''s stream (after A''s first byte, before A''s curl end)' }
    if ($cState -ne 'present') { Halt 1 'STOP: L1B''s C, answered after B, does not show Lab Probe L1B; L2 is not run; the run coordinator decides' }
}
if ($Row -in @('L2', 'L3') -and $cResult.End -le $aResult.FirstByte) {
    Note 'FINDING: C ended before A''s first byte (C did not queue behind A)'
} elseif ($Row -in @('L2', 'L3') -and $cResult.End -lt $aResult.End) {
    Note 'NOTE: C ended during A''s stream (after A''s first byte, before A''s curl end)'
}

# Observation is timed from C's answer; nothing is sent while C is unanswered.
foreach ($minutes in @(1, 5, 15, 30)) {
    Wait-Until $cResult.End.AddMinutes($minutes)
    $state = Read-State "plus$minutes"
    $now = Probe-State $state.Read
    if ($now -ne $cState) { Note "FINDING: the probe was $cState in C and is $now at C + $minutes minutes" }
}

if ($Row -in @('L1B', 'L2', 'L3')) {
    $busy = Read-Check @('busy', $aResult.File, $CompanyGuid, "$BusyRows")
    Note "read A: $($busy.Out)"
    if ($busy.Code -ne 0) { Halt 1 'STOP: A''s answer failed its read' }
}
Note 'row done'
exit 0
