# Lab diagnostic for sheet CRT2 (not product code), run after the sheet's steps 3 and 4 as written: the sheet's Invoke-Mcp with
# one change, standard input written as UTF-8 with no byte-order mark (Windows PowerShell 5.1's console input encoding here is
# UTF-8 with a 3-byte preamble, and the as-written runs of both files answered the first line with a JSON-RPC parse error).
# It also returns every tool name and every module name (Format-List shortened the as-written module lists).
# Usage: powershell -NoProfile -File diag.ps1 <row folder>
param([string]$Root)
$FormatEnumerationLimit = -1
[Console]::InputEncoding = New-Object Text.UTF8Encoding $false
function Invoke-McpDiag([string]$Exe) {
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
  $exit = if (-not $p.WaitForExit(20000)) { $p.Kill(); "did not exit within 20 s after stdin closed" } else { "exit code " + $p.ExitCode }
  $tools = ($replies[1] | ConvertFrom-Json).result.tools
  [pscustomobject]@{ Exit = $exit; InitializeReply = $replies[0]; ToolCount = $tools.Count; ToolNames = ($tools | ForEach-Object name) -join ", "; ModuleCount = @($modules).Count; Modules = $modules }
}
# Then, for the new file (s1\unpacked) and the 0.7.0 file (s4\unpacked-070): run Invoke-McpDiag, write evidence\diag-start-<new|070>.txt,
# and compare the two module lists and tool lists with Compare-Object into evidence\diag-s5-compare.txt.
