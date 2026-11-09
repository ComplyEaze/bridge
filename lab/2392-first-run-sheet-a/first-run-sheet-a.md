# Sheet FR2a: the published Windows file, measured from the shell, for #2390, #2392 and #2393

Purpose: measure what the published 0.7.0 Windows file is and how it answers when it is started by hand, so the install text says only what was seen: its hash, the internet-zone mark a browser download gives it, whether it is signed, its import table, its manifest, and the answers its `tally_status` gives when Tally cannot be reached and when the Terms are off. This sheet measures; it is not a pass or fail of the product. Nothing is installed, and the Claude Desktop window is not driven (a separate checklist for a person covers that).

Every output is a raw file in one folder, `C:\Users\Public\fr2a-hub-only`. **Nothing from that folder is copied, committed or pasted anywhere by this sheet.** (`C:\Users\Public` is readable by every account on the PC; the lab is a lab PC, and the files are handed over by hand and then removed from it by whoever the person who queued this sheet names.) The files hold the user name, paths and, possibly, Windows or antivirus text. The lab's reviewer reads them, and decides what, if anything, is committed. Write each output in full and do not tidy it.

Marks used in the result: M = measured by this run; U = not established, say what was missing.

## Limits
1. **The download needs the owner's OK.** The queue entry for this sheet must say that the owner allowed downloading the published 0.7.0 file on the lab PC. Without that line, stop before step B1.
2. **Install nothing** on this PC for the run: no Visual C++ Redistributable, no tool, no browser extension, and not the extension itself. Do not remove anything either.
3. **State this run changes:** the downloaded file and its `.sha256` file (left in place); the folder `C:\Users\Public\fr2a-hub-only`; a scratch folder in `%TEMP%` that step B4 makes and step B8 deletes; and, when the program contacts Tally, empty lock files and the folders that hold them (`native-dispatch-leases` under `%LOCALAPPDATA%\Bridge\agent` and under `%USERPROFILE%\.complyeaze-bridge\coordination`, created if absent), which it cannot be told to put elsewhere; step B7 lists them, and the installed extension's data files, before and after. Windows and the browser keep their own traces (download history, Defender history, and a crash report if the program crashes), and Windows may look up the file's hash with Microsoft; this sheet does not control or read those. Tally is not opened or changed. The only contact is a status request to a port with nothing listening.
4. **The block rule.** If any step stops with an error, or prints text from Windows Security, that names Defender, SmartScreen, Smart App Control, a virus, a block, a policy or Application Control, after the step has run, add the line `block: yes, step <name>` at the top of that step's file (open the file and add it; do not run the block again), stop, and tell the person who queued this sheet. That is a result.
5. Use **one 64-bit Windows PowerShell window**, not ISE and not the 32-bit shell, so the variables stay set from step to step. Step B0 prints whether the process is 64-bit.

## Saving output
Each step below is one block of two parts. The lines before `. { try {` set variables and functions and are pasted as they are; the part from `. { try {` to `} *>&1 | Out-File ...` runs the work and saves everything it prints to the step's file, and a stopping error is caught and written into the file as a line that starts "ERROR:" (otherwise it would reach only the console). To run a step again, paste only that part. Do not use `>` (it writes UTF-16 here) and do not use `Start-Transcript` (its header names the user and the machine).

## The input
File: `bridge-tally-mcp-v0.7.0-windows-x64.mcpb`, from the download page `https://bridge.complyeaze.com/download.html`, 15,598,419 bytes. Its sha256 is `291a4ed95c758fad9fa04cd21a66be889e90a2fd03960fbceeec2c2a7242d4ec`, as GitHub's release listing reports it (read on 9 November 2026 with `gh release view mcp-v0.7.0 --json assets`). The release also holds the `.sha256` file (107 bytes) that the install text points at. If the listing shows another name, size or hash on the day, or a newer release is the latest, stop and say so: that is a different test.

