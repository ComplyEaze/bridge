# Fill one 1050-B template exactly as the README says (replace only its placeholder, UTF-16 with BOM) and record its hash.
# Usage: fill.ps1 <template name> <placeholder> <value> <out name>
param([string]$Template, [string]$Placeholder, [string]$Value, [string]$Out)
$root = 'C:\bridge-lab\1050-B'
$src = Join-Path $root "req\$Template"
$exp = (Get-Content (Join-Path $root 'req\SHA256.txt') | Where-Object { $_ -match [regex]::Escape($Template) + '$' }) -split '\s+' | Select-Object -First 1
$got = (Get-FileHash $src -Algorithm SHA256).Hash.ToLower()
if ($exp -ne $got) { Write-Output "TEMPLATE HASH MISMATCH $Template got $got expected $exp"; exit 8 }
New-Item -ItemType Directory -Force (Join-Path $root 'filled') | Out-Null
$dst = Join-Path $root "filled\$Out"
$t = [IO.File]::ReadAllText($src, [Text.Encoding]::Unicode)
$n = ([regex]::Matches($t, [regex]::Escape($Placeholder))).Count
$t = $t.Replace($Placeholder, $Value)
[IO.File]::WriteAllText($dst, $t, [Text.Encoding]::Unicode)
$h = (Get-FileHash $dst -Algorithm SHA256).Hash.ToLower()
$len = (Get-Item $dst).Length
Add-Content -Path (Join-Path $root 'filled\FILLED-SHA256.txt') -Value "$h  $Out" -Encoding ascii
Write-Output "filled $Out from $Template ($Placeholder x$n -> $Value) bytes $len sha256 $h"
