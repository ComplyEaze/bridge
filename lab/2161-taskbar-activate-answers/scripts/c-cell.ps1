# Lab helper (#2161 part C, not product code; b-cell.ps1 turned to part C). The product's child under the test parent, error output
# kept. The box's task button is located in each cell from a PrintWindow capture taken while the box is open (locate-button.ps1: the
# rightmost button), since the screen copy and hover show no buttons while a box is open. Reads R right before each taskbar click,
# at once after it and 3 s after it. Modes:
#   C1: click into Tally at T0+16; at C+6 ONE left click on the box's button; then No if the box is open.
#   C2: no click into Tally; R at T0+12; at T0+16 ONE left click on the box's button; if the box is then minimised, at +6 s ONE
#       more left click on it; then No if the box is open and visible, nothing if still minimised (the 60 s bound) or gone.
#   C3: click into Tally at T0+16; at C+6 ONE right-click on the box's button; a whole-screen copy of the menu; then the lab's
#       command file cmd\<cell>-menu.txt: "click x y" (physical; ONE left click on the item labelled Close window) or "none"
#       (one click on the blank Tally point, then No).
# It never clicks Yes and sends no key.
param([Parameter(Mandatory = $true)][ValidateSet('C1', 'C2', 'C3')][string]$Mode, [int]$TitleClickAfterMs = 4000)
$rd = '<lab folder>'; $ev = "$rd\evidence"; $Cell = $Mode; $c = $Cell.ToLower(); $log = "$ev\$c-timeline.txt"; $title = 'ComplyEaze Bridge'
$exe = '<build folder>\bin\x86_64-pc-windows-msvc\bridge_mcp.exe'
Add-Type -AssemblyName System.Windows.Forms
Add-Type @'
using System; using System.Text; using System.Collections.Generic; using System.Runtime.InteropServices;
public class CC { public delegate bool P(IntPtr h, IntPtr l);
 [DllImport("user32.dll")] public static extern bool EnumWindows(P f, IntPtr l);
 [DllImport("user32.dll")] public static extern bool EnumChildWindows(IntPtr w, P f, IntPtr l);
 [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
 [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetClassName(IntPtr h, StringBuilder s, int n);
 [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
 [DllImport("user32.dll")] public static extern bool IsWindow(IntPtr h);
 [DllImport("user32.dll")] public static extern bool IsIconic(IntPtr h);
 [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
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
 public static void ClickAt(int x, int y) { SetCursorPos(x, y); mouse_event(2,0,0,0,UIntPtr.Zero); mouse_event(4,0,0,0,UIntPtr.Zero); }
 public static void RightClickAt(int x, int y) { SetCursorPos(x, y); mouse_event(8,0,0,0,UIntPtr.Zero); mouse_event(16,0,0,0,UIntPtr.Zero); } }
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
    $bl = (Select-String -Path "$ev\rect-$c-$label.txt" -Pattern '^B ' -ErrorAction SilentlyContinue | ForEach-Object { $_.Line }) -join ' | '
    "R $label $s to $(Stamp): $sl ;; box: $bl ;; probe: $(($o2 | % { "$_" }) -join ' | ')"
}
function CapK([string]$label) {
    $s = Stamp; $o = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$rd\requests\taskbar-capture.ps1" -Name "$c-$label" -OutDir "$rd\shots" -Out "$ev\captures.txt" 2>&1
    "K $label $s to $(Stamp): $(($o | % { "$_" }) -join ' | ')"
}
function Box { [CC]::ByTitle($title) }
function AnswerNo([IntPtr]$box, [string]$why) {
    if (-not [CC]::IsWindow($box)) { L "$Cell $why`: the box is gone; nothing clicked"; return }
    $no = [IntPtr]::Zero; foreach ($k in [CC]::Kids($box)) { if ([CC]::Cls($k) -eq 'Button' -and [CC]::Txt($k) -eq '&No') { $no = $k } }
    $bp = [CC]::Pos($box); $tp = [CC]::Pos($tally); $min = $false
    if (-not ($bp -gt 0 -and $bp -lt $tp)) { [void][CC]::ShowWindow($tally, 6); Start-Sleep -Milliseconds 600; $min = $true; L "$Cell $why`: the box was below Tally (z box $bp, Tally $tp): Tally minimised by ShowWindow(SW_MINIMIZE) $(Stamp)" }
    $nr = [CC]::Rect($no); $x = [int](($nr.L + $nr.R) / 2); $y = [int](($nr.T + $nr.B) / 2)
    if ($x -lt 0 -or $y -lt 0) { L "$Cell $why`: the No button is off-screen ($x,$y): nothing clicked"; if ($min) { [void][CC]::ShowWindow($tally, 9) }; return }
    [CC]::ClickAt($x, $y); L "$Cell $why`: No clicked $(Stamp) at T0+$([Math]::Round(((NowUtc) - $t0).TotalSeconds, 2)) s, logical ($x,$y) = screen tool $(Tool $x $y)"
    if ($min) { Start-Sleep -Milliseconds 800; [void][CC]::ShowWindow($tally, 9); L "$Cell Tally restored by ShowWindow(SW_RESTORE) $(Stamp)" }
}
function TaskClick([string]$label, [switch]$Right) {
    L "$Cell $(ReadR "$label-pre")"
    if ($Right) { [CC]::RightClickAt($script:bx, $script:by) } else { [CC]::ClickAt($script:bx, $script:by) }
    $script:lastClick = NowUtc
    L "$Cell $label $(if ($Right) { 'RIGHT-click' } else { 'left click' }) ONCE on the box's task button $(Stamp) at T0+$([Math]::Round(($script:lastClick - $t0).TotalSeconds, 2)) s, logical ($($script:bx),$($script:by)) = physical ($($script:px),$($script:py))"
    if (-not $Right) { L "$Cell $(ReadR "$label-c0")"; while (((NowUtc) - $script:lastClick).TotalSeconds -lt 3) { Start-Sleep -Milliseconds 20 }; L "$Cell $(ReadR "$label-c3")" }
}
$tally = [CC]::ByTitle('TallyPrime:9001'); $tx = 330; $ty = 14; $Px = 263; $Py = 475
L "$Cell runner pid $PID; Tally window $tally; mode $Mode"
$trArgs = "-NoProfile -ExecutionPolicy Bypass -File $rd\requests\box-trace-early.ps1 -Title `"$title`" -Out $ev\trace-$c.txt -Seconds 100"
$tr = Start-Process powershell -ArgumentList $trArgs -PassThru; $null = $tr.Handle
L "$Cell trace pre-armed $(Stamp) pid $($tr.Id): powershell $trArgs"
Start-Sleep -Seconds 2
[CC]::ClickAt($tx, $ty); L "$Cell Tally title clicked (Tally in front) $(Stamp)"
Start-Sleep -Milliseconds 700
$line2 = "powershell -NoProfile -ExecutionPolicy Bypass -File $rd\requests\spawn-parent.ps1 -Target child -ChildExe $exe -InputFile $rd\requests\confirm-input.txt -Stdio files -Out $ev\$c.txt -TimeoutSeconds 60"
$cmdFile = "$rd\cmd\$c.cmd"
[IO.File]::WriteAllText($cmdFile, "powershell -NoProfile -Command Start-Sleep -Seconds 8`r`n$line2`r`n", (New-Object Text.ASCIIEncoding))
$pp = Start-Process explorer.exe -ArgumentList "`"$cmdFile`"" -PassThru; $st = NowUtc
L "$Cell started $(Stamp): explorer.exe `"$cmdFile`"; the .cmd holds: powershell -NoProfile -Command Start-Sleep -Seconds 8 | $line2"
Start-Sleep -Milliseconds $TitleClickAfterMs
[CC]::ClickAt($tx, $ty); L "$Cell Tally title clicked ONCE $(Stamp) ($([Math]::Round(((NowUtc) - $st).TotalSeconds, 2)) s after the start)"
$deadline = (Get-Date).AddSeconds(60); $t0 = $null
while ((Get-Date) -lt $deadline -and -not $t0) {
    if (Test-Path "$ev\$c.txt") { $ln = Get-Content "$ev\$c.txt" -ErrorAction SilentlyContinue | Where-Object { $_ -match 'child_pid=' } | Select-Object -First 1; if ($ln -and $ln -match '^SPAWN (\S+) ') { $t0 = [DateTime]::Parse($Matches[1]).ToUniversalTime(); L "$Cell-T0 $($Matches[1]) from: $ln" } }
    Start-Sleep -Milliseconds 50
}
if (-not $t0) { L "$Cell no child_pid line within 60 s: nothing clicked; stop"; exit 20 }
while (((NowUtc) - $t0).TotalSeconds -lt 6) { Start-Sleep -Milliseconds 20 }
L "$Cell-T0+6 $(ReadR 't0p6')"; L "$Cell $(CapK 'open')"
$tl = (Select-String -Path "$ev\rect-$c-t0p6.txt" -Pattern 'class=MSTaskListWClass.*rect=\d+,\d+,(\d+),' | Select-Object -First 1); $right = if ($tl) { [int]$tl.Matches[0].Groups[1].Value - 8 } else { 1250 }
$loc = (& powershell.exe -NoProfile -File "$rd\locate-button.ps1" -Image "$rd\shots\$c-open-print.png" -Right $right) -split ' '
L "$Cell task list right edge $right (from rect-$c-t0p6.txt)"
if ($loc[0] -eq 'none') { L "$Cell no task button located in the PrintWindow capture: no taskbar click; No"; AnswerNo (Box) 'no button'; exit 22 }
$script:px = [int]$loc[0]; $script:py = [int]$loc[1]; $script:bx = [int][Math]::Round($script:px / 1.25); $script:by = [int][Math]::Round($script:py / 1.25)
L "$Cell box task button located at physical ($($script:px),$($script:py)) = logical ($($script:bx),$($script:by)) from shots\$c-open-print.png"
$box = Box; if ($box -eq [IntPtr]::Zero) { L "$Cell no box at T0+6: ended nothing"; exit 21 }
L "$Cell box $box rect $(([CC]::Rect($box)).L),$(([CC]::Rect($box)).T),$(([CC]::Rect($box)).R),$(([CC]::Rect($box)).B)"
if ($Mode -eq 'C1' -or $Mode -eq 'C3') {
    while (((NowUtc) - $t0).TotalSeconds -lt 16) { Start-Sleep -Milliseconds 20 }
    [CC]::ClickAt($Px, $Py); $tc = NowUtc; L "$Cell click into Tally $(Stamp) at T0+$([Math]::Round(($tc - $t0).TotalSeconds, 2)) s at logical ($Px,$Py)"
    L "$Cell C+0 $(ReadR 'c0')"
    while (((NowUtc) - $tc).TotalSeconds -lt 6) { Start-Sleep -Milliseconds 20 }
    if ($Mode -eq 'C1') {
        TaskClick 'tb1'
        if ([CC]::IsWindow($box)) { AnswerNo $box 'C1 after the taskbar click' } else { L "$Cell the box is gone after the taskbar click; nothing more clicked" }
    } else {
        TaskClick 'tb1' -Right
        Start-Sleep -Milliseconds 1200
        & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$rd\hover-shot.ps1" -Name "$c-menu" -Points 'none' | Out-Null
        L "$Cell menu capture shots\$c-menu-full.png $(Stamp); waiting for cmd\$c-menu.txt"
        $mf = "$rd\cmd\$c-menu.txt"; $w0 = Get-Date
        while (-not (Test-Path $mf) -and ((Get-Date) - $w0).TotalSeconds -lt 25) { Start-Sleep -Milliseconds 100 }
        $cmdw = if (Test-Path $mf) { (Get-Content $mf -Raw).Trim() -split '\s+' } else { @('none') }
        if ($cmdw[0] -eq 'click') {
            $mx = [int][Math]::Round([int]$cmdw[1] / 1.25); $my = [int][Math]::Round([int]$cmdw[2] / 1.25)
            L "$Cell R before the menu click: $(ReadR 'menu-pre')"
            [CC]::ClickAt($mx, $my); $script:lastClick = NowUtc; L "$Cell left click ONCE on 'Close window' $(Stamp) at physical ($($cmdw[1]),$($cmdw[2])) = logical ($mx,$my)"
            L "$Cell $(ReadR 'menu-c0')"; Start-Sleep -Seconds 3; L "$Cell $(ReadR 'menu-c3')"
            if ([CC]::IsWindow($box)) { AnswerNo $box 'C3 box still open after Close window' }
        } else {
            [CC]::ClickAt($Px, $Py); L "$Cell no Close window item: one click on the blank Tally point $(Stamp)"
            AnswerNo $box 'C3 no Close window item'
        }
    }
} else {
    while (((NowUtc) - $t0).TotalSeconds -lt 12) { Start-Sleep -Milliseconds 20 }
    $fg = [CC]::GetForegroundWindow(); L "$Cell T0+12 $(ReadR 't0p12'); box in front: $($fg -eq $box)"
    if ($fg -ne $box) { L "$Cell C2 not reached: the box is not in front"; AnswerNo $box 'C2 not reached' } else {
        while (((NowUtc) - $t0).TotalSeconds -lt 16) { Start-Sleep -Milliseconds 20 }
        TaskClick 'tb1'
        if (-not [CC]::IsWindow($box)) { L "$Cell the box is gone after the first click" }
        elseif ([CC]::IsIconic($box)) {
            L "$Cell the box reads minimised after the first click"
            while (((NowUtc) - $script:lastClick).TotalSeconds -lt 6) { Start-Sleep -Milliseconds 20 }
            TaskClick 'tb2'
            if (-not [CC]::IsWindow($box)) { L "$Cell the box is gone after the restoring click" }
            elseif ([CC]::IsIconic($box)) { L "$Cell still minimised: nothing more clicked; the 60 s bound will end it" }
            else { AnswerNo $box 'C2 restored and open' }
        } else { AnswerNo $box 'C2 open and not minimised' }
    }
}
$e0 = Get-Date
while (((Get-Date) - $e0).TotalSeconds -lt 70) { if ((Get-Content "$ev\$c.txt" -ErrorAction SilentlyContinue) -match 'exit_code=') { break }; Start-Sleep -Milliseconds 200 }
L "$Cell spawn file: $((Get-Content "$ev\$c.txt" -ErrorAction SilentlyContinue) -join ' || ')"
if (Test-Path "$ev\$c.stderr") { L "$Cell stderr: $((Get-Content "$ev\$c.stderr") -join ' || ')" }
$gone = (Select-String -Path "$ev\trace-$c.txt" -Pattern '^GONE (\S+)' -ErrorAction SilentlyContinue | Select-Object -First 1)
if ($gone -and $script:lastClick) { $gt = [DateTime]::Parse(($gone.Line -split ' ')[1]).ToUniversalTime(); L "$Cell last click to GONE: $([Math]::Round(($gt - $script:lastClick).TotalMilliseconds)) ms (GONE $($gone.Line))" }
L "$Cell DONE $(Stamp)"
exit 0