## Step B0. State of the PC (read only)
```powershell
$hub = "C:\Users\Public\fr2a-hub-only"; New-Item -ItemType Directory $hub -Force | Out-Null
function Show-Data([string]$When) {
  "installed extension's data files, " + $When + " (name, length, last write; contents not read; this run must not change them):"
  $roots = @("$env:LOCALAPPDATA\Bridge\agent") + @(Get-Item -Path "$env:LOCALAPPDATA\Packages\Claude_*\LocalCache\Local\Bridge\agent" -ErrorAction SilentlyContinue | ForEach-Object FullName)
  foreach ($d in $roots) {
    "  folder " + $d + ": " + $(if (Test-Path -LiteralPath $d) { "exists" } else { "absent" })
    foreach ($n in "terms-acceptance.jsonl", "agent-egress.jsonl") { $f1 = Join-Path $d $n; "    " + $n + ": " + $(if (Test-Path -LiteralPath $f1) { (Get-Item -LiteralPath $f1).Length.ToString() + ", " + (Get-Item -LiteralPath $f1).LastWriteTime.ToString("o") } else { "absent" }) }
  }
}
function Show-Locks {
  "lock files the program can create (names only):"
  $store = @(Get-Item -Path "$env:LOCALAPPDATA\Packages\Claude_*\LocalCache\Local\Bridge\agent\native-dispatch-leases" -ErrorAction SilentlyContinue | ForEach-Object FullName)
  foreach ($d in (@("$env:LOCALAPPDATA\Bridge\agent\native-dispatch-leases", "$env:USERPROFILE\.complyeaze-bridge\coordination\native-dispatch-leases") + $store)) {
    $items = @(Get-ChildItem -LiteralPath $d -Force -ErrorAction SilentlyContinue)
    "  folder " + $d + ": " + $(if (Test-Path -LiteralPath $d) { "exists, " + $items.Count + " item(s): " + (($items | ForEach-Object Name) -join ", ") } else { "absent" })
  }
}
. { try {
  "date: " + (Get-Date -Format o)
  "PowerShell: " + $PSVersionTable.PSVersion
  "Windows: " + [Environment]::OSVersion.VersionString + "  64-bit OS: " + [Environment]::Is64BitOperatingSystem + "  64-bit process: " + [Environment]::Is64BitProcess
  $cv = Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion"
  "edition: " + $cv.EditionID + "  display version: " + $(if ($cv.DisplayVersion) { $cv.DisplayVersion } else { "absent" }) + "  build: " + $cv.CurrentBuild + "." + $cv.UBR
  $sac = Get-ItemProperty "HKLM:\SYSTEM\CurrentControlSet\Control\CI\Policy" -ErrorAction SilentlyContinue
  "Smart App Control (VerifiedAndReputablePolicyState; 0 off, 1 on, 2 evaluation): " + $(if ($null -ne $sac -and $null -ne $sac.VerifiedAndReputablePolicyState) { $sac.VerifiedAndReputablePolicyState } else { "absent" })
  $ss = Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer" -ErrorAction SilentlyContinue
  "SmartScreenEnabled (Explorer): " + $(if ($null -ne $ss -and $null -ne $ss.SmartScreenEnabled) { $ss.SmartScreenEnabled } else { "absent" })
  "Visual C++ runtime, x64 (Installed, Version):"
  $vc = Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\X64" -ErrorAction SilentlyContinue
  if ($vc) { "  Installed=" + $vc.Installed + "  Version=" + $vc.Version } else { "  key absent" }
  "vcruntime140.dll in System32: " + (Test-Path "$env:SystemRoot\System32\vcruntime140.dll")
  "installed programs named Visual C++:"
  $vcp = @(Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\*", "HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\*" -ErrorAction SilentlyContinue | Where-Object { $_.DisplayName -match "Visual C\+\+" })
  if ($vcp.Count) { $vcp | ForEach-Object { "  " + $_.DisplayName + "  " + $_.DisplayVersion } } else { "  none listed" }
  Show-Locks
  Show-Data "now"
} catch { "ERROR: " + ($_ | Out-String) } } *>&1 | Out-File -Encoding utf8 "$hub\fr2a-B0.txt"
```
An absent value is written "absent", never 0. If the runtime is present, the installed-programs list says which program most likely put it there. This PC having the runtime is expected; it is why a start with no runtime is a different question (U here).

