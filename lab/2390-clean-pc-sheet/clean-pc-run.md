# Sheet CRT3: the Windows MCP server with the C runtime linked in, on a PC with no Visual C++ runtime, for #2390

Purpose: check, on a real Windows PC that has no Visual C++ Redistributable installed, that the new `bridge_mcp.exe` starts and answers `initialize` and `tools/list`. The earlier sheet (CRT2) ran on a PC that has the runtime, so it could show only the import table and the loaded-module list; this sheet is the start that sheet could not make. Nothing here starts Tally, installs, uninstalls or changes a setting. **Do not install anything on this PC for the run, including the Visual C++ Redistributable, the desktop app, a browser extension or a tool.** If the PC is not clean (step 0), stop there and say so: that is a result, not a failure. Record every output in full; do not tidy it.

Steps 1 to 5 are the steps of sheet CRT2 as they stand after its first run, with one change: the two hash listings in step 1 are written with `Format-Table` so the full sha256 and the file name both print. Step 0 is new, and step 4 is a control that is expected to fail to start. If something does not run as written, that is a finding.

Marks used in the result: M = measured by this run.

## Step 0. Prove the PC is clean (read only)
The loader looks for `vcruntime140.dll` in the program's folder, then in System32, then along `PATH`. Check all three, and the installed-programs list. Nothing is changed.
```powershell
$names = "vcruntime140.dll", "vcruntime140_1.dll", "msvcp140.dll", "concrt140.dll", "vcomp140.dll"
"Windows: " + [Environment]::OSVersion.VersionString + "  64-bit OS: " + [Environment]::Is64BitOperatingSystem
foreach ($dir in "$env:SystemRoot\System32", "$env:SystemRoot\SysWOW64") {
  foreach ($n in $names) { "{0,-45} {1}" -f (Join-Path $dir $n), (Test-Path (Join-Path $dir $n)) }
}
"PATH folders holding any of them:"
$env:Path -split ';' | Where-Object { $_ } | ForEach-Object { foreach ($n in $names) { if (Test-Path (Join-Path $_ $n)) { "  " + (Join-Path $_ $n) } } }
"installed programs named Visual C++:"
Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\*", "HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\*" -ErrorAction SilentlyContinue | Where-Object { $_.DisplayName -match "Visual C\+\+" } | ForEach-Object { "  " + $_.DisplayName + "  " + $_.DisplayVersion }
Get-Item "$env:SystemRoot\System32\ucrtbase.dll" | ForEach-Object { "ucrtbase.dll: " + $_.VersionInfo.ProductVersion + "  " + $_.VersionInfo.FileDescription }
```
Record the whole output. The PC is clean for this sheet only if every `Test-Path` line reads `False`, the two lists after them are empty, and `ucrtbase.dll` exists (it is a Windows component on Windows 10 and later). If any file is present or any Visual C++ entry is listed, stop: the PC is not clean, and say which line shows it. If `ucrtbase.dll` is absent (an older Windows without the Universal CRT update), say so and carry on: the run then measures that case, which is a different question from a PC that only lacks the Visual C++ runtime, and the result must say which one it was.

## The input
The file under test is `bridge-tally.mcpb` from the CI run of the pull request for #2390, artifact `unsigned-bundle-smoke-windows-latest`. The person who queues this sheet hands the lab that file together with two values in the queue entry: the pull request head it was built from, and the file's sha256. The sheet does not download anything. If either value is missing from the queue entry, stop and say so. Check the file's sha256 with `Get-FileHash -Algorithm SHA256` against the queue entry's value before anything else, and record whether they are equal.

## Step 1. Unpack a copy, in a scratch folder
A `.mcpb` is a zip. In a new empty folder under the lab's scratch area (a plain folder copy and unzip: nothing is installed):
```powershell
Copy-Item "<handed bridge-tally.mcpb>" .\bundle.zip
Expand-Archive .\bundle.zip -DestinationPath .\unpacked
Get-ChildItem .\unpacked\bin\x86_64-pc-windows-msvc | Select-Object Name, Length
Get-FileHash -Algorithm SHA256 .\unpacked\bin\x86_64-pc-windows-msvc\bridge_mcp.exe, .\unpacked\bin\x86_64-pc-windows-msvc\pdfium.dll | Format-Table -AutoSize Hash, @{n='File';e={Split-Path $_.Path -Leaf}}
```
Record the two files' sizes and sha256. The 0.7.0 `bridge_mcp.exe` was 32,705,536 bytes; write down the difference.

