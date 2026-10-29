# Lab helper (#1780-G, not product code; row 1780-F's b3.ps1 with the folder changed; the 1780-D helper with the folder changed and Tally found by title): B3 (the probe's known answer with Tally's title clicked), B3b (the trace's known
# answer on control-forms.ps1) and B4 (the frames). Clicks only Tally's title area; sends no key.
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
$s = Stamp; $o = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$rd\requests\zorder-probe.ps1" 2>&1; $e = Stamp
$o | Set-Content -Encoding utf8 "$rd\evidence\probe-b3.txt"
L "B3 probe start $s end $e : $(($o | % { "$_" }) -join ' | ')"
L "B3 Tally IsZoomed $([B3W]::IsZoomed((Get-Process tally | Where-Object { $_.MainWindowTitle -like "TallyPrime:9001*" } | Select-Object -First 1).MainWindowHandle))"
$dc = [B3W]::GetDC([IntPtr]::Zero); $b = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
L "B4 screens $([System.Windows.Forms.Screen]::AllScreens.Count); logical $($b.Width) x $($b.Height); physical $([B3W]::GetDeviceCaps($dc,118)) x $([B3W]::GetDeviceCaps($dc,117)); screen tool frame 1456 x 819"
# B3b
$cf = "powershell -NoProfile -ExecutionPolicy Bypass -File $rd\requests\control-forms.ps1 -Seconds 20"
$tr = "powershell -NoProfile -ExecutionPolicy Bypass -File $rd\requests\box-trace-early.ps1 -Title `"Tally probe control`" -Out $rd\evidence\trace-b3b.txt -Seconds 10"
L "B3b control-forms start $(Stamp): $cf"
$pf = Start-Process powershell -ArgumentList "-NoProfile -ExecutionPolicy Bypass -File $rd\requests\control-forms.ps1 -Seconds 20" -PassThru
Start-Sleep -Seconds 3
L "B3b trace start $(Stamp): $tr"
$pt = Start-Process powershell -ArgumentList "-NoProfile -ExecutionPolicy Bypass -File $rd\requests\box-trace-early.ps1 -Title `"Tally probe control`" -Out $rd\evidence\trace-b3b.txt -Seconds 10" -PassThru
$null = $pt.Handle; $pt.WaitForExit(); L "B3b trace ended $(Stamp) exit $($pt.ExitCode)"
$null = $pf.Handle; $pf.WaitForExit(); L "B3b control-forms ended $(Stamp) exit $($pf.ExitCode)"
