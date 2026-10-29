# Lab helper (#1780-G cells, not product code; row 1780-F's cell-run.ps1 cut to this sheet's recipe). One cell: pre-arms the trace
# (box-trace-early.ps1, its own window); writes the sheet's two-line <cell>.cmd and opens it through explorer.exe (a visible console
# whose parent is explorer; nothing typed); clicks Tally's title ONCE about 4 s after the start; T0 is the stamp of the SPAWN ...
# child_pid= line of <cell>.txt; probe pairs (zorder-probe.ps1 then zorder-probe-k.ps1) at T0+8 and T0+11 and a whole-screen capture;
# no other click until No by mouse at about T0+20; if the box is below Tally then, Tally is minimised to uncover it (see below), No is
# clicked and Tally restored. It never clicks Yes or Cancel and sends no key.
param([Parameter(Mandatory = $true)][string]$Cell, [Parameter(Mandatory = $true)][ValidateSet('c', 'g', 'h', 'i')][string]$Variant,
      [int]$TitleClickAfterMs = 4000)
$rd = '<lab folder>'; $ev = "$rd\evidence"; $c = $Cell.ToLower(); $log = "$ev\$c-timeline.txt"; $title = 'Lab topmost control'
Add-Type -AssemblyName System.Windows.Forms, System.Drawing
Add-Type @'
using System; using System.Text; using System.Collections.Generic; using System.Runtime.InteropServices;
public class GR { public delegate bool P(IntPtr h, IntPtr l);
 [DllImport("user32.dll")] public static extern bool EnumWindows(P f, IntPtr l);
 [DllImport("user32.dll")] public static extern bool EnumChildWindows(IntPtr w, P f, IntPtr l);
 [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
 [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetClassName(IntPtr h, StringBuilder s, int n);
 [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
 [DllImport("user32.dll")] public static extern bool IsWindow(IntPtr h);
 [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
 [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
 [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
 [DllImport("user32.dll", SetLastError=true)] public static extern bool SetWindowPos(IntPtr h, IntPtr after, int x, int y, int cx, int cy, uint f);
 [DllImport("user32.dll")] public static extern int GetWindowLong(IntPtr h, int i);
 [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h, int c);
 [DllImport("user32.dll")] public static extern void mouse_event(uint f, uint x, uint y, uint d, UIntPtr e);
 public static string Txt(IntPtr h) { var s = new StringBuilder(8192); GetWindowText(h, s, 8192); return s.ToString(); }
 public static string Cls(IntPtr h) { var s = new StringBuilder(64); GetClassName(h, s, 64); return s.ToString(); }
 public static IntPtr ByTitle(string t) { IntPtr r = IntPtr.Zero; EnumWindows((h,l)=>{ if (IsWindowVisible(h) && Txt(h).StartsWith(t)) { r = h; return false; } return true; }, IntPtr.Zero); return r; }
 public static int Pos(IntPtr t) { int i = 0, f = -1; EnumWindows((h,l)=>{ if (!IsWindowVisible(h)) return true; i++; if (h == t) { f = i; return false; } return true; }, IntPtr.Zero); return f; }
 public static List<IntPtr> Kids(IntPtr w) { var o = new List<IntPtr>(); EnumChildWindows(w, (h,l)=>{ o.Add(h); return true; }, IntPtr.Zero); return o; }
 public static RECT Rect(IntPtr h) { RECT r; GetWindowRect(h, out r); return r; }
 public static void ClickAt(int x, int y) { SetCursorPos(x, y); mouse_event(2,0,0,0,UIntPtr.Zero); mouse_event(4,0,0,0,UIntPtr.Zero); } }
'@
function Stamp { (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
function NowUtc { (Get-Date).ToUniversalTime() }
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
    $s2 = Stamp; $o2 = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$rd\requests\zorder-probe-k.ps1" 2>&1; $e2 = Stamp
    $o2 | Set-Content -Encoding utf8 "$ev\$name-k.txt"
    "sheet probe $s to $e : " + (($o | % { "$_" }) -join ' | ') + " ;; K probe $s2 to $e2 : " + (($o2 | % { "$_" }) -join ' | ')
}
$tally = [GR]::ByTitle('TallyPrime:9001'); $tx = 330; $ty = 14
L "$Cell variant=$Variant; Tally window $tally; runner pid $PID"
# 1. Pre-arm the trace.
$trArgs = "-NoProfile -ExecutionPolicy Bypass -File $rd\requests\box-trace-early.ps1 -Title `"$title`" -Out $ev\trace-$c.txt -Seconds 100"
$tr = Start-Process powershell -ArgumentList $trArgs -PassThru; $null = $tr.Handle
L "$Cell trace pre-armed $(Stamp) pid $($tr.Id): powershell $trArgs"
Start-Sleep -Seconds 2
[GR]::ClickAt($tx, $ty); L "$Cell Tally title clicked (Tally in front) $(Stamp)"
Start-Sleep -Milliseconds 700
# 2. The sheet's two-line .cmd, opened through explorer; Tally's title clicked once about 4 s later.
$line2 = "powershell -NoProfile -ExecutionPolicy Bypass -File $rd\requests\spawn-parent.ps1 -Target control -ControlScript $rd\requests\control-box-fixes.ps1 -ControlVariant $Variant -Stdio tokio -Out $ev\$c.txt -TimeoutSeconds 60"
$cmdFile = "$rd\cmd\$c.cmd"
[IO.File]::WriteAllText($cmdFile, "powershell -NoProfile -Command Start-Sleep -Seconds 8`r`n$line2`r`n", (New-Object Text.ASCIIEncoding))
$pp = Start-Process explorer.exe -ArgumentList "`"$cmdFile`"" -PassThru; $st = NowUtc
L "$Cell started $(Stamp): explorer.exe `"$cmdFile`" (explorer pid $($pp.Id)); the .cmd holds: powershell -NoProfile -Command Start-Sleep -Seconds 8 | $line2"
Start-Sleep -Milliseconds $TitleClickAfterMs
[GR]::ClickAt($tx, $ty); L "$Cell Tally title clicked ONCE $(Stamp) ($([Math]::Round(((NowUtc) - $st).TotalSeconds, 2)) s after the start)"
# 3. T0, probes.
$deadline = (Get-Date).AddSeconds(60); $t0 = $null
while ((Get-Date) -lt $deadline -and -not $t0) {
    if (Test-Path "$ev\$c.txt") {
        $ln = Get-Content "$ev\$c.txt" -ErrorAction SilentlyContinue | Where-Object { $_ -match 'child_pid=' } | Select-Object -First 1
        if ($ln -and $ln -match '^SPAWN (\S+) ') { $t0 = [DateTime]::Parse($Matches[1]).ToUniversalTime(); L "$Cell-T0 $($Matches[1]) from: $ln" }
    }
    Start-Sleep -Milliseconds 50
}
if (-not $t0) { L "$Cell no child_pid line within 60 s: nothing clicked; stop"; exit 20 }
$sp = (Get-Content "$ev\$c.txt" | Where-Object { $_ -match 'parent_pid=' } | Select-Object -First 1)
if ($sp -match 'parent_pid=(\d+)') { $ppid = [int]$Matches[1]; $pi = Get-CimInstance Win32_Process -Filter "ProcessId=$ppid"; $gp = Get-CimInstance Win32_Process -Filter "ProcessId=$($pi.ParentProcessId)"
    L "$Cell spawn-parent pid $ppid image $($pi.Name), its parent pid $($pi.ParentProcessId) image $($gp.Name)" }
foreach ($at in 8, 11) {
    while (((NowUtc) - $t0).TotalSeconds -lt $at) { Start-Sleep -Milliseconds 20 }
    L "$Cell-T0+$at $(Probe "probe-$c-t0p$at")"
}
Shot "$c-t0p11-screen"; L "$Cell-T0+11 shot $(Stamp)"
while (((NowUtc) - $t0).TotalSeconds -lt 15) { Start-Sleep -Milliseconds 20 }
$box = [GR]::ByTitle($title)
if ($box -eq [IntPtr]::Zero) { L "$Cell-T0+15 $(Stamp): no box; nothing ended"; exit 21 }
$br = [GR]::Rect($box); $no = [IntPtr]::Zero
foreach ($k in [GR]::Kids($box)) { if ([GR]::Cls($k) -eq 'Button' -and [GR]::Txt($k) -eq '&No') { $no = $k } }
L "$Cell box $box rect $($br.L),$($br.T),$($br.R),$($br.B)"
# 4. No at about T0+20.
while (((NowUtc) - $t0).TotalSeconds -lt 20) { Start-Sleep -Milliseconds 20 }
$bp = [GR]::Pos($box); $tp = [GR]::Pos($tally); L "$Cell T0+20 $(Stamp): z box $bp, Tally $tp"
if (-not ($bp -gt 0 -and $bp -lt $tp)) {
    # The box's taskbar button is grouped with the PowerShell windows (a click opens thumbnails), UI Automation does not expose the
    # task buttons on this PC, and a z-order lift (SetWindowPos HWND_TOP, tried in G1d-2) is ignored. So, after the readings, Tally is
    # minimised (ShowWindow SW_MINIMIZE) to uncover the box, No is clicked, and Tally is restored (SW_RESTORE, back to maximised).
    $ex0 = [GR]::GetWindowLong($box, -20)
    [void][GR]::ShowWindow($tally, 6); Start-Sleep -Milliseconds 600; $script:minimised = $true
    $bp = [GR]::Pos($box); $tp = [GR]::Pos($tally)
    L "$Cell the box was below Tally: Tally minimised by ShowWindow(SW_MINIMIZE) $(Stamp); box exstyle 0x$($ex0.ToString('x8')); z box $bp"
    $tp = [int]::MaxValue
}
if ($no -ne [IntPtr]::Zero -and $bp -gt 0 -and $bp -lt $tp) {
    $nr = [GR]::Rect($no); $x = [int](($nr.L + $nr.R) / 2); $y = [int](($nr.T + $nr.B) / 2)
    [GR]::ClickAt($x, $y); L "$Cell No clicked $(Stamp) at T0+$([Math]::Round(((NowUtc) - $t0).TotalSeconds, 2)) s, logical ($x,$y)"
} else { L "$Cell No not clicked (box $bp, Tally $tp)"; if ($script:minimised) { [void][GR]::ShowWindow($tally, 9); L "$Cell Tally restored $(Stamp)" }; exit 26 }
if ($script:minimised) { Start-Sleep -Milliseconds 800; [void][GR]::ShowWindow($tally, 9); L "$Cell Tally restored by ShowWindow(SW_RESTORE) $(Stamp)" }
$e0 = Get-Date
while (((Get-Date) - $e0).TotalSeconds -lt 15) { if ((Get-Content "$ev\$c.txt" -ErrorAction SilentlyContinue) -match 'exit_code=') { break }; Start-Sleep -Milliseconds 200 }
L "$Cell spawn file: $((Get-Content "$ev\$c.txt" -ErrorAction SilentlyContinue) -join ' || ')"
if (Test-Path "$ev\$c.control.txt") { L "$Cell control file: $((Get-Content "$ev\$c.control.txt") -join ' ')" }
if (Test-Path "$ev\$c.control.txt.reassert.txt") { L "$Cell reassert: $((Get-Content "$ev\$c.control.txt.reassert.txt") -join ' || ')" }
exit 0