## Step 2. The import table of the new file
Use the reader that sheet CRT1 used (`Get-PeImports`, the text in `lab/2390-crt-sheet/crt-imports.md`, step 3; it ran there without error). If `dumpbin` is present, run `dumpbin /dependents` too; do not install it.
```powershell
Get-PeImports ".\unpacked\bin\x86_64-pc-windows-msvc\bridge_mcp.exe"
Get-PeImports ".\unpacked\bin\x86_64-pc-windows-msvc\pdfium.dll"
```
Record the full output of both, and the count of lines. Then count, by name, the lines that begin `VCRUNTIME`, `MSVCP`, `api-ms-win-crt-` or `ucrtbase` (case does not matter) in each output, and write the number even when it is 0. For 0.7.0 the first file listed 22 names, 12 of them in that family (`VCRUNTIME140.dll` and eleven `api-ms-win-crt-*`).

## Step 3. Start it and ask for `tools/list`, with the loaded modules
This starts the new `bridge_mcp.exe` with no environment set, so it is not accepted for any terms and writes no data. It writes two requests and a notification to standard input, as the bundle's own smoke test does.

Windows PowerShell 5.1 gives the child's standard input the console input encoding, which can be UTF-8 **with** a 3-byte byte-order mark. The server then answers the first line with a JSON-RPC parse error (`-32700`) and the tool count reads 0: this happened on the lab PC when the first version of this sheet ran, for the new file and for the 0.7.0 file alike. So the function sets the encoding to UTF-8 with no mark before it starts the process, and prints the preamble length before and after (the second must be 0). It also prints every tool and module name on its own line, because `Format-List` cuts a long list.
```powershell
$FormatEnumerationLimit = -1
"console input preamble bytes before: " + [Console]::InputEncoding.GetPreamble().Length
[Console]::InputEncoding = New-Object Text.UTF8Encoding $false
"console input preamble bytes after:  " + [Console]::InputEncoding.GetPreamble().Length
function Invoke-Mcp([string]$Exe) {
  $psi = New-Object Diagnostics.ProcessStartInfo $Exe
  $psi.UseShellExecute = $false; $psi.RedirectStandardInput = $true; $psi.RedirectStandardOutput = $true; $psi.RedirectStandardError = $true
  $p = [Diagnostics.Process]::Start($psi)
  $init = '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"lab-runtime-check","version":"1.0.0"}}}'
  $note = '{"jsonrpc":"2.0","method":"notifications/initialized"}'
  $list = '{"jsonrpc":"2.0","id":2,"method":"tools/list"}'
  foreach ($line in $init, $note, $list) { $p.StandardInput.WriteLine($line) }
  $p.StandardInput.Flush()
  $replies = @()
  foreach ($n in 1, 2) {
    $task = $p.StandardOutput.ReadLineAsync()
    if (-not $task.Wait(20000)) { $p.Kill(); throw "no reply $n within 20 s" }
    $replies += $task.Result
  }
  $files = @($p.Modules | ForEach-Object { $_.FileName } | Sort-Object -Unique)
  $p.StandardInput.Close()
  $exit = if (-not $p.WaitForExit(20000)) { $p.Kill(); "did not exit within 20 s after stdin closed" } else { "exit code " + $p.ExitCode }
  "--- " + [IO.Path]::GetFileName($Exe) + ": " + $exit
  "initialize reply: " + $replies[0]
  if ($replies[0] -match '"error"') { "NOTE: the server answered with an error; the tool count below is not a result" }
  $tools = ($replies[1] | ConvertFrom-Json).result.tools
  "tool count: " + @($tools).Count
  "tools: " + (($tools | ForEach-Object name) -join ", ")
  "module count: " + $files.Count
  $files | ForEach-Object { "  module: " + [IO.Path]::GetFileName($_) }
  $script:LastModuleFiles = $files
}
Invoke-Mcp ".\unpacked\bin\x86_64-pc-windows-msvc\bridge_mcp.exe"
$newFiles = $script:LastModuleFiles
```
Record the whole output: the two preamble lines, the exit line, the initialize reply, the tool count and names, and every module name. The module list is read while the process is running. If the initialize reply is an error, the run is not a result: say so and stop.

