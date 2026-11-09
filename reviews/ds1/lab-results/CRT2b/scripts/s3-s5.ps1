# Lab helper (CRT2b, item 19): steps 3, 4 and 5 of sheet CRT2 at ea931452, in one Windows PowerShell 5.1 session, run from the
# scratch folder w. Each block is the sheet's text as written, in order; the only line added before them loads Get-PeImports, the
# step 2 reader (CRT1's, a byte copy), which step 5 calls. Markers "=== step N" are printed between the blocks.
. "$PSScriptRoot\reader.ps1"
"=== step 3"
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
"=== step 4"
Invoke-Mcp ".\unpacked-070\bin\x86_64-pc-windows-msvc\bridge_mcp.exe"
$oldFiles = $script:LastModuleFiles
"=== step 5"
$leaf = { param($a) @($a | ForEach-Object { [IO.Path]::GetFileName($_).ToLower() }) }
"loaded by the new file only:"; Compare-Object (& $leaf $newFiles) (& $leaf $oldFiles) | Where-Object SideIndicator -eq "<=" | ForEach-Object InputObject
"loaded by the 0.7.0 file only:"; Compare-Object (& $leaf $newFiles) (& $leaf $oldFiles) | Where-Object SideIndicator -eq "=>" | ForEach-Object InputObject
"modules loaded by the NEW file that import ucrtbase, an api-ms-win-crt-* set, vcruntime or msvcp140:"
foreach ($f in $newFiles) {
  if ([IO.Path]::GetFileName($f) -eq "bridge_mcp.exe") { continue }
  try { $hits = @(Get-PeImports $f | Where-Object { $_ -match "ucrtbase|api-ms-win-crt-|vcruntime|msvcp140" }) } catch { $hits = @("unreadable: " + $_.Exception.Message) }
  if ($hits.Count) { "  " + [IO.Path]::GetFileName($f) + ": " + ($hits -join "; ") }
}
Get-Item "$env:SystemRoot\System32\ucrtbase.dll" | Select-Object @{n="Name";e={$_.Name}}, @{n="ProductVersion";e={$_.VersionInfo.ProductVersion}}, @{n="FileDescription";e={$_.VersionInfo.FileDescription}} | Format-List
"=== end"
