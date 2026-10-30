# Lab helper (#2142 drain repeat, not product code; the first row's block runner with the rule-4 watcher): one block of the sheet.
# A capture of Tally's whole window just before it; probe.py run by the sheet's own Start-Process line from the sheet's folder;
# the watcher (watch.ps1: checks each second, a capture every 30 s, ends within 2 s of probe.py's exit) started with probe.py's
# pid; the closing capture straight after both end. With -Masters, then the timed ledger_masters (basic) through the row's stdio
# client, sent at once (rule 5). With -Wait, rule 6: 120 s with a capture every 30 s, then ONE GET /status by rule 2's line.
# Sends nothing else; ends nothing. Log: evidence\blocks.txt.
param([Parameter(Mandatory = $true)][string]$Block, [switch]$Read, [switch]$Cut, [switch]$AfterCut, [switch]$Masters, [switch]$Wait)
$labDir = '<lab folder>'; $sheetDir = "$labDir\sheet"; $ans = "$labDir\answers"; $blockLog = "$labDir\evidence\blocks.txt"
function Stamp { (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
function Note([string]$text) { Add-Content -Encoding utf8 $blockLog "$(Stamp) $Block $text"; Write-Host "$(Stamp) $Block $text" }
$tallyHandle = [int64](Get-Process tally | Select-Object -First 1).MainWindowHandle
function Capture([string]$name) { $o = & powershell -NoProfile -File "$labDir\capwin.ps1" -Handle $tallyHandle -Out "$labDir\shots\$name.png"; Note "capture $name.png: $($o -join ' ')" }
Capture "$Block-before"
$argList = @('-I', 'probe.py', '--block', $Block, '--out', '<lab folder>\answers')
if ($Read) { $argList += @('--read', 'requests/r-4-receivable-20260331.utf16le.xml', '--read-sha256', '6520d94ec6d013e3e44d9af62ee9c9830a65085b4cb5b54710340df0d03c750f') }
if ($Cut) { $argList += @('--cut-after', '20') }
if ($AfterCut) { $argList += @('--first-probe-after-cut', '1') }
Note "probe.py $($argList -join ' ')"
Push-Location $sheetDir
$p = Start-Process -PassThru -NoNewWindow -FilePath python -ArgumentList $argList -RedirectStandardOutput "<lab folder>\answers\$Block-stdout.txt" -RedirectStandardError "<lab folder>\answers\$Block-stderr.txt"; $null = $p.Handle
Pop-Location
$watcher = Start-Process powershell -ArgumentList "-NoProfile -ExecutionPolicy Bypass -File $labDir\watch.ps1 -Tag $Block -ProbePid $($p.Id) -StopFile $ans\stop-$Block" -WindowStyle Hidden -PassThru
$p.WaitForExit(); $code = $p.ExitCode; $exitStamp = Stamp
$null = $watcher.WaitForExit(5000)
Capture "$Block-after"
$endLine = Get-Content "$ans\$Block-log.jsonl" -ErrorAction SilentlyContinue | Where-Object { $_ -match '"event": "(end|stop)"' } | Select-Object -Last 1
Note "probe.py exit $code at $exitStamp; watcher ended: $($watcher.HasExited); last line: $endLine"
if ($code -ne 0) { Note "stop: probe.py exit $code"; exit $code }
if ($Masters) {
    $sendStamp = Stamp
    $o = & python -I "$labDir\one.py" "$Block-masters" ledger_masters '{\"company_guid\":\"d45bc1b0-e5e3-4261-b3b2-cce3915f42d3\",\"fields\":\"basic\"}' 2>&1
    Note "timed ledger_masters sent $sendStamp (the end line above is the block's end): $(($o | ForEach-Object { "$_" }) -join ' ;; ')"
}
if ($Wait) {
    for ($k = 1; $k -le 4; $k++) { Start-Sleep -Seconds 30; Capture "$Block-wait$k" }
    $s = & curl.exe -sS -w "%{http_code} %{size_download} %{time_total}" -o "$ans\$Block-wait-status.body" http://127.0.0.1:9001/status 2>&1
    Note "rule 6 GET /status: exit $LASTEXITCODE printed $s"
}
exit 0
