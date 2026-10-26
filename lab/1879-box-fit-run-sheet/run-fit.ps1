# Lab script (not product code): B-rows of the box-fit item (#1879). Open the real approval box with N lines of synthetic text
# through the executable's child mode, with the child's stdout written to a file, find its window by process id and print its
# rectangle against the screen's work area (physical pixels), then print how the child ended: exit status, how long it ran and
# what it printed. NO Tally, NO posting: the child only shows a box and prints a token if Yes is clicked. The box is answered by
# MOUSE only (Yes, No, Cancel or the X, as the sheet says for the case). Type NOTHING while a box is open: Y answers it.
# Written without a PowerShell to test it on: run -DryRun first (prints, opens nothing), then one real case.
# Usage: powershell -NoProfile -File run-fit.ps1 -Exe <bridge_mcp.exe> -InputDir <folder holding probe-NN.txt> -Case probe-48 -OutDir <new folder> [-DryRun]
param(
  [Parameter(Mandatory=$true)][string]$Exe,
  [Parameter(Mandatory=$true)][string]$InputDir,
  [Parameter(Mandatory=$true)][ValidateSet('probe-29','probe-33','probe-40','probe-44','probe-48','probe-60','probe-80','probe-120')][string]$Case,
  [Parameter(Mandatory=$true)][string]$OutDir,
  [switch]$DryRun
)
$ErrorActionPreference = 'Stop'
$expect = @{
  'probe-29'  = 'af4bbcdcfb94f3625c217397716ff029c272a7da3ef2e6fcc54392bdcd87c64f'
  'probe-33'  = '62d9bf2839fe4b7d2b677813db97420ff17c1ce6a2715cd88a5dc28675c982a2'
  'probe-40'  = 'e50eb2cdbb9e62abfc633083dd50f2e3c1584c2cb06aa7cd2e0b6e7adb6cd11f'
  'probe-44'  = 'd4f9f8f5da7b67ff5cd722c1abd9f8c4f6507838c2ddf3fd06cd6a2c7b0b6af5'
  'probe-48'  = 'b97a04097a33fe1c0de29c4e4e193c1f3f3de52f8f24423774ca1c0dcb9d5881'
  'probe-60'  = 'b1e65b1d1ea73ca6c5b77025d42880ba7b783d4e87fec151ded099616b63f874'
  'probe-80'  = '8701614b4b66617fa22bdfcd90cc3b12c4219d94aa10ba77520edae0f1c8150d'
  'probe-120' = '6d508e879db2bf69bddffb6cf8873009143dfbb9f22fe6bfac33a6f3223ac47c'
}
$file = Join-Path $InputDir "$Case.txt"
if (-not (Test-Path -LiteralPath $file)) { throw "missing input $file" }
$got = (Get-FileHash -Algorithm SHA256 -LiteralPath $file).Hash.ToLower()
if ($got -ne $expect[$Case]) { throw "STOP: $Case.txt hash $got differs from $($expect[$Case])" }
$exeHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $Exe).Hash.ToLower()
Add-Type -AssemblyName System.Windows.Forms
Add-Type @'
using System; using System.Text; using System.Runtime.InteropServices; using System.Collections.Generic;
public static class W {
  public delegate bool EnumProc(IntPtr h, IntPtr l);
  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
  [DllImport("user32.dll")] public static extern bool EnumWindows(EnumProc p, IntPtr l);
  [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
  [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowTextW(IntPtr h, StringBuilder s, int n);
  [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
  public static string[] Find(uint pid) {
    var found = new List<string>();
    EnumWindows((h, l) => {
      uint p; GetWindowThreadProcessId(h, out p);
      if (p == pid && IsWindowVisible(h)) {
        RECT r; GetWindowRect(h, out r);
        var sb = new StringBuilder(256); GetWindowTextW(h, sb, 256);
        found.Add(r.L + "," + r.T + "," + r.R + "," + r.B + "|" + sb.ToString());
      }
      return true;
    }, IntPtr.Zero);
    return found.ToArray();
  }
}
'@
[void][W]::SetProcessDPIAware()
$bounds = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
$wa = [System.Windows.Forms.Screen]::PrimaryScreen.WorkingArea
if (-not (Test-Path -LiteralPath $OutDir)) { New-Item -ItemType Directory -Path $OutDir | Out-Null }
$outFile = Join-Path $OutDir "$Case.stdout.txt"
if (Test-Path -LiteralPath $outFile) { throw "STOP: $outFile exists already; use a new -OutDir" }
$errFile = Join-Path $OutDir "$Case.stderr.txt"
if (Test-Path -LiteralPath $errFile) { throw "STOP: $errFile exists already; use a new -OutDir" }
$cmd = '"{0}" --confirm-journal < "{1}" > "{2}" 2> "{3}"' -f $Exe, $file, $outFile, $errFile
Write-Host "case=$Case exe_sha256=$exeHash"
Write-Host "screen=$($bounds.Width)x$($bounds.Height) workarea=$($wa.Width)x$($wa.Height) at ($($wa.X),$($wa.Y)) (physical pixels)"
Write-Host "command: cmd.exe /s /c `"$cmd`""
if ($DryRun) { Write-Host 'DRY RUN: nothing opened'; return }
$clock = [System.Diagnostics.Stopwatch]::StartNew()
$p = Start-Process -FilePath 'cmd.exe' -ArgumentList ('/s /c "' + $cmd + '"') -PassThru -WindowStyle Hidden
Write-Host "started cmd pid=$($p.Id). If the box cannot be answered by mouse, from ANOTHER window: taskkill /PID $($p.Id) /T /F (that closes only this probe)."
$child = $null
for ($i = 0; $i -lt 150 -and $null -eq $child -and -not $p.HasExited; $i++) {
  Start-Sleep -Milliseconds 100
  $child = Get-CimInstance Win32_Process -Filter "Name='bridge_mcp.exe'" | Where-Object { $_.CommandLine -like '*--confirm-journal*' } | Select-Object -First 1
}
$rects = @()
if ($null -ne $child) {
  Write-Host "child pid=$($child.ProcessId)"
  for ($i = 0; $i -lt 100 -and $rects.Count -eq 0 -and -not $p.HasExited; $i++) { Start-Sleep -Milliseconds 100; $rects = [W]::Find([uint32]$child.ProcessId) }
  if ($rects.Count -eq 0) { Write-Host 'no visible window of the child (it ended, or none appeared in 10 s)' }
} elseif ($p.HasExited) {
  # A box that does not fit is cancelled within a fraction of a second, before this loop can see the child.
  Write-Host 'the child ended before it could be found (a box cancelled at once ends this fast): no window to measure; the exit status and the box-fit line below say what happened'
} else {
  Write-Host 'child process not found in 15 s while the command is still running: closing nothing; answer or end the box as the sheet says, then read the status below'
}
foreach ($r in $rects) {
  $parts = $r.Split('|', 2); $v = $parts[0].Split(',') | ForEach-Object { [int]$_ }
  Write-Host ("window title='{0}' rect(L,T,R,B)={1} size={2}x{3} top_off_screen={4} bottom_below_workarea={5} left_off={6} right_off={7}" -f $parts[1], $parts[0], ($v[2]-$v[0]), ($v[3]-$v[1]), ($v[1] -lt $wa.Y), ($v[3] -gt ($wa.Y + $wa.Height)), ($v[0] -lt $wa.X), ($v[2] -gt ($wa.X + $wa.Width)))
}
if (-not $p.HasExited) { Write-Host 'NOW take a WHOLE-SCREEN screenshot with the lab tool (not the keyboard), then answer by MOUSE as the sheet says for this case.' }
$p.WaitForExit()
$clock.Stop()
Write-Host "cmd exit=$($p.ExitCode) after $($clock.ElapsedMilliseconds) ms from start (77 box did not fit; 1 No/Cancel/X; 0 with the token Yes; 0 with no token = box not measured or not shown)"
$bytes = [System.IO.File]::ReadAllBytes($outFile)
Write-Host "stdout file=$outFile bytes=$($bytes.Length) sha256=$((Get-FileHash -Algorithm SHA256 -LiteralPath $outFile).Hash.ToLower())"
Write-Host ("stdout text: [" + [System.Text.Encoding]::UTF8.GetString($bytes).TrimEnd() + "]")
$errText = [System.IO.File]::ReadAllText($errFile).TrimEnd()
Write-Host "stderr file=$errFile bytes=$((Get-Item -LiteralPath $errFile).Length) sha256=$((Get-FileHash -Algorithm SHA256 -LiteralPath $errFile).Hash.ToLower())"
Write-Host ("stderr text: [" + $errText + "] (the box-fit line: the box and the work area the product compared, and fits=true or false)")
