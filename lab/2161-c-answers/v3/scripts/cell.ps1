# Lab helper (#2161-C, not product code; row 2161-B's cell.ps1 rebuilt to this sheet). One cell: x0, b3, n, c1 or c2.
# Per-monitor DPI aware: every point and rectangle it uses is in physical pixels (the screen is 1920x1080 at 125 %, so a logical
# point is the physical one divided by 1.25; both are recorded for every click). Every clicked point has its own variable:
# $tallyTitlePoint, $noButtonPoint. The keys of step HA (Alt, Tab, Esc) are pressed by keybd_event, each stamped, only after the
# gate probe shows Tally in front; it never presses any other key, never a second Tab, and never releases Alt while the switcher
# is open. It never clicks Yes, clicks nothing on the taskbar or in the switcher, ends no process, and minimises only a recorded
# console before T0 (the sheet's rule) or Tally (a box under it). The read set X: box-rect-probe, taskbar-uia, taskbar-cover,
# taskbar-capture. The gate probe G-P: zorder-probe-k. In cell n only, the hold of step HA waits for the lab's control screenshot
# (cmd\n-ha-shot-done, at most 12 s; at least 3 s).
param([Parameter(Mandatory = $true)][ValidateSet('x0', 'b3', 'n', 'c1', 'c2')][string]$Cell)
$labDir = '<LAB_DIR>'; $evDir = "$labDir\evidence"; $shotDir = "$labDir\shots"; $reqDir = "$labDir\requests"; $wordDir = "$labDir\cmd"
$timeline = "$evDir\$Cell-timeline.txt"
$boxExe = '<LAB_DIR>\build\bin\x86_64-pc-windows-msvc\bridge_mcp.exe'
$boxTitle = 'ComplyEaze Bridge'
$scaleFactor = 1.25
$tallyTitlePoint = @(412, 17)
Add-Type @'
using System; using System.Text; using System.Collections.Generic; using System.Runtime.InteropServices;
public class RC { public delegate bool EP(IntPtr h, IntPtr l);
 [DllImport("user32.dll")] public static extern bool SetProcessDpiAwarenessContext(IntPtr v);
 [DllImport("user32.dll")] public static extern bool EnumWindows(EP f, IntPtr l);
 [DllImport("user32.dll")] public static extern bool EnumChildWindows(IntPtr w, EP f, IntPtr l);
 [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
 [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetClassName(IntPtr h, StringBuilder s, int n);
 [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
 [DllImport("user32.dll")] public static extern bool IsWindow(IntPtr h);
 [DllImport("user32.dll")] public static extern bool IsIconic(IntPtr h);
 [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
 [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
 [DllImport("dwmapi.dll")] public static extern int DwmGetWindowAttribute(IntPtr h, int a, out int v, int s);
 [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
 [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
 [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
 [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h, int c);
 [DllImport("user32.dll")] public static extern void mouse_event(uint f, uint x, uint y, uint d, UIntPtr e);
 [DllImport("user32.dll")] public static extern void keybd_event(byte vk, byte scan, uint f, UIntPtr e);
 public static bool Aware() { return SetProcessDpiAwarenessContext(new IntPtr(-4)); }
 public static string Txt(IntPtr h) { var s = new StringBuilder(1024); GetWindowText(h, s, 1024); return s.ToString().Replace("\r", " ").Replace("\n", " "); }
 public static string Cls(IntPtr h) { var s = new StringBuilder(256); GetClassName(h, s, 256); return s.ToString(); }
 public static uint PidOf(IntPtr h) { uint p; GetWindowThreadProcessId(h, out p); return p; }
 public static int Cloaked(IntPtr h) { int v; return DwmGetWindowAttribute(h, 14, out v, 4) == 0 ? v : -1; }
 public static IntPtr ByTitle(string t) { IntPtr r = IntPtr.Zero; EnumWindows((h,l)=>{ if (IsWindowVisible(h) && Txt(h).StartsWith(t)) { r = h; return false; } return true; }, IntPtr.Zero); return r; }
 public static IntPtr Switcher() { IntPtr r = IntPtr.Zero; EnumWindows((h,l)=>{ if (IsWindowVisible(h) && Cls(h) == "MultitaskingViewFrame" && Cloaked(h) == 0) { r = h; return false; } return true; }, IntPtr.Zero); return r; }
 public static int Pos(IntPtr t) { int i = 0, f = -1; EnumWindows((h,l)=>{ if (!IsWindowVisible(h)) return true; i++; if (h == t) { f = i; return false; } return true; }, IntPtr.Zero); return f; }
 public static List<IntPtr> Kids(IntPtr w) { var o = new List<IntPtr>(); EnumChildWindows(w, (h,l)=>{ o.Add(h); return true; }, IntPtr.Zero); return o; }
 public static List<IntPtr> OfClassAndPids(string cls, uint[] pids) { var o = new List<IntPtr>(); EnumWindows((h,l)=>{ if (Cls(h) == cls && Array.IndexOf(pids, PidOf(h)) >= 0) o.Add(h); return true; }, IntPtr.Zero); return o; }
 public static RECT Rect(IntPtr h) { RECT r; GetWindowRect(h, out r); return r; }
 public static void LeftAt(int x, int y) { SetCursorPos(x, y); mouse_event(2,0,0,0,UIntPtr.Zero); mouse_event(4,0,0,0,UIntPtr.Zero); } }
'@
$dpiState = if ([RC]::Aware()) { 'ok' } else { 'refused' }
function Stamp { (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
function NowUtc { (Get-Date).ToUniversalTime() }
function Log([string]$text) { Add-Content -Encoding utf8 $timeline $text; Write-Host $text }
function HexOf($handle) { '0x' + ([int64]$handle).ToString('x') }
function ImageOf([uint32]$procId) { try { (Get-Process -Id $procId -ErrorAction Stop).ProcessName + '.exe' } catch { 'gone' } }
function FgText { $fgHandle = [RC]::GetForegroundWindow(); $fgPid = [RC]::PidOf($fgHandle); "fg hwnd=$(HexOf $fgHandle) pid=$fgPid image=$(ImageOf $fgPid) title=$([RC]::Txt($fgHandle))" }
function Pts([int]$physX, [int]$physY) { "logical ($([Math]::Round($physX / $scaleFactor)),$([Math]::Round($physY / $scaleFactor))) physical ($physX,$physY)" }
function ClickAt([string]$pointName, [int]$physX, [int]$physY) {
    [RC]::LeftAt($physX, $physY); $clickStamp = Stamp; Start-Sleep -Milliseconds 300
    Log "CLICK left $pointName $clickStamp $(Pts $physX $physY); +300 ms $(FgText)"; return $clickStamp
}
function KeyEvent([string]$keyName, [byte]$virtualKey, [switch]$Up) {
    if ($Up) { [RC]::keybd_event($virtualKey, 0, 2, [UIntPtr]::Zero) } else { [RC]::keybd_event($virtualKey, 0, 0, [UIntPtr]::Zero) }
    Log "KEY $keyName $(if ($Up) { 'up' } else { 'down' }) $(Stamp)"
}
function ReadX([string]$name) {
    $xStart = Get-Date; $xStamp = Stamp
    $rectOut = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$reqDir\box-rect-probe.ps1" -Out "$evDir\rect-$name.txt" 2>&1
    $uiaOut = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$reqDir\taskbar-uia.ps1" -Out "$evDir\uia-$name.txt" 2>&1
    $coverOut = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$reqDir\taskbar-cover.ps1" -Out "$evDir\cover-$name.txt" 2>&1
    $capOut = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$reqDir\taskbar-capture.ps1" -Name $name -OutDir $shotDir -Out "$evDir\captures.txt" 2>&1
    $xSecs = [Math]::Round(((Get-Date) - $xStart).TotalSeconds, 2)
    Log "X $name $xStamp to $(Stamp) ($xSecs s) ;; rect out: $(($rectOut | ForEach-Object { "$_" }) -join ' ') ;; uia out: $(($uiaOut | ForEach-Object { "$_" }) -join ' ') ;; cover out: $(($coverOut | ForEach-Object { "$_" }) -join ' ') ;; capture out: $(($capOut | ForEach-Object { "$_" }) -join ' ')"
    return $xSecs
}
function GateProbe([string]$label) {
    $gpStart = Get-Date; $gpStamp = Stamp
    $probeOut = @(& powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$reqDir\zorder-probe-k.ps1" 2>&1 | ForEach-Object { "$_" })
    $probeOut | Set-Content -Encoding utf8 "$evDir\probe-$Cell-$label.txt"
    $gpSecs = [Math]::Round(((Get-Date) - $gpStart).TotalSeconds, 2)
    Log "G-P $label $gpStamp to $(Stamp) ($gpSecs s): $($probeOut -join ' | ')"
    return $probeOut
}
function TallyFirstInFront($probeLines) { $tallyRow = @($probeLines | Where-Object { $_ -match ',TallyPrime:9001' }) | Select-Object -First 1; return ($tallyRow -match '^\D?\d+,\d+,1,') }
function BoxOpenTopmost($probeLines) { $boxRow = @($probeLines | Where-Object { $_ -match ',ComplyEaze Bridge' }) | Select-Object -First 1; return ($boxRow -match '^\D?\d+,1,') }
function RecordConsole([string]$consoleName, [uint32]$consolePid, $consoleWindow) {
    if ($consoleWindow) { $script:consoleHandles[$consoleName] = $consoleWindow }
    $entry = "$(Stamp) cell=$Cell name=$consoleName pid=$consolePid window=$(if ($consoleWindow) { HexOf $consoleWindow } else { 'not-found' })"
    Add-Content -Encoding utf8 "$evDir\consoles.txt" $entry; Log "recorded console: $entry"
}
function MinimiseConsole([string]$consoleName, [string]$why) {
    $consoleHandle = $script:consoleHandles[$consoleName]
    if (-not $consoleHandle) { Log "minimise ${consoleName}: no window recorded; nothing done"; return $false }
    if ($script:minimisedOnce.ContainsKey($consoleName)) { Log "minimise ${consoleName}: already minimised once in this cell; nothing done"; return $false }
    if ([RC]::GetForegroundWindow() -eq $consoleHandle) { Log "minimise ${consoleName}: it is the foreground window; nothing done"; return $false }
    [void][RC]::ShowWindow($consoleHandle, 6); $script:minimisedOnce[$consoleName] = 1
    Start-Sleep -Milliseconds 300; Log "MINIMISED $consoleName $(HexOf $consoleHandle) by ShowWindow(SW_MINIMIZE) $(Stamp) ($why); iconic=$([int][RC]::IsIconic($consoleHandle)); $(FgText)"; return $true
}
function StartMinimised([string]$consoleName, [string]$argText) {
    $startedProc = Start-Process powershell -ArgumentList $argText -WindowStyle Minimized -PassThru; $null = $startedProc.Handle
    Start-Sleep -Milliseconds 1500
    $foundWindow = @([RC]::OfClassAndPids('ConsoleWindowClass', [uint32[]]@($startedProc.Id))) | Select-Object -First 1
    RecordConsole $consoleName ([uint32]$startedProc.Id) $foundWindow
    Log "$consoleName started minimised $(Stamp) pid $($startedProc.Id): powershell $argText"
}
function BoxHandle { [RC]::ByTitle($boxTitle) }
function AnswerNo([string]$why) {
    $boxHandle = BoxHandle
    if ($boxHandle -eq [IntPtr]::Zero) { Log "no box to answer $(Stamp) ($why)"; return }
    if ([RC]::IsIconic($boxHandle)) { Log "the box is minimised $(Stamp): No cannot be clicked ($why); nothing clicked"; return }
    $noHandle = [IntPtr]::Zero; foreach ($kid in [RC]::Kids($boxHandle)) { if ([RC]::Cls($kid) -eq 'Button' -and [RC]::Txt($kid) -eq '&No') { $noHandle = $kid } }
    if ($noHandle -eq [IntPtr]::Zero) { Log "no No button found on the box $(Stamp); nothing clicked"; return }
    $boxZ = [RC]::Pos($boxHandle); $tallyZ = [RC]::Pos($script:tallyHandle); $tallyMinimised = $false
    $boxRectNow = [RC]::Rect($boxHandle); $noRect = [RC]::Rect($noHandle)
    $rectLine = @(Get-Content "$evDir\rect-$Cell.txt" -ErrorAction SilentlyContinue | Where-Object { $_ -like 'B *' }) | Select-Object -First 1
    Log "box rect now $($boxRectNow.L),$($boxRectNow.T),$($boxRectNow.R),$($boxRectNow.B); rect-$Cell.txt B line: $rectLine; No button rect $($noRect.L),$($noRect.T),$($noRect.R),$($noRect.B)"
    if (-not ($boxZ -gt 0 -and $boxZ -lt $tallyZ)) { [void][RC]::ShowWindow($script:tallyHandle, 6); Start-Sleep -Milliseconds 600; $tallyMinimised = $true; Log "the box is under Tally (z box $boxZ, Tally $tallyZ): Tally MINIMISED by ShowWindow(SW_MINIMIZE) $(Stamp)" }
    $noButtonPoint = @([int](($noRect.L + $noRect.R) / 2), [int](($noRect.T + $noRect.B) / 2))
    [void](ClickAt 'noButtonPoint' $noButtonPoint[0] $noButtonPoint[1]); Log "No clicked ($why)"
    if ($tallyMinimised) { Start-Sleep -Milliseconds 800; [void][RC]::ShowWindow($script:tallyHandle, 9); Log "Tally RESTORED by ShowWindow(SW_RESTORE) $(Stamp)" }
}
function StepHA($t0Value) {
    $gpLines = GateProbe 'ha'
    if (((NowUtc) - $t0Value).TotalSeconds -gt 36) { Log "HA skipped: late (G-P ended at T0+$([Math]::Round(((NowUtc) - $t0Value).TotalSeconds, 2)) s)"; return }
    $tallyOk = TallyFirstInFront $gpLines; $boxOk = if ($Cell -eq 'n') { $true } else { BoxOpenTopmost $gpLines }
    if (-not $tallyOk) { Log "HA skipped: Tally did not hold the foreground"; return }
    if (-not $boxOk) { Log "HA skipped: the box was not open and topmost at the gate"; return }
    Log "HA gate met $(Stamp): Tally in front$(if ($Cell -ne 'n') { ', the box open and topmost' })"
    KeyEvent 'Alt' 0x12; Start-Sleep -Milliseconds 80
    KeyEvent 'Tab' 0x09; Start-Sleep -Milliseconds 40; KeyEvent 'Tab' 0x09 -Up
    $heldAt = Get-Date; [IO.File]::WriteAllText("$wordDir\$Cell-ha-held", (Stamp))
    $holdMin = 3; $holdMax = if ($Cell -eq 'n') { 12 } else { 3 }
    while ($true) {
        $heldSecs = ((Get-Date) - $heldAt).TotalSeconds
        if ($heldSecs -ge $holdMax) { break }
        if ($heldSecs -ge $holdMin -and ($Cell -ne 'n' -or (Test-Path "$wordDir\n-ha-shot-done"))) { break }
        Start-Sleep -Milliseconds 50
    }
    $switcherBefore = [RC]::Switcher()
    Log "HA held $([Math]::Round(((Get-Date) - $heldAt).TotalSeconds, 2)) s $(Stamp); switcher before Esc: $(if ($switcherBefore -ne [IntPtr]::Zero) { HexOf $switcherBefore } else { 'none' }); $(FgText)$(if ($Cell -eq 'n') { "; control shot word: $(Test-Path "$wordDir\n-ha-shot-done")" })"
    KeyEvent 'Esc' 0x1B; Start-Sleep -Milliseconds 40; KeyEvent 'Esc' 0x1B -Up
    Start-Sleep -Milliseconds 400
    $switcherAfter = [RC]::Switcher()
    if ($switcherAfter -ne [IntPtr]::Zero) { Log "stop: the switcher is still open after the Esc ($(HexOf $switcherAfter)) $(Stamp): Alt stays held, nothing more pressed or clicked"; exit 40 }
    KeyEvent 'Alt' 0x12 -Up
    Start-Sleep -Milliseconds 500; Log "after HA $(Stamp): $(FgText); box $(if ((BoxHandle) -ne [IntPtr]::Zero) { 'open' } else { 'none' })"
}
function WaitEndLines([string]$spawnFile) {
    $waitStart = Get-Date
    if ($spawnFile) { while (((Get-Date) - $waitStart).TotalSeconds -lt 150) { if ((Get-Content $spawnFile -ErrorAction SilentlyContinue) -match 'exit_code=') { break }; Start-Sleep -Milliseconds 200 } }
    $waitStart = Get-Date
    while (((Get-Date) - $waitStart).TotalSeconds -lt 130) {
        $traceDone = (Get-Content "$evDir\trace-$Cell.txt" -ErrorAction SilentlyContinue) -match '^END '
        $watchDone = (Get-Content "$evDir\switch-$Cell.txt" -ErrorAction SilentlyContinue) -match '^Z '
        if ($traceDone -and $watchDone) { break }; Start-Sleep -Milliseconds 300
    }
    if ($spawnFile) {
        Log "spawn file: $((Get-Content $spawnFile -ErrorAction SilentlyContinue) -join ' || ')"
        Log "stdout bytes: $(if (Test-Path "$evDir\$Cell.stdout") { (Get-Item "$evDir\$Cell.stdout").Length } else { 'no file' })"
        Log "stderr whole: $(if (Test-Path "$evDir\$Cell.stderr") { (Get-Content "$evDir\$Cell.stderr") -join ' || ' } else { 'no file' })"
    }
    Log "trace lines: $((Get-Content "$evDir\trace-$Cell.txt" -ErrorAction SilentlyContinue | Where-Object { $_ -match '^(FIRST|GONE|END) ' }) -join ' | ')"
    Log "watcher lines A/C/G/F/Z: $((Get-Content "$evDir\switch-$Cell.txt" -ErrorAction SilentlyContinue | Where-Object { $_ -match '^(A|C|G|F|Z) ' }) -join ' | ')"
    foreach ($entry in $script:consoleHandles.GetEnumerator()) { Log "console $($entry.Key) $(HexOf $entry.Value) exists=$([int][RC]::IsWindow($entry.Value)) iconic=$([int][RC]::IsIconic($entry.Value))" }
}

$script:tallyHandle = [RC]::ByTitle('TallyPrime:9001')
$script:consoleHandles = @{}; $script:minimisedOnce = @{}
Log "$Cell runner pid $PID $(Stamp); dpi_context=$dpiState; Tally window $(HexOf $script:tallyHandle); $(FgText)"

if ($Cell -eq 'x0') {
    [void](ClickAt 'tallyTitlePoint' $tallyTitlePoint[0] $tallyTitlePoint[1]); Start-Sleep -Milliseconds 500; Log "Tally in front: $([RC]::GetForegroundWindow() -eq $script:tallyHandle)"
    $xTime = ReadX 'x0'; Log "X-time $xTime s"; Log "$Cell DONE $(Stamp)"; exit 0
}
if ($Cell -eq 'b3') {
    [void](ClickAt 'tallyTitlePoint' $tallyTitlePoint[0] $tallyTitlePoint[1]); Start-Sleep -Milliseconds 500
    $gpLines = GateProbe 'b3'; Log "Tally first and in front: $(TallyFirstInFront $gpLines)"; Log "$Cell DONE $(Stamp)"; exit 0
}

# n, c1, c2: pre-arm the trace and the watcher, minimised
StartMinimised 'trace console' "-NoProfile -ExecutionPolicy Bypass -File $reqDir\box-trace-early.ps1 -Title `"$boxTitle`" -Out $evDir\trace-$Cell.txt -Seconds 100"
StartMinimised 'watcher console' "-NoProfile -ExecutionPolicy Bypass -File $reqDir\switcher-watch.ps1 -Name $Cell -OutDir $shotDir -Out $evDir\switch-$Cell.txt -Seconds 100"
[void](ClickAt 'tallyTitlePoint' $tallyTitlePoint[0] $tallyTitlePoint[1]); Log "Tally in front: $([RC]::GetForegroundWindow() -eq $script:tallyHandle)"
if ($Cell -eq 'n') {
    [IO.File]::WriteAllText("$wordDir\n-pre-ready", (Stamp)); Log "WAIT $(Stamp): the lab's screen-tool copy before the cell (cmd\n-pre-done, at most 120 s)"
    $waitUntil = (Get-Date).AddSeconds(120); while ((Get-Date) -lt $waitUntil -and -not (Test-Path "$wordDir\n-pre-done")) { Start-Sleep -Milliseconds 200 }
    Log "pre copy word: $(Test-Path "$wordDir\n-pre-done") $(Stamp)"
    if ([RC]::GetForegroundWindow() -ne $script:tallyHandle) { [void](ClickAt 'tallyTitlePoint' $tallyTitlePoint[0] $tallyTitlePoint[1]); Log "Tally in front again: $([RC]::GetForegroundWindow() -eq $script:tallyHandle)" }
}

$cmdFile = "$wordDir\$Cell.cmd"
if ($Cell -eq 'n') { $cmdText = "powershell -NoProfile -Command Start-Sleep -Seconds 8`r`npowershell -NoProfile -Command Start-Sleep -Seconds 60`r`n" }
else { $cmdText = "powershell -NoProfile -Command Start-Sleep -Seconds 8`r`npowershell -NoProfile -ExecutionPolicy Bypass -File $reqDir\spawn-parent.ps1 -Target child -ChildExe $boxExe -InputFile $reqDir\confirm-input.txt -Stdio files -Out $evDir\$Cell.txt -TimeoutSeconds 120`r`n" }
[IO.File]::WriteAllText($cmdFile, $cmdText, (New-Object Text.ASCIIEncoding))
$null = Start-Process explorer.exe -ArgumentList "`"$cmdFile`"" -PassThru; $cellStart = NowUtc; $openStamp = Stamp
Log "OPEN ${openStamp}: explorer.exe `"$cmdFile`"; the .cmd holds: $($cmdText.Trim() -replace "`r`n", ' | ')"
$cmdProcId = 0; $cmdWindow = $null; $findUntil = (Get-Date).AddSeconds(3)
while ((Get-Date) -lt $findUntil -and -not $cmdWindow) {
    $cmdProc = @(Get-CimInstance Win32_Process -Filter "Name='cmd.exe'" | Where-Object { $_.CommandLine -and $_.CommandLine.Contains("$Cell.cmd") }) | Select-Object -First 1
    if ($cmdProc) {
        $cmdProcId = [uint32]$cmdProc.ProcessId; $kidIds = @(Get-CimInstance Win32_Process -Filter "ParentProcessId=$cmdProcId" | ForEach-Object { [uint32]$_.ProcessId })
        $cmdWindow = @([RC]::OfClassAndPids('ConsoleWindowClass', [uint32[]](@($cmdProcId) + $kidIds))) | Select-Object -First 1
    }
    if (-not $cmdWindow) { Start-Sleep -Milliseconds 100 }
}
RecordConsole 'cmd console' $cmdProcId $cmdWindow
while (((NowUtc) - $cellStart).TotalSeconds -lt 2.5) { Start-Sleep -Milliseconds 50 }
$minimisedBefore = MinimiseConsole 'cmd console' 'P-min: during the 8 s sleep, before T0 and before the title click'
while (((NowUtc) - $cellStart).TotalSeconds -lt 4) { Start-Sleep -Milliseconds 50 }
[void](ClickAt 'tallyTitlePoint' $tallyTitlePoint[0] $tallyTitlePoint[1]); Log "title click $([Math]::Round(((NowUtc) - $cellStart).TotalSeconds, 2)) s after the open; Tally in front: $([RC]::GetForegroundWindow() -eq $script:tallyHandle)"
if (-not $minimisedBefore -and ((NowUtc) - $cellStart).TotalSeconds -lt 7.5) { [void](MinimiseConsole 'cmd console' 'P-min: after the title click, the console was in front before it') }

if ($Cell -eq 'n') {
    $t0 = $cellStart.AddSeconds(8); Log "n-T0 $($t0.ToString('yyyy-MM-ddTHH:mm:ss.fffZ')) (the open's stamp + 8 s)"
} else {
    $t0 = $null; $t0Until = (Get-Date).AddSeconds(60)
    while ((Get-Date) -lt $t0Until -and -not $t0) {
        $spawnLine = Get-Content "$evDir\$Cell.txt" -ErrorAction SilentlyContinue | Where-Object { $_ -match 'child_pid=' } | Select-Object -First 1
        if ($spawnLine -and $spawnLine -match '^\W*SPAWN (\S+) ') { $t0 = [DateTime]::Parse($Matches[1]).ToUniversalTime(); Log "$Cell-T0 $($Matches[1]) from: $spawnLine" }
        Start-Sleep -Milliseconds 50
    }
    if (-not $t0) { Log "no child_pid line in 60 s: nothing more clicked"; WaitEndLines "$evDir\$Cell.txt"; Log "$Cell DONE $(Stamp)"; exit 20 }
    $boxWait = $t0.AddSeconds(15)
    while ((NowUtc) -lt $boxWait -and (BoxHandle) -eq [IntPtr]::Zero) { Start-Sleep -Milliseconds 100 }
    if ((BoxHandle) -eq [IntPtr]::Zero) { Log "no box by T0+15 $(Stamp): ended nothing"; WaitEndLines "$evDir\$Cell.txt"; Log "$Cell DONE $(Stamp)"; exit 21 }
}
function SinceT0 { [Math]::Round(((NowUtc) - $t0).TotalSeconds, 2) }
while (((NowUtc) - $t0).TotalSeconds -lt 8) { Start-Sleep -Milliseconds 20 }
$t8Lines = GateProbe 't8'; Log "T0+$(SinceT0) Tally in front: $(TallyFirstInFront $t8Lines)$(if ($Cell -ne 'n') { "; box open and topmost: $(BoxOpenTopmost $t8Lines)" })"
while (((NowUtc) - $t0).TotalSeconds -lt 10) { Start-Sleep -Milliseconds 20 }
[void](ReadX $Cell)
Log "HA starts at T0+$(SinceT0) s"
StepHA $t0
if ($Cell -eq 'n') {
    WaitEndLines $null
    [IO.File]::WriteAllText("$wordDir\n-post-ready", (Stamp)); Log "the watcher's Z line is written: the lab's screen-tool copy after the cell"
    Log "$Cell DONE $(Stamp)"; exit 0
}
while (((NowUtc) - $t0).TotalSeconds -lt 46) { Start-Sleep -Milliseconds 20 }
[void](GateProbe 't46')
while (((NowUtc) - $t0).TotalSeconds -lt 50) { Start-Sleep -Milliseconds 20 }
AnswerNo "about T0+50 (T0+$(SinceT0) s)"
WaitEndLines "$evDir\$Cell.txt"
Log "$Cell DONE $(Stamp)"; exit 0
