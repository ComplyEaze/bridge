# Lab helper (#1233 TIEOUT D end date, not product code): one raw read by LAB-COMMON C3's line exactly. The request's sha256 is
# checked against the sheet's SHA256.txt first; the answer is never overwritten (never resent). UTC noted before and after.
# Then size, sha256, CONTENT-LENGTH, the read bound, and a capture of Tally's whole window (each first send is a NEW SHAPE).
# Usage: send.ps1 -Req <file name> -Name a1 -Bound 2000000
param([Parameter(Mandatory=$true)][string]$Req, [Parameter(Mandatory=$true)][string]$Name, [Parameter(Mandatory=$true)][long]$Bound)
$ErrorActionPreference = 'Stop'
$S = '<lab folder>\sheet\lab\1233-tieout-d-end-date'; $E = '<lab folder>\evidence'
$cap = '<capture helper>'
function Now { (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
$rel = "requests/$Req"
$line = Get-Content "$S\SHA256.txt" | Where-Object { $_ -match ('\s' + [regex]::Escape($rel) + '$') } | Select-Object -First 1
if (-not $line) { "HALT: $rel not in SHA256.txt; nothing sent"; exit 5 }
$want = ($line -split '\s+')[0]; $got = (Get-FileHash -Algorithm SHA256 "$S\requests\$Req").Hash.ToLower()
if ($got -ne $want) { "HALT: request sha256 $got differs; nothing sent"; exit 4 }
$ans = "$E\$Name.answer.xml"; $hdr = "$E\$Name.headers.txt"
if (Test-Path $ans) { "HALT: $Name answer exists (never resend)"; exit 6 }
$t0 = Now
$p = Start-Process curl.exe -ArgumentList @('-sS', '-w', '"%{http_code} %{size_download}"', '--data-binary', "`"@$S\requests\$Req`"", '-H', '"Content-Type: text/xml; charset=utf-16"', '-D', "`"$hdr`"", 'http://127.0.0.1:9001', '-o', "`"$ans`"") -RedirectStandardOutput "$E\$Name.printed.txt" -RedirectStandardError "$E\$Name.stderr.txt" -WindowStyle Hidden -PassThru
$null = $p.Handle
$sw = [Diagnostics.Stopwatch]::StartNew()
while (-not $p.HasExited) { Start-Sleep -Milliseconds 100; if ($sw.Elapsed.TotalSeconds -gt 600) { "$(Now) HALT: $Name has no answer after 600 s; curl pid $($p.Id) left running"; exit 7 } }
$t1 = Now; $p.WaitForExit(); $ec = $p.ExitCode
$printed = (Get-Content "$E\$Name.printed.txt" -Raw).Trim()
$size = if (Test-Path $ans) { (Get-Item $ans).Length } else { -1 }
$sha = if ($size -ge 0) { (Get-FileHash -Algorithm SHA256 $ans).Hash.ToLower() } else { 'none' }
$cl = (Select-String -Path $hdr -Pattern '^CONTENT-LENGTH:\s*(\d+)' -ErrorAction SilentlyContinue | ForEach-Object { $_.Matches[0].Groups[1].Value }) -join ','
$entry = "$Name $Req req_sha256=$got start=$t0 end=$t1 curl_exit=$ec printed='$printed' content_length=$cl bytes=$size sha256=$sha"
Add-Content -Encoding ascii "$E\send-log.txt" $entry; $entry
if ($ec -ne 0 -or -not $printed.StartsWith('200 ') -or -not $cl -or "$size" -ne "$cl" -or $size -le 0) { 'HALT: the request failed (C3)'; exit 2 }
if ($size -gt $Bound) { "HALT: $size bytes above the bound $Bound"; exit 3 }
& $cap -Handle 13379766 -Out "<lab folder>\shots\$Name-after.png" | Out-Null
"capture after: shots\$Name-after.png"
exit 0
