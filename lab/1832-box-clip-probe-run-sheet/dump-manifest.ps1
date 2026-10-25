# Lab script (not product code): W1. Print the application manifest(s) embedded in an executable and say whether any
# declares DPI awareness. Reads the file's bytes only; runs nothing; touches nothing. Written without a PowerShell to test it
# on: if it prints nothing or fails, use the manual check in the sheet (Task Manager, Details, "DPI awareness" column).
# Usage: powershell -NoProfile -ExecutionPolicy Bypass -File dump-manifest.ps1 -Exe <path to bridge_mcp.exe>
param([Parameter(Mandatory=$true)][string]$Exe)
$ErrorActionPreference = 'Stop'
$bytes = [System.IO.File]::ReadAllBytes($Exe)
$sha = (Get-FileHash -Algorithm SHA256 -LiteralPath $Exe).Hash.ToLower()
Write-Host "exe=$Exe bytes=$($bytes.Length) sha256=$sha"
$text = [System.Text.Encoding]::GetEncoding(28591).GetString($bytes)   # Latin-1: one char per byte
$found = [regex]::Matches($text, '<assembly[\s\S]*?</assembly>')
Write-Host "manifests found (UTF-8 or ASCII): $($found.Count)"
$i = 0
foreach ($m in $found) {
  $i++
  Write-Host "----- manifest $i ($($m.Length) characters) -----"
  Write-Host $m.Value
}
$wide = [System.Text.Encoding]::GetEncoding(28591).GetString([System.Text.Encoding]::Unicode.GetBytes('dpiAware'))
Write-Host "-----"
foreach ($word in @('dpiAware', 'dpiAwareness', 'PerMonitor', 'longPathAware', 'activeCodePage', 'Microsoft.Windows.Common-Controls', 'supportedOS')) {
  Write-Host ("{0,-40} utf8/ascii occurrences: {1}" -f $word, ([regex]::Matches($text, [regex]::Escape($word))).Count)
}
Write-Host ("dpiAware as UTF-16 occurrences: {0}" -f ([regex]::Matches($text, [regex]::Escape($wide))).Count)
$declares = ($text -match 'dpiAware') -or ($text -match 'dpiAwareness')
Write-Host ("RESULT: the file {0} a dpiAware / dpiAwareness setting." -f $(if ($declares) { 'CONTAINS' } else { 'DOES NOT CONTAIN' }))
Write-Host 'NOTE: a missing setting means the process is DPI-unaware unless the OS or a compatibility setting says otherwise; the running process is read in Task Manager (sheet, W1b).'