## Step B1. Download by a click, then the file against its hash
Open `https://bridge.complyeaze.com/download.html` in Edge, click "Download for Windows", and download the `.sha256` file the page links. Do not use a script or `Invoke-WebRequest`: a script download carries no internet-zone mark and is a different test. If a browser or Windows prompt appears, choose Keep only; never Open or Run (on a PC with Claude Desktop that hands the file to the extension installer, which this sheet does not do). After the block below has run, open its file and add the line `prompt: yes` or `prompt: no` at the top; the words of the prompt stay in the person's notes and are not part of this sheet. Then set the two paths:
```powershell
$f = "<full path of the downloaded bridge-tally-mcp-v0.7.0-windows-x64.mcpb>"
$s = "<full path of the downloaded .sha256 file>"
$expected = "291a4ed95c758fad9fa04cd21a66be889e90a2fd03960fbceeec2c2a7242d4ec"
. { try {
  "size: " + (Get-Item -LiteralPath $f).Length + "   expected 15598419"
  $h = (Get-FileHash -Algorithm SHA256 -LiteralPath $f).Hash.ToLower()
  "Get-FileHash: " + $h
  "equals the release listing: " + ($h -eq $expected)
  "the .sha256 file, whole: "; Get-Content -LiteralPath $s
  "the .sha256 file holds the same hash: " + ((Get-Content -LiteralPath $s -Raw) -match $h)
  "the .sha256 file names this file: " + ((Get-Content -LiteralPath $s -Raw) -match [regex]::Escape((Split-Path $f -Leaf)))
  certutil -hashfile $f SHA256
} catch { "ERROR: " + ($_ | Out-String) } } *>&1 | Out-File -Encoding utf8 "$hub\fr2a-B1.txt"
```
`certutil` is the command the install text gives "For IT staff": its hash line must equal `Get-FileHash`'s. A file name with "(1)" in it makes the "names this file" line False and is not a hash mismatch; the hash lines decide. **A hash mismatch is a hard stop:** tell the person who queued this sheet and go no further.

## Step B2. The internet-zone mark
```powershell
. { try {
  foreach ($p in $f, $s) {
    "--- " + (Split-Path $p -Leaf)
    Get-Item -LiteralPath $p -Stream * | ForEach-Object { "stream " + $_.Stream + "  length " + $_.Length }
    $zone = Get-Content -LiteralPath $p -Stream Zone.Identifier -ErrorAction SilentlyContinue
    if ($zone) { $zone } else { "no Zone.Identifier stream (or it could not be read)" }
  }
} catch { "ERROR: " + ($_ | Out-String) } } *>&1 | Out-File -Encoding utf8 "$hub\fr2a-B2.txt"
```
Record the whole output. `ZoneId=3` is the internet zone. One browser copy only.

## Step B3. Is the file signed
```powershell
. { try {
  Get-AuthenticodeSignature -LiteralPath $f | Format-List Status, StatusMessage, SignerCertificate
} catch { "ERROR: " + ($_ | Out-String) } } *>&1 | Out-File -Encoding utf8 "$hub\fr2a-B3.txt"
```
A `.mcpb` is a zip: record the exact status and message Windows gives and do not interpret it. The same call is made on the unpacked program in step B4.

