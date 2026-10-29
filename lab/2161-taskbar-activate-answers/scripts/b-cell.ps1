# Lab helper (#2161 part B, not product code; row 1780-H's h-cell.ps1 turned to this sheet). One cell: pre-arms the trace (its
# own window); writes the sheet's two-line <cell>.cmd and opens it through explorer.exe; clicks Tally's title ONCE about 4 s after
# the start; T0 is the SPAWN ... child_pid= stamp of <cell>.txt. At T0+6 a read R then a capture K labelled open, then it writes
# runs hover-shot.ps1 (a whole-screen copy and two hovers, in place of the screen tool, which hides the box); the click into Tally
# at T0+16 (or at once if the go came later), R at once, K labelled clicked, then hover-shot.ps1 again;
# No by mouse at C+12 (if the box lies under Tally, Tally is minimised for the click and restored after it).
# A read R, inline and in order: <cell>.stderr's line count and last line, box-rect-probe.ps1, zorder-probe-k.ps1. A capture K:
# taskbar-capture.ps1. It never clicks Yes and sends no key.
param([Parameter(Mandatory = $true)][string]$Cell, [Parameter(Mandatory = $true)][ValidateSet('control', 'child')][string]$Target,
      [string]$Variant = 'h', [int]$TitleClickAfterMs = 4000)
