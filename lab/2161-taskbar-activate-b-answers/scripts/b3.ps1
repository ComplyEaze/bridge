# Lab helper (#2161-B B3, not product code): clicks Tally's title area once (physical 412,17 = logical 330,14, per-monitor DPI
# aware), then runs zorder-probe-k.ps1 and box-rect-probe.ps1 and times each (P-time, R-time). Sends no key.
$rd = '<lab folder>'
Add-Type 'using System; using System.Runtime.InteropServices; public class B3 { [DllImport("user32.dll")] public static extern bool SetProcessDpiAwarenessContext(IntPtr v); [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y); [DllImport("user32.dll")] public static extern void mouse_event(uint f, uint x, uint y, uint d, UIntPtr e); }'
[void][B3]::SetProcessDpiAwarenessContext([IntPtr](-4))
function Stamp { (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
[void][B3]::SetCursorPos(412, 17); [B3]::mouse_event(2, 0, 0, 0, [UIntPtr]::Zero); [B3]::mouse_event(4, 0, 0, 0, [UIntPtr]::Zero)
"B3 Tally title clicked $(Stamp) at logical (330,14) physical (412,17)"; Start-Sleep -Milliseconds 500
$s = Get-Date; $o = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$rd\requests\zorder-probe-k.ps1" 2>&1; $pt = ((Get-Date) - $s).TotalSeconds
"zorder-probe-k $(Stamp) P-time $([Math]::Round($pt, 2)) s: $(($o | ForEach-Object { "$_" }) -join ' | ')"
$s = Get-Date; $o = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$rd\requests\box-rect-probe.ps1" -Out "$rd\evidence\rect-b3.txt" 2>&1; $rt = ((Get-Date) - $s).TotalSeconds
"box-rect-probe $(Stamp) R-time $([Math]::Round($rt, 2)) s: $(($o | ForEach-Object { "$_" }) -join ' | ')"