## Step B4. Unpack a copy in a scratch folder (nothing is installed)
```powershell
$w = Join-Path $env:TEMP ("fr2a-scratch-" + (Get-Date -Format yyyyMMddHHmmss)); New-Item -ItemType Directory $w | Out-Null
$bin = "$w\unpacked\bin\x86_64-pc-windows-msvc"
. { try {
  Copy-Item -LiteralPath $f "$w\bundle.zip"
  Expand-Archive "$w\bundle.zip" -DestinationPath "$w\unpacked"
  "files in the program folder:"; Get-ChildItem -LiteralPath $bin | ForEach-Object { $_.Name + "  " + $_.Length }
  Get-FileHash -Algorithm SHA256 "$bin\bridge_mcp.exe", "$bin\pdfium.dll" | ForEach-Object { (Split-Path $_.Path -Leaf) + "  " + $_.Hash.ToLower() }
  foreach ($x in "$bin\bridge_mcp.exe", "$bin\pdfium.dll") {
    "--- " + (Split-Path $x -Leaf)
    Get-AuthenticodeSignature -LiteralPath $x | Format-List Status, StatusMessage
    "Zone.Identifier on the unpacked copy: " + $(if (Get-Content -LiteralPath $x -Stream Zone.Identifier -ErrorAction SilentlyContinue) { "present" } else { "absent" })
  }
} catch { "ERROR: " + ($_ | Out-String) } } *>&1 | Out-File -Encoding utf8 "$hub\fr2a-B4.txt"
```
If the architecture folder is named differently, list `$w\unpacked`, set `$bin` to the folder that holds `bridge_mcp.exe`, run the block again, and write the difference at the top of the file. This is a copy made by `Expand-Archive`, not the copy Claude Desktop would make, so its Mark of the Web says nothing about the installed one.

## Step B5. The import table of the unpacked program
Paste this function first. It is the reader of sheet CRT1, reproduced here so that the sheet is whole; its offsets were checked there by porting the same steps to Python on an older Windows build of the program, and the PowerShell text has to be treated as unrun until this run's file shows it worked. If it errors, the error is the result: copy it and use `dumpbin /dependents` if that is present (do not install it).
```powershell
function Get-PeImports([string]$Path) {
  $b = [IO.File]::ReadAllBytes($Path)
  $pe = [BitConverter]::ToInt32($b, 0x3C)
  if ([Text.Encoding]::ASCII.GetString($b, $pe, 2) -ne "PE") { throw "not a PE file: $Path" }
  $sections = [BitConverter]::ToUInt16($b, $pe + 6)
  $optSize = [BitConverter]::ToUInt16($b, $pe + 20)
  $opt = $pe + 24
  if ([BitConverter]::ToUInt16($b, $opt) -ne 0x20B) { throw "not a 64-bit (PE32+) file: $Path" }
  $table = $opt + $optSize
  function ToOffset([uint32]$rva) {
    for ($i = 0; $i -lt $sections; $i++) {
      $s = $table + 40 * $i
      $va = [BitConverter]::ToUInt32($b, $s + 12); $raw = [BitConverter]::ToUInt32($b, $s + 16); $ptr = [BitConverter]::ToUInt32($b, $s + 20)
      $size = [Math]::Max([BitConverter]::ToUInt32($b, $s + 8), $raw)
      if ($rva -ge $va -and $rva -lt $va + $size) { return [int]($ptr + ($rva - $va)) }
    }
    throw "rva not in any section"
  }
  function Name([uint32]$rva) {
    $o = ToOffset $rva; $e = $o; while ($b[$e] -ne 0) { $e++ }
    [Text.Encoding]::ASCII.GetString($b, $o, $e - $o)
  }
  $out = @()
  $imp = [BitConverter]::ToUInt32($b, $opt + 120)   # data directory 1: imports
  if ($imp -ne 0) {
    $d = ToOffset $imp
    while ([BitConverter]::ToUInt32($b, $d + 12) -ne 0) { $out += "import:      " + (Name ([BitConverter]::ToUInt32($b, $d + 12))); $d += 20 }
  }
  $del = [BitConverter]::ToUInt32($b, $opt + 216)   # data directory 13: delay-load imports
  if ($del -ne 0) {
    $d = ToOffset $del
    while ([BitConverter]::ToUInt32($b, $d + 4) -ne 0) { $out += "delay-load:  " + (Name ([BitConverter]::ToUInt32($b, $d + 4))); $d += 32 }
  }
  $out | Sort-Object -Unique
}
```
Then:
```powershell
. { try {
  $rx = "(?i)^(import|delay-load):\s+(vcruntime|msvcp|api-ms-win-crt-|ucrtbase)"
  $main = Get-PeImports "$bin\bridge_mcp.exe"; "--- bridge_mcp.exe"; $main
  "lines: " + @($main).Count
  "runtime-family lines (names that begin VCRUNTIME, MSVCP, api-ms-win-crt-, ucrtbase): " + @($main | Where-Object { $_ -match $rx }).Count
  $pdf = Get-PeImports "$bin\pdfium.dll"; "--- pdfium.dll"; $pdf
  "lines: " + @($pdf).Count
  "runtime-family lines: " + @($pdf | Where-Object { $_ -match $rx }).Count
} catch { "ERROR: " + ($_ | Out-String) } } *>&1 | Out-File -Encoding utf8 "$hub\fr2a-B5.txt"
```
For the 0.7.0 `bridge_mcp.exe` sheet CRT1 listed 22 names, 12 of them in the runtime family (`VCRUNTIME140.dll` and eleven `api-ms-win-crt-*`); write the difference, or "same", at the end of the file.

