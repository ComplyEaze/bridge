# Lab helper (#1780-D C4, not product code): ONE click on the box's taskbar button, a probe straight after, then No by
# mouse if the box is open and above Tally. If the box is gone after the taskbar click: one probe, one capture, nothing more.
param([int]$Bx = 756, [int]$By = 844, [long]$Box = 26678580)
$rd = '<lab folder>'; $ev = "$rd\evidence"; $log = "$ev\c-timeline.txt"
Add-Type -AssemblyName System.Windows.Forms, System.Drawing
Add-Type @'
using System; using System.Text; using System.Collections.Generic; using System.Runtime.InteropServices;
public class TW { public delegate bool P(IntPtr h, IntPtr l);
 [DllImport("user32.dll")] public static extern bool EnumWindows(P f, IntPtr l);
 [DllImport("user32.dll")] public static extern bool EnumChildWindows(IntPtr w, P f, IntPtr l);
 [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
 [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetClassName(IntPtr h, StringBuilder s, int n);
 [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
 [DllImport("user32.dll")] public static extern bool IsWindow(IntPtr h);
 [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
 [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
 [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
 [DllImport("user32.dll")] public static extern void mouse_event(uint f, uint x, uint y, uint d, UIntPtr e);
 public static string Txt(IntPtr h) { var s = new StringBuilder(8192); GetWindowText(h, s, 8192); return s.ToString(); }
 public static string Cls(IntPtr h) { var s = new StringBuilder(64); GetClassName(h, s, 64); return s.ToString(); }
 public static int Pos(IntPtr t) { int i = 0, f = -1; EnumWindows((h,l)=>{ if (!IsWindowVisible(h)) return true; i++; if (h == t) { f = i; return false; } return true; }, IntPtr.Zero); return f; }
 public static List<IntPtr> Kids(IntPtr w) { var o = new List<IntPtr>(); EnumChildWindows(w, (h,l)=>{ o.Add(h); return true; }, IntPtr.Zero); return o; }
 public static RECT Rect(IntPtr h) { RECT r; GetWindowRect(h, out r); return r; }
 public static void ClickAt(int x, int y) { SetCursorPos(x, y); mouse_event(2,0,0,0,UIntPtr.Zero); mouse_event(4,0,0,0,UIntPtr.Zero); } }
'@
function Stamp { (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
function L($s) { Add-Content -Encoding utf8 $log $s; $s }
function Shot([string]$name) {
    $b = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
    $bmp = New-Object System.Drawing.Bitmap $b.Width, $b.Height
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.CopyFromScreen($b.Location, [System.Drawing.Point]::Empty, $b.Size)
    $bmp.Save("$rd\shots\$name.png"); $g.Dispose(); $bmp.Dispose()
}
function Probe([string]$name) {
    $s = Stamp; $o = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$rd\requests\zorder-probe.ps1" 2>&1; $e = Stamp
    $o | Set-Content -Encoding utf8 "$ev\$name.txt"
    "start $s end $e : " + (($o | % { "$_" }) -join ' | ')
}
$box = [IntPtr]$Box; $tally = [IntPtr]13379766
L "C4 before the taskbar click $(Stamp): box open $([TW]::IsWindow($box)) visible $([TW]::IsWindowVisible($box)) z $([TW]::Pos($box)), Tally z $([TW]::Pos($tally))"
if (-not [TW]::IsWindow($box)) { L "C4 the box is already gone before the taskbar click: nothing clicked"; exit 30 }
[TW]::ClickAt($Bx, $By); L "C4 taskbar button clicked ONCE $(Stamp) at logical ($Bx,$By) = screen tool ($([Math]::Round($Bx*1456/1536)),$([Math]::Round($By*819/864)))"
L "C4 probe after the taskbar click $(Probe 'probe-c-taskbar')"
$open = [TW]::IsWindow($box)
L "C4 $(Stamp) box open $open; z box $([TW]::Pos($box)), Tally $([TW]::Pos($tally))"
if (-not $open) { Shot 'c4-gone-screen'; L "C4 the box is gone after the taskbar click without No: one probe, one capture, nothing more clicked ($(Stamp))"; exit 31 }
$no = [IntPtr]::Zero
foreach ($k in [TW]::Kids($box)) { if ([TW]::Cls($k) -eq 'Button' -and [TW]::Txt($k) -eq '&No') { $no = $k } }
$bp = [TW]::Pos($box); $tp = [TW]::Pos($tally)
if ($no -ne [IntPtr]::Zero -and $bp -gt 0 -and $bp -lt $tp) {
    $nr = [TW]::Rect($no); $x = [int](($nr.L + $nr.R) / 2); $y = [int](($nr.T + $nr.B) / 2)
    [TW]::ClickAt($x, $y); L "C4 No clicked $(Stamp), logical ($x,$y)"
    Start-Sleep -Milliseconds 800; L "C4 box still open: $([TW]::IsWindow($box))"; Shot 'c4-after-no-screen'; L "C4 shot $(Stamp)"
} else { Shot 'c4-not-above-screen'; L "C4 box still not above Tally (box $bp, Tally $tp): nothing more clicked"; exit 32 }
exit 0
