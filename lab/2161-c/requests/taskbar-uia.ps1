# taskbar-uia.ps1 (row 2161-C) - the taskbar's buttons as UI Automation lists them. Read-only: it changes no window, sends no
# key or click, invokes no pattern, ends nothing and sends nothing to Tally. Written on a Mac and NOT RUN anywhere; its C# part
# was compiled there with Mono's mcs (no errors), the PowerShell around it was not run.
# Per-monitor DPI aware (SetProcessDpiAwarenessContext(-4)), so every rectangle is in physical pixels.
# Writes to -Out (a new file each time; it refuses one that exists), one line each:
#   E <stamp> dpi_context=ok|refused tray=0x..
#   B <stamp> n=<n> parent_class=.. class=.. offscreen=0|1 rect=l,t,r,b|empty name=...
#                       every element of control type Button under the taskbar (Shell_TrayWnd), in the order UI Automation
#                       finds them; parent_class MSTaskListWClass marks a running application's button on Windows 10
#   Z <stamp> buttons=<n> in_task_list=<n>
#                       in_task_list counts the B lines whose parent_class is MSTaskListWClass
param([Parameter(Mandatory = $true)][string]$Out)
if (Test-Path $Out) { throw ('exists already: ' + $Out) }
Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public static class TUia {
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] public static extern IntPtr FindWindow(string cls, string name);
    [DllImport("user32.dll")] static extern bool SetProcessDpiAwarenessContext(IntPtr value);
    public static bool MakeAware() { return SetProcessDpiAwarenessContext(new IntPtr(-4)); }
}
"@
$dpi = if ([TUia]::MakeAware()) { 'ok' } else { 'refused' }
Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes
function Stamp { (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ss.fffZ") }
function OneLine([string]$s) { if ($null -eq $s) { return '' }; return $s.Replace("`r", ' ').Replace("`n", ' ') }
$lines = New-Object System.Collections.Generic.List[string]
$tray = [TUia]::FindWindow('Shell_TrayWnd', $null)
if ($tray -eq [IntPtr]::Zero) { throw 'no Shell_TrayWnd' }
$lines.Add("E $(Stamp) dpi_context=$dpi tray=0x$($tray.ToInt64().ToString('x'))")
$root = [System.Windows.Automation.AutomationElement]::FromHandle($tray)
$cond = [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::Button)
$found = $root.FindAll([System.Windows.Automation.TreeScope]::Descendants, $cond)
$walker = [System.Windows.Automation.TreeWalker]::ControlViewWalker
$n = 0; $inList = 0
foreach ($el in $found) {
    $n++
    $c = $el.Current
    $parent = $walker.GetParent($el)
    $pcls = if ($null -eq $parent) { '' } else { $parent.Current.ClassName }
    if ($pcls -eq 'MSTaskListWClass') { $inList++ }
    $b = $c.BoundingRectangle
    $rect = if ($b.IsEmpty) { 'empty' } else { "$([int]$b.Left),$([int]$b.Top),$([int]$b.Right),$([int]$b.Bottom)" }
    $off = if ($c.IsOffscreen) { 1 } else { 0 }
    $lines.Add("B $(Stamp) n=$n parent_class=$pcls class=$($c.ClassName) offscreen=$off rect=$rect name=$(OneLine $c.Name)")
}
$lines.Add("Z $(Stamp) buttons=$n in_task_list=$inList")
[System.IO.File]::WriteAllText($Out, ($lines -join "`r`n") + "`r`n", [System.Text.Encoding]::UTF8)
$lines
