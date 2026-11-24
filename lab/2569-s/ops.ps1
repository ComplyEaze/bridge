# Lab helper (row 2569-S; not product code). ASCII. The sheet's file and process acts, each with a fixed interface,
# so no command line holds a $ or a | for the shell to change. Clicks nothing, sends no key, ends no process, and
# never overwrites a file except the two named by -Op refresh.
# Usage (64-bit Windows PowerShell 5.1, from Git Bash, every path with forward slashes):
#   powershell -NoProfile -File C:/bridge-lab/2569-s/ops.ps1 -Op <op> [arguments] [-Out <new file>]
#   copytree -From <folder> -To <folder>   copy every file of -From into -To, which must not exist
#   refresh  -From <folder> -To <folder>   replace RUN-SHEET.md and SHA256.txt of -To by -From's (old ones kept
#                                          as evidence/before-reply-<name> under -To)
#   parse                                  ParseFile on every .ps1 beside this file
#   procs                                  every tally.exe, bridge_mcp.exe and python.exe: pid, parent, created
#   hash     -Path <file> -Expect <sha256> the file's SHA-256 and size against -Expect
#   absent   -Path <p1>,<p2>,...           whether each path exists
#   mkdir    -Path <p1>,<p2>,...           make each folder; refuses one that exists
#   place    -From <file> -To <file> -Expect <sha256>   copy to <To>.part, check it, rename it to <To>
#   commit   -Path <file>.part -Expect <sha256>          check a .part file and rename it without .part
#   stop     -Path <run folder>            create <run folder>/inbox/stop, an empty file
#   gone     -ProcId <pid>                 whether that process still runs
#   list     -Path <folder>                every file under it: SHA-256, size, UTC write time, relative path
#                                          (at most 200)
#   dir      -Path <folder>                every file under it: size, UTC write time, relative path; no file is
#                                          opened (for a folder whose files may be in use), at most 200
#   now                                    the UTC time
# With -Out, the lines also go to that new file (it must not exist).
# Exit: 0 done, or the state the op checks for holds (hash equal, all absent, process gone, 0 parse errors);
# 1 the op ran and the state differs (hash differs, a path present, process running, a parse error, a target
# exists); 2 the op itself failed (a typed OPS_FAILED line names how). A failed query is never read as "gone".
param([ValidateSet('copytree', 'refresh', 'parse', 'procs', 'hash', 'absent', 'mkdir', 'place', 'commit', 'stop',
                   'gone', 'list', 'dir', 'now')][string]$Op,
      [string[]]$Path, [string]$From, [string]$To, [string]$Expect, [int]$ProcId = 0, [string]$Out)
