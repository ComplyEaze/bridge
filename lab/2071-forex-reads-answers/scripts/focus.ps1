# Lab helper (not product code): bring the TallyPrime window (the one tally.exe) to the foreground, with an Alt tap first
# so Windows allows it. Sends no key to Tally. Prints True when Windows accepted it.
Add-Type @'
using System; using System.Runtime.InteropServices;
public class LabFocus { [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h); [DllImport("user32.dll")] public static extern void keybd_event(byte k, byte s, uint f, UIntPtr e); }
'@
$p = @(Get-Process tally)
if ($p.Count -ne 1) { "tally.exe count $($p.Count): nothing done"; exit 1 }
[LabFocus]::keybd_event(0x12, 0, 0, [UIntPtr]::Zero); [LabFocus]::keybd_event(0x12, 0, 2, [UIntPtr]::Zero)
[LabFocus]::SetForegroundWindow($p[0].MainWindowHandle)
