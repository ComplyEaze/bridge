# control-box.ps1 - shows one plain Windows message box from this process, with no Bridge code.
# Written on a Mac and NOT RUN anywhere. It sends nothing to Tally and reads nothing from it.
# Its C# part was compiled on that Mac with Mono's mcs (no errors); the PowerShell around it was not run.
# The box is MessageBoxW with no owner and MB_YESNO | MB_TOPMOST | MB_SETFOREGROUND, the flags the
# approval box asks for (Yes/No here, not Yes/No/Cancel). Its title is "Lab topmost control".
# It waits -DelaySeconds first, so the person can put Tally in front and this process is in the
# background when the box is asked for. It writes the answer code (6 Yes, 7 No) and two stamps.
param(
    [int]$DelaySeconds = 8,
    [Parameter(Mandatory = $true)][string]$Out
)
Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public static class ControlBox {
    [DllImport("user32.dll", CharSet = CharSet.Unicode)]
    public static extern int MessageBoxW(IntPtr owner, string text, string caption, uint type);
}
"@
function Stamp { (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ss.fffZ") }
Start-Sleep -Seconds $DelaySeconds
$asked = Stamp
# MB_YESNO 0x4, MB_SETFOREGROUND 0x10000, MB_TOPMOST 0x40000.
$answer = [ControlBox]::MessageBoxW([IntPtr]::Zero, "Lab topmost control. Nothing is sent anywhere. Click No.", "Lab topmost control", [uint32](0x4 -bor 0x10000 -bor 0x40000))
$done = Stamp
[System.IO.File]::AppendAllText($Out, ("asked=" + $asked + " answered=" + $done + " answer=" + $answer + "`r`n"), [System.Text.Encoding]::UTF8)
