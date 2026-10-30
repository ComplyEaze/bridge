# Lab helper (#2145, not product code; row 2161-B's step0.ps1 with the folder changed): LAB-COMMON C2 a, b and c plus the sheet's rule 5, each kept as a
# file under evidence\ with its command and exit code; a capture of Tally's window first. Reads only.
param([Parameter(Mandatory = $true)][string]$Tag)
$rd = '<lab folder>'; $ev = "$rd\evidence"
try { Set-PSReadLineOption -HistorySaveStyle SaveNothing -ErrorAction Stop } catch {}
function Stamp { (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
$cap = '<lab folder>\capwin.ps1'; $tallyHwnd = [int64](Get-Process tally | Where-Object { $_.MainWindowTitle -like 'TallyPrime:9001*' } | Select-Object -First 1).MainWindowHandle
& powershell -NoProfile -File $cap -Handle $tallyHwnd -Out "$rd\shots\$Tag-tally.png" | Out-Null
"capture $(Stamp) shots\$Tag-tally.png exit $LASTEXITCODE"
$o = cmd.exe /c 'tasklist /FI "IMAGENAME eq tally.exe" /FO CSV /NH 2>&1'; $x = $LASTEXITCODE
@("command: tasklist /FI ""IMAGENAME eq tally.exe"" /FO CSV /NH 2>&1", "time: $(Stamp)", "exit: $x") + $o | Set-Content -Encoding utf8 "$ev\$Tag-tasklist.txt"
"tasklist exit $x : $($o -join ' / ')"
$o = cmd.exe /c 'netstat -ano 2>&1'; $x = $LASTEXITCODE
@('command: netstat -ano 2>&1', "time: $(Stamp)", "exit: $x") + $o | Set-Content -Encoding utf8 "$ev\$Tag-netstat.txt"
"netstat exit $x : 9001 lines: $((@($o | Where-Object { $_ -match 'LISTENING' -and $_ -match ':9001\s' }) | ForEach-Object { $_.Trim() -replace '\s+', ' ' }) -join ' / '); LISTENING lines $(@($o | Where-Object { $_ -match 'LISTENING' }).Count)"
try { $n = @(Get-NetTCPConnection -LocalPort 9001 -State Listen -ErrorAction Stop | ForEach-Object { "$($_.LocalAddress):$($_.LocalPort) pid $($_.OwningProcess)" }); $x = 0 } catch { $n = @("failed: $($_.Exception.Message)"); $x = 1 }
@('command: Get-NetTCPConnection -LocalPort 9001 -State Listen', "time: $(Stamp)", "exit: $x") + $n | Set-Content -Encoding utf8 "$ev\$Tag-nettcp.txt"
"Get-NetTCPConnection exit $x : $($n -join ' / ')"
$o = & curl.exe -sS -w "%{http_code} %{size_download}" -o "$ev\$Tag-status.body" http://127.0.0.1:9001/status 2>&1; $x = $LASTEXITCODE
@('command: curl.exe -sS -w "%{http_code} %{size_download}" -o <file> http://127.0.0.1:9001/status', "time: $(Stamp)", "exit: $x", "printed: $o") | Set-Content -Encoding utf8 "$ev\$Tag-status.txt"
"GET /status exit $x : $o"

