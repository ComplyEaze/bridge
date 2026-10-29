# Lab helper (#1780-E part H, not product code): the dialog child run by hand, steps H0 to H6. No server, no request to
# Tally. Starts the sheet's trace in its own window, clicks Tally's title, starts bridge_mcp.exe --confirm-journal with stdin
# from confirm-input.txt and stdout/stderr to files (H-T0), clicks Tally's title at once, probes at H-T0+8 and +13, reads the
# box and stderr, ONE click into Tally at H-T0+16, probes at once and 6 s later. Then it obeys the lab's command files
# cmd\h1-<n>.txt: "taskbar x y" (one click on the box's taskbar button, probes at once and 1.5 s later), "no" (No by mouse,
# only if the box is above Tally), "end". It never clicks Yes and sends no key.
$rd = '<lab folder>'; $ev = "$rd\evidence"; $log = "$ev\h1-timeline.txt"
$exe = '<build folder>\mcpb\bin\x86_64-pc-windows-msvc\bridge_mcp.exe'
Add-Type -AssemblyName System.Windows.Forms, System.Drawing
Add-Type @'
using System; using System.Text; using System.Collections.Generic; using System.Runtime.InteropServices;
public class HW { public delegate bool P(IntPtr h, IntPtr l);
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
function ErrLines {
    $f = "$ev\h1-stderr.txt"
    if (-not (Test-Path $f)) { return @() }
    $fs = [IO.File]::Open($f, 'Open', 'Read', 'ReadWrite'); $sr = New-Object IO.StreamReader($fs)
    $t = $sr.ReadToEnd(); $sr.Close(); $fs.Close()
    @($t -split "`r?`n" | Where-Object { $_ -ne '' })
}
$tally = [HW]::ByTitle('TallyPrime:9001'); $tx = 330; $ty = 14; $Px = 263; $Py = 475
L "H0 Tally window $tally (found by title)"
[HW]::ClickAt($tx, $ty); L "H0 Tally title clicked $(Stamp)"
$flag = '--confirm-' + 'journal'
$pre = @(Get-CimInstance Win32_Process | Where-Object { $_.CommandLine -and $_.CommandLine.Contains($flag) })
L "H0 $(Stamp) processes with the confirm-journal flag: $($pre.Count)"
if ($pre.Count -ne 0) { L "H0 a child is already there: STOP"; exit 40 }
$trArgs = "-NoProfile -ExecutionPolicy Bypass -File $rd\requests\box-trace.ps1 -Title `"ComplyEaze Bridge`" -Out $ev\trace-h1.txt -Seconds 100"
$tr = Start-Process powershell -ArgumentList $trArgs -PassThru; $null = $tr.Handle
L "H1 trace started $(Stamp) pid $($tr.Id) in its own window: powershell $trArgs"
Start-Sleep -Seconds 2
[HW]::ClickAt($tx, $ty); L "H1 Tally title clicked $(Stamp)"
Start-Sleep -Milliseconds 700
$p = Start-Process -FilePath $exe -ArgumentList $flag -NoNewWindow -PassThru -RedirectStandardInput "$rd\requests\confirm-input.txt" -RedirectStandardOutput "$ev\h1-stdout.txt" -RedirectStandardError "$ev\h1-stderr.txt"; $null = $p.Handle
[HW]::ClickAt($tx, $ty); $t0 = Get-Date
L "H-T0 $(Stamp): child pid $($p.Id): `$p = Start-Process -FilePath '$exe' -ArgumentList '$flag' -NoNewWindow -PassThru -RedirectStandardInput '$rd\requests\confirm-input.txt' -RedirectStandardOutput '$ev\h1-stdout.txt' -RedirectStandardError '$ev\h1-stderr.txt'; `$null = `$p.Handle ; Tally title clicked at once"
foreach ($at in 8, 13) {
    while (((Get-Date) - $t0).TotalSeconds -lt $at) { Start-Sleep -Milliseconds 20 }
    L "H-T0+$at probe $(Probe "probe-h1-t0p$at")"
}
Shot 'h1-t0p13-screen'; L "H-T0+13 shot $(Stamp)"
L "H3 stderr at H-T0+13: $((ErrLines) -join ' || ')"
while (((Get-Date) - $t0).TotalSeconds -lt 13.5) { Start-Sleep -Milliseconds 20 }
$box = [HW]::ByTitle('ComplyEaze Bridge')
if ($box -eq [IntPtr]::Zero) {
    while (((Get-Date) - $t0).TotalSeconds -lt 15) { Start-Sleep -Milliseconds 20 }
    $box = [HW]::ByTitle('ComplyEaze Bridge')
}
if ($box -eq [IntPtr]::Zero) { L "H3 $(Stamp): no box by H-T0+15; exited $($p.HasExited); end nothing; STOP"; exit 41 }
$br = [HW]::Rect($box); $no = [IntPtr]::Zero; $kids = @(); $static = ''
foreach ($k in [HW]::Kids($box)) { $c = [HW]::Cls($k); $t = [HW]::Txt($k); $r = [HW]::Rect($k); $kids += "CHILD`t$c`t$($r.L),$($r.T),$($r.R),$($r.B)`t$t"; if ($c -eq 'Button' -and $t -eq '&No') { $no = $k }; if ($c -eq 'Static' -and $t.Length -gt $static.Length) { $static = $t } }
$txt = @("TITLE`t$([HW]::Txt($box))", "RECT`t$($br.L),$($br.T),$($br.R),$($br.B)") + $kids
[IO.File]::WriteAllText("$ev\h1-box-text.txt", (($txt -join "`n") -replace "`r`n", "`n") + "`n")
$boxLines = @($static -split "`r?`n").Count
$inLines = @(Get-Content "$rd\requests\confirm-input.txt").Count - 2
L "H3 $(Stamp) box $box rect $($br.L),$($br.T),$($br.R),$($br.B); text lines $boxLines; confirm-input text lines $inLines"
if ($boxLines -ne $inLines) { L "H3 the box's line count differs: end nothing; STOP"; exit 42 }
$dx = [Math]::Max(0, [Math]::Max($br.L - $Px, $Px - $br.R)); $dy = [Math]::Max(0, [Math]::Max($br.T - $Py, $Py - $br.B)); $dist = [Math]::Sqrt($dx * $dx + $dy * $dy)
L "H4 point logical ($Px,$Py) = screen tool ($([Math]::Round($Px * 1456 / 1536)),$([Math]::Round($Py * 819 / 864))); distance from the box $([Math]::Round($dist,1)) px"
if ($dist -lt 150) { L "H4 point too near the box: no click; waiting for commands" } else {
    while (((Get-Date) - $t0).TotalSeconds -lt 16) { Start-Sleep -Milliseconds 20 }
    [HW]::ClickAt($Px, $Py); $tc = Get-Date
    L "H4 click into Tally $(Stamp) at H-T0+$([Math]::Round(($tc - $t0).TotalSeconds, 2)) s"
    L "H4 C+0 probe $(Probe 'probe-h1-c0')"; Shot 'h1-c0-screen'; L "H4 C+0 shot $(Stamp)"
    while (((Get-Date) - $tc).TotalSeconds -lt 6) { Start-Sleep -Milliseconds 20 }
    L "H4 C+6 probe $(Probe 'probe-h1-c6')"; Shot 'h1-c6-screen'; L "H4 C+6 shot $(Stamp)"
    L "H4 stderr lines at C+6: $(@(ErrLines).Count): $((ErrLines) -join ' || ')"
}
L "H $(Stamp) box a window $([HW]::IsWindow($box)); z box $([HW]::Pos($box)), Tally $([HW]::Pos($tally)); child exited $($p.HasExited)"
$n = 1
while (-not $p.HasExited) {
    $cmd = "$rd\cmd\h1-$n.txt"
    if (-not (Test-Path $cmd)) { Start-Sleep -Milliseconds 200; continue }
    $w = (Get-Content $cmd -Raw).Trim() -split '\s+'; $n++
    if ($w[0] -eq 'taskbar') {
        $bx = [int]$w[1]; $by = [int]$w[2]; [HW]::ClickAt($bx, $by); $tt = Get-Date
        L "H5 taskbar button clicked ONCE $(Stamp) at logical ($bx,$by) = screen tool ($([Math]::Round($bx*1456/1536)),$([Math]::Round($by*819/864)))"
        L "H5 +0 probe $(Probe 'probe-h1-tb0'); box a window $([HW]::IsWindow($box)); exited $($p.HasExited)"
        while (((Get-Date) - $tt).TotalSeconds -lt 1.5) { Start-Sleep -Milliseconds 20 }
        L "H5 +1.5 probe $(Probe 'probe-h1-tb15'); box a window $([HW]::IsWindow($box)); exited $($p.HasExited)"
        L "H5 stderr: $((ErrLines) -join ' || ')"
        if (-not [HW]::IsWindow($box)) { Shot 'h1-gone-screen'; L "H5 the box is gone after the taskbar click: nothing more clicked" }
        else { L "H5 z box $([HW]::Pos($box)), Tally $([HW]::Pos($tally))" }
    } elseif ($w[0] -eq 'no') {
        $bp = [HW]::Pos($box); $tp = [HW]::Pos($tally)
        if ([HW]::IsWindow($box) -and $bp -gt 0 -and $bp -lt $tp -and $no -ne [IntPtr]::Zero) {
            $nr = [HW]::Rect($no); $x = [int](($nr.L + $nr.R) / 2); $y = [int](($nr.T + $nr.B) / 2)
            [HW]::ClickAt($x, $y); L "H6 No clicked $(Stamp), logical ($x,$y)"
        } else { L "H6 No not clicked: box a window $([HW]::IsWindow($box)), z box $bp, Tally $tp" }
    } elseif ($w[0] -eq 'end') { L "H end command $(Stamp) with the child still running"; break }
}
$null = $p.WaitForExit(5000)
L "H6 $(Stamp) child exited $($p.HasExited) exit code $($p.ExitCode); stdout $((Get-Item "$ev\h1-stdout.txt").Length) B"
L "H6 stderr whole: $((ErrLines) -join ' || ')"
exit 0
