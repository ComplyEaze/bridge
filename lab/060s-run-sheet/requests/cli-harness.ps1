# cli-harness.ps1 -- step I1a of the sheet. Starts the by-hand bridge_mcp.exe with redirected stdio, a counting
# listener on 127.0.0.1, one JSON message per line in and out. Written on a Mac, NEVER RUN: UNTESTED (U).
# It gives NO value to any posting, batch or Terms name. Exception: Run 2 sets the posting name EMPTY, and only
# when -RulingFile is the owner's relayed ruling file and -RulingSha256 is the sha256 relayed with the go-ahead
# AND the file's recomputed sha256 equals it; otherwise Run 2 does not start ('Run 2 not run: no ruling').
# In Run 3 the three names the OWNER set in this window pass through to the program unchanged and unread.
# Run 3 with none of the owner's names set is the lab-only dry run of step S6 (EXPECT terms_not_accepted).
# Run in 64-bit PowerShell from the owner's command window for Run 3:
#   powershell.exe -NoProfile -File cli-harness.ps1 -Run 3 -Exe <bridge_mcp.exe> -OutDir <C:\bridge-lab\060s\cli\run3\> -Port <unused, not 9001>
param(
  [Parameter(Mandatory = $true)][ValidateSet('1', '2', '3')][string]$Run,
  [Parameter(Mandatory = $true)][string]$Exe,
  [Parameter(Mandatory = $true)][string]$OutDir,
  [Parameter(Mandatory = $true)][int]$Port,
  [string]$Reader = (Join-Path $PSScriptRoot 'class-reader.ps1'),
  [string]$RulingFile = '',
  [string]$RulingSha256 = ''
)
$ErrorActionPreference = 'Stop'
function Stop-Here([string]$why) { Write-Output ("STOP: " + $why); exit 3 }
if (-not [Environment]::Is64BitProcess) { Stop-Here 'not a 64-bit PowerShell' }
if ($Port -eq 9001 -or $Port -lt 1024 -or $Port -gt 65535) { Stop-Here 'the port must be an unused one that is not 9001' }
if ($Run -eq '2') {
    $okRuling = $false
    if ($RulingSha256 -match '^[0-9a-f]{64}$' -and $RulingFile -ne '' -and (Test-Path -LiteralPath $RulingFile)) {
        $okRuling = ((Get-FileHash -Algorithm SHA256 -LiteralPath $RulingFile).Hash.ToLower() -ceq $RulingSha256)
    }
    if (-not $okRuling) { Write-Output 'Run 2 not run: no ruling'; exit 0 }
}
if (-not (Test-Path -LiteralPath $Exe)) { Stop-Here 'the program is not there' }
if (@(Get-NetTCPConnection -State Listen -LocalPort $Port -ErrorAction SilentlyContinue).Count -gt 0) { Stop-Here 'something already listens on the port' }

$data = Join-Path $OutDir 'data'; $out = Join-Path $OutDir 'out'
foreach ($d in $data, $out) { New-Item -ItemType Directory -Force -Path $d | Out-Null }
if (@(Get-ChildItem -LiteralPath $data -Force).Count -gt 0) { Stop-Here 'the data folder is not empty' }

Add-Type -TypeDefinition @'
using System;
using System.Net;
using System.Net.Sockets;
using System.Threading;
public class Counter
{
    TcpListener l; int n; Thread t; volatile bool stop;
    public Counter(int port) { l = new TcpListener(IPAddress.Parse("127.0.0.1"), port); l.ExclusiveAddressUse = true; }
    public void Start() { l.Start(); t = new Thread(Loop); t.IsBackground = true; t.Start(); }
    void Loop() { while (!stop) { try { TcpClient c = l.AcceptTcpClient(); Interlocked.Increment(ref n); c.Close(); } catch { if (stop) return; } } }
    public int Count { get { return Interlocked.CompareExchange(ref n, 0, 0); } }
    public void Reset() { Interlocked.Exchange(ref n, 0); }
    public void Stop() { stop = true; try { l.Stop(); } catch { } }
}
'@

$counter = New-Object Counter $Port
try { $counter.Start() } catch { Stop-Here ('the listener could not bind 127.0.0.1:' + $Port) }
# known answer of the listener: one connection made by this harness itself
$c = New-Object System.Net.Sockets.TcpClient; $c.Connect('127.0.0.1', $Port); $c.Close(); Start-Sleep -Milliseconds 700
$known = $counter.Count
$counter.Reset()
"listener known answer: " + $known + " (EXPECT 1)" | Out-File -LiteralPath (Join-Path $out 'summary.txt') -Encoding ascii
if ($known -ne 1) { $counter.Stop(); Stop-Here 'the listener known answer is not 1' }

$psi = New-Object System.Diagnostics.ProcessStartInfo
$psi.FileName = $Exe
$psi.UseShellExecute = $false
$psi.CreateNoWindow = $true
$psi.RedirectStandardInput = $true; $psi.RedirectStandardOutput = $true; $psi.RedirectStandardError = $true
$keep = @()
if ($Run -eq '3') { $keep = @('BRIDGE_TERMS_ACCEPTED', 'BRIDGE_AGENT_ENABLE_WRITES', 'BRIDGE_AGENT_ENABLE_BATCH_POST') }
foreach ($k in @($psi.EnvironmentVariables.Keys)) {
    if ($k -match '^(?i)BRIDGE_(AGENT|TALLY|TERMS|PDFIUM)_' -and ($keep -notcontains $k)) { $psi.EnvironmentVariables.Remove($k) }
}
$psi.EnvironmentVariables['BRIDGE_AGENT_DATA_DIR'] = $data
$psi.EnvironmentVariables['BRIDGE_TALLY_HOST'] = 'localhost'
if ($Run -eq '1') { $psi.EnvironmentVariables['BRIDGE_TALLY_PORT'] = '0' } else { $psi.EnvironmentVariables['BRIDGE_TALLY_PORT'] = [string]$Port }
$psi.EnvironmentVariables['BRIDGE_AGENT_ENABLE_IMPORT'] = 'true'
if ($Run -eq '2') { $psi.EnvironmentVariables['BRIDGE_AGENT_ENABLE_WRITES'] = '' }   # the owner's ruling cited by -RulingSha256

