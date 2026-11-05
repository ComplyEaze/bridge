# Lab helper (852-C2, from 1392-E, from 1029-R, not product code): brings the one tally.exe window to the front with SwitchToThisWindow; nothing is
# done to any other window. Usage: powershell -NoProfile -File front.ps1
Add-Type -Namespace L -Name W -MemberDefinition '[DllImport("user32.dll")] public static extern void SwitchToThisWindow(System.IntPtr h, bool alt);'
$p = Get-Process tally -ErrorAction Stop | Where-Object { $_.MainWindowHandle -ne 0 } | Select-Object -First 1
[L.W]::SwitchToThisWindow($p.MainWindowHandle, $true)
"front: tally pid $($p.Id)"
