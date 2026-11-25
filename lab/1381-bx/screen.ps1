# Lab helper (row 1381-BX; not product code). ASCII. Read-only: the display facts of the lab PC, in the units the
# dialog child measures its box in. Before anything else it makes this process per-monitor DPI aware (version 2), as
# the child does (src-tauri/src/tally/box_fit.rs, make_dpi_aware, at 48a64965), so each monitor's rectangle and work
# area are physical pixels. Prints the Windows release (registry ProductName, DisplayVersion, CurrentBuild, UBR, and
# the runtime's OSVersion), then every monitor: primary or not, its rectangle, its work area and its effective DPI
# (GetDpiForMonitor, MDT_EFFECTIVE_DPI) with the scale it stands for (DPI / 96). Clicks nothing, sends no key, moves
# no window, changes no setting.
# Usage: powershell -NoProfile -File screen.ps1 -Out <new file>   (never overwrites <file>)
# Exit: 0 read whole; 2 a call failed (named by a FAILED line; never read as "no monitor").
param([string]$Out)
$ErrorActionPreference = 'Stop'
trap { Write-Output ('FAILED ' + $_.Exception.GetType().Name + ': ' + [string]$_.Exception.Message); exit 2 }
$Out = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($Out)
if (Test-Path -LiteralPath $Out) { throw "exists: $Out" }
Add-Type @"
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
public static class ScreenFacts {
  [StructLayout(LayoutKind.Sequential)] public struct Rect { public int Left, Top, Right, Bottom; }
  [StructLayout(LayoutKind.Sequential)] public struct MonitorInfo { public int cbSize; public Rect rcMonitor; public Rect rcWork; public uint dwFlags; }
  public delegate bool MonitorEnum(IntPtr monitor, IntPtr hdc, IntPtr rect, IntPtr data);
  [DllImport("user32.dll")] public static extern IntPtr SetProcessDpiAwarenessContext(IntPtr context);
  [DllImport("user32.dll")] public static extern IntPtr GetThreadDpiAwarenessContext();
  [DllImport("user32.dll")] public static extern bool AreDpiAwarenessContextsEqual(IntPtr a, IntPtr b);
  [DllImport("user32.dll")] public static extern bool EnumDisplayMonitors(IntPtr hdc, IntPtr clip, MonitorEnum f, IntPtr data);
  [DllImport("user32.dll")] public static extern bool GetMonitorInfoW(IntPtr monitor, ref MonitorInfo info);
  [DllImport("shcore.dll")] public static extern int GetDpiForMonitor(IntPtr monitor, int kind, out uint x, out uint y);
  public static List<IntPtr> Monitors() {
    var found = new List<IntPtr>();
    if (!EnumDisplayMonitors(IntPtr.Zero, IntPtr.Zero, (m, h, r, d) => { found.Add(m); return true; }, IntPtr.Zero)) { return null; }
    return found;
  }
}
"@
$lines = New-Object 'System.Collections.Generic.List[string]'
$lines.Add('read at ' + [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ss.fffZ'))
$code = 0
# DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2 is the pseudo handle -4.
$v2 = [IntPtr](-4)
[void][ScreenFacts]::SetProcessDpiAwarenessContext($v2)
$aware = [ScreenFacts]::AreDpiAwarenessContextsEqual([ScreenFacts]::GetThreadDpiAwarenessContext(), $v2)
$lines.Add('DPI_AWARE per_monitor_v2=' + $aware)
if (-not $aware) { $lines.Add('FAILED not per-monitor DPI aware: rectangles below are not physical pixels'); $code = 2 }
$nt = Get-ItemProperty -LiteralPath 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion'
$lines.Add('OS product=[' + $nt.ProductName + '] display_version=[' + $nt.DisplayVersion + '] build=' + $nt.CurrentBuild +
  '.' + $nt.UBR + ' osversion=' + [Environment]::OSVersion.Version.ToString())
$monitors = [ScreenFacts]::Monitors()
if ($null -eq $monitors) {
  $lines.Add('FAILED EnumDisplayMonitors'); $code = 2
} else {
  $lines.Add('MONITORS ' + $monitors.Count)
  $i = 0
  foreach ($m in $monitors) {
    $i++
    $info = New-Object ScreenFacts+MonitorInfo
    $info.cbSize = [System.Runtime.InteropServices.Marshal]::SizeOf($info)
    if (-not [ScreenFacts]::GetMonitorInfoW($m, [ref]$info)) { $lines.Add('FAILED GetMonitorInfoW monitor ' + $i); $code = 2; continue }
    $dx = [uint32]0; $dy = [uint32]0
    $hr = [ScreenFacts]::GetDpiForMonitor($m, 0, [ref]$dx, [ref]$dy)
    $r = $info.rcMonitor; $w = $info.rcWork
    $dpi = 'unread hr=' + $hr
    if ($hr -eq 0) { $dpi = [string]$dx + ' scale_pct=' + [Math]::Round($dx * 100 / 96) } else { $code = 2 }
    $lines.Add('MONITOR ' + $i + ' primary=' + (($info.dwFlags -band 1) -ne 0) +
      ' rect=(' + $r.Left + ',' + $r.Top + ',' + $r.Right + ',' + $r.Bottom + ')' +
      ' size=' + ($r.Right - $r.Left) + 'x' + ($r.Bottom - $r.Top) +
      ' work=(' + $w.Left + ',' + $w.Top + ',' + $w.Right + ',' + $w.Bottom + ') dpi=' + $dpi)
  }
}
$lines.Add('RESULT exit ' + $code)
[System.IO.File]::WriteAllLines($Out, $lines)
$lines
exit $code
