# Lab helper (0.7.0 RC sitting, not product code; the 2142r step0.ps1 with the row folder as a parameter): LAB-COMMON C2 a,
# b and c plus the bridge_mcp.exe process list with parents (README rule 3), each kept as a file under <RowDir>\evidence
# with its command and exit code; a capture of Tally's window first. Reads only.
param([Parameter(Mandatory = $true)][string]$RowDir, [Parameter(Mandatory = $true)][string]$Tag)
$rd = $RowDir; $ev = "$rd\evidence"
foreach ($sub in 'evidence', 'shots') { if (-not (Test-Path "$rd\$sub")) { New-Item -ItemType Directory "$rd\$sub" | Out-Null } }
function Stamp { (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
$cap = '<sitting folder>\tools\capwin.ps1'; $tallyHwnd = [int64](Get-Process tally | Select-Object -First 1).MainWindowHandle
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
try {
    $pl = @(Get-CimInstance Win32_Process -Filter "Name='bridge_mcp.exe'" -ErrorAction Stop | ForEach-Object {
        $par = Get-CimInstance Win32_Process -Filter "ProcessId=$($_.ParentProcessId)" -ErrorAction SilentlyContinue
        "bridge_mcp.exe pid $($_.ProcessId) parent $($_.ParentProcessId) $($par.Name) path $($_.ExecutablePath)" }); $x = 0
} catch { $pl = @("failed: $($_.Exception.Message)"); $x = 1 }
@('command: Get-CimInstance Win32_Process (bridge_mcp.exe, with each parent)', "time: $(Stamp)", "exit: $x") + $pl | Set-Content -Encoding utf8 "$ev\$Tag-bridge-processes.txt"
"bridge_mcp.exe processes ($($pl.Count)): $($pl -join ' / ')"
