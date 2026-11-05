# Lab helper (#2265 R4 on THE CLIENT BOOK, not product code): while sales_register's read is open, every 10 minutes records
# GET /status (code and seconds), tally.exe CPU seconds and working set, and bridge_mcp's, to evidence\samples.txt; stops at
# the call's answer file or at the 60-minute mark (11:05:20Z). Sends nothing else to Tally and takes no screenshot.
$R = '<LAB_DIR>'; $out = "$R\evidence\samples.txt"; $deadline = [datetime]::Parse('2026-11-05T11:05:20.552Z').ToUniversalTime()
function Sample([string]$why) {
    $s = curl.exe -sS -m 60 -o NUL -w "%{http_code} %{time_total}" http://127.0.0.1:9001/status
    $t = Get-Process tally -ErrorAction SilentlyContinue | Select-Object -First 1; $b = Get-Process -Id 16688 -ErrorAction SilentlyContinue
    $line = "$((Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ssZ')) $why /status $s; tally.exe CPU $([int]$t.CPU) s, working set $([int]($t.WorkingSet64/1MB)) MB; bridge_mcp CPU $([math]::Round($b.CPU,1)) s, $([int]($b.WorkingSet64/1MB)) MB"
    Add-Content -Encoding utf8 $out $line
}
Sample 'start'
$next = (Get-Date).ToUniversalTime().AddMinutes(10)
while ($true) {
    if (Test-Path "$R\agent-run\answers\*-sales.json") { Add-Content -Encoding utf8 $out "$((Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ssZ')) answer file seen: sampler stops"; break }
    $now = (Get-Date).ToUniversalTime()
    if ($now -ge $deadline) { Sample 'no answer in 60 min'; break }
    if ($now -ge $next) { Sample 'every 10 min'; $next = $next.AddMinutes(10) }
    Start-Sleep -Seconds 5
}
