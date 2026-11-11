# Lab helper (row 1780-W B3c, new; not product code; written whole by an editor): clicks Tally's title area ONCE (logical (330,14), as
# b3.ps1 does) so that Tally is in front and draws its Gateway as active, waits 700 ms, then runs the sheet's tally-window-capture.ps1
# -Name b3c unchanged. Sends no key. Usage: powershell -NoProfile -ExecutionPolicy Bypass -File b3c.ps1
$rd = '<LAB_DIR>'; $ev = "$rd\evidence"
Add-Type @'
using System; using System.Runtime.InteropServices;
public class B3C {
 [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
 [DllImport("user32.dll")] public static extern void mouse_event(uint f, uint x, uint y, uint d, UIntPtr e);
 public static void ClickAt(int x, int y) { SetCursorPos(x, y); mouse_event(2,0,0,0,UIntPtr.Zero); mouse_event(4,0,0,0,UIntPtr.Zero); } }
'@
[B3C]::ClickAt(330, 14); "B3c Tally title clicked $((Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ')) at logical (330,14)"
Start-Sleep -Milliseconds 700
& powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$rd\requests\tally-window-capture.ps1" -Name b3c -OutDir $ev -Out "$ev\tally-b3c.txt"
"capture exit $LASTEXITCODE"
