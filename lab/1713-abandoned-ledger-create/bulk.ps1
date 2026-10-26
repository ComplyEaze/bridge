# The bulk Journal sender of the #1713 sheet (README, steps S4 and M0). Windows PowerShell 5.1. Run from this folder:
#   powershell -NoProfile -File .\bulk.ps1 -First 1 -Last 96 [-DryRun]
#   powershell -NoProfile -File .\bulk.ps1 -First 97 -Last <the file read.py plan names> [-DryRun]
# -DryRun prints the plan and sends nothing. Sends bulk\bulk-NNN.xml (made by `python -I gen.py bulk`) one at a time,
# in order, each once in the sheet's life, by LAB-COMMON C3's line. Before each send: the STOP file check (a file named
# STOP in this folder, made by whoever sees a dialog on Tally's screen) and the file's sha256 against BULK-SHA256.txt.
# After each: C3's checks (exit 0, status 200, a body whose size equals CONTENT-LENGTH, at most 2,000,000 bytes) and
# `python -I read.py import <answer> 250`. The next file is sent only when both hold; any other answer is a STOP.
# It pauses after the first and the last file of the run for the screenshot of Tally's window. A send with no answer
# in 10 minutes is a STOP that leaves the window open and the curl running (C3, C9); the script never ends a process.
# Exit codes: 0 = done; 1 = STOP, a check failed; 3 = STOP, a sha256 differs; 4 = STOP, no answer in 10 minutes;
# 5 = STOP, the STOP file.
param(
    [Parameter(Mandatory = $true)][ValidateRange(1, 192)][int]$First,
    [Parameter(Mandatory = $true)][ValidateRange(1, 192)][int]$Last,
    [string]$Evidence = 'C:\bridge-lab\1713\evidence',
    [switch]$DryRun
)
$ErrorActionPreference = 'Stop'
$here = Split-Path -Parent $MyInvocation.MyCommand.Path
$stopFile = Join-Path $here 'STOP'
$readPy = Join-Path $here 'read.py'
$url = 'http://127.0.0.1:9001'
$out = Join-Path $Evidence 'bulk'
$log = Join-Path $out 'bulk-answers.txt'
$readBound = 2000000

if (-not (($First -eq 1 -and $Last -eq 96) -or ($First -eq 97 -and $Last -ge 97))) {
    Write-Output 'STOP: the runs are -First 1 -Last 96 (S4) and -First 97 -Last <n> (M0); nothing sent'; exit 1
}
$names = @($First..$Last | ForEach-Object { 'bulk-{0:000}' -f $_ })
$plan = @(
    "bulk: $($names[0]).xml to $($names[-1]).xml ($($names.Count) files of 250 Journals), one at a time, to $url",
    "answers: $out; one line each in $log",
    "screenshot pause after $($names[0]) and after $($names[-1]); STOP file $stopFile"
)
if ($DryRun) { $plan | ForEach-Object { Write-Output $_ }; exit 0 }

if (Test-Path $stopFile) { Write-Output "STOP: the STOP file $stopFile is present; nothing sent"; exit 5 }
if (-not (Get-Command python -ErrorAction SilentlyContinue)) { Write-Output 'STOP: python is not on PATH; nothing sent'; exit 1 }
$expected = @{}
foreach ($line in Get-Content -Path (Join-Path $here 'BULK-SHA256.txt')) {
    $parts = $line -split '  ', 2
    if ($parts.Count -eq 2) { $expected[$parts[1]] = $parts[0] }
}
foreach ($n in $names) {
    if (-not (Test-Path (Join-Path $here "bulk\$n.xml"))) { Write-Output "STOP: bulk\$n.xml is missing (run python -I gen.py bulk); nothing sent"; exit 1 }
    if (Test-Path (Join-Path $out "$n.answer.xml")) { Write-Output "STOP: $n was already sent (its answer file exists); nothing sent"; exit 1 }
}
if ($First -eq 97 -and -not (Test-Path (Join-Path $out 'bulk-096.answer.xml'))) {
    Write-Output 'STOP: bulk-096 has no answer file, so S4 did not end; nothing sent'; exit 1
}
if (-not (Test-Path $out)) { New-Item -ItemType Directory -Path $out | Out-Null }
$invariant = [Globalization.CultureInfo]::InvariantCulture

