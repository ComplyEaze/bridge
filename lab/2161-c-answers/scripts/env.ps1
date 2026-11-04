# Lab helper (#2161-C B4, not product code): the screen's two frames and the PC facts the sheet asks to read afresh. Prints the
# logical size (a DPI-unaware read), the physical size and scale (per-monitor aware), the monitor count, the Windows build, and
# Tally's window: handle, rectangle, maximised or not. Reads only.
Add-Type -AssemblyName System.Windows.Forms
Add-Type @'
using System; using System.Runtime.InteropServices;
public class EV { [DllImport("user32.dll")] public static extern bool SetProcessDpiAwarenessContext(IntPtr v);
 [DllImport("user32.dll")] public static extern int GetSystemMetrics(int i);
 [DllImport("user32.dll")] public static extern bool IsZoomed(IntPtr h);
 [DllImport("user32.dll")] public static extern uint GetDpiForWindow(IntPtr h);
 [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
 [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r); }
'@
function Stamp { (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
$logicalW = [EV]::GetSystemMetrics(0); $logicalH = [EV]::GetSystemMetrics(1)
[void][EV]::SetProcessDpiAwarenessContext([IntPtr](-4))
$physW = [EV]::GetSystemMetrics(0); $physH = [EV]::GetSystemMetrics(1)
$tally = Get-Process tally | Where-Object { $_.MainWindowTitle -like 'TallyPrime:9001*' } | Select-Object -First 1
$h = $tally.MainWindowHandle; $r = New-Object EV+RECT; [void][EV]::GetWindowRect($h, [ref]$r)
$os = Get-CimInstance Win32_OperatingSystem
"B4 $(Stamp)"
"logical screen ${logicalW}x${logicalH}; physical ${physW}x${physH}; scale $([Math]::Round($physW / $logicalW * 100)) %; Tally window DPI $([EV]::GetDpiForWindow($h))"
"monitors: $([System.Windows.Forms.Screen]::AllScreens.Count)"
"Windows: $($os.Caption) version $($os.Version) build $($os.BuildNumber)"
"Tally window 0x$(([int64]$h).ToString('x')) title '$($tally.MainWindowTitle)' rect physical $($r.L),$($r.T),$($r.R),$($r.B) maximised $([EV]::IsZoomed($h))"
