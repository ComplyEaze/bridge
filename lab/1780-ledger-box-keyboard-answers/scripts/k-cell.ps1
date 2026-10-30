# Lab helper (#1780-K, not product code; v2: the read function renamed from R, which PowerShell took as Invoke-History's alias; row 1780-H's h-cell.ps1 changed to this sheet's cell recipe). One cell: pre-arms the trace
# (box-trace-early.ps1, its own window); clicks Tally's title once (the pre-arm click); writes the cell's .cmd (late: an 8 s sleep line,
# then the spawn-parent line; early: the spawn-parent line only) and opens it through explorer.exe; makes the timing click on Tally's
# title about 4 s (late) or 1.5 s (early) after the start; T0 is the stamp of the SPAWN ... child_pid= line of <cell>.txt, and Fg (the
# foreground window) is recorded when T0 is seen. Read R = the line count and last line of <cell>.stderr, box-rect-probe.ps1 and
# zorder-probe-k.ps1, each with its stamps, then a whole-screen capture (CopyFromScreen). R at T0+6. With -ClickInto (K1-K1 only):
# ONE click on a blank part of Tally's Gateway at T0+12 (at least 150 logical px from the box), R at once (C+0) and 6 s later (C+6),
# then No not before T0+24. Otherwise No at about T0+12. No is clicked only at the centre of the box's &No button read from the box
# at that moment; a box under Tally is answered with Tally minimised for the click and restored after it. No box by T0+15: nothing
# clicked. It never clicks Yes, sends no key, and ends nothing. It waits for <cell>.txt's exit line and the trace's END line.
param([Parameter(Mandatory = $true)][string]$Cell, [Parameter(Mandatory = $true)][ValidateSet('H', 'K')][string]$Build,
      [Parameter(Mandatory = $true)][ValidateSet('masters', 'journal')][string]$Mode,
      [Parameter(Mandatory = $true)][ValidateSet('late', 'early')][string]$Timing, [switch]$ClickInto)
