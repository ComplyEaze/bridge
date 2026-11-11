# Lab helper (row 1780-W, not product code; row 2161-B's step0.ps1 with the folder changed and its Get-NetTCPConnection line left out):
# LAB-COMMON C2 a, b and c, each kept as a file under evidence\ with its command and exit code; a PrintWindow copy of Tally's window
# first (capwin.ps1, row 2161-B's), kept under shots\. Reads only.
param([Parameter(Mandatory = $true)][string]$Tag)
$rd = '<LAB_DIR>'; $ev = "$rd\evidence"
function Stamp { (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
$cap = "$rd\capwin.ps1"; $tallyHwnd = [int64](Get-Process tally | Where-Object { $_.MainWindowTitle -like 'TallyPrime:9001*' } | Select-Object -First 1).MainWindowHandle
$co = & powershell -NoProfile -File $cap -Handle $tallyHwnd -Out "$rd\shots\$Tag-tally.png"
"capture $(Stamp) shots\$Tag-tally.png exit $LASTEXITCODE : $($co -join ' / ')"
$o = cmd.exe /c 'tasklist /FI "IMAGENAME eq tally.exe" /FO CSV /NH 2>&1'; $x = $LASTEXITCODE
@("command: tasklist /FI ""IMAGENAME eq tally.exe"" /FO CSV /NH 2>&1", "time: $(Stamp)", "exit: $x") + $o | Set-Content -Encoding utf8 "$ev\$Tag-tasklist.txt"
"tasklist exit $x : $($o -join ' / ')"
$o = cmd.exe /c 'netstat -ano 2>&1'; $x = $LASTEXITCODE
@('command: netstat -ano 2>&1', "time: $(Stamp)", "exit: $x") + $o | Set-Content -Encoding utf8 "$ev\$Tag-netstat.txt"
"netstat exit $x : 9001 lines: $((@($o | Where-Object { $_ -match 'LISTENING' -and $_ -match ':9001\s' }) | ForEach-Object { $_.Trim() -replace '\s+', ' ' }) -join ' / '); LISTENING lines $(@($o | Where-Object { $_ -match 'LISTENING' }).Count)"
$o = & curl.exe -sS -w "%{http_code} %{size_download}" -o "$ev\$Tag-status.body" http://127.0.0.1:9001/status 2>&1; $x = $LASTEXITCODE
@('command: curl.exe -sS -w "%{http_code} %{size_download}" -o <file> http://127.0.0.1:9001/status', "time: $(Stamp)", "exit: $x", "printed: $o") | Set-Content -Encoding utf8 "$ev\$Tag-status.txt"
"GET /status exit $x : $o"
