# Lab helper (CRT2b, item 19): the input check, step 1 and step 2 of sheet CRT2 at ea931452, run from the scratch folder w.
# The sheet's own lines are kept as written; only "<handed bridge-tally.mcpb>" is replaced by the kept file's path.
param([string]$Step)
$Handed = "<lab folder>\2390-crt2\ex\bridge-tally.mcpb"
$Zip = "<lab folder>\2390-crt2\dl\artifact.zip"
$Control = "<lab folder>\rc070-post\bridge-tally-mcp-v0.7.0-windows-x64.mcpb"
. "$PSScriptRoot\reader.ps1"
if ($Step -eq "s0") {
  "date UTC " + (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")
  Get-CimInstance Win32_OperatingSystem | Format-List Caption, Version, BuildNumber, OSArchitecture
  $q = @{ zip = "d5bc5aed099b3720d65242fb816443a50dfb470b21b2c685db9e8328df376924"; mcpb = "f52d8315278220ac698399287b09d2ec029f008da037aed11d2f15ded84920dd" }
  $z = (Get-FileHash -Algorithm SHA256 $Zip).Hash.ToLower()
  "artifact zip " + (Get-Item $Zip).Length + " B sha256 " + $z + " ; queue (item 16's full value) " + $q.zip + " ; equal " + ($z -eq $q.zip)
  $m = (Get-FileHash -Algorithm SHA256 $Handed).Hash.ToLower()
  "handed mcpb " + (Get-Item $Handed).Length + " B sha256 " + $m + " ; queue (item 16's full value) " + $q.mcpb + " ; equal " + ($m -eq $q.mcpb)
}
if ($Step -eq "s1") {
  Copy-Item "$Handed" .\bundle.zip
  Expand-Archive .\bundle.zip -DestinationPath .\unpacked
  Get-ChildItem .\unpacked\bin\x86_64-pc-windows-msvc | Select-Object Name, Length
  Get-FileHash -Algorithm SHA256 .\unpacked\bin\x86_64-pc-windows-msvc\bridge_mcp.exe, .\unpacked\bin\x86_64-pc-windows-msvc\pdfium.dll
  $e = (Get-Item .\unpacked\bin\x86_64-pc-windows-msvc\bridge_mcp.exe).Length
  "bridge_mcp.exe " + $e + " B; 0.7.0 was 32705536 B; difference " + ($e - 32705536) + " B"
  "exe sha256 equal to the queue's 1e9a2328dee09365bdd787ce7c143a997c9eb23c092105e3b4801e444e6a6035: " + ((Get-FileHash -Algorithm SHA256 .\unpacked\bin\x86_64-pc-windows-msvc\bridge_mcp.exe).Hash.ToLower() -eq "1e9a2328dee09365bdd787ce7c143a997c9eb23c092105e3b4801e444e6a6035")
}
if ($Step -eq "s1c") {
  # step 4's control, unpacked the same way in the scratch folder (not the installed extension)
  $c = (Get-FileHash -Algorithm SHA256 $Control).Hash.ToLower()
  "0.7.0 mcpb " + (Get-Item $Control).Length + " B sha256 " + $c + " ; CRT1 291a4ed95c758fad9fa04cd21a66be889e90a2fd03960fbceeec2c2a7242d4ec ; equal " + ($c -eq "291a4ed95c758fad9fa04cd21a66be889e90a2fd03960fbceeec2c2a7242d4ec")
  Copy-Item "$Control" .\bundle-070.zip
  Expand-Archive .\bundle-070.zip -DestinationPath .\unpacked-070
  $x = Get-Item .\unpacked-070\bin\x86_64-pc-windows-msvc\bridge_mcp.exe
  "0.7.0 bridge_mcp.exe " + $x.Length + " B sha256 " + (Get-FileHash -Algorithm SHA256 $x.FullName).Hash.ToLower() + " ; CRT1 48d2f7430b26da0d8f670eb74a27df46b99b7d45e05e102504482ddf24c8ba2d"
}
if ($Step -eq "s2") {
  $d = Get-Command dumpbin -ErrorAction SilentlyContinue
  if ($d) { "dumpbin: " + $d.Source } else { "dumpbin: not on PATH (not installed by this run)" }
  foreach ($f in "bridge_mcp.exe", "pdfium.dll") {
    $out = @(Get-PeImports ".\unpacked\bin\x86_64-pc-windows-msvc\$f")
    "--- Get-PeImports $f"
    $out
    "lines: " + $out.Count
    $fam = @($out | Where-Object { (($_ -split '\s+', 2)[1]) -match '^(vcruntime|msvcp|api-ms-win-crt-|ucrtbase)' })
    "runtime-family names (VCRUNTIME, MSVCP, api-ms-win-crt-, ucrtbase; case ignored): " + $fam.Count
    $fam | ForEach-Object { "  family: " + $_ }
  }
}
