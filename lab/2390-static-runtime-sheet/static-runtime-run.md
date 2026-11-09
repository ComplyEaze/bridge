# Sheet CRT2: the Windows MCP server with the C runtime linked in (reads, and a start with no Tally), for #2390

Purpose: check, on a real Windows PC, that the new `bridge_mcp.exe` imports none of the Visual C++ or Universal CRT DLLs, and that it still starts and answers `tools/list`. Nothing here starts Tally, installs, uninstalls or changes a setting. The lab PC has the Visual C++ runtime installed (the earlier sheet found it in System32), so a start here cannot by itself show that the file works without it; the import table and the loaded-module list are the evidence, and the clean-PC run stays a separate step. Record every output in full; do not tidy it.

Marks used in the result: M = measured by this run.

## The input
The file under test is `bridge-tally.mcpb` from the CI run of the pull request for #2390, artifact `unsigned-bundle-smoke-windows-latest`. The person who queues this sheet hands the lab that file together with two values in the queue entry: the pull request head it was built from, and the file's sha256. The sheet does not download anything. If either value is missing from the queue entry, stop and say so. Check the file's sha256 with `Get-FileHash -Algorithm SHA256` against the queue entry's value before anything else, and record whether they are equal.

## Step 1. Unpack a copy, in a scratch folder
A `.mcpb` is a zip. In a new empty folder under the lab's scratch area:
```powershell
Copy-Item "<handed bridge-tally.mcpb>" .\bundle.zip
Expand-Archive .\bundle.zip -DestinationPath .\unpacked
Get-ChildItem .\unpacked\bin\x86_64-pc-windows-msvc | Select-Object Name, Length
Get-FileHash -Algorithm SHA256 .\unpacked\bin\x86_64-pc-windows-msvc\bridge_mcp.exe, .\unpacked\bin\x86_64-pc-windows-msvc\pdfium.dll
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
This starts the new `bridge_mcp.exe` with no environment set, so it is not accepted for any terms and writes no data. It reads two JSON-RPC lines and a notification from standard input, as the bundle's own smoke test does.
```powershell
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
  $modules = $p.Modules | ForEach-Object { $_.ModuleName } | Sort-Object -Unique
  $p.StandardInput.Close()
  if (-not $p.WaitForExit(20000)) { $p.Kill(); "did not exit within 20 s after stdin closed" } else { "exit code " + $p.ExitCode }
  $tools = ($replies[1] | ConvertFrom-Json).result.tools
  [pscustomobject]@{ InitializeReply = $replies[0]; ToolCount = $tools.Count; FirstTools = ($tools | Select-Object -First 5 -ExpandProperty name) -join ", "; Modules = $modules }
}
Invoke-Mcp ".\unpacked\bin\x86_64-pc-windows-msvc\bridge_mcp.exe" | Format-List
```
Record the whole output: the exit line, the initialize reply, the tool count, and every module name. The module list is read while the process is running.

## Step 4. The control: the same start on the 0.7.0 file
Without this, an empty module list proves nothing. Run the same function on the 0.7.0 `bridge_mcp.exe` that sheet CRT1 measured (the lab kept the 0.7.0 `.mcpb`; unpack a copy the same way in the scratch folder, not the installed extension).
```powershell
Invoke-Mcp ".\unpacked-070\bin\x86_64-pc-windows-msvc\bridge_mcp.exe" | Format-List
```
Record whether `vcruntime140.dll` appears in its module list. If it does not, say so: then the module list cannot show a difference and only the import table counts.

## Step 5. Compare
Write the modules that the new file loaded and the 0.7.0 file did not, and the modules the 0.7.0 file loaded and the new one did not. Name each one. `ucrtbase.dll` may be loaded by Windows components that are not ours, so a `ucrtbase.dll` in the new file's list is recorded, not judged.

## Record (save under `reviews/ds1/lab-results/CRT2/`)
1. Date, Windows version, the pull request head and the handed file's sha256 with the match result.
2. Sizes and sha256 of the new `bridge_mcp.exe` and `pdfium.dll`, and the size difference from 0.7.0.
3. Full import output for both files, the line count, and the count of runtime-family names in each.
4. Full output of step 3 and step 4, including each module list.
5. The step 5 comparison.
6. One line: the new file asks for the C runtime (names) or does not; it started and answered `tools/list` with N tools or it did not. Anything that did not run as written is a finding, not an error to hide.

What the result decides (not part of the run): with 0 runtime-family imports and a normal `tools/list`, the file is ready for a run on a PC with no Visual C++ runtime installed. With any runtime-family import, the build flag did not reach this file and the pull request is wrong.
