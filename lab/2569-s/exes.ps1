# Lab helper (row 2569-S; not product code). ASCII. Read-only: lists every file named bridge_mcp.exe under -Root
# with its size, SHA-256 and which build line it equals, so the sheet can name a path by hash, never by memory.
# Usage: powershell -NoProfile -File exes.ps1 -Root C:\bridge-lab -Out <new file>
# Exit: 0 listed with no error; 3 listed, but some folder could not be read (the list may be incomplete: a path
# missing from it is then unknown, not absent); 2 the root is missing or the listing failed.
param([string]$Root, [string]$Out)
$ErrorActionPreference = 'Stop'
$Out = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($Out)
if (Test-Path -LiteralPath $Out) { throw "exists: $Out" }
$old = 'c437138a08dc10a8609e42d28104920401b4615daa69f3cdf404f6e37d6323dd'
$new = '1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59'
$lines = New-Object 'System.Collections.Generic.List[string]'
$lines.Add('exes under ' + $Root + ' read at ' + [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ss.fffZ'))
if (-not (Test-Path -LiteralPath $Root -PathType Container)) {
  $lines.Add('RESULT ROOT_MISSING')
  [System.IO.File]::WriteAllLines($Out, $lines)
  exit 2
}
$errs = @()
$files = @(Get-ChildItem -LiteralPath $Root -Recurse -Force -File -Filter 'bridge_mcp.exe' -ErrorAction SilentlyContinue -ErrorVariable +errs)
$lines.Add('found ' + $files.Count + ' errors ' + $errs.Count)
$i = 0
foreach ($e in $errs) {
  $i++
  if ($i -gt 5) { $lines.Add('errors omitted ' + ($errs.Count - 5)); break }
  $lines.Add('ERROR ' + $e.CategoryInfo.Category + ' ' + [string]$e.TargetObject)
}
$i = 0
foreach ($f in $files) {
  $i++
  if ($i -gt 40) { $lines.Add('files omitted ' + ($files.Count - 40)); break }
  $hash = (Get-FileHash -LiteralPath $f.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
  $label = 'OTHER'
  if ($hash -eq $old) { $label = 'OLD' } elseif ($hash -eq $new) { $label = 'NEW' }
  $lines.Add('EXE ' + $label + ' ' + $hash + ' ' + $f.Length + ' ' + $f.LastWriteTimeUtc.ToString('yyyy-MM-ddTHH:mm:ssZ') + ' ' + $f.FullName)
}
$rc = 0
if ($errs.Count -gt 0) { $rc = 3 }
$lines.Add('RESULT ' + $(if ($rc -eq 0) { 'OK' } else { 'INCOMPLETE' }) + ' exit ' + $rc)
[System.IO.File]::WriteAllLines($Out, $lines)
Write-Output ('wrote ' + $Out + ' exit ' + $rc)
exit $rc