$stdoutFile = Join-Path $out 'stdout.jsonl'
$script:pend = $null
$p = [System.Diagnostics.Process]::Start($psi)
$errTask = $p.StandardError.ReadToEndAsync()
function Send($obj) {
    $line = ($obj | ConvertTo-Json -Compress -Depth 8)
    try { $p.StandardInput.WriteLine($line); $p.StandardInput.Flush(); return $true }
    catch { Add-Content -LiteralPath $stdoutFile -Value ('<<WRITE FAILED: ' + $_.Exception.Message + '>>') -Encoding UTF8; return $false }
}
function Read-Answer([int]$ms) {
    # one read stays pending after a timeout: it is awaited again, never started twice
    if ($null -eq $script:pend) { $script:pend = $p.StandardOutput.ReadLineAsync() }
    if ($script:pend.Wait($ms)) { $r = $script:pend.Result; $script:pend = $null; if ($null -eq $r) { $r = '<<END OF OUTPUT>>' } } else { $r = '<<TIMEOUT>>' }
    Add-Content -LiteralPath $stdoutFile -Value $r -Encoding UTF8   # after EACH answer, so a later failure loses nothing
    return $r
}
function Run-Reader([string[]]$names, [string[]]$presence, [string]$file) {
    $argsList = @('-NoProfile', '-File', $Reader, '-ProcessId', $p.Id, '-ExpectImage', $Exe)
    if ($names.Count -gt 0) { $argsList += '-Name'; $argsList += ($names -join ',') }
    $argsList += '-Presence'; $argsList += ($presence -join ',')
    & powershell.exe @argsList 2>&1 | Out-File -LiteralPath (Join-Path $out $file) -Encoding ascii
}
$answered = $false
$hello = @{ jsonrpc = '2.0'; id = 1; method = 'initialize'; params = @{ protocolVersion = '2025-06-18'; capabilities = @{}; clientInfo = @{ name = 'lab-harness'; version = '0' } } }
if (Send $hello) {
    $a = Read-Answer 30000
    $answered = ($a -like '*"serverInfo"*')
}
if ($Run -eq '1') {
    $null = $p.WaitForExit(15000)
} elseif ($answered) {
    $null = Send @{ jsonrpc = '2.0'; method = 'notifications/initialized' }
    if ($Run -eq '2') {
        Run-Reader @('BRIDGE_AGENT_ENABLE_WRITES') @('BRIDGE_AGENT_ENABLE_BATCH_POST', 'SystemRoot') 'reader.txt'
        $null = Send @{ jsonrpc = '2.0'; id = 2; method = 'tools/list' }
        $null = Read-Answer 30000
    } else {
        Run-Reader @('BRIDGE_TERMS_ACCEPTED', 'BRIDGE_AGENT_ENABLE_WRITES') @('BRIDGE_AGENT_ENABLE_BATCH_POST', 'SystemRoot') 'reader.txt'
        $post = @{ jsonrpc = '2.0'; id = 2; method = 'tools/call'; params = @{ name = 'post_import'; arguments = @{ company_guid = 'a30f23c3-4819-4334-96ab-ffa6fba70976'; batch_id = 'bridge-00000000-0000-4000-8000-000000000000' } } }
        $null = Send $post
        $null = Read-Answer 60000
        $beforeControl = $counter.Count
        "listener count after post_import, before the control: " + $beforeControl | Out-File -LiteralPath (Join-Path $out 'summary.txt') -Append -Encoding ascii
        $null = Send @{ jsonrpc = '2.0'; id = 3; method = 'tools/call'; params = @{ name = 'tally_status'; arguments = @{} } }
        $null = Read-Answer 130000
        Start-Sleep -Milliseconds 700
    }
}
try { $p.StandardInput.Close() } catch { }
$exited = $p.WaitForExit(30000)
if (-not $exited) { "the program did not exit within 30 s after stdin closed; it is left alone (stop it by its process id only)" | Out-File -LiteralPath (Join-Path $out 'summary.txt') -Append -Encoding ascii }
$exit = if ($exited) { $p.ExitCode } else { 'not exited' }
$null = $errTask.Wait(5000)
$errTask.Result | Out-File -LiteralPath (Join-Path $out 'stderr.txt') -Encoding utf8
$verdict = if ($answered -and $exit -eq 0) { 'REFUSED-OR-SERVED (initialize answered, exit 0): the answers say which' } elseif (-not $answered -and $exit -eq 1) { 'FAILED TO START (no initialize answer, exit 1): stderr names the code' } else { 'NEITHER: record whole' }
@("run: " + $Run, "initialize answered: " + $answered, "exit code: " + $exit, "listener final count: " + $counter.Count, "verdict: " + $verdict) | Out-File -LiteralPath (Join-Path $out 'summary.txt') -Append -Encoding ascii
$counter.Stop()
Get-ChildItem -LiteralPath $OutDir -Recurse -File | ForEach-Object { $h = (Get-FileHash -Algorithm SHA256 -LiteralPath $_.FullName).Hash.ToLower(); Write-Output ($_.FullName + " " + $_.Length + " " + $h) }
exit 0
