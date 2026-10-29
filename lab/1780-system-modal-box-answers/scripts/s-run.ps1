# Lab helper (#1780-H part S, not product code; h-cell.ps1 turned to the server path). One post: pre-arms the trace (150 s); clicks
# Tally's title ONCE; queues post_import (company_guid and -BatchId) in the stdio client's inbox: T0 is that moment; probes at T0+3,
# +8 and +13 and a whole-screen capture after the last; the box's text to evidence\<Run>-box-text.txt; ONE click on a blank part of
# Tally's Gateway at T0+16 (at least 150 physical px from the box), probes and captures at C+0 and C+6. Then:
#   -Full (S6 to S8): S7, ONE Down key only if the box is open and Tally holds the foreground (a capture just before and just after);
#     S8, from T0+26 a taskbar capture, then the lab's command files cmd\<Run>-<n>.txt ("taskbar x y", "thumb x y", "no"), with No
#     by mouse by itself at T0+36 if no command answered the box.
#   otherwise (S9): No by mouse at T0+30.
# No by mouse: if the box lies under Tally, Tally is minimised for the click and restored after it. It never clicks Yes and presses
# no key other than S7's one Down.
param([Parameter(Mandatory = $true)][string]$Run, [Parameter(Mandatory = $true)][string]$BatchId, [switch]$Full,
      [string]$Tool = 'post_import', [int]$WaitBeforeT0Ms = 500)