$ErrorActionPreference = 'Stop'
# powershell -File passes "a,b" as one string, so a list of paths is split here, on commas.
$Path = @($Path | Where-Object { $_ } | ForEach-Object { $_ -split ',' } | Where-Object { $_ })
$lines = New-Object 'System.Collections.Generic.List[string]'
function Fail([string]$why) {
  $script:lines.Add('OPS_FAILED ' + $why)
  Finish 2
}
function Finish([int]$rc) {
  $script:lines.Add('exit ' + $rc)
  foreach ($l in $script:lines) { Write-Output $l }
  if ($Out) { [System.IO.File]::WriteAllLines($script:outFull, $script:lines, (New-Object System.Text.UTF8Encoding $false)) }
  exit $rc
}
trap {
  $m = [string]$_.Exception.Message
  if ($m.Length -gt 200) { $m = $m.Substring(0, 200) + '...' }
  $script:lines.Add('OPS_FAILED ' + $_.Exception.GetType().Name + ': ' + $m)
  $script:lines.Add('exit 2')
  foreach ($l in $script:lines) { Write-Output $l }
  exit 2
}
function Full([string]$p) { return $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($p) }
function Sha([string]$p) { return (Get-FileHash -LiteralPath $p -Algorithm SHA256).Hash.ToLowerInvariant() }
$outFull = ''
if ($Out) {
  $outFull = Full $Out
  if (Test-Path -LiteralPath $outFull) { $Out = ''; Fail ('out exists: ' + $outFull) }
}
$lines.Add('ops ' + $Op + ' at ' + [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ss.fffZ'))
$rc = 0
switch ($Op) {
  'copytree' {
    $src = Full $From
    $dst = Full $To
    if (-not (Test-Path -LiteralPath $src -PathType Container)) { Fail ('from missing: ' + $src) }
    if (Test-Path -LiteralPath $dst) { $lines.Add('TARGET_EXISTS ' + $dst); Finish 1 }
    $n = 0
    foreach ($f in @(Get-ChildItem -LiteralPath $src -Recurse -Force -File)) {
      $rel = $f.FullName.Substring($src.TrimEnd('\').Length + 1)
      $target = Join-Path $dst $rel
      [void](New-Item -ItemType Directory -Force -Path (Split-Path -Parent $target))
      Copy-Item -LiteralPath $f.FullName -Destination $target
      if ((Sha $f.FullName) -ne (Sha $target)) { Fail ('copy differs: ' + $rel) }
      $n++
    }
    $lines.Add('COPIED files ' + $n)
  }
  'refresh' {
    $src = Full $From
    $dst = Full $To
    foreach ($name in @('RUN-SHEET.md', 'SHA256.txt')) {
      $a = Join-Path $src $name
      $b = Join-Path $dst $name
      $keep = Join-Path $dst ('evidence\before-reply-' + $name)
      if (-not (Test-Path -LiteralPath $a -PathType Leaf)) { Fail ('from missing: ' + $a) }
      if (Test-Path -LiteralPath $keep) { $lines.Add('TARGET_EXISTS ' + $keep); Finish 1 }
      if (Test-Path -LiteralPath ($b + '.part')) { $lines.Add('TARGET_EXISTS ' + $b + '.part'); Finish 1 }
      Copy-Item -LiteralPath $a -Destination ($b + '.part')
      if ((Sha $a) -ne (Sha ($b + '.part'))) { Fail ('copy differs: ' + $name) }
      Move-Item -LiteralPath $b -Destination $keep
      Move-Item -LiteralPath ($b + '.part') -Destination $b
      $lines.Add('REFRESHED ' + $name + ' sha256 ' + (Sha $b) + ' (old kept as evidence/before-reply-' + $name + ')')
    }
  }
  'parse' {
    foreach ($f in @(Get-ChildItem -LiteralPath $PSScriptRoot -Filter '*.ps1' -File | Sort-Object Name)) {
      $tokens = $null
      $errors = $null
      [void][System.Management.Automation.Language.Parser]::ParseFile($f.FullName, [ref]$tokens, [ref]$errors)
      $lines.Add('PARSE ' + $f.Name + ' errors ' + @($errors).Count)
      if (@($errors).Count -ne 0) { $rc = 1 }
    }
  }
  'procs' {
    $all = @(Get-CimInstance -ClassName Win32_Process -ErrorAction Stop | Where-Object {
      @('tally.exe', 'bridge_mcp.exe', 'python.exe') -contains $_.Name.ToLowerInvariant() })
    foreach ($p in $all | Sort-Object Name, ProcessId) {
      $lines.Add('PROC ' + $p.Name + ' pid ' + $p.ProcessId + ' parent ' + $p.ParentProcessId + ' created ' +
        $p.CreationDate.ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ'))
    }
    foreach ($name in @('tally.exe', 'bridge_mcp.exe', 'python.exe')) {
      $lines.Add('count ' + $name + ' ' + @($all | Where-Object { $_.Name.ToLowerInvariant() -eq $name }).Count)
    }
  }
  'hash' {
    $p = Full $Path[0]
    if (-not (Test-Path -LiteralPath $p -PathType Leaf)) { Fail ('file missing: ' + $p) }
    $h = Sha $p
    $lines.Add('HASH ' + $h + ' bytes ' + (Get-Item -LiteralPath $p).Length + ' ' + $Path[0])
    if ($h -eq $Expect.ToLowerInvariant()) { $lines.Add('MATCH yes') } else { $lines.Add('MATCH no'); $rc = 1 }
  }
  'absent' {
    foreach ($q in $Path) {
      if (Test-Path -LiteralPath (Full $q)) { $lines.Add('PRESENT ' + $q); $rc = 1 } else { $lines.Add('ABSENT ' + $q) }
    }
  }
  'mkdir' {
    foreach ($q in $Path) { if (Test-Path -LiteralPath (Full $q)) { $lines.Add('TARGET_EXISTS ' + $q); Finish 1 } }
    foreach ($q in $Path) { [void](New-Item -ItemType Directory -Path (Full $q)); $lines.Add('MADE ' + $q) }
  }
  'place' {
    $a = Full $From
    $b = Full $To
    if (-not (Test-Path -LiteralPath $a -PathType Leaf)) { Fail ('from missing: ' + $a) }
    if ((Test-Path -LiteralPath $b) -or (Test-Path -LiteralPath ($b + '.part'))) { $lines.Add('TARGET_EXISTS ' + $To); Finish 1 }
    Copy-Item -LiteralPath $a -Destination ($b + '.part')
    $h = Sha ($b + '.part')
    $lines.Add('PART ' + $h + ' ' + $To + '.part')
    if ($h -ne $Expect.ToLowerInvariant()) { $lines.Add('MATCH no (left as .part, not renamed)'); Finish 1 }
    Move-Item -LiteralPath ($b + '.part') -Destination $b
    $lines.Add('MATCH yes; PLACED ' + $To)
  }
  'commit' {
    $a = Full $Path[0]
    if (-not $a.EndsWith('.part')) { Fail ('not a .part name: ' + $Path[0]) }
    $b = $a.Substring(0, $a.Length - 5)
    if (-not (Test-Path -LiteralPath $a -PathType Leaf)) { Fail ('file missing: ' + $Path[0]) }
    if (Test-Path -LiteralPath $b) { $lines.Add('TARGET_EXISTS ' + $b); Finish 1 }
    $h = Sha $a
    $lines.Add('PART ' + $h + ' ' + $Path[0])
    if ($h -ne $Expect.ToLowerInvariant()) { $lines.Add('MATCH no (left as .part, not renamed)'); Finish 1 }
    Move-Item -LiteralPath $a -Destination $b
    $lines.Add('MATCH yes; RENAMED to ' + (Split-Path -Leaf $b))
  }
  'stop' {
    $inbox = Join-Path (Full $Path[0]) 'inbox'
    if (-not (Test-Path -LiteralPath $inbox -PathType Container)) { Fail ('inbox missing: ' + $inbox) }
    $s = Join-Path $inbox 'stop'
    if (Test-Path -LiteralPath $s) { $lines.Add('TARGET_EXISTS ' + $s); Finish 1 }
    [System.IO.File]::WriteAllBytes($s, (New-Object byte[] 0))
    $lines.Add('STOP_FILE made ' + $Path[0] + '/inbox/stop')
  }
  'gone' {
    if ($ProcId -le 0) { Fail 'no -ProcId' }
    $p = @(Get-CimInstance -ClassName Win32_Process -Filter ('ProcessId = ' + $ProcId) -ErrorAction Stop)
    if ($p.Count -eq 0) { $lines.Add('GONE pid ' + $ProcId) } else { $lines.Add('RUNNING pid ' + $ProcId + ' ' + $p[0].Name); $rc = 1 }
  }
  'list' {
    $root = Full $Path[0]
    if (-not (Test-Path -LiteralPath $root -PathType Container)) { Fail ('folder missing: ' + $root) }
    $files = @(Get-ChildItem -LiteralPath $root -Recurse -Force -File | Sort-Object FullName)
    $i = 0
    foreach ($f in $files) {
      if ($i -ge 200) { break }
      $lines.Add((Sha $f.FullName) + '  ' + $f.Length + '  ' + $f.LastWriteTimeUtc.ToString('yyyy-MM-ddTHH:mm:ssZ') + '  ' +
        $f.FullName.Substring($root.TrimEnd('\').Length + 1).Replace('\', '/'))
      $i++
    }
    $lines.Add('files ' + $files.Count)
    if ($files.Count -gt 200) { $lines.Add('files omitted ' + ($files.Count - 200)) }
  }
  'dir' {
    $root = Full $Path[0]
    if (-not (Test-Path -LiteralPath $root -PathType Container)) { Fail ('folder missing: ' + $root) }
    $files = @(Get-ChildItem -LiteralPath $root -Recurse -Force -File | Sort-Object FullName)
    $i = 0
    foreach ($f in $files) {
      if ($i -ge 200) { break }
      $lines.Add([string]$f.Length + '  ' + $f.LastWriteTimeUtc.ToString('yyyy-MM-ddTHH:mm:ssZ') + '  ' +
        $f.FullName.Substring($root.TrimEnd('\').Length + 1).Replace('\', '/'))
      $i++
    }
    $lines.Add('files ' + $files.Count)
    if ($files.Count -gt 200) { $lines.Add('files omitted ' + ($files.Count - 200)) }
  }
  'now' { }
}
Finish $rc
