# Lab helper (2377-L Part A, not product code): LAB-COMMON C2 a, b and c, read only. a and b from cmd.exe with 2>&1,
# each with its exit code; c is curl.exe GET /status. Writes evidence/<tag>-tasklist.txt, -netstat.txt, -status.txt
# (+ -status.headers.txt), and prints the lines that matter.
# Usage: powershell -NoProfile -File c2.ps1 <tag>
param([string]$Tag)
$E = '<LAB_DIR>\evidence'
$t = (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ssZ')
cmd.exe /c "tasklist /FI ""IMAGENAME eq tally.exe"" /FO CSV /NH > ""$E\$Tag-tasklist.txt"" 2>&1"
$a = $LASTEXITCODE; Add-Content "$E\$Tag-tasklist.txt" "time: $t`r`nexit: $a"
cmd.exe /c "netstat -ano > ""$E\$Tag-netstat.txt"" 2>&1"
$b = $LASTEXITCODE; Add-Content "$E\$Tag-netstat.txt" "time: $t`r`nexit: $b"
$s = & curl.exe -sS -w "%{http_code} %{size_download}" -D "$E\$Tag-status.headers.txt" -o "$E\$Tag-status.txt" http://127.0.0.1:9001/status
$c = $LASTEXITCODE
"$t C2 ($Tag): tasklist exit $a"; Get-Content "$E\$Tag-tasklist.txt" | Select-String 'tally'
$L = Get-Content "$E\$Tag-netstat.txt"
"netstat exit $b; LISTENING lines $(($L | Select-String 'LISTENING').Count); on :9001:"; $L | Select-String 'LISTENING' | Where-Object { $_.Line -match '^\s*TCP\s+\S+:9001\s' } | ForEach-Object { $_.Line.Trim() }
"GET /status: curl exit $c, printed $s"
