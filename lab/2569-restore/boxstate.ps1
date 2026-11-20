# Lab helper (row 2569-R; not product code): read-only. For each top-level window of one process, prints its
# handle, title, class, whether Windows holds it minimised (IsIconic), visible, topmost, its rectangle and the UI
# Automation visual state, and whether it is the foreground window; then the foreground window's process and the
# element holding keyboard focus. Clicks nothing, sends no key, moves no window.
# Usage: powershell -NoProfile -File boxstate.ps1 -ProcId <pid> -Out <file>   (never overwrites <file>)
param([int]$ProcId, [string]$Out)
$Out = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($Out)
if (Test-Path -LiteralPath $Out) { throw "exists: $Out" }
Add-Type -AssemblyName UIAutomationClient, UIAutomationTypes
Add-Type @"
using System;
using System.Runtime.InteropServices;
public static class BoxState {
  [StructLayout(LayoutKind.Sequential)] public struct Rect { public int Left, Top, Right, Bottom; }
  [DllImport("user32.dll")] public static extern bool IsIconic(IntPtr h);
  [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
  [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
  [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out Rect r);
  [DllImport("user32.dll")] public static extern int GetWindowLong(IntPtr h, int index);
}
"@
$foreground = [BoxState]::GetForegroundWindow()
$foregroundPid = [uint32]0
[void][BoxState]::GetWindowThreadProcessId($foreground, [ref]$foregroundPid)
$lines = @("read at " + [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ss.fffZ') + " pid " + $ProcId)
$root = [System.Windows.Automation.AutomationElement]::RootElement
$cond = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::ProcessIdProperty, $ProcId)
$windows = $root.FindAll([System.Windows.Automation.TreeScope]::Children, $cond)
$lines += "windows " + $windows.Count
foreach ($w in $windows) {
  $h = [IntPtr]$w.Current.NativeWindowHandle
  $r = New-Object BoxState+Rect
  [void][BoxState]::GetWindowRect($h, [ref]$r)
  $topmost = ([BoxState]::GetWindowLong($h, -20) -band 0x8) -ne 0
  $state = 'unreadable'
  try { $state = $w.GetCurrentPattern([System.Windows.Automation.WindowPattern]::Pattern).Current.WindowVisualState } catch {}
  $lines += ("WINDOW handle=" + $h + " name=[" + $w.Current.Name + "] class=" + $w.Current.ClassName +
    " iconic=" + [BoxState]::IsIconic($h) + " visible=" + [BoxState]::IsWindowVisible($h) + " topmost=" + $topmost +
    " rect=(" + $r.Left + "," + $r.Top + "," + $r.Right + "," + $r.Bottom + ") visual_state=" + $state +
    " foreground=" + ($h -eq $foreground))
}
$lines += "FOREGROUND handle=" + $foreground + " pid=" + $foregroundPid
$focus = $null
try { $focus = [System.Windows.Automation.AutomationElement]::FocusedElement } catch {}
if ($focus) {
  $lines += ("FOCUS name=[" + $focus.Current.Name + "] type=" + $focus.Current.ControlType.ProgrammaticName + " pid=" + $focus.Current.ProcessId)
} else {
  $lines += "FOCUS none"
}
$lines | Out-File -FilePath $Out -Encoding utf8
$lines
