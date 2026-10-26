# Sends the 40 census requests of this folder one at a time, with a fixed gap between the end of one answer and the
# next send, and records each with LAB-COMMON C3's checks. Reads only. Usage (Windows PowerShell, from this folder):
#   powershell -NoProfile -File .\driver.ps1 -GapMs 500 -RunName A1
# It stops at the first request that fails C3 or takes more than 20 s, and never sends a request twice.
# It also sends nothing more once a file named STOP exists in this folder (made by a person who sees a dialog).
param(
    [Parameter(Mandatory = $true)][ValidateSet(250, 500)][int]$GapMs,
    [Parameter(Mandatory = $true)][ValidatePattern('^[AB][12]$')][string]$RunName
)
$ErrorActionPreference = 'Stop'
$here = Split-Path -Parent $MyInvocation.MyCommand.Path
$out = Join-Path $here ('runs\' + $RunName)
if (Test-Path $out) { throw "run folder already exists, nothing sent: $out" }
New-Item -ItemType Directory -Path $out | Out-Null

$expected = @{}
foreach ($line in Get-Content (Join-Path $here 'SHA256.txt')) {
    if ($line -match '^#' -or -not $line.Trim()) { continue }
    $p = $line -split '  '
    $expected[$p[2]] = $p[0]
}

$log = Join-Path $out 'log.tsv'
"file`tutc_send`tcurl_exit`tstatus`tbytes`tcontent_length`ttime_starttransfer`ttime_total" | Set-Content $log
$files = @(Get-ChildItem (Join-Path $here 'requests') -Filter 'c*_census_*.request.utf16le.xml' | Sort-Object Name)
if ($files.Count -ne 40) { "STOP: expected 40 census files, found $($files.Count); nothing sent" | Add-Content $log; exit 2 }

$first = $true
foreach ($f in $files) {
    if (-not $first) { Start-Sleep -Milliseconds $GapMs }
    $first = $false
    if (Test-Path (Join-Path $here 'STOP')) { "STOP before $($f.Name): the STOP file is present; not sent" | Add-Content $log; exit 5 }
    $rel = 'requests/' + $f.Name
    $hash = (Get-FileHash $f.FullName -Algorithm SHA256).Hash.ToLower()
    if ($hash -ne $expected[$rel]) { "STOP before $($f.Name): its sha256 differs from SHA256.txt; not sent" | Add-Content $log; exit 3 }
    $base = Join-Path $out $f.BaseName
    $sent = (Get-Date).ToUniversalTime().ToString('o')
    # Windows PowerShell 5.1 turns a native command's redirected stderr into errors; 'Stop' would end the script
    # before the STOP line below is written, so only this call runs with 'Continue'.
    $ErrorActionPreference = 'Continue'
    $w = & curl.exe -sS -w "%{http_code} %{size_download} %{time_starttransfer} %{time_total}" --data-binary ('@' + $f.FullName) -H "Content-Type: text/xml; charset=utf-16" -D ($base + '.headers.txt') http://127.0.0.1:9001 -o ($base + '.answer.xml') 2> ($base + '.curl.err')
    $exit = $LASTEXITCODE
    $ErrorActionPreference = 'Stop'
    $parts = ("$w".Trim()) -split ' '
    $cl = ''
    if (Test-Path ($base + '.headers.txt')) {
        $m = Select-String -Path ($base + '.headers.txt') -Pattern '^CONTENT-LENGTH:\s*(\d+)'
        if ($m) { $cl = $m.Matches[0].Groups[1].Value }
    }
    "$($f.Name)`t$sent`t$exit`t$($parts[0])`t$($parts[1])`t$cl`t$($parts[2])`t$($parts[3])" | Add-Content $log
    $slow = $false
    try { $slow = ([double]::Parse($parts[3], [Globalization.CultureInfo]::InvariantCulture) -gt 20) } catch { $slow = $true }
    if ($exit -ne 0 -or $parts[0] -ne '200' -or $parts[1] -eq '0' -or $cl -ne $parts[1] -or $slow) {
        "STOP after $($f.Name): curl exit $exit, status $($parts[0]), bytes $($parts[1]), content-length $cl, time $($parts[3])" | Add-Content $log
        exit 4
    }
}
"DONE: $($files.Count) requests, gap $GapMs ms" | Add-Content $log
exit 0
