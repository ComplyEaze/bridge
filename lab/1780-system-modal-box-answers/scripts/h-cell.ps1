# Lab helper (#1780-H part H, not product code; row 1780-G's g-run.ps1 with the sheet's child recipe, the click into Tally and the
# T4 command loop of row 1780-E's h-run.ps1). One cell: pre-arms the trace (box-trace-early.ps1, its own window); writes the sheet's
# two-line <cell>.cmd and opens it through explorer.exe (a visible console whose parent is explorer; nothing typed); clicks Tally's
# title ONCE about 4 s after the start; T0 is the stamp of the SPAWN ... child_pid= line of <cell>.txt; probe pairs (zorder-probe.ps1
# then zorder-probe-k.ps1) at T0+8 and T0+11 and a whole-screen capture. Then, unless -NoClickInto, ONE click on a blank part of
# Tally's Gateway at T0+14 (at least 150 logical px from the box), probes and captures at once (C+0) and 6 s later (C+6). Then it
# obeys the lab's command files cmd\<cell>-<n>.txt: "taskbar x y" or "thumb x y" (one click at that logical point, one probe at once),
# "no" (No by mouse; if the box lies under Tally, Tally is minimised for the click and restored after it), "end". With -NoAt it
# clicks No by itself at T0+NoAt. If no command has answered the box by T0+48, it clicks No by itself (recorded). It never clicks Yes
# and sends no key.
param([Parameter(Mandatory = $true)][string]$Cell, [switch]$NoClickInto, [int]$NoAt = 0, [int]$TitleClickAfterMs = 4000)
$rd = '<lab folder>'; $ev = "$rd\evidence"; $c = $Cell.ToLower(); $log = "$ev\$c-timeline.txt"; $title = 'ComplyEaze Bridge'
$exe = "$rd\build\bin\x86_64-pc-windows-msvc\bridge_mcp.exe"
Add-Type -AssemblyName System.Windows.Forms, System.Drawing
Add-Type @'
using System; using System.Text; using System.Collections.Generic; using System.Runtime.InteropServices;
public class HR { public delegate bool P(IntPtr h, IntPtr l);
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
function Probe([string]$name, [switch]$One) {
    $s = Stamp; $o = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$rd\requests\zorder-probe.ps1" 2>&1; $e = Stamp
    $o | Set-Content -Encoding utf8 "$ev\$name.txt"
    $line = "probe $s to $e : " + (($o | % { "$_" }) -join ' | ')
    if ($One) { return $line }
    $s2 = Stamp; $o2 = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$rd\requests\zorder-probe-k.ps1" 2>&1; $e2 = Stamp
    $o2 | Set-Content -Encoding utf8 "$ev\$name-k.txt"
    "$line ;; K probe $s2 to $e2 : " + (($o2 | % { "$_" }) -join ' | ')
}
function Fg { $f = [HR]::GetForegroundWindow(); [uint32]$fp = 0; $null = [HR]::GetWindowThreadProcessId($f, [ref]$fp); "fg 0x$($f.ToInt64().ToString('x')) pid $fp '$([HR]::Txt($f))'" }
function AnswerNo([string]$why) {
    if (-not [HR]::IsWindow($script:box)) { L "$Cell $why`: the box is gone; nothing clicked"; return }
    $bp = [HR]::Pos($script:box); $tp = [HR]::Pos($tally); $min = $false
    if (-not ($bp -gt 0 -and $bp -lt $tp)) {
        [void][HR]::ShowWindow($tally, 6); Start-Sleep -Milliseconds 600; $min = $true
        L "$Cell $why`: the box was below Tally (z box $bp, Tally $tp): Tally minimised by ShowWindow(SW_MINIMIZE) $(Stamp)"
    }
    $nr = [HR]::Rect($script:no); $x = [int](($nr.L + $nr.R) / 2); $y = [int](($nr.T + $nr.B) / 2)
    [HR]::ClickAt($x, $y); L "$Cell $why`: No clicked $(Stamp) at T0+$([Math]::Round(((NowUtc) - $t0).TotalSeconds, 2)) s, logical ($x,$y) = screen tool $(Tool $x $y)"
    if ($min) { Start-Sleep -Milliseconds 800; [void][HR]::ShowWindow($tally, 9); L "$Cell Tally restored by ShowWindow(SW_RESTORE) $(Stamp)" }
}
$tally = [HR]::ByTitle('TallyPrime:9001'); $tx = 330; $ty = 14
L "$Cell runner pid $PID; Tally window $tally; NoClickInto=$NoClickInto NoAt=$NoAt"
# 1. Pre-arm the trace.
$trArgs = "-NoProfile -ExecutionPolicy Bypass -File $rd\requests\box-trace-early.ps1 -Title `"$title`" -Out $ev\trace-$c.txt -Seconds 100"
$tr = Start-Process powershell -ArgumentList $trArgs -PassThru; $null = $tr.Handle
L "$Cell trace pre-armed $(Stamp) pid $($tr.Id): powershell $trArgs"
Start-Sleep -Seconds 2
[HR]::ClickAt($tx, $ty); L "$Cell Tally title clicked (Tally in front) $(Stamp)"
Start-Sleep -Milliseconds 700
# 2. The sheet's two-line .cmd, opened through explorer; Tally's title clicked once about 4 s later.
$line2 = "powershell -NoProfile -ExecutionPolicy Bypass -File $rd\requests\spawn-parent.ps1 -Target child -ChildExe $exe -InputFile $rd\requests\confirm-input.txt -Stdio files -Out $ev\$c.txt -TimeoutSeconds 60"
$cmdFile = "$rd\cmd\$c.cmd"
[IO.File]::WriteAllText($cmdFile, "powershell -NoProfile -Command Start-Sleep -Seconds 8`r`n$line2`r`n", (New-Object Text.ASCIIEncoding))
$pp = Start-Process explorer.exe -ArgumentList "`"$cmdFile`"" -PassThru; $st = NowUtc
L "$Cell started $(Stamp): explorer.exe `"$cmdFile`"; the .cmd holds: powershell -NoProfile -Command Start-Sleep -Seconds 8 | $line2"
Start-Sleep -Milliseconds $TitleClickAfterMs
[HR]::ClickAt($tx, $ty); L "$Cell Tally title clicked ONCE $(Stamp) ($([Math]::Round(((NowUtc) - $st).TotalSeconds, 2)) s after the start)"
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
$script:box = [IntPtr]::Zero; $script:no = [IntPtr]::Zero
function FindBox {
    if ($script:box -ne [IntPtr]::Zero) { return }
    $b = [HR]::ByTitle($title); if ($b -eq [IntPtr]::Zero) { return }
    $script:box = $b; $br = [HR]::Rect($b); $kids = @()
    foreach ($k in [HR]::Kids($b)) { $kc = [HR]::Cls($k); $kt = [HR]::Txt($k); $kr = [HR]::Rect($k); $kids += "CHILD`t$kc`t$($kr.L),$($kr.T),$($kr.R),$($kr.B)`t$kt"; if ($kc -eq 'Button' -and $kt -eq '&No') { $script:no = $k } }
    [IO.File]::WriteAllText("$ev\$c-box-text.txt", ((@("TITLE`t$([HR]::Txt($b))", "RECT`t$($br.L),$($br.T),$($br.R),$($br.B)", "EXSTYLE`t0x$(([HR]::GetWindowLong($b, -20)).ToString('x8'))") + $kids) -join "`n") + "`n")
    L "$Cell box $b found $(Stamp) rect logical $($br.L),$($br.T),$($br.R),$($br.B); exstyle 0x$(([HR]::GetWindowLong($b, -20)).ToString('x8')); No button $($script:no)"
}
if ($NoAt -gt 0) {
    while (((NowUtc) - $t0).TotalSeconds -lt 8) { Start-Sleep -Milliseconds 20 }
    L "$Cell-T0+8 $(Probe "probe-$c-t0p8")"; FindBox
    while (((NowUtc) - $t0).TotalSeconds -lt $NoAt) { Start-Sleep -Milliseconds 20 }
    FindBox; if ($script:box -eq [IntPtr]::Zero) { L "$Cell-T0+$NoAt no box; nothing clicked" } else { AnswerNo "T0+$NoAt" }
} else {
    foreach ($at in 8, 11) {
        while (((NowUtc) - $t0).TotalSeconds -lt $at) { Start-Sleep -Milliseconds 20 }
        L "$Cell-T0+$at $(Probe "probe-$c-t0p$at")"; FindBox
    }
    Shot "$c-t0p11-screen"; L "$Cell-T0+11 shot $(Stamp)"
    while (((NowUtc) - $t0).TotalSeconds -lt 15 -and $script:box -eq [IntPtr]::Zero) { Start-Sleep -Milliseconds 100; FindBox }
    if ($script:box -eq [IntPtr]::Zero) { L "$Cell-T0+15 $(Stamp): no box; ended nothing" }
    elseif (-not $NoClickInto) {
        # 4. The click into Tally at T0+14: a blank part of the Gateway's left pane, at least 150 logical px from the box.
        $br = [HR]::Rect($script:box); $Px = 263; $Py = 475
        $dx = [Math]::Max(0, [Math]::Max($br.L - $Px, $Px - $br.R)); $dy = [Math]::Max(0, [Math]::Max($br.T - $Py, $Py - $br.B)); $dist = [Math]::Sqrt($dx * $dx + $dy * $dy)
        L "$Cell click point logical ($Px,$Py) = screen tool $(Tool $Px $Py); distance from the box $([Math]::Round($dist, 1)) px"
        if ($dist -lt 150) { L "$Cell the point is too near the box: no click into Tally" } else {
            while (((NowUtc) - $t0).TotalSeconds -lt 14) { Start-Sleep -Milliseconds 20 }
            [HR]::ClickAt($Px, $Py); $tc = NowUtc
            L "$Cell click into Tally $(Stamp) at T0+$([Math]::Round(($tc - $t0).TotalSeconds, 2)) s; $(Fg)"
            L "$Cell C+0 $(Probe "probe-$c-c0")"; Shot "$c-c0-screen"; L "$Cell C+0 shot $(Stamp)"
            while (((NowUtc) - $tc).TotalSeconds -lt 6) { Start-Sleep -Milliseconds 20 }
            L "$Cell C+6 $(Probe "probe-$c-c6")"; Shot "$c-c6-screen"; L "$Cell C+6 shot $(Stamp); $(Fg)"
        }
    }
    L "$Cell READY-FOR-COMMANDS $(Stamp) T0+$([Math]::Round(((NowUtc) - $t0).TotalSeconds, 1)) s; box a window $([HR]::IsWindow($script:box))"
    # 5. Commands (T4 and No).
    $n = 1
    while ([HR]::IsWindow($script:box)) {
        if (((NowUtc) - $t0).TotalSeconds -gt 48) { AnswerNo 'T0+48 fallback (no command answered the box)'; break }
        $cf = "$rd\cmd\$c-$n.txt"
        if (-not (Test-Path $cf)) { Start-Sleep -Milliseconds 100; continue }
        $w = (Get-Content $cf -Raw).Trim() -split '\s+'; $n++
        if ($w[0] -eq 'taskbar' -or $w[0] -eq 'thumb') {
            $bx = [int]$w[1]; $by = [int]$w[2]; [HR]::ClickAt($bx, $by)
            L "$Cell T4 $($w[0]) clicked ONCE $(Stamp) at T0+$([Math]::Round(((NowUtc) - $t0).TotalSeconds, 2)) s, logical ($bx,$by) = screen tool $(Tool $bx $by)"
            L "$Cell T4 probe $(Probe "probe-$c-t4-$($w[0])" -One); box a window $([HR]::IsWindow($script:box)); $(Fg)"
            Shot "$c-t4-$($w[0])-screen"
        } elseif ($w[0] -eq 'no') { AnswerNo 'command no'; break }
        elseif ($w[0] -eq 'end') { L "$Cell end command $(Stamp)"; break }
    }
}
$e0 = Get-Date
while (((Get-Date) - $e0).TotalSeconds -lt 70) { if ((Get-Content "$ev\$c.txt" -ErrorAction SilentlyContinue) -match 'exit_code=') { break }; Start-Sleep -Milliseconds 200 }
L "$Cell spawn file: $((Get-Content "$ev\$c.txt" -ErrorAction SilentlyContinue) -join ' || ')"
if (Test-Path "$ev\$c.stderr") { L "$Cell stderr: $((Get-Content "$ev\$c.stderr") -join ' || ')" }
L "$Cell DONE $(Stamp)"
exit 0
