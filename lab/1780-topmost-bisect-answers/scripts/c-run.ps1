# Lab helper (#1780-E part C, not product code; row 1780-D's c-run.ps1 with these edits: the folder; Tally found by title;
# the trace for 150 s; after C3 nothing is clicked: it stamps P-T when the post_import answer is saved, probes at P-T+30, +50
# and +70 with a capture after the last, and records the box's end if it goes. If the box is still open at P-T+70 it clicks
# the taskbar button ONCE at the logical "x y" in cmd\c-taskbar.txt, probes, then No by mouse if the box is above Tally).
param([Parameter(Mandatory = $true)][string]$BatchId, [int]$Px = 263, [int]$Py = 475)
$rd = '<lab folder>'; $ev = "$rd\evidence"; $log = "$ev\c-timeline.txt"; $flag = '--confirm-' + 'journal'
Add-Type -AssemblyName System.Windows.Forms, System.Drawing
Add-Type @'
using System; using System.Text; using System.Collections.Generic; using System.Runtime.InteropServices;
public class CW { public delegate bool P(IntPtr h, IntPtr l);
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
 public static IntPtr ByTitle(string t) { IntPtr r = IntPtr.Zero; EnumWindows((h,l)=>{ if (IsWindowVisible(h) && Txt(h).StartsWith(t)) { r = h; return false; } return true; }, IntPtr.Zero); return r; }
 public static int Count(string part) { int n = 0; EnumWindows((h,l)=>{ if (IsWindowVisible(h) && Txt(h).IndexOf(part, StringComparison.OrdinalIgnoreCase) >= 0) n++; return true; }, IntPtr.Zero); return n; }
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
$script:last = @()
function Probe([string]$name) {
    $s = Stamp; $o = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$rd\requests\zorder-probe.ps1" 2>&1; $e = Stamp
    $o | Set-Content -Encoding utf8 "$ev\$name.txt"; $script:last = @($o | ForEach-Object { "$_" })
    "start $s end $e : " + ($script:last -join ' | ')
}
$tally = [CW]::ByTitle('TallyPrime:9001'); $tx = 330; $ty = 14
L "C1 Tally window $tally (found by title)"
$pre = [CW]::Count('ComplyEaze Bridge')
L "C1 $(Stamp) visible windows titled with 'ComplyEaze Bridge' before the trace: $pre"
if ($pre -ne 0) { L "C1 a window already carries the box's title: nothing queued; stop"; exit 20 }
$trArgs = "-NoProfile -ExecutionPolicy Bypass -File $rd\requests\box-trace.ps1 -Title `"ComplyEaze Bridge`" -Out $ev\trace-c.txt -Seconds 150"
$tr = Start-Process powershell -ArgumentList $trArgs -PassThru; $null = $tr.Handle
L "C1 trace started $(Stamp) pid $($tr.Id) in its own window: powershell $trArgs"
Start-Sleep -Seconds 2
[CW]::ClickAt($tx, $ty); L "C1 Tally title clicked $(Stamp) at logical ($tx,$ty)"
Start-Sleep -Milliseconds 700
$seqNo = '{0:D3}' -f (@(Get-ChildItem "$rd\inbox" -Filter *.json).Count)
$fn = "$seqNo-c1-post_import.json"
$req = '{"tool":"post_import","arguments":{"company_guid":"a30f23c3-4819-4334-96ab-ffa6fba70976","batch_id":"' + $BatchId + '"}}'
$null = $req | ConvertFrom-Json
$t0 = Get-Date
[IO.File]::WriteAllText("$rd\inbox\$fn", $req, (New-Object Text.UTF8Encoding $false))
L "T0 $(Stamp) queued $fn"
foreach ($at in 8, 13) {
    while (((Get-Date) - $t0).TotalSeconds -lt $at) { Start-Sleep -Milliseconds 20 }
    L "T0+$at probe $(Probe "probe-c-t0p$at")"
}
Shot 'c1-t0p13-screen'; L "T0+13 shot $(Stamp)"
$box = [CW]::ByTitle('ComplyEaze Bridge')
if ($box -eq [IntPtr]::Zero) { L "C2 $(Stamp) no box open at T0+13: no click; stop"; exit 21 }
$first = @(Get-Content "$ev\trace-c.txt" | Where-Object { $_ -like 'FIRST *' -or $_ -like 'PROC *' })
L "C1 trace: $($first -join ' || ')"
$cmdline = ($first | Where-Object { $_ -like 'PROC command_line=*' } | Select-Object -First 1)
$br = [CW]::Rect($box)
$txt = @("TITLE`t$([CW]::Txt($box))", "RECT`t$($br.L),$($br.T),$($br.R),$($br.B)")
$no = [IntPtr]::Zero
foreach ($k in [CW]::Kids($box)) { $c = [CW]::Cls($k); $t = [CW]::Txt($k); $r = [CW]::Rect($k); $txt += "CHILD`t$c`t$($r.L),$($r.T),$($r.R),$($r.B)`t$t"; if ($c -eq 'Button' -and $t -eq '&No') { $no = $k } }
[IO.File]::WriteAllText("$ev\c2-box-text.txt", (($txt -join "`n") -replace "`r`n", "`n") + "`n")
L "C2 $(Stamp) box $box rect $($br.L),$($br.T),$($br.R),$($br.B); text saved"
function AnswerNo([string]$why) {
    if ($no -eq [IntPtr]::Zero) { L "$why : no No button found; nothing clicked"; exit 22 }
    $bp = [CW]::Pos($box); $tp = [CW]::Pos($tally)
    if ($bp -gt 0 -and $bp -lt $tp) {
        $nr = [CW]::Rect($no); $x = [int](($nr.L + $nr.R) / 2); $y = [int](($nr.T + $nr.B) / 2)
        [CW]::ClickAt($x, $y); L "$why : No clicked $(Stamp) at T0+$([Math]::Round(((Get-Date) - $t0).TotalSeconds, 2)) s, logical ($x,$y)"
        Start-Sleep -Milliseconds 800; L "box still open: $([CW]::IsWindow($box))"
    } else { L "$why : box not above Tally (box $bp, Tally $tp): nothing clicked"; exit 23 }
}
if (-not $cmdline -or -not $cmdline.Contains($flag) -or -not [CW]::Txt($box).StartsWith('ComplyEaze Bridge')) { AnswerNo 'C1 STOP: the trace names another process or title'; exit 24 }
$dx = [Math]::Max(0, [Math]::Max($br.L - $Px, $Px - $br.R)); $dy = [Math]::Max(0, [Math]::Max($br.T - $Py, $Py - $br.B)); $dist = [Math]::Sqrt($dx * $dx + $dy * $dy)
L "C3 point logical ($Px,$Py) = screen tool ($([Math]::Round($Px * 1456 / 1536)),$([Math]::Round($Py * 819 / 864))); distance from the box $([Math]::Round($dist,1)) px"
if ($dist -lt 150) { AnswerNo 'C3 point too near the box, no click'; exit 25 }
while (((Get-Date) - $t0).TotalSeconds -lt 16) { Start-Sleep -Milliseconds 20 }
[CW]::ClickAt($Px, $Py); $tc = Get-Date
L "C3 click $(Stamp) at T0+$([Math]::Round(($tc - $t0).TotalSeconds, 2)) s"
L "C+0 probe $(Probe 'probe-c-c0')"; Shot 'c3-c0-screen'; L "C+0 shot $(Stamp)"
while (((Get-Date) - $tc).TotalSeconds -lt 6) { Start-Sleep -Milliseconds 20 }
L "C+6 probe $(Probe 'probe-c-c6')"; Shot 'c3-c6-screen'; L "C+6 shot $(Stamp)"
if (-not [CW]::IsWindow($box)) { L "C3 the box is gone after the click: stop (no further post_import)"; exit 26 }
L "C3 z box $([CW]::Pos($box)), Tally $([CW]::Pos($tally))"
# C4: nothing is clicked. P-T is when the post_import answer is saved.
$ans = "$rd\answers\$fn"
while (-not (Test-Path $ans)) { Start-Sleep -Milliseconds 50 }
$pt = Get-Date
Start-Sleep -Milliseconds 300
$a = Get-Content $ans -Raw | ConvertFrom-Json
L "C4 P-T $(Stamp) answer saved ($fn, round trip $($a.round_trip_seconds) s; T0+$([Math]::Round(($pt - $t0).TotalSeconds, 2)) s): $($a.response.result.content[0].text)"
$gone = $null
foreach ($at in 30, 50, 70) {
    while (((Get-Date) - $pt).TotalSeconds -lt $at) {
        if (-not $gone -and -not [CW]::IsWindow($box)) { $gone = Get-Date; L "C4 $(Stamp) the box is no longer a window (P-T+$([Math]::Round(($gone - $pt).TotalSeconds, 2)) s)" }
        Start-Sleep -Milliseconds 50
    }
    if ($gone) { break }
    L "P-T+$at probe $(Probe "probe-c-pt$at"); box a window $([CW]::IsWindow($box)); z box $([CW]::Pos($box)), Tally $([CW]::Pos($tally))"
}
if ($gone) { L "C4 gone probe $(Probe 'probe-c-gone')"; Shot 'c4-gone-screen'; L "C4 shot $(Stamp); nothing clicked"; exit 30 }
Shot 'c4-pt70-screen'; L "P-T+70 shot $(Stamp)"
if (-not [CW]::IsWindow($box)) { L "C4 the box went during the P-T+70 probe; nothing clicked"; exit 31 }
$cmd = "$rd\cmd\c-taskbar.txt"
if (-not (Test-Path $cmd)) { L "C4 no taskbar position given: nothing clicked"; exit 32 }
$xy = (Get-Content $cmd -Raw).Trim() -split '\s+'; $bx = [int]$xy[0]; $by = [int]$xy[1]
[CW]::ClickAt($bx, $by); L "C4 taskbar button clicked ONCE $(Stamp) at logical ($bx,$by) = screen tool ($([Math]::Round($bx*1456/1536)),$([Math]::Round($by*819/864)))"
L "C4 probe $(Probe 'probe-c-taskbar'); box a window $([CW]::IsWindow($box))"
if (-not [CW]::IsWindow($box)) { Shot 'c4-tb-gone-screen'; L "C4 the box is gone after the taskbar click: nothing more clicked"; exit 33 }
AnswerNo 'C4'
exit 0
