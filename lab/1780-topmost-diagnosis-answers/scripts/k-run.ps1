# Lab helper (#1780-D part K, not product code): one control box (control-box.ps1, no Bridge code) with Tally maximised
# in front. Starts the sheet's trace in its own window, clicks Tally's title, starts the box (hidden for K1, a normal
# window for K2), clicks Tally's title at once (T0), probes (the sheet's probe, then the K copy that also prints the box)
# at the given times, a whole-screen capture after the last, ONE click on a blank point of Tally at T0+14 at least 150
# logical px from the box, probes and captures at C+0 and C+6, then No by mouse only if the box is above Tally.
param([Parameter(Mandatory = $true)][string]$Part, [switch]$Hidden, [int]$Delay = 5, [string]$ProbeAt = '8 11', [switch]$Existing, [int]$Px = 263, [int]$Py = 475)
$rd = '<lab folder>'; $ev = "$rd\evidence"; $p = $Part.ToLower(); $log = "$ev\$p-timeline.txt"
Add-Type -AssemblyName System.Windows.Forms, System.Drawing
Add-Type @'
using System; using System.Text; using System.Collections.Generic; using System.Runtime.InteropServices;
public class KW { public delegate bool P(IntPtr h, IntPtr l);
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
 public static IntPtr Box() { IntPtr r = IntPtr.Zero; EnumWindows((h,l)=>{ if (IsWindowVisible(h) && Txt(h) == "Lab topmost control") { r = h; return false; } return true; }, IntPtr.Zero); return r; }
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
    $s2 = Stamp; $o2 = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$rd\zorder-probe-k.ps1" 2>&1; $e2 = Stamp
    $o2 | Set-Content -Encoding utf8 "$ev\$name-k.txt"
    "sheet probe $s to $e : " + (($o | % { "$_" }) -join ' | ') + " ;; K probe $s2 to $e2 : " + (($o2 | % { "$_" }) -join ' | ')
}
$tally = [IntPtr]13379766; $tx = 330; $ty = 14
$probeTimes = @($ProbeAt -split '[ ;]+' | Where-Object { $_ } | ForEach-Object { [int]$_ })
$tf = if ($Existing) { "trace-$p-late.txt" } else { "trace-$p.txt" }
$trArgs = "-NoProfile -ExecutionPolicy Bypass -File $rd\requests\box-trace.ps1 -Title `"Lab topmost control`" -Out $ev\$tf -Seconds 60"
$tr = Start-Process powershell -ArgumentList $trArgs -PassThru; $null = $tr.Handle
L "$Part trace started $(Stamp) pid $($tr.Id) in its own window: powershell $trArgs"
Start-Sleep -Seconds 2
if ($Existing) {
    # The box from the stopped run is still open: probe and capture now, then the click and No below, with no new box.
    $t0 = (Get-Date).AddSeconds(-11)
    L "$Part late $(Probe "probe-$p-late")"; Shot "$p-late-screen"; L "$Part late shot $(Stamp)"
    Start-Sleep -Seconds 2
} else {
[KW]::ClickAt($tx, $ty); L "$Part Tally title clicked $(Stamp) at logical ($tx,$ty)"
Start-Sleep -Milliseconds 700
$cbArgs = "-NoProfile -ExecutionPolicy Bypass -File $rd\requests\control-box.ps1 -DelaySeconds $Delay -Out $ev\control-$p.txt"
if ($Hidden) { $cb = Start-Process powershell -WindowStyle Hidden -ArgumentList $cbArgs -PassThru } else { $cb = Start-Process powershell -ArgumentList $cbArgs -PassThru }
$null = $cb.Handle
[KW]::ClickAt($tx, $ty); $t0 = Get-Date
$how = if ($Hidden) { '-WindowStyle Hidden ' } else { '' }
L "$Part-T0 $(Stamp): control box started (pid $($cb.Id), hidden $([bool]$Hidden)): Start-Process powershell $how-ArgumentList '$cbArgs'; Tally title clicked at once"
foreach ($at in $probeTimes) {
    while (((Get-Date) - $t0).TotalSeconds -lt $at) { Start-Sleep -Milliseconds 20 }
    L "$Part-T0+$at $(Probe "probe-$p-t0p$at")"
}
Shot "$p-t0p$($probeTimes[-1])-screen"; L "$Part-T0+$($probeTimes[-1]) shot $(Stamp)"
}
while (((Get-Date) - $t0).TotalSeconds -lt 14) { Start-Sleep -Milliseconds 20 }
$box = [KW]::Box()
if ($box -eq [IntPtr]::Zero) {
    while (((Get-Date) - $t0).TotalSeconds -lt 15) { Start-Sleep -Milliseconds 20 }
    $box = [KW]::Box()
    if ($box -eq [IntPtr]::Zero) { L "$Part-T0+15 $(Stamp): the box did not appear (hidden $([bool]$Hidden)); no click"; exit 10 }
}
$br = [KW]::Rect($box); $no = [IntPtr]::Zero; $kids = @()
foreach ($k in [KW]::Kids($box)) { $c = [KW]::Cls($k); $t = [KW]::Txt($k); $kids += "$c '$t'"; if ($c -eq 'Button' -and $t -eq '&No') { $no = $k } }
L "$Part box hwnd $box rect $($br.L),$($br.T),$($br.R),$($br.B); children: $($kids -join '; ')"
$dx = [Math]::Max(0, [Math]::Max($br.L - $Px, $Px - $br.R)); $dy = [Math]::Max(0, [Math]::Max($br.T - $Py, $Py - $br.B)); $dist = [Math]::Sqrt($dx * $dx + $dy * $dy)
L "$Part point logical ($Px,$Py) = screen tool ($([Math]::Round($Px * 1456 / 1536)),$([Math]::Round($Py * 819 / 864))); distance from the box $([Math]::Round($dist,1)) px"
if ($dist -lt 150) { L "$Part point too near the box: no click; stop"; exit 11 }
[KW]::ClickAt($Px, $Py); $tc = Get-Date
L "$Part click into Tally $(Stamp) at T0+$([Math]::Round(($tc - $t0).TotalSeconds, 2)) s"
L "$Part C+0 $(Probe "probe-$p-c0")"; Shot "$p-c0-screen"; L "$Part C+0 shot $(Stamp)"
while (((Get-Date) - $tc).TotalSeconds -lt 6) { Start-Sleep -Milliseconds 20 }
L "$Part C+6 $(Probe "probe-$p-c6")"; Shot "$p-c6-screen"; L "$Part C+6 shot $(Stamp)"
if (-not [KW]::IsWindow($box)) { L "$Part the box is gone after the click: stop"; exit 12 }
$bp = [KW]::Pos($box); $tp = [KW]::Pos($tally)
L "$Part z positions now: box $bp, Tally $tp"
if ($bp -gt 0 -and $bp -lt $tp -and $no -ne [IntPtr]::Zero) {
    $nr = [KW]::Rect($no); $x = [int](($nr.L + $nr.R) / 2); $y = [int](($nr.T + $nr.B) / 2)
    [KW]::ClickAt($x, $y); L "$Part No clicked $(Stamp) at T0+$([Math]::Round(((Get-Date) - $t0).TotalSeconds, 2)) s, logical ($x,$y)"
    Start-Sleep -Milliseconds 800; L "$Part box still open: $([KW]::IsWindow($box))"
} else { L "$Part box not above Tally (box $bp, Tally $tp): nothing clicked; the taskbar step is needed"; exit 13 }
exit 0
