# Lab helper (#1780-G, not product code): reads the taskbar through UI Automation and prints each task button's name pattern and
# centre in logical pixels, or, with -Match, only the first button whose name contains that text. Reads only; clicks nothing.
# Its output is for the lab's own use and is not committed (button names can carry other windows' titles).
param([string]$Match = '')
Add-Type -AssemblyName UIAutomationClient, UIAutomationTypes
$root = [System.Windows.Automation.AutomationElement]::RootElement
$tray = $root.FindFirst([System.Windows.Automation.TreeScope]::Children,
    (New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::ClassNameProperty, 'Shell_TrayWnd')))
if (-not $tray) { 'no taskbar'; exit 2 }
$btnCond = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::Button)
$list = $tray.FindAll([System.Windows.Automation.TreeScope]::Descendants, $btnCond)
foreach ($b in $list) {
    $n = $b.Current.Name; $r = $b.Current.BoundingRectangle
    if ($r.Width -le 0) { continue }
    $x = [int]($r.X + $r.Width / 2); $y = [int]($r.Y + $r.Height / 2)
    if ($Match) { if ($n -like "*$Match*") { "$x $y"; exit 0 } } else { "$x $y | $n" }
}
if ($Match) { 'not found'; exit 1 }
