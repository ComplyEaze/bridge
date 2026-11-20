# Lab helper (2440-CP): read-only. Lists the top-level windows of one process and prints every text it holds via UI Automation; clicks nothing.
param([int]$ProcId, [string]$Out)
Add-Type -AssemblyName UIAutomationClient, UIAutomationTypes
$root = [System.Windows.Automation.AutomationElement]::RootElement
$cond = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::ProcessIdProperty, $ProcId)
$wins = $root.FindAll([System.Windows.Automation.TreeScope]::Children, $cond)
$lines = @("read at " + [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ss.fffZ') + " windows " + $wins.Count)
foreach ($w in $wins) {
  $lines += "WINDOW name=[" + $w.Current.Name + "] class=" + $w.Current.ClassName
  foreach ($e in $w.FindAll([System.Windows.Automation.TreeScope]::Descendants, [System.Windows.Automation.Condition]::TrueCondition)) {
    $n = $e.Current.Name; $v = ''
    try { $vp = $e.GetCurrentPattern([System.Windows.Automation.ValuePattern]::Pattern); $v = $vp.Current.Value } catch {}
    if ($n -or $v) { $lines += ($e.Current.ControlType.ProgrammaticName + " | " + $n + $(if ($v) { " | value=" + $v } else { "" })) }
  }
}
$lines | Out-File -FilePath $Out -Encoding utf8
$lines
