# Lab helper (2119-L copy, with the sheet's timing fields): LAB-COMMON v3 C3 send, one request, the exact curl line; checks the request's sha256 first; prints exit, status,
# bytes, CONTENT-LENGTH and the answer's sha256; refuses to overwrite an answer; enforces the read bound.
# Usage: c3-send.ps1 -Req <request file> -Sha <expected sha256> -AnsDir <folder> -Ans <answer file name> -Bound <bytes> -Log <send-log file>
param([Parameter(Mandatory=$true)][string]$Req, [Parameter(Mandatory=$true)][string]$Sha, [Parameter(Mandatory=$true)][string]$AnsDir,
      [Parameter(Mandatory=$true)][string]$Ans, [Parameter(Mandatory=$true)][int]$Bound, [Parameter(Mandatory=$true)][string]$Log)
$ErrorActionPreference = 'Stop'
$ansPath = Join-Path $AnsDir $Ans
$hdrPath = $ansPath + '.headers.txt'
if (Test-Path $ansPath) { throw "answer exists: $ansPath (never resend)" }
$got = (Get-FileHash -Algorithm SHA256 $Req).Hash.ToLower()
if ($got -ne $Sha) { Write-Host "stop: request hash $got differs from $Sha; nothing sent"; exit 4 }
$t0 = (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ')
$out = & curl.exe -sS -w "%{http_code} %{size_download} %{time_starttransfer} %{time_total}" --data-binary "@$Req" -H "Content-Type: text/xml; charset=utf-16" -D $hdrPath http://127.0.0.1:9001 -o $ansPath
$ec = $LASTEXITCODE
$cl = (Select-String -Path $hdrPath -Pattern '^CONTENT-LENGTH:\s*(\d+)' | ForEach-Object { $_.Matches[0].Groups[1].Value }) -join ','
$size = if (Test-Path $ansPath) { (Get-Item $ansPath).Length } else { -1 }
$sha = if (Test-Path $ansPath) { (Get-FileHash -Algorithm SHA256 $ansPath).Hash.ToLower() } else { 'none' }
$line = "{0} req={1} req_sha256={2} curl_exit={3} printed='{4}' content_length={5} bytes={6} sha256={7}" -f $t0, (Split-Path $Req -Leaf), $got, $ec, $out, $cl, $size, $sha
Add-Content -Encoding ascii $Log $line
$line
if ($ec -ne 0 -or -not "$out".StartsWith('200 ') -or "$size" -ne "$cl" -or $size -le 0) { Write-Host 'stop: the request failed'; exit 2 }
if ($size -gt $Bound) { Write-Host "stop: answer $size bytes above bound $Bound"; exit 3 }
exit 0