$rd = '<lab folder>'; $ev = "$rd\evidence"; $c = $Run.ToLower(); $log = "$ev\$c-timeline.txt"; $title = 'ComplyEaze Bridge'
$guid = 'a30f23c3-4819-4334-96ab-ffa6fba70976'
Add-Type -AssemblyName System.Windows.Forms, System.Drawing
Add-Type @'
using System; using System.Text; using System.Collections.Generic; using System.Runtime.InteropServices;
public class SR { public delegate bool P(IntPtr h, IntPtr l);
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
 [DllImport("user32.dll")] public static extern IntPtr GetFocus();
 [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint p);
 [DllImport("user32.dll")] public static extern void mouse_event(uint f, uint x, uint y, uint d, UIntPtr e);
 [DllImport("user32.dll")] public static extern void keybd_event(byte vk, byte scan, uint f, UIntPtr e);
 public static string Txt(IntPtr h) { var s = new StringBuilder(8192); GetWindowText(h, s, 8192); return s.ToString(); }
 public static string Cls(IntPtr h) { var s = new StringBuilder(64); GetClassName(h, s, 64); return s.ToString(); }
 public static IntPtr ByTitle(string t) { IntPtr r = IntPtr.Zero; EnumWindows((h,l)=>{ if (IsWindowVisible(h) && Txt(h).StartsWith(t)) { r = h; return false; } return true; }, IntPtr.Zero); return r; }
 public static int Pos(IntPtr t) { int i = 0, f = -1; EnumWindows((h,l)=>{ if (!IsWindowVisible(h)) return true; i++; if (h == t) { f = i; return false; } return true; }, IntPtr.Zero); return f; }
 public static List<IntPtr> Kids(IntPtr w) { var o = new List<IntPtr>(); EnumChildWindows(w, (h,l)=>{ o.Add(h); return true; }, IntPtr.Zero); return o; }
 public static RECT Rect(IntPtr h) { RECT r; GetWindowRect(h, out r); return r; }
 public static void ClickAt(int x, int y) { SetCursorPos(x, y); mouse_event(2,0,0,0,UIntPtr.Zero); mouse_event(4,0,0,0,UIntPtr.Zero); }
 [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
 public static void Down() { keybd_event(0x28, 0, 0, UIntPtr.Zero); keybd_event(0x28, 0, 2, UIntPtr.Zero); } }
'@
[void][SR]::SetProcessDPIAware()  # physical pixels throughout (1920 x 1080): captures show the whole screen
function Stamp { (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
function NowUtc { (Get-Date).ToUniversalTime() }
function L($s) { Add-Content -Encoding utf8 $log $s; $s }
function Tool([int]$x, [int]$y) { "($([Math]::Round($x * 1456 / 1920)),$([Math]::Round($y * 819 / 1080)))" }
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
    "probe $s to $e : " + (($o | % { "$_" }) -join ' | ')
}
function Fg { $f = [SR]::GetForegroundWindow(); [uint32]$fp = 0; $null = [SR]::GetWindowThreadProcessId($f, [ref]$fp); "fg 0x$($f.ToInt64().ToString('x')) pid $fp '$([SR]::Txt($f))'" }
function FgIsTally { [SR]::GetForegroundWindow() -eq $tally }
function WaitUntil([double]$at) { while (((NowUtc) - $t0).TotalSeconds -lt $at) { Start-Sleep -Milliseconds 20 } }
$script:box = [IntPtr]::Zero; $script:no = [IntPtr]::Zero
function FindBox {
    if ($script:box -ne [IntPtr]::Zero) { return }
    $b = [SR]::ByTitle($title); if ($b -eq [IntPtr]::Zero) { return }
    $script:box = $b; $br = [SR]::Rect($b); $kids = @()
    foreach ($k in [SR]::Kids($b)) { $kc = [SR]::Cls($k); $kt = [SR]::Txt($k); $kr = [SR]::Rect($k); $kids += "CHILD`t$kc`t$($kr.L),$($kr.T),$($kr.R),$($kr.B)`t$kt"; if ($kc -eq 'Button' -and $kt -eq '&No') { $script:no = $k } }
    [IO.File]::WriteAllText("$ev\$c-box-text.txt", ((@("TITLE`t$([SR]::Txt($b))", "RECT`t$($br.L),$($br.T),$($br.R),$($br.B)", "EXSTYLE`t0x$(([SR]::GetWindowLong($b, -20)).ToString('x8'))") + $kids) -join "`n") + "`n", (New-Object Text.UTF8Encoding $false))
    L "$Run box $b found $(Stamp) at T0+$([Math]::Round(((NowUtc) - $t0).TotalSeconds, 2)) s; rect physical $($br.L),$($br.T),$($br.R),$($br.B); exstyle 0x$(([SR]::GetWindowLong($b, -20)).ToString('x8')); No button $($script:no)"
}
function AnswerNo([string]$why) {
    if (-not [SR]::IsWindow($script:box)) { L "$Run $why`: the box is gone; nothing clicked"; return }
    $bp = [SR]::Pos($script:box); $tp = [SR]::Pos($tally); $min = $false
    if (-not ($bp -gt 0 -and $bp -lt $tp)) {
        [void][SR]::ShowWindow($tally, 6); Start-Sleep -Milliseconds 600; $min = $true
        L "$Run $why`: the box was below Tally (z box $bp, Tally $tp): Tally minimised by ShowWindow(SW_MINIMIZE) $(Stamp)"
    }
    $nr = [SR]::Rect($script:no); $x = [int](($nr.L + $nr.R) / 2); $y = [int](($nr.T + $nr.B) / 2)
    [SR]::ClickAt($x, $y); L "$Run $why`: No clicked $(Stamp) at T0+$([Math]::Round(((NowUtc) - $t0).TotalSeconds, 2)) s, physical ($x,$y) = screen tool $(Tool $x $y)"
    if ($min) { Start-Sleep -Milliseconds 800; [void][SR]::ShowWindow($tally, 9); L "$Run Tally restored by ShowWindow(SW_RESTORE) $(Stamp)" }
}
$tally = [SR]::ByTitle('TallyPrime:9001'); $tx = 412; $ty = 18
L "$Run runner pid $PID; Tally window $tally; Full=$Full; batch $BatchId"
$trArgs = "-NoProfile -ExecutionPolicy Bypass -File $rd\requests\box-trace-early.ps1 -Title `"$title`" -Out $ev\trace-$c.txt -Seconds 150"
$tr = Start-Process powershell -ArgumentList $trArgs -PassThru; $null = $tr.Handle
L "$Run trace pre-armed $(Stamp) pid $($tr.Id): powershell $trArgs"
Start-Sleep -Seconds 3
[SR]::ClickAt($tx, $ty); $tclick = Stamp; L "$Run Tally title clicked ONCE $tclick; $(Fg)"
Start-Sleep -Milliseconds $WaitBeforeT0Ms
$seq = '{0:D3}' -f @(Get-ChildItem "$rd\inbox" -Filter *.json).Count
$fn = "$seq-$c-call.json"
if ($Tool -eq 'post_import') { $callArgs = @{ company_guid = $guid; batch_id = $BatchId } } else { $callArgs = @{ batch_id = $BatchId } }
[IO.File]::WriteAllText("$rd\inbox\$fn", (@{ tool = $Tool; arguments = $callArgs } | ConvertTo-Json -Compress), (New-Object Text.UTF8Encoding $false))
$t0 = NowUtc; L "$Run-T0 $(Stamp): $Tool queued as inbox\$fn ($(($callArgs.Keys | Sort-Object) -join ' and ')); waited $WaitBeforeT0Ms ms after the title click"
foreach ($at in 3, 8, 13) { WaitUntil $at; L "$Run-T0+$at $(Probe "probe-$c-t0p$at")"; FindBox }
Shot "$c-t0p13-screen"; L "$Run-T0+13 shot $(Stamp); $(Fg)"
while (((NowUtc) - $t0).TotalSeconds -lt 15.5 -and $script:box -eq [IntPtr]::Zero) { Start-Sleep -Milliseconds 100; FindBox }
if ($script:box -eq [IntPtr]::Zero) { L "$Run no box by T0+15.5 $(Stamp): nothing clicked"; exit 21 }
$br = [SR]::Rect($script:box); $Px = 329; $Py = 594
$dx = [Math]::Max(0, [Math]::Max($br.L - $Px, $Px - $br.R)); $dy = [Math]::Max(0, [Math]::Max($br.T - $Py, $Py - $br.B)); $dist = [Math]::Sqrt($dx * $dx + $dy * $dy)
L "$Run click point physical ($Px,$Py) = screen tool $(Tool $Px $Py); distance from the box $([Math]::Round($dist, 1)) px"
if ($dist -lt 150) { L "$Run the point is too near the box: no click into Tally"; AnswerNo 'no click possible'; exit 22 }
WaitUntil 16
[SR]::ClickAt($Px, $Py); $tc = NowUtc
L "$Run click into Tally $(Stamp) at T0+$([Math]::Round(($tc - $t0).TotalSeconds, 2)) s; $(Fg)"
L "$Run C+0 $(Probe "probe-$c-c0")"; Shot "$c-c0-screen"; L "$Run C+0 shot $(Stamp)"
while (((NowUtc) - $tc).TotalSeconds -lt 6) { Start-Sleep -Milliseconds 20 }
L "$Run C+6 $(Probe "probe-$c-c6")"; L "$Run C+6 $(Fg)"
if ($Full) {
    # S7: one Down key, only with the box open and Tally holding the foreground.
    Shot "$c-s7-before"
    $open = [SR]::IsWindow($script:box); $tf = FgIsTally
    L "$Run S7 check $(Stamp): box open $open; Tally foreground $tf; capture s7-before"
    if ($open -and $tf) { [SR]::Down(); L "$Run S7 Down pressed ONCE $(Stamp) at T0+$([Math]::Round(((NowUtc) - $t0).TotalSeconds, 2)) s"; Start-Sleep -Milliseconds 700; Shot "$c-s7-after"; L "$Run S7 capture after $(Stamp); $(Fg)" }
    elseif ($open) { L "$Run S7 skipped: the box held the foreground ($(Fg))" } else { L "$Run S7 skipped: the box is gone" }
    WaitUntil 26
    & powershell -NoProfile -File "$rd\taskbar-shot.ps1" -Name "$c-s8-taskbar" | Out-Null
    L "$Run S8 READY-FOR-COMMANDS $(Stamp) T0+$([Math]::Round(((NowUtc) - $t0).TotalSeconds, 1)) s; taskbar capture shots\$c-s8-taskbar.png; box a window $([SR]::IsWindow($script:box))"
    $n = 1
    while ([SR]::IsWindow($script:box)) {
        if (((NowUtc) - $t0).TotalSeconds -gt 36) { AnswerNo 'T0+36 fallback (no command answered the box)'; break }
        $cf = "$rd\cmd\$c-$n.txt"
        if (-not (Test-Path $cf)) { Start-Sleep -Milliseconds 100; continue }
        $w = (Get-Content $cf -Raw).Trim() -split '\s+'; $n++
        if ($w[0] -eq 'taskbar' -or $w[0] -eq 'thumb') {
            $bx = [int]$w[1]; $by = [int]$w[2]; [SR]::ClickAt($bx, $by)
            L "$Run S8 $($w[0]) clicked ONCE $(Stamp) at T0+$([Math]::Round(((NowUtc) - $t0).TotalSeconds, 2)) s, physical ($bx,$by) = screen tool $(Tool $bx $by)"
            L "$Run S8 probe $(Probe "probe-$c-s8-$($w[0])"); box a window $([SR]::IsWindow($script:box)); $(Fg)"
        } elseif ($w[0] -eq 'no') { AnswerNo 'command no'; break }
        elseif ($w[0] -eq 'none') { L "$Run S8 $(Stamp): no box taskbar button to click (recorded); No next"; AnswerNo 'after S8'; break }
    }
} else {
    WaitUntil 30; AnswerNo 'T0+30'
}
$e0 = Get-Date
while (((Get-Date) - $e0).TotalSeconds -lt 60 -and -not (Test-Path "$rd\answers\$fn")) { Start-Sleep -Milliseconds 200 }
if (Test-Path "$rd\answers\$fn") {
    Start-Sleep -Milliseconds 300; $a = Get-Content "$rd\answers\$fn" -Raw
    L "$Run answer $fn at $(Stamp): $($a.Substring(0, [Math]::Min(1500, $a.Length)))"
} else { L "$Run no answer within 60 s after the box" }
L "$Run DONE $(Stamp)"
exit 0
