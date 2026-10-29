# Lab helper (#1780-F cells F0 to F4, G1 to G3 and part T, not product code). One cell of the sheet's recipe:
# pre-arms the trace (box-trace-early.ps1, its own window) and the reader (ctx-reader.ps1 -SpawnFile, its own window), clicks
# Tally's title, starts the parent of the cell's kind, clicks Tally's title at once; T0 is the stamp of the SPAWN ... child_pid= line
# of <cell>.txt; probe pairs (zorder-probe.ps1 then zorder-probe-k.ps1) at T0+8 and T0+11 and a whole-screen capture; at T0+14 ONE
# click on a blank point of Tally at least 150 logical px from the box; probe pairs and captures at once and 6 s later; then No by
# mouse if the box is above Tally, else it waits for cmd\<cell>-taskbar.txt (the taskbar button's logical "x y") and clicks it once
# before No. It never clicks Yes and sends no key. Parent kinds (the lab's deviations, recorded):
#   interactive  a .cmd file opened through explorer.exe (a visible console whose parent is explorer; nothing typed)
#   hidden       Start-Process powershell -WindowStyle Hidden, from this runner
#   launcher     python -I launcher.py (the lab launcher with its command replaced), started with -NoNewWindow from this runner
param([Parameter(Mandatory = $true)][string]$Cell, [Parameter(Mandatory = $true)][ValidateSet('child', 'control')][string]$Target,
      [Parameter(Mandatory = $true)][ValidateSet('tokio', 'files')][string]$Stdio,
      [Parameter(Mandatory = $true)][ValidateSet('interactive', 'hidden', 'launcher')][string]$Kind, [int]$Px = 263, [int]$Py = 475)
