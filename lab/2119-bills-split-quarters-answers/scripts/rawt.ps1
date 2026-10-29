# Lab helper (#2119 bills split by quarter, not product code; the 2119 timing row's rawt.ps1 with the folders changed): one raw read by C3's line with the sheet's rule 3 timing fields, the request's sha256
# checked against SHA256.txt first; UTC noted just before the start and just after curl exits; a capture of Tally's window every
# 30 s while it runs (capture only: no click, no focus change, no key); after 600 s with no end: leave curl running, HALT.
# Then size, sha256, CONTENT-LENGTH, the 200,000,000-byte bound, read.py bills, a capture. Usage: rawt.ps1 -Req <file name> -Name R1
param([Parameter(Mandatory=$true)][string]$Req, [Parameter(Mandatory=$true)][string]$Name)
$ErrorActionPreference = 'Stop'
$S = '<lab folder>\sheet\lab\2119-bills-split-quarters'; $E = '<lab folder>\evidence'
$cap = '<capture helper>'
function Now { (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
$rel = "requests/$Req"
$line = Get-Content "$S\SHA256.txt" | Where-Object { $_ -match ('\s' + [regex]::Escape($rel) + '$') } | Select-Object -First 1
if (-not $line) { "HALT: $rel not in SHA256.txt; nothing sent"; exit 5 }
$want = ($line -split '\s+')[0]; $got = (Get-FileHash -Algorithm SHA256 "$S\requests\$Req").Hash.ToLower()
if ($got -ne $want) { "HALT: request sha256 $got differs; nothing sent"; exit 4 }
$ans = "$E\$Name.answer.xml"; $hdr = "$E\$Name.headers.txt"
if (Test-Path $ans) { "HALT: $Name answer exists (never resend)"; exit 6 }
$fmt = '"%{http_code} %{size_download} %{size_upload} %{time_connect} %{time_starttransfer} %{time_total}"'
$t0 = Now
$p = Start-Process curl.exe -ArgumentList @('-sS', '-w', $fmt, '--data-binary', "`"@$S\requests\$Req`"", '-H', '"Content-Type: text/xml; charset=utf-16"', '-D', "`"$hdr`"", 'http://127.0.0.1:9001', '-o', "`"$ans`"") -RedirectStandardOutput "$E\$Name.printed.txt" -RedirectStandardError "$E\$Name.stderr.txt" -WindowStyle Hidden -PassThru
$null = $p.Handle  # keeps the exit code readable after the process ends
$sw = [Diagnostics.Stopwatch]::StartNew(); $k = 0
while (-not $p.HasExited) {
  Start-Sleep -Milliseconds 250
  if ($sw.Elapsed.TotalSeconds -ge 30 * ($k + 1)) { $k++; & $cap -Handle 13379766 -Out "$E\$Name-during-$('{0:d2}' -f $k).png" | Out-Null }
  if ($sw.Elapsed.TotalSeconds -gt 600) { "$(Now) HALT: $Name has no answer after 600 s; curl pid $($p.Id) left running"; exit 7 }
}
$t1 = Now; $p.WaitForExit(); $ec = $p.ExitCode
$printed = (Get-Content "$E\$Name.printed.txt" -Raw).Trim()
$size = if (Test-Path $ans) { (Get-Item $ans).Length } else { -1 }
$sha = if ($size -ge 0) { (Get-FileHash -Algorithm SHA256 $ans).Hash.ToLower() } else { 'none' }
$cl = (Select-String -Path $hdr -Pattern '^CONTENT-LENGTH:\s*(\d+)' -ErrorAction SilentlyContinue | ForEach-Object { $_.Matches[0].Groups[1].Value }) -join ','
$entry = "$Name $Req req_sha256=$got start=$t0 end=$t1 curl_exit=$ec w='$printed' content_length=$cl bytes=$size sha256=$sha captures_during=$k"
Add-Content -Encoding ascii "$E\times.txt" $entry; $entry
if ($ec -ne 0 -or -not $printed.StartsWith('200 ') -or -not $cl -or "$size" -ne "$cl" -or $size -le 0) { 'HALT: the request failed (C3)'; exit 2 }
if ($size -gt 200000000) { "HALT: $size bytes above the bound"; exit 3 }
$r = & python -I "$S\read.py" bills $ans 2>&1 | Out-String
"read.py: $($r.Trim())"; Add-Content -Encoding utf8 "$E\times.txt" "$Name read.py: $($r.Trim())"
& $cap -Handle 13379766 -Out "$E\$Name-after.png" | Out-Null
exit 0