Then list any loaded module in the runtime family, by name, writing the number even when it is 0:
```powershell
$runtimeFamily = @($newFiles | ForEach-Object { [IO.Path]::GetFileName($_) } | Where-Object { $_ -match '^(vcruntime|msvcp|concrt|vcomp)' })
"runtime-family modules loaded by the new file: " + $runtimeFamily.Count
$runtimeFamily
```
On a clean PC the expected answer is 0 and a normal reply; a Windows error window or a process that exits at once is the other possible result, and is recorded as it appears (step 4 says how).


## Step 4. The control: the 0.7.0 file on the same PC (optional; skip it if the lab no longer has the 0.7.0 `.mcpb`)
The 0.7.0 `bridge_mcp.exe` imports `VCRUNTIME140.dll` (sheet CRT1), so on a clean PC it is expected not to start. This is the evidence that step 0's PC really lacks the runtime, and that a start of the new file is a difference and not an accident of the PC. Run it after step 3, never before. Unpack a copy of the 0.7.0 `.mcpb` the same way in the scratch folder (not the installed extension). Windows may show an error window naming the missing DLL; leave it, write down its title and text word for word, then close it.
```powershell
try { Invoke-Mcp ".\unpacked-070\bin\x86_64-pc-windows-msvc\bridge_mcp.exe" } catch { "0.7.0 file: " + $_.Exception.Message }
```
Also write down the process exit code if one is printed (a missing DLL at load time ends a process with `-1073741515`, hex `C0000135`). If the 0.7.0 file starts and answers, say so plainly: then step 0's PC is not clean in the way the loader sees it, and the new file's start proves nothing.

## Step 5. Who loads `ucrtbase.dll`
On a PC without the Visual C++ runtime the new file can still load `ucrtbase.dll`: it is a Windows component, and some system DLLs import it. This step names which ones, so the result says where it comes from.
```powershell
"modules loaded by the NEW file that import ucrtbase, an api-ms-win-crt-* set, vcruntime or msvcp140:"
foreach ($f in $newFiles) {
  if ([IO.Path]::GetFileName($f) -eq "bridge_mcp.exe") { continue }
  try { $hits = @(Get-PeImports $f | Where-Object { $_ -match "ucrtbase|api-ms-win-crt-|vcruntime|msvcp140" }) } catch { $hits = @("unreadable: " + $_.Exception.Message) }
  if ($hits.Count) { "  " + [IO.Path]::GetFileName($f) + ": " + ($hits -join "; ") }
}
```
`Get-PeImports` is the step 2 reader and reads only the system files named by the process's own module list. Record the output, including an empty list.

## Record (save under `reviews/ds1/lab-results/CRT3/`)
1. Date, Windows version, whether `ucrtbase.dll` exists, the pull request head and the handed file's sha256 with the match result.
2. Step 0 output in full, and the one-line verdict: clean, or not clean and which line shows it.
3. Sizes and sha256 of the new `bridge_mcp.exe` and `pdfium.dll`.
4. Full import output for both files, the line count, and the count of runtime-family names in each.
5. Full output of step 3, including the two preamble lines, the exit line, the tool count and names, every module name and the runtime-family count.
6. Step 4 output, the error window's title and text if one appeared, and the exit code; or "skipped" with the reason.
7. The step 5 list.
8. One line: on a PC with no Visual C++ runtime, the new file started and answered `tools/list` with N tools, or it did not (with the window text or the reply it gave). Anything that did not run as written is a finding, not an error to hide.

What the result decides (not part of the run): with a clean step 0, 0 runtime-family modules and a normal `tools/list`, the file starts on a PC with no Visual C++ runtime, which is the claim the pull request makes; calls beyond `tools/list` remain unmeasured and are named as such. If the 0.7.0 control also started, the PC was not clean and the run decides nothing. A start of the new file that needs a file outside Windows itself is a finding for the pull request.
