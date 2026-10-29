# Lab helper (#1780-H, not product code; row 1780-G's b3.ps1 cut to B3 and B4): B3 (the probe's known answer with Tally's title
# clicked, and P-time, one probe's run time) and B4 (the frames). Clicks only Tally's title area; sends no key.
$rd = '<lab folder>'; $log = "$rd\evidence\b3-timeline.txt"
Add-Type @'
using System; using System.Runtime.InteropServices;
public class B3W {
 [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
 [DllImport("user32.dll")] public static extern void mouse_event(uint f, uint x, uint y, uint d, UIntPtr e);
 [DllImport("user32.dll")] public static extern bool IsZoomed(IntPtr h);
 [DllImport("user32.dll")] public static extern IntPtr GetDC(IntPtr h);
 [DllImport("gdi32.dll")] public static extern int GetDeviceCaps(IntPtr dc, int i);
 public static void ClickAt(int x, int y) { SetCursorPos(x, y); mouse_event(2,0,0,0,UIntPtr.Zero); mouse_event(4,0,0,0,UIntPtr.Zero); } }
'@
Add-Type -AssemblyName System.Windows.Forms
function Stamp { (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
function L($s) { Add-Content -Encoding utf8 $log $s; $s }
$tx = 330; $ty = 14
[B3W]::ClickAt($tx, $ty); L "B3 Tally title clicked $(Stamp) at logical ($tx,$ty) = screen tool ($([Math]::Round($tx*1456/1536)),$([Math]::Round($ty*819/864)))"
Start-Sleep -Milliseconds 500
$t0 = Get-Date; $s = Stamp; $o = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$rd\requests\zorder-probe.ps1" 2>&1; $e = Stamp
$o | Set-Content -Encoding utf8 "$rd\evidence\probe-b3.txt"
L "B3 probe start $s end $e (P-time $([Math]::Round(((Get-Date) - $t0).TotalSeconds, 2)) s) : $(($o | % { "$_" }) -join ' | ')"
L "B3 Tally IsZoomed $([B3W]::IsZoomed((Get-Process tally | Where-Object { $_.MainWindowTitle -like "TallyPrime:9001*" } | Select-Object -First 1).MainWindowHandle))"
$dc = [B3W]::GetDC([IntPtr]::Zero); $b = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
L "B4 screens $([System.Windows.Forms.Screen]::AllScreens.Count); logical $($b.Width) x $($b.Height); physical $([B3W]::GetDeviceCaps($dc,118)) x $([B3W]::GetDeviceCaps($dc,117)); screen tool frame 1456 x 819"
