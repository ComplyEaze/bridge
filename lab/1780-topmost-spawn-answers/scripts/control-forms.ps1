# Lab helper (#1780 probe positive control, not product code): shows two small windows for -Seconds: one created with
# TopMost = true ("Tally probe control TOPMOST") and one ordinary ("Tally probe control PLAIN"), writes each window's handle and
# this process's id to forms.txt, then closes both. It sends no key and clicks nothing. Usage: control-forms.ps1 -Seconds 60
param([int]$Seconds = 60)
Add-Type -AssemblyName System.Windows.Forms, System.Drawing
$out = '<lab folder>\evidence\forms.txt'
$top = New-Object System.Windows.Forms.Form
$top.Text = 'Tally probe control TOPMOST'; $top.TopMost = $true; $top.StartPosition = 'Manual'
$top.Location = New-Object System.Drawing.Point(40, 40); $top.Size = New-Object System.Drawing.Size(360, 120); $top.ShowInTaskbar = $true
$plain = New-Object System.Windows.Forms.Form
$plain.Text = 'Tally probe control PLAIN'; $plain.TopMost = $false; $plain.StartPosition = 'Manual'
$plain.Location = New-Object System.Drawing.Point(40, 200); $plain.Size = New-Object System.Drawing.Size(360, 120); $plain.ShowInTaskbar = $true
$top.Show(); $plain.Show(); [System.Windows.Forms.Application]::DoEvents()
"pid $PID; TOPMOST hwnd $([int64]$top.Handle) TopMost=$($top.TopMost); PLAIN hwnd $([int64]$plain.Handle) TopMost=$($plain.TopMost); shown $((Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ'))" | Out-File -Encoding ascii $out
$sw = [Diagnostics.Stopwatch]::StartNew()
while ($sw.Elapsed.TotalSeconds -lt $Seconds) { [System.Windows.Forms.Application]::DoEvents(); Start-Sleep -Milliseconds 50 }
$top.Close(); $plain.Close()
"closed $((Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ'))" | Out-File -Encoding ascii -Append $out
