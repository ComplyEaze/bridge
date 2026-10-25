# Lab script (not product code): W2. Open the real approval box with N lines of synthetic text through the executable's child
# mode, find its window by process id, and print its rectangle against the screen's work area (physical pixels). NO Tally, NO
# posting. The box is answered by MOUSE only (No, Cancel or the X). Type NOTHING while a box is open: Y answers it.
# Written without a PowerShell to test it on: run -DryRun first (prints, opens nothing), then one real case.
# Usage: powershell -NoProfile -ExecutionPolicy Bypass -File run-probe.ps1 -Exe <bridge_mcp.exe> -InputDir <folder holding probe-NN.txt> -Case probe-80 [-DryRun]
param(
  [Parameter(Mandatory=$true)][string]$Exe,
  [Parameter(Mandatory=$true)][string]$InputDir,
  [Parameter(Mandatory=$true)][ValidateSet('probe-40','probe-44','probe-48','probe-60','probe-80','probe-120')][string]$Case,
  [switch]$DryRun
)
$ErrorActionPreference = 'Stop'
$expect = @{
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
$cmd = '"{0}" --confirm-journal < "{1}"' -f $Exe, $file
Write-Host "case=$Case exe_sha256=$exeHash"
Write-Host "screen=$($bounds.Width)x$($bounds.Height) workarea=$($wa.Width)x$($wa.Height) at ($($wa.X),$($wa.Y)) (physical pixels)"
Write-Host "command: cmd.exe /s /c `"$cmd`""
if ($DryRun) { Write-Host 'DRY RUN: nothing opened'; return }
$p = Start-Process -FilePath 'cmd.exe' -ArgumentList ('/s /c "' + $cmd + '"') -PassThru -WindowStyle Hidden
Write-Host "started cmd pid=$($p.Id). If the box cannot be answered by mouse, from ANOTHER window: taskkill /PID $($p.Id) /T /F (that closes only this probe)."
$child = $null
for ($i = 0; $i -lt 150 -and $null -eq $child; $i++) {
  Start-Sleep -Milliseconds 100
  $child = Get-CimInstance Win32_Process -Filter "Name='bridge_mcp.exe'" | Where-Object { $_.CommandLine -like '*--confirm-journal*' } | Select-Object -First 1
}
if ($null -eq $child) { Write-Host 'child process not found in 15 s'; return }
Write-Host "child pid=$($child.ProcessId)"
$rects = @()
for ($i = 0; $i -lt 100 -and $rects.Count -eq 0; $i++) { Start-Sleep -Milliseconds 100; $rects = [W]::Find([uint32]$child.ProcessId) }
if ($rects.Count -eq 0) { Write-Host 'no visible window of the child in 10 s' }
foreach ($r in $rects) {
  $parts = $r.Split('|', 2); $v = $parts[0].Split(',') | ForEach-Object { [int]$_ }
  Write-Host ("window title='{0}' rect(L,T,R,B)={1} size={2}x{3} top_off_screen={4} bottom_below_workarea={5} left_off={6} right_off={7}" -f $parts[1], $parts[0], ($v[2]-$v[0]), ($v[3]-$v[1]), ($v[1] -lt $wa.Y), ($v[3] -gt ($wa.Y + $wa.Height)), ($v[0] -lt $wa.X), ($v[2] -gt ($wa.X + $wa.Width)))
}
Write-Host 'NOW take a WHOLE-SCREEN screenshot with the lab tool (not the keyboard), then answer by MOUSE (No / Cancel / X).'
$p.WaitForExit()
Write-Host "cmd exit=$($p.ExitCode) (1 when answered No/Cancel)"