$rd = '<LAB_DIR>'; $ev = "$rd\evidence"; $c = $Cell.ToLower(); $log = "$ev\$c-timeline.txt"; $title = 'ComplyEaze Bridge'
$exe = if ($Build -eq 'K') { "$rd\build\bin\x86_64-pc-windows-msvc\bridge_mcp.exe" } else { '<BUILD-H folder>\bin\x86_64-pc-windows-msvc\bridge_mcp.exe' }
$inp = if ($Mode -eq 'masters') { "$rd\requests\confirm-masters-input.txt" } else { "$rd\requests\confirm-input.txt" }
Add-Type -AssemblyName System.Windows.Forms, System.Drawing
Add-Type @'
using System; using System.Text; using System.Collections.Generic; using System.Runtime.InteropServices;
public class KR { public delegate bool P(IntPtr h, IntPtr l);
 [DllImport("user32.dll")] public static extern bool EnumWindows(P f, IntPtr l);
 [DllImport("user32.dll")] public static extern bool EnumChildWindows(IntPtr w, P f, IntPtr l);
 [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
 [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetClassName(IntPtr h, StringBuilder s, int n);
 [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
 [DllImport("user32.dll")] public static extern bool IsWindow(IntPtr h);
 [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
 [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
 [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
 [DllImport("user32.dll")] public static extern int GetWindowLong(IntPtr h, int i);
 [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h, int c);
 [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
 [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint p);
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
function Shot([string]$name) {
    $b = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
    $bmp = New-Object System.Drawing.Bitmap $b.Width, $b.Height
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.CopyFromScreen($b.Location, [System.Drawing.Point]::Empty, $b.Size)
    $bmp.Save("$rd\shots\$name.png"); $g.Dispose(); $bmp.Dispose()
}
function Fg { $f = [KR]::GetForegroundWindow(); [uint32]$fp = 0; $null = [KR]::GetWindowThreadProcessId($f, [ref]$fp); $im = (Get-Process -Id $fp -ErrorAction SilentlyContinue).ProcessName; "Fg hwnd 0x$($f.ToInt64().ToString('x')) pid $fp image $im title '$([KR]::Txt($f))'" }
function Read-CellState([string]$label) {
    $s0 = Stamp; $se = @(Get-Content "$ev\$c.stderr" -ErrorAction SilentlyContinue); $e0 = Stamp
    $a = "stderr $s0..$e0 lines $($se.Count) last '$(if ($se.Count) { $se[-1] })'"
    $s1 = Stamp; $o1 = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$rd\requests\box-rect-probe.ps1" -Out "$ev\rect-$c-$label.txt" 2>&1; $e1 = Stamp
    $b = "rect $s1..$e1 -> rect-$c-$label.txt: " + ((Get-Content "$ev\rect-$c-$label.txt" -ErrorAction SilentlyContinue | Where-Object { $_ -match '^(B|F|E) ' }) -join ' | ')
    $s2 = Stamp; $o2 = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$rd\requests\zorder-probe-k.ps1" 2>&1; $e2 = Stamp
    $o2 | Set-Content -Encoding utf8 "$ev\zorder-$c-$label.txt"
    $z = "zorder $s2..$e2 : " + (($o2 | % { "$_" }) -join ' | ')
    Shot "$c-$label-screen"
    "R $label :: $a ;; $b ;; $z ;; shot $c-$label-screen.png $(Stamp)"
}
function AnswerNo([string]$why) {
    if (-not [KR]::IsWindow($script:box)) { L "$Cell $why`: the box is gone; nothing clicked"; return }
    $script:no = [IntPtr]::Zero; foreach ($k in [KR]::Kids($script:box)) { if ([KR]::Cls($k) -eq 'Button' -and [KR]::Txt($k) -eq '&No') { $script:no = $k } }
    if ($script:no -eq [IntPtr]::Zero) { L "$Cell $why`: the No button was not found: nothing clicked (the lab answers No by mouse)"; return }
    $bp = [KR]::Pos($script:box); $tp = [KR]::Pos($tally); $min = $false
    if (-not ($bp -gt 0 -and $bp -lt $tp)) {
        [void][KR]::ShowWindow($tally, 6); Start-Sleep -Milliseconds 600; $min = $true
        L "$Cell $why`: the box was below Tally (z box $bp, Tally $tp): Tally minimised by ShowWindow(SW_MINIMIZE) $(Stamp)"
    }
    $nr = [KR]::Rect($script:no); $x = [int](($nr.L + $nr.R) / 2); $y = [int](($nr.T + $nr.B) / 2)
    [KR]::ClickAt($x, $y); L "$Cell $why`: No clicked $(Stamp) at T0+$([Math]::Round(((NowUtc) - $t0).TotalSeconds, 2)) s, logical ($x,$y) = screen tool $(Tool $x $y)"
    if ($min) { Start-Sleep -Milliseconds 800; [void][KR]::ShowWindow($tally, 9); L "$Cell Tally restored by ShowWindow(SW_RESTORE) $(Stamp)" }
}
$tally = [KR]::ByTitle('TallyPrime:9001'); $tx = 330; $ty = 14
L "$Cell runner pid $PID; build $Build ($exe); mode $Mode; timing $Timing; ClickInto=$ClickInto; Tally window $tally"
# 1. Pre-arm.
$trArgs = "-NoProfile -ExecutionPolicy Bypass -File $rd\requests\box-trace-early.ps1 -Title `"$title`" -Out $ev\trace-$c.txt -Seconds 100"
$tr = Start-Process powershell -ArgumentList $trArgs -PassThru; $null = $tr.Handle
L "$Cell trace pre-armed $(Stamp) pid $($tr.Id): powershell $trArgs"
Start-Sleep -Seconds 2
[KR]::ClickAt($tx, $ty); L "$Cell pre-arm click on Tally's title $(Stamp), logical ($tx,$ty) = screen tool $(Tool $tx $ty); $(Fg)"
Start-Sleep -Milliseconds 700
# 2. Start.
$sp = "powershell -NoProfile -ExecutionPolicy Bypass -File $rd\requests\spawn-parent.ps1 -Target child -ChildExe $exe -ChildMode $Mode -InputFile $inp -Stdio files -Out $ev\$c.txt -TimeoutSeconds 60"
$cmdFile = "$rd\cmd\$c.cmd"
$body = if ($Timing -eq 'late') { "powershell -NoProfile -Command Start-Sleep -Seconds 8`r`n$sp`r`n" } else { "$sp`r`n" }
[IO.File]::WriteAllText($cmdFile, $body, (New-Object Text.ASCIIEncoding))
$null = Start-Process explorer.exe -ArgumentList "`"$cmdFile`"" -PassThru; $st = NowUtc
L "$Cell started $(Stamp): explorer.exe `"$cmdFile`"; the .cmd holds: $($body -replace "`r`n", ' | ')"
Start-Sleep -Milliseconds $(if ($Timing -eq 'late') { 4000 } else { 1500 })
[KR]::ClickAt($tx, $ty); $tclick = NowUtc
L "$Cell timing click on Tally's title $(Stamp) ($([Math]::Round(($tclick - $st).TotalSeconds, 2)) s after the start), logical ($tx,$ty) = screen tool $(Tool $tx $ty)"
# 3. T0, Fg, R at T0+6.
$deadline = (Get-Date).AddSeconds(60); $t0 = $null
while ((Get-Date) -lt $deadline -and -not $t0) {
    if (Test-Path "$ev\$c.txt") {
        $ln = Get-Content "$ev\$c.txt" -ErrorAction SilentlyContinue | Where-Object { $_ -match 'child_pid=' } | Select-Object -First 1
        if ($ln -and $ln -match '^SPAWN (\S+) ') { $t0 = [DateTime]::Parse($Matches[1]).ToUniversalTime(); L "$Cell-T0 $($Matches[1]) from: $ln"; L "$Cell Fg at T0 seen $(Stamp): $(Fg)" }
    }
    Start-Sleep -Milliseconds 20
}
if (-not $t0) { L "$Cell no child_pid line within 60 s: nothing clicked"; L "$Cell DONE $(Stamp)"; exit 20 }
L "$Cell timing click to T0: $([Math]::Round(($t0 - $tclick).TotalSeconds, 2)) s"
$script:box = [IntPtr]::Zero; $script:no = [IntPtr]::Zero
function FindBox {
    if ($script:box -ne [IntPtr]::Zero) { return }
    $b = [KR]::ByTitle($title); if ($b -eq [IntPtr]::Zero) { return }
    $script:box = $b; $br = [KR]::Rect($b); $kids = @()
    foreach ($k in [KR]::Kids($b)) { $kc = [KR]::Cls($k); $kt = [KR]::Txt($k); $kr = [KR]::Rect($k); $kids += "CHILD`t$kc`t$($kr.L),$($kr.T),$($kr.R),$($kr.B)`t$kt" }
    [IO.File]::WriteAllText("$ev\$c-box-text.txt", ((@("TITLE`t$([KR]::Txt($b))", "RECT`t$($br.L),$($br.T),$($br.R),$($br.B)", "EXSTYLE`t0x$(([KR]::GetWindowLong($b, -20)).ToString('x8'))") + $kids) -join "`n") + "`n")
    L "$Cell box $b found $(Stamp) rect logical $($br.L),$($br.T),$($br.R),$($br.B); exstyle 0x$(([KR]::GetWindowLong($b, -20)).ToString('x8'))"
}
while (((NowUtc) - $t0).TotalSeconds -lt 6) { Start-Sleep -Milliseconds 20; FindBox }
L "$Cell-T0+6 $(Read-CellState 't0p6')"; FindBox
while (((NowUtc) - $t0).TotalSeconds -lt 15 -and $script:box -eq [IntPtr]::Zero) { Start-Sleep -Milliseconds 100; FindBox }
if ($script:box -eq [IntPtr]::Zero) { L "$Cell-T0+15 $(Stamp): no box; nothing clicked, nothing ended" }
elseif ($ClickInto) {
    $br = [KR]::Rect($script:box); $Px = 263; $Py = 475
    $dx = [Math]::Max(0, [Math]::Max($br.L - $Px, $Px - $br.R)); $dy = [Math]::Max(0, [Math]::Max($br.T - $Py, $Py - $br.B)); $dist = [Math]::Sqrt($dx * $dx + $dy * $dy)
    L "$Cell click point logical ($Px,$Py) = screen tool $(Tool $Px $Py); distance from the box $([Math]::Round($dist, 1)) px"
    if ($dist -lt 150) { L "$Cell the point is too near the box: no click into Tally" } else {
        while (((NowUtc) - $t0).TotalSeconds -lt 12) { Start-Sleep -Milliseconds 20 }
        [KR]::ClickAt($Px, $Py); $tc = NowUtc
        L "$Cell click into Tally $(Stamp) at T0+$([Math]::Round(($tc - $t0).TotalSeconds, 2)) s; $(Fg)"
        L "$Cell C+0 $(Read-CellState 'c0')"
        while (((NowUtc) - $tc).TotalSeconds -lt 6) { Start-Sleep -Milliseconds 20 }
        L "$Cell C+6 $(Read-CellState 'c6'); $(Fg)"
    }
    while (((NowUtc) - $t0).TotalSeconds -lt 24) { Start-Sleep -Milliseconds 50 }
    AnswerNo 'No (after C+6, not before T0+24)'
} else {
    while (((NowUtc) - $t0).TotalSeconds -lt 12) { Start-Sleep -Milliseconds 20 }
    AnswerNo 'No at about T0+12'
}
$e0 = Get-Date
while (((Get-Date) - $e0).TotalSeconds -lt 75) { if ((Get-Content "$ev\$c.txt" -ErrorAction SilentlyContinue) -match 'exit_code=') { break }; Start-Sleep -Milliseconds 200 }
L "$Cell spawn file: $((Get-Content "$ev\$c.txt" -ErrorAction SilentlyContinue) -join ' || ')"
if (Test-Path "$ev\$c.stderr") { L "$Cell stderr: $((Get-Content "$ev\$c.stderr") -join ' || ')" }
L "$Cell stdout bytes: $(if (Test-Path "$ev\$c.stdout") { (Get-Item "$ev\$c.stdout").Length } else { 'absent' })"
$e1 = Get-Date
while (((Get-Date) - $e1).TotalSeconds -lt 105) { if ((Get-Content "$ev\trace-$c.txt" -ErrorAction SilentlyContinue) -match '^END') { break }; Start-Sleep -Milliseconds 500 }
L "$Cell trace END seen: $([bool]((Get-Content "$ev\trace-$c.txt" -ErrorAction SilentlyContinue) -match '^END')) $(Stamp)"
L "$Cell DONE $(Stamp)"
exit 0