$rd = '<lab folder>'; $ev = "$rd\evidence"; $c = $Cell.ToLower(); $log = "$ev\$c-timeline.txt"
$title = if ($Target -eq 'control') { 'Lab topmost control' } else { 'ComplyEaze Bridge' }
$exe = '<build folder>\bin\x86_64-pc-windows-msvc\bridge_mcp.exe'
Add-Type -AssemblyName System.Windows.Forms
Add-Type @'
using System; using System.Text; using System.Collections.Generic; using System.Runtime.InteropServices;
public class BC { public delegate bool P(IntPtr h, IntPtr l);
 [DllImport("user32.dll")] public static extern bool EnumWindows(P f, IntPtr l);
 [DllImport("user32.dll")] public static extern bool EnumChildWindows(IntPtr w, P f, IntPtr l);
 [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
 [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetClassName(IntPtr h, StringBuilder s, int n);
 [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
 [DllImport("user32.dll")] public static extern bool IsWindow(IntPtr h);
 [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
 [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
 [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
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
function Tool([int]$x, [int]$y) { "($([Math]::Round($x * 1456 / 1536)),$([Math]::Round($y * 819 / 864)))" }
function ReadR([string]$label) {
    $s = Stamp; $err = "$ev\$c.stderr"; $sl = 'no stderr file'
    if (Test-Path $err) { $fs = [IO.File]::Open($err, 'Open', 'Read', 'ReadWrite'); $sr = New-Object IO.StreamReader($fs); $all = @($sr.ReadToEnd() -split "`r?`n" | Where-Object { $_ -ne '' }); $sr.Close(); $fs.Close(); $sl = "stderr lines $($all.Count); last: $(if ($all.Count) { $all[-1] } else { '-' })" }
    $o1 = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$rd\requests\box-rect-probe.ps1" -Out "$ev\rect-$c-$label.txt" 2>&1
    $o2 = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$rd\requests\zorder-probe-k.ps1" 2>&1
    $o2 | Set-Content -Encoding utf8 "$ev\probe-$c-$label.txt"
    "R $label $s to $(Stamp): $sl ;; rect file rect-$c-$label.txt ($(($o1 | % { "$_" }) -join ' | ')) ;; probe: $(($o2 | % { "$_" }) -join ' | ')"
}
function CapK([string]$label) {
    $s = Stamp; $o = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$rd\requests\taskbar-capture.ps1" -Name "$c-$label" -OutDir "$rd\shots" -Out "$ev\captures.txt" 2>&1
    "K $label $s to $(Stamp): $(($o | % { "$_" }) -join ' | ')"
}
function WaitGo([string]$name, [double]$until, [datetime]$from) {
    New-Item -ItemType File -Force "$rd\cmd\$c-$name-ready" | Out-Null
    while (((NowUtc) - $from).TotalSeconds -lt $until -and -not (Test-Path "$rd\cmd\$c-$name-go")) { Start-Sleep -Milliseconds 100 }
    "wait $name ended $(Stamp): go $(Test-Path "$rd\cmd\$c-$name-go")"
}
$tally = [BC]::ByTitle('TallyPrime:9001'); $tx = 330; $ty = 14
L "$Cell runner pid $PID; Tally window $tally; target $Target variant $Variant"
$trArgs = "-NoProfile -ExecutionPolicy Bypass -File $rd\requests\box-trace-early.ps1 -Title `"$title`" -Out $ev\trace-$c.txt -Seconds 100"
$tr = Start-Process powershell -ArgumentList $trArgs -PassThru; $null = $tr.Handle
L "$Cell trace pre-armed $(Stamp) pid $($tr.Id): powershell $trArgs"
Start-Sleep -Seconds 2
[BC]::ClickAt($tx, $ty); L "$Cell Tally title clicked (Tally in front) $(Stamp)"
Start-Sleep -Milliseconds 700
if ($Target -eq 'control') {
    $line2 = "powershell -NoProfile -ExecutionPolicy Bypass -File $rd\requests\spawn-parent.ps1 -Target control -ControlScript $rd\requests\control-box-fixes.ps1 -ControlVariant $Variant -Stdio tokio -Out $ev\$c.txt -TimeoutSeconds 60"
} else {
    $line2 = "powershell -NoProfile -ExecutionPolicy Bypass -File $rd\requests\spawn-parent.ps1 -Target child -ChildExe $exe -InputFile $rd\requests\confirm-input.txt -Stdio files -Out $ev\$c.txt -TimeoutSeconds 60"
}
$cmdFile = "$rd\cmd\$c.cmd"
[IO.File]::WriteAllText($cmdFile, "powershell -NoProfile -Command Start-Sleep -Seconds 8`r`n$line2`r`n", (New-Object Text.ASCIIEncoding))
$pp = Start-Process explorer.exe -ArgumentList "`"$cmdFile`"" -PassThru; $st = NowUtc
L "$Cell started $(Stamp): explorer.exe `"$cmdFile`"; the .cmd holds: powershell -NoProfile -Command Start-Sleep -Seconds 8 | $line2"
Start-Sleep -Milliseconds $TitleClickAfterMs
[BC]::ClickAt($tx, $ty); L "$Cell Tally title clicked ONCE $(Stamp) ($([Math]::Round(((NowUtc) - $st).TotalSeconds, 2)) s after the start)"
$deadline = (Get-Date).AddSeconds(60); $t0 = $null
while ((Get-Date) -lt $deadline -and -not $t0) {
    if (Test-Path "$ev\$c.txt") {
        $ln = Get-Content "$ev\$c.txt" -ErrorAction SilentlyContinue | Where-Object { $_ -match 'child_pid=' } | Select-Object -First 1
        if ($ln -and $ln -match '^SPAWN (\S+) ') { $t0 = [DateTime]::Parse($Matches[1]).ToUniversalTime(); L "$Cell-T0 $($Matches[1]) from: $ln" }
    }
    Start-Sleep -Milliseconds 50
}
if (-not $t0) { L "$Cell no child_pid line within 60 s: nothing clicked; stop"; exit 20 }
while (((NowUtc) - $t0).TotalSeconds -lt 6) { Start-Sleep -Milliseconds 20 }
L "$Cell-T0+6 $(ReadR 't0p6')"; L "$Cell $(CapK 'open')"
$box = [BC]::ByTitle($title)
L "$Cell lab view 3: $(& powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$rd\hover-shot.ps1" -Name "$c-view3" 2>&1)"
if ($box -eq [IntPtr]::Zero) { $box = [BC]::ByTitle($title) }
if ($box -eq [IntPtr]::Zero) { L "$Cell no box by T0+15.5 $(Stamp): ended nothing"; $e0 = Get-Date; while (((Get-Date) - $e0).TotalSeconds -lt 70) { if ((Get-Content "$ev\$c.txt" -ErrorAction SilentlyContinue) -match 'exit_code=') { break }; Start-Sleep -Milliseconds 200 }; exit 21 }
$no = [IntPtr]::Zero; foreach ($k in [BC]::Kids($box)) { if ([BC]::Cls($k) -eq 'Button' -and [BC]::Txt($k) -eq '&No') { $no = $k } }
$br = [BC]::Rect($box); $Px = 263; $Py = 475
$dx = [Math]::Max(0, [Math]::Max($br.L - $Px, $Px - $br.R)); $dy = [Math]::Max(0, [Math]::Max($br.T - $Py, $Py - $br.B)); $dist = [Math]::Sqrt($dx * $dx + $dy * $dy)
L "$Cell box $box rect logical $($br.L),$($br.T),$($br.R),$($br.B); click point logical ($Px,$Py) = screen tool $(Tool $Px $Py); distance $([Math]::Round($dist, 1)) px"
while (((NowUtc) - $t0).TotalSeconds -lt 16) { Start-Sleep -Milliseconds 20 }
if ($dist -lt 150) { L "$Cell the point is too near the box: no click into Tally" } else {
    [BC]::ClickAt($Px, $Py); $tc = NowUtc; L "$Cell click into Tally $(Stamp) at T0+$([Math]::Round(($tc - $t0).TotalSeconds, 2)) s"
    L "$Cell C+0 $(ReadR 'c0')"; L "$Cell $(CapK 'clicked')"
    L "$Cell lab view 4: $(& powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$rd\hover-shot.ps1" -Name "$c-view4" 2>&1)"
    while (((NowUtc) - $tc).TotalSeconds -lt 12) { Start-Sleep -Milliseconds 20 }
}
if ([BC]::IsWindow($box)) {
    $bp = [BC]::Pos($box); $tp = [BC]::Pos($tally); $min = $false
    if (-not ($bp -gt 0 -and $bp -lt $tp)) { [void][BC]::ShowWindow($tally, 6); Start-Sleep -Milliseconds 600; $min = $true; L "$Cell the box was below Tally (z box $bp, Tally $tp): Tally minimised by ShowWindow(SW_MINIMIZE) $(Stamp)" }
    $nr = [BC]::Rect($no); $x = [int](($nr.L + $nr.R) / 2); $y = [int](($nr.T + $nr.B) / 2)
    [BC]::ClickAt($x, $y); L "$Cell No clicked $(Stamp) at T0+$([Math]::Round(((NowUtc) - $t0).TotalSeconds, 2)) s, logical ($x,$y) = screen tool $(Tool $x $y)"
    if ($min) { Start-Sleep -Milliseconds 800; [void][BC]::ShowWindow($tally, 9); L "$Cell Tally restored by ShowWindow(SW_RESTORE) $(Stamp)" }
} else { L "$Cell the box is gone before No $(Stamp)" }
$e0 = Get-Date
while (((Get-Date) - $e0).TotalSeconds -lt 70) { if ((Get-Content "$ev\$c.txt" -ErrorAction SilentlyContinue) -match 'exit_code=') { break }; Start-Sleep -Milliseconds 200 }
L "$Cell spawn file: $((Get-Content "$ev\$c.txt" -ErrorAction SilentlyContinue) -join ' || ')"
if (Test-Path "$ev\$c.control.txt") { L "$Cell control file: $((Get-Content "$ev\$c.control.txt") -join ' ')" }
if (Test-Path "$ev\$c.stderr") { L "$Cell stderr: $((Get-Content "$ev\$c.stderr") -join ' || ')" }
L "$Cell DONE $(Stamp)"
exit 0