$rd = '<lab folder>'; $ev = "$rd\evidence"; $c = $Cell.ToLower(); $log = "$ev\$c-timeline.txt"
$title = if ($Target -eq 'child') { 'ComplyEaze Bridge' } else { 'Lab topmost control' }
Add-Type -AssemblyName System.Windows.Forms, System.Drawing
Add-Type @'
using System; using System.Text; using System.Collections.Generic; using System.Runtime.InteropServices;
public class CR { public delegate bool P(IntPtr h, IntPtr l);
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
    $s2 = Stamp; $o2 = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$rd\requests\zorder-probe-k.ps1" 2>&1; $e2 = Stamp
    $o2 | Set-Content -Encoding utf8 "$ev\$name-k.txt"
    "sheet probe $s to $e : " + (($o | % { "$_" }) -join ' | ') + " ;; K probe $s2 to $e2 : " + (($o2 | % { "$_" }) -join ' | ')
}
$tally = [CR]::ByTitle('TallyPrime:9001'); $tx = 330; $ty = 14
L "$Cell target=$Target stdio=$Stdio kind=$Kind; Tally window $tally; runner pid $PID"
# The cell's command (the sheet's), as one argument string for powershell -File.
$spArgs = "-Target $Target -Stdio $Stdio -Out $ev\$c.txt -TimeoutSeconds 60"
if ($Target -eq 'child') { $spArgs += " -ChildExe <build folder>\mcpb\bin\x86_64-pc-windows-msvc\bridge_mcp.exe -InputFile $rd\requests\confirm-input.txt" }
else { $spArgs += " -ControlScript $rd\requests\control-box-product.ps1" }
# 1. Pre-arm.
$trArgs = "-NoProfile -ExecutionPolicy Bypass -File $rd\requests\box-trace-early.ps1 -Title `"$title`" -Out $ev\trace-$c.txt -Seconds 100"
$tr = Start-Process powershell -ArgumentList $trArgs -PassThru; $null = $tr.Handle
L "$Cell trace pre-armed $(Stamp) pid $($tr.Id): powershell $trArgs"
$rdArgs = "-NoProfile -ExecutionPolicy Bypass -File $rd\requests\ctx-reader.ps1 -SpawnFile $ev\$c.txt -DelaySeconds 6 -Out $ev\ctx-$c.txt"
$rp = Start-Process powershell -ArgumentList $rdArgs -PassThru; $null = $rp.Handle
L "$Cell reader pre-armed $(Stamp) pid $($rp.Id): powershell $rdArgs"
Start-Sleep -Seconds 2
[CR]::ClickAt($tx, $ty); L "$Cell Tally title clicked $(Stamp)"
Start-Sleep -Milliseconds 700
# 2. The parent.
$cmdLine = "powershell -NoProfile -ExecutionPolicy Bypass -File $rd\requests\spawn-parent.ps1 $spArgs"
if ($Kind -eq 'interactive') {
    $cmdFile = "$rd\cmd\$c.cmd"
    [IO.File]::WriteAllText($cmdFile, "@$cmdLine`r`n", (New-Object Text.ASCIIEncoding))
    $pp = Start-Process explorer.exe -ArgumentList "`"$cmdFile`"" -PassThru
    L "$Cell parent started $(Stamp): explorer.exe `"$cmdFile`" (explorer pid $($pp.Id)); the .cmd holds: $cmdLine"
} elseif ($Kind -eq 'hidden') {
    $pp = Start-Process powershell -WindowStyle Hidden -ArgumentList "-NoProfile -ExecutionPolicy Bypass -File $rd\requests\spawn-parent.ps1 $spArgs" -PassThru
    L "$Cell parent started $(Stamp): Start-Process powershell -WindowStyle Hidden -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File $rd\requests\spawn-parent.ps1 $spArgs' (pid $($pp.Id), parent this runner $PID)"
} else {
    $lArgs = "-I $rd\launcher.py $c " + $spArgs
    $pp = Start-Process python -ArgumentList $lArgs -NoNewWindow -PassThru
    L "$Cell parent started $(Stamp): python $lArgs (launcher pid $($pp.Id), parent this runner $PID)"
}
[CR]::ClickAt($tx, $ty); L "$Cell Tally title clicked $(Stamp)"
# T0: the stamp on the SPAWN ... child_pid= line.
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
function NowUtc { (Get-Date).ToUniversalTime() }
foreach ($at in 8, 11) {
    while (((NowUtc) - $t0).TotalSeconds -lt $at) { Start-Sleep -Milliseconds 20 }
    L "$Cell-T0+$at $(Probe "probe-$c-t0p$at")"
}
Shot "$c-t0p11-screen"; L "$Cell-T0+11 shot $(Stamp)"
while (((NowUtc) - $t0).TotalSeconds -lt 14) { Start-Sleep -Milliseconds 20 }
$box = [CR]::ByTitle($title)
if ($box -eq [IntPtr]::Zero) {
    while (((NowUtc) - $t0).TotalSeconds -lt 15) { Start-Sleep -Milliseconds 20 }
    $box = [CR]::ByTitle($title)
    if ($box -eq [IntPtr]::Zero) { L "$Cell-T0+15 $(Stamp): the box did not appear; nothing ended"; exit 21 }
}
$br = [CR]::Rect($box); $no = [IntPtr]::Zero
foreach ($k in [CR]::Kids($box)) { if ([CR]::Cls($k) -eq 'Button' -and [CR]::Txt($k) -eq '&No') { $no = $k } }
L "$Cell box $box rect $($br.L),$($br.T),$($br.R),$($br.B)"
$dx = [Math]::Max(0, [Math]::Max($br.L - $Px, $Px - $br.R)); $dy = [Math]::Max(0, [Math]::Max($br.T - $Py, $Py - $br.B)); $dist = [Math]::Sqrt($dx * $dx + $dy * $dy)
L "$Cell point logical ($Px,$Py) = screen tool ($([Math]::Round($Px * 1456 / 1536)),$([Math]::Round($Py * 819 / 864))); distance from the box $([Math]::Round($dist,1)) px"
if ($dist -lt 150) { L "$Cell point too near the box: no click"; exit 22 }
[CR]::ClickAt($Px, $Py); $tc = NowUtc
L "$Cell click into Tally $(Stamp) at T0+$([Math]::Round(($tc - $t0).TotalSeconds, 2)) s"
L "$Cell C+0 $(Probe "probe-$c-c0")"; Shot "$c-c0-screen"; L "$Cell C+0 shot $(Stamp)"
while (((NowUtc) - $tc).TotalSeconds -lt 6) { Start-Sleep -Milliseconds 20 }
L "$Cell C+6 $(Probe "probe-$c-c6")"; Shot "$c-c6-screen"; L "$Cell C+6 shot $(Stamp)"
if (-not [CR]::IsWindow($box)) { L "$Cell the box is gone after the click"; exit 23 }
$bp = [CR]::Pos($box); $tp = [CR]::Pos($tally); L "$Cell z positions: box $bp, Tally $tp"
if (-not ($bp -gt 0 -and $bp -lt $tp)) {
    $cmd = "$rd\cmd\$c-taskbar.txt"; L "$Cell the box is below Tally; waiting for $cmd $(Stamp)"
    $w0 = Get-Date
    while (-not (Test-Path $cmd) -and ((Get-Date) - $w0).TotalSeconds -lt 30) { Start-Sleep -Milliseconds 200 }
    if (-not (Test-Path $cmd)) { L "$Cell no taskbar position given in 30 s: nothing clicked"; exit 24 }
    $xy = (Get-Content $cmd -Raw).Trim() -split '\s+'; [CR]::ClickAt([int]$xy[0], [int]$xy[1])
    L "$Cell taskbar button clicked ONCE $(Stamp) at logical ($($xy[0]),$($xy[1])); box a window: $([CR]::IsWindow($box))"
    Start-Sleep -Milliseconds 700
    if (-not [CR]::IsWindow($box)) { Shot "$c-gone-screen"; L "$Cell the box is gone after the taskbar click: nothing more clicked"; exit 25 }
    $bp = [CR]::Pos($box); $tp = [CR]::Pos($tally); L "$Cell z after the taskbar click: box $bp, Tally $tp"
}
if ($no -ne [IntPtr]::Zero -and $bp -gt 0 -and $bp -lt $tp) {
    $nr = [CR]::Rect($no); $x = [int](($nr.L + $nr.R) / 2); $y = [int](($nr.T + $nr.B) / 2)
    [CR]::ClickAt($x, $y); L "$Cell No clicked $(Stamp) at T0+$([Math]::Round(((NowUtc) - $t0).TotalSeconds, 2)) s, logical ($x,$y)"
} else { L "$Cell No not clicked (box $bp, Tally $tp)"; exit 26 }
$e0 = Get-Date
while (((Get-Date) - $e0).TotalSeconds -lt 15) { if ((Get-Content "$ev\$c.txt" -ErrorAction SilentlyContinue) -match 'exit_code=') { break }; Start-Sleep -Milliseconds 200 }
L "$Cell spawn file: $((Get-Content "$ev\$c.txt" -ErrorAction SilentlyContinue) -join ' || ')"
if (Test-Path "$ev\$c.control.txt") { L "$Cell control file: $((Get-Content "$ev\$c.control.txt") -join ' ')" }
if (Test-Path "$ev\$c.stderr") { L "$Cell stderr: $((Get-Content "$ev\$c.stderr" -ErrorAction SilentlyContinue) -join ' || ')" }
exit 0