## Step B6. What the file says it is
```powershell
. { try {
  $m = Get-Content -LiteralPath "$w\unpacked\manifest.json" -Raw | ConvertFrom-Json
  "name: " + $m.name + "   version: " + $m.version
  "settings keys: " + (($m.user_config.PSObject.Properties.Name) -join ", ")
  "environment passed to the program:"; $m.server.mcp_config.env.PSObject.Properties | ForEach-Object { "  " + $_.Name + " = " + $_.Value }
} catch { "ERROR: " + ($_ | Out-String) } } *>&1 | Out-File -Encoding utf8 "$hub\fr2a-B6.txt"
```
It must name version 0.7.0; if not, stop.

## Step B7. Start it and ask `tally_status`: Tally unreachable (#2393), and Terms off (#2392)
This starts the unpacked `bridge_mcp.exe` as its own process. Every inherited `BRIDGE_*` value is removed from its environment first, and its data folder is set to a new folder under the scratch folder, so it records no Terms acceptance and no receipt in the data folder of any installed extension (the program reads `BRIDGE_AGENT_DATA_DIR` for both). It reads no company: the port used has nothing listening, so it cannot reach any Tally. Posting is off, tax audit is off. It calls `tally_status` once per process, which takes no arguments. Windows PowerShell 5.1 can give a child's standard input a byte-order mark, which the server answers with a parse error (`-32700`; sheet CRT3 found this). So the function writes the requests through its own writer on the input stream, UTF-8 with no mark, and does not touch the console's encoding.
```powershell
function Invoke-Status([string]$Exe, [string]$Terms, [int]$Port) {
  $psi = New-Object Diagnostics.ProcessStartInfo $Exe
  $psi.UseShellExecute = $false; $psi.RedirectStandardInput = $true; $psi.RedirectStandardOutput = $true; $psi.RedirectStandardError = $true
  @($psi.EnvironmentVariables.Keys | Where-Object { $_ -like "BRIDGE_*" }) | ForEach-Object { $psi.EnvironmentVariables.Remove($_) }
  $data = Join-Path $w ("data-" + $Terms + "-" + $Port); New-Item -ItemType Directory $data -Force | Out-Null
  $psi.EnvironmentVariables["BRIDGE_AGENT_DATA_DIR"] = $data
  $psi.EnvironmentVariables["BRIDGE_TERMS_ACCEPTED"] = $Terms
  $psi.EnvironmentVariables["BRIDGE_TALLY_HOST"] = "localhost"
  $psi.EnvironmentVariables["BRIDGE_TALLY_PORT"] = [string]$Port
  $psi.EnvironmentVariables["BRIDGE_AGENT_REDACTION"] = "none"
  $psi.EnvironmentVariables["BRIDGE_AGENT_ENABLE_IMPORT"] = "true"
  $psi.EnvironmentVariables["BRIDGE_AGENT_ENABLE_WRITES"] = "false"
  $psi.EnvironmentVariables["BRIDGE_AGENT_ENABLE_TAX_AUDIT"] = "false"
  $p = [Diagnostics.Process]::Start($psi)
  try {
    $errTask = $p.StandardError.ReadToEndAsync()
    $init = '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"lab-first-run","version":"1.0.0"}}}'
    $note = '{"jsonrpc":"2.0","method":"notifications/initialized"}'
    $call = '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"tally_status","arguments":{}}}'
    $sw = New-Object IO.StreamWriter($p.StandardInput.BaseStream, (New-Object Text.UTF8Encoding $false))
    $sw.NewLine = "`n"
    foreach ($line in $init, $note, $call) { $sw.WriteLine($line) }
    $sw.Flush()
    $replies = @()
    foreach ($n in 1, 2) {
      $task = $p.StandardOutput.ReadLineAsync()
      if (-not $task.Wait(30000)) { $p.Kill(); $errTask.Wait(2000) | Out-Null; throw ("no reply " + $n + " within 30 s; standard error: " + $(if ($errTask.IsCompleted) { $errTask.Result } else { "not yet read" })) }
      if ($null -eq $task.Result) { $p.WaitForExit(5000) | Out-Null; $errTask.Wait(2000) | Out-Null; throw ("reply " + $n + ": the output stream closed; " + $(if ($p.HasExited) { "exit code " + $p.ExitCode } else { "process still running" }) + "; standard error: " + $(if ($errTask.IsCompleted) { $errTask.Result } else { "not yet read" })) }
      $replies += $task.Result
    }
    $sw.Close()
    $exit = if (-not $p.WaitForExit(20000)) { $p.Kill(); "did not exit within 20 s after stdin closed" } else { "exit code " + $p.ExitCode }
    "--- Terms=" + $Terms + " port=" + $Port + ": " + $exit
    "initialize reply: " + $replies[0]
    $errTask.Wait(2000) | Out-Null
    $stderr = if ($errTask.IsCompleted) { $errTask.Result } else { "not yet read" }
    if ($stderr.Length -gt 4096) { $stderr = $stderr.Substring(0, 4096) + " [cut at 4096 characters]" }
    "standard error: " + $(if ($stderr) { $stderr } else { "empty" })
    "tally_status reply, raw: " + $replies[1]
    try { if (($replies[0] | ConvertFrom-Json).error) { "NOTE: initialize answered with a protocol error; the answer below is not a result" } } catch { "initialize reply is not JSON: " + $_.Exception.Message }
    try {
      $r1 = $replies[1] | ConvertFrom-Json
      if ($r1.error) { "NOTE: the call answered with a protocol error (not the tool's own refusal)" }
      "tool flagged its own answer as an error (isError): " + $r1.result.isError
      $text = ($r1.result.content | ForEach-Object { $_.text }) -join "`n"
      "tally_status reply, the text a person's assistant is given:"; $text
    } catch { "tally_status reply is not JSON: " + $_.Exception.Message }
  } finally { try { if (-not $p.HasExited) { $p.Kill() } } catch { } }
}
$port = 9099
if (Get-NetTCPConnection -State Listen -LocalPort $port -ErrorAction SilentlyContinue) { $port = 9199 }
$portBusy = [bool](Get-NetTCPConnection -State Listen -LocalPort $port -ErrorAction SilentlyContinue)
. { try {
  "closed port used: " + $port
  if ($portBusy) { "ERROR: ports 9099 and 9199 both have a listener; nothing was started. Choose another port, set the variable port to it, check it the same way and run this block again." }
  else {
    Show-Locks
    Show-Data "before"
    try { Invoke-Status "$bin\bridge_mcp.exe" "true" $port } catch { "ERROR (Terms=true run): " + $_.Exception.Message }
    Start-Sleep -Seconds 15
    try { Invoke-Status "$bin\bridge_mcp.exe" "false" $port } catch { "ERROR (Terms=false run): " + $_.Exception.Message }
    Show-Locks
    Show-Data "after"
    "data folders of the two runs (names, lengths, and the Terms record, which holds a time and a Terms version):"
    Get-ChildItem -LiteralPath $w -Directory -Filter "data-*" | ForEach-Object { "  " + $_.Name; Get-ChildItem -LiteralPath $_.FullName -Recurse -File | ForEach-Object { "    " + $_.Name + "  " + $_.Length }; $tr = Join-Path $_.FullName "terms-acceptance.jsonl"; if (Test-Path -LiteralPath $tr) { "    terms-acceptance.jsonl: " + (Get-Content -LiteralPath $tr -Raw) } else { "    terms-acceptance.jsonl: absent" } }
  }
} catch { "ERROR: " + ($_ | Out-String) } } *>&1 | Out-File -Encoding utf8 "$hub\fr2a-B7.txt"
```
Everything goes to one file, `fr2a-B7.txt`: the port line, the lock files and the installed extension's data files (name, length, last write time) before and after, both runs (exit line, initialize reply, standard error capped at 4 KB, raw reply, the tool's own error flag and text) and the lock files before and after. A line that starts "ERROR" is a result, not a reason to retry: a start that Windows or an antivirus product stopped appears here as an error from starting the process, and then the block rule (limit 4) applies. To run only B7 again after a mistake in setting up, paste the `. { try {` part alone, so the port choice made above is kept. After all the steps have run, search every output for the words the block rule names:
```powershell
Select-String -Path "$hub\fr2a-B*.txt" -Pattern 'virus|malware|threat|quarantin|blocked|block[ .:]|Application Control|SmartScreen|Defender|Windows Security|protected your PC|Smart App Control|0x800700E1|not allowed|access is denied' | ForEach-Object { $_.Filename + ":" + $_.LineNumber + ": " + $_.Line }
```
Copy what it prints into the note. A line that starts "ERROR:" and a hit together, outside the `tally_status` reply text, mean `block: yes` at the top of that file. A hit only inside the `tally_status` reply text (the program's own words) is not a block. An "ERROR:" line from starting the process is a block whatever the hit says. Do not repeat a call to look again.

## Step B8. Clean up
```powershell
if ($w -like "*\fr2a-scratch-*") { Remove-Item -Recurse -Force -LiteralPath $w } else { "not deleting: `$w is not a scratch folder of step B4: " + $w }
```
Delete only the scratch folder `$w` of step B4, after the files above are saved. The downloaded `.mcpb` and `.sha256` stay where they are, and so does `C:\Users\Public\fr2a-hub-only`: it is handed over by hand. Delete nothing else.

## What comes back
The folder `C:\Users\Public\fr2a-hub-only` with `fr2a-B0.txt` to `fr2a-B7.txt`, whole, and a note of one line each, marked M or U with what was missing:
1. The file's hash equals the release listing's and the `.sha256` file's (B1).
2. A download by click gave the file the internet-zone mark: yes or no (B2).
3. Windows reports the file and the program as signed or not (B3, B4).
4. The program's import table: line count and runtime-family count, against CRT1's 22 and 12 (B5).
5. The words the program gave with Tally unreachable (#2393), and with the Terms off (#2392) (B7).
6. Whether any step hit the block rule.

The lab's reviewer reads the folder. What the result decides (not part of the run): the install text may say only what a step measured; a step marked U stays out of the text.