function Note([string]$what) {
    $line = '{0} {1}' -f [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ss.fffZ', $invariant), $what
    Add-Content -Encoding UTF8 -Path $log -Value $line
    Write-Output $line | Out-Host
}

foreach ($n in $names) {
    if (Test-Path $stopFile) { Note "STOP before ${n}: the STOP file is present; not sent"; exit 5 }
    $file = Join-Path $here "bulk\$n.xml"
    $hash = (Get-FileHash -Algorithm SHA256 -Path $file).Hash.ToLower()
    if ($hash -ne $expected["bulk/$n.xml"]) { Note "STOP before ${n}: its sha256 differs from BULK-SHA256.txt; not sent"; exit 3 }
    $answer = Join-Path $out "$n.answer.xml"
    $headers = Join-Path $out "$n.headers.txt"
    $curlArgs = @('-sS', '-w', '%{http_code} %{size_download} %{time_total}', '--data-binary', ('@' + $file),
        '-H', 'Content-Type: text/xml; charset=utf-16', '-D', $headers, $url, '-o', $answer,
        '--stderr', (Join-Path $out "$n.curl.err"))
    Note "send $n (sha256 $hash)"
    $job = Start-Job -ScriptBlock {
        param($argv)
        $printed = & curl.exe @argv
        [pscustomobject]@{ Code = $LASTEXITCODE; Printed = ("$printed").Trim() }
    } -ArgumentList (, $curlArgs)
    $deadline = [DateTime]::UtcNow.AddMinutes(10)
    $late = $false
    while (-not (Wait-Job -Job $job -Timeout 1)) {
        if (-not $late -and [DateTime]::UtcNow -gt $deadline) {
            Note "STOP: $n has no answer after 10 minutes. Leave this window open, press no Ctrl+C and no key in Tally, and report (C9). Nothing more is sent."
            $late = $true
        }
    }
    $result = Receive-Job -Job $job
    Remove-Job -Job $job
    $fields = "$($result.Printed)" -split ' '
    $length = ''
    if (Test-Path $headers) {
        $m = @(Select-String -Path $headers -Pattern '^CONTENT-LENGTH:\s*(\d+)')
        if ($m.Count -gt 0) { $length = $m[$m.Count - 1].Matches[0].Groups[1].Value }
    }
    $size = -1L
    try { $size = [long]$fields[1] } catch { }
    Note "answer $n exit=$($result.Code) curl: $($result.Printed) content-length=$length"
    if ($late) { Note "STOP: $n ended only after 10 minutes; nothing more is sent"; exit 4 }
    if ($result.Code -ne 0 -or $fields[0] -ne '200' -or $size -le 0 -or "$size" -ne $length -or $size -gt $readBound) {
        Note "STOP after ${n}: the request failed (exit $($result.Code), status $($fields[0]), size $size, content-length $length)"; exit 1
    }
    $check = & python -I $readPy import $answer 250
    $code = $LASTEXITCODE
    Note "read ${n}: $("$check".Trim())"
    if ($code -ne 0) { Note "STOP after ${n}: its answer is not CREATED 250 with every other counter 0"; exit 1 }
    if ($n -eq $names[0] -or $n -eq $names[-1]) {
        Read-Host "Take the screenshot of Tally's whole window now (a dialog: create the STOP file, press no key in Tally), then press Enter here" | Out-Null
        Note "screenshot pause after $n ended"
    }
}
Note "DONE: $($names[0]) to $($names[-1]), $($names.Count) files, each CREATED 250"
exit 0
