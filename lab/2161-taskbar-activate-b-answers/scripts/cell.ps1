# Lab helper (#2161-B, not product code; row 2161-A's b-cell.ps1 rebuilt to this sheet). One cell: n1, n2, p, pmin, c1, c2 or c3.
# Per-monitor DPI aware: every point and rectangle it uses is in physical pixels (the screen is 1920x1080 at 125 %, so a logical
# point is the physical one divided by 1.25; both are recorded for every click). Every clicked point has its own variable:
# $tallyTitlePoint, $tallyBlankPoint, $boxButtonPoint, $noButtonPoint, $previewPoint, $menuItemPoint.
# A read R: <cell>.stderr's line count and last line, box-rect-probe.ps1, zorder-probe-k.ps1, taskbar-cover.ps1 (-X -Y when a
# click follows). A capture K: taskbar-capture.ps1 and the runner's own whole-screen copy. Never the screen tool while a box is open.
# At each taskbar gate it hovers, copies the whole screen, writes cmd\<cell>-<label>-ready and waits for the lab's word in
# cmd\<cell>-<label>-go (the thumbnail's title read from the copy) or cmd\<cell>-<label>-nogo; no word in 120 s = no click.
# It never clicks Yes, sends no key, ends no process and minimises only a recorded console (the sheet's rules) or Tally (a box under it).
param([Parameter(Mandatory = $true)][ValidateSet('n1', 'n2', 'p', 'pmin', 'c1', 'c2', 'c3')][string]$Cell)
$labDir = '<lab folder>'; $evDir = "$labDir\evidence"; $shotDir = "$labDir\shots"; $reqDir = "$labDir\requests"; $wordDir = "$labDir\cmd"
$timeline = "$evDir\$Cell-timeline.txt"
$boxExe = '<build folder>\bin\x86_64-pc-windows-msvc\bridge_mcp.exe'
$boxTitle = 'ComplyEaze Bridge'
$scaleFactor = 1.25
$tallyTitlePoint = @(412, 17)
$tallyBlankPoint = @(329, 594)
Add-Type -AssemblyName System.Drawing
Add-Type @'
using System; using System.Text; using System.Collections.Generic; using System.Runtime.InteropServices;
public class RB { public delegate bool EP(IntPtr h, IntPtr l);
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
 [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
 [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
 [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
 [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h, int c);
 [DllImport("user32.dll")] public static extern void mouse_event(uint f, uint x, uint y, uint d, UIntPtr e);
 public static bool Aware() { return SetProcessDpiAwarenessContext(new IntPtr(-4)); }
 public static string Txt(IntPtr h) { var s = new StringBuilder(1024); GetWindowText(h, s, 1024); return s.ToString().Replace("\r", " ").Replace("\n", " "); }
 public static string Cls(IntPtr h) { var s = new StringBuilder(256); GetClassName(h, s, 256); return s.ToString(); }
 public static uint PidOf(IntPtr h) { uint p; GetWindowThreadProcessId(h, out p); return p; }
 public static IntPtr ByTitle(string t) { IntPtr r = IntPtr.Zero; EnumWindows((h,l)=>{ if (IsWindowVisible(h) && Txt(h).StartsWith(t)) { r = h; return false; } return true; }, IntPtr.Zero); return r; }
 public static int Pos(IntPtr t) { int i = 0, f = -1; EnumWindows((h,l)=>{ if (!IsWindowVisible(h)) return true; i++; if (h == t) { f = i; return false; } return true; }, IntPtr.Zero); return f; }
 public static List<IntPtr> Kids(IntPtr w) { var o = new List<IntPtr>(); EnumChildWindows(w, (h,l)=>{ o.Add(h); return true; }, IntPtr.Zero); return o; }
 public static List<IntPtr> OfClassAndPids(string cls, uint[] pids) { var o = new List<IntPtr>(); EnumWindows((h,l)=>{ if (Cls(h) == cls && Array.IndexOf(pids, PidOf(h)) >= 0) o.Add(h); return true; }, IntPtr.Zero); return o; }
 public static List<string> Thumbs() { var o = new List<string>(); EnumWindows((h,l)=>{ if (IsWindowVisible(h) && Cls(h) == "TaskListThumbnailWnd") { RECT r; GetWindowRect(h, out r); o.Add("0x" + h.ToInt64().ToString("x") + " rect=" + r.L + "," + r.T + "," + r.R + "," + r.B + " title=" + Txt(h)); } return true; }, IntPtr.Zero); return o; }
 public static RECT Rect(IntPtr h) { RECT r; GetWindowRect(h, out r); return r; }
 public static void LeftAt(int x, int y) { SetCursorPos(x, y); mouse_event(2,0,0,0,UIntPtr.Zero); mouse_event(4,0,0,0,UIntPtr.Zero); }
 public static void RightAt(int x, int y) { SetCursorPos(x, y); mouse_event(8,0,0,0,UIntPtr.Zero); mouse_event(16,0,0,0,UIntPtr.Zero); } }
'@
$dpiState = if ([RB]::Aware()) { 'ok' } else { 'refused' }
function Stamp { (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
function NowUtc { (Get-Date).ToUniversalTime() }
function Log([string]$text) { Add-Content -Encoding utf8 $timeline $text; Write-Host $text }
function HexOf($handle) { '0x' + ([int64]$handle).ToString('x') }
function ImageOf([uint32]$procId) { try { (Get-Process -Id $procId -ErrorAction Stop).ProcessName + '.exe' } catch { 'gone' } }
function FgText { $fgHandle = [RB]::GetForegroundWindow(); $fgPid = [RB]::PidOf($fgHandle); "fg hwnd=$(HexOf $fgHandle) pid=$fgPid image=$(ImageOf $fgPid) title=$([RB]::Txt($fgHandle))" }
function Pts([int]$physX, [int]$physY) { "logical ($([Math]::Round($physX / $scaleFactor)),$([Math]::Round($physY / $scaleFactor))) physical ($physX,$physY)" }
function ClickAt([string]$pointName, [int]$physX, [int]$physY, [switch]$RightButton) {
    if ($RightButton) { [RB]::RightAt($physX, $physY) } else { [RB]::LeftAt($physX, $physY) }
    $clickStamp = Stamp; Start-Sleep -Milliseconds 300
    Log "CLICK $(if ($RightButton) { 'right' } else { 'left' }) $pointName $clickStamp $(Pts $physX $physY); +300 ms $(FgText)"
    return $clickStamp
}
function StderrText {
    $errFile = "$evDir\$Cell.stderr"
    if (-not (Test-Path $errFile)) { return 'no stderr file' }
    $fs = [IO.File]::Open($errFile, 'Open', 'Read', 'ReadWrite'); $sr = New-Object IO.StreamReader($fs); $allLines = @($sr.ReadToEnd() -split "`r?`n" | Where-Object { $_ -ne '' }); $sr.Close(); $fs.Close()
    "stderr lines $($allLines.Count) (box-fit $(@($allLines | Where-Object { $_ -like 'box-fit:*' }).Count)); last: $(if ($allLines.Count) { $allLines[-1] } else { '-' })"
}
function ReadR([string]$label, [int]$atX = -1, [int]$atY = -1) {
    $readStart = Stamp; $errText = StderrText
    $rectOut = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$reqDir\box-rect-probe.ps1" -Out "$evDir\rect-$Cell-$label.txt" 2>&1
    $probeOut = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$reqDir\zorder-probe-k.ps1" 2>&1
    $probeOut | Set-Content -Encoding utf8 "$evDir\probe-$Cell-$label.txt"
    $coverFile = "$evDir\cover-$Cell-$label.txt"
    if ($atX -ge 0) { $coverOut = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$reqDir\taskbar-cover.ps1" -Out $coverFile -X $atX -Y $atY 2>&1 }
    else { $coverOut = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$reqDir\taskbar-cover.ps1" -Out $coverFile 2>&1 }
    $boxLines = @(Get-Content "$evDir\rect-$Cell-$label.txt" -ErrorAction SilentlyContinue | Where-Object { $_ -like 'B *' -or $_ -like 'F *' })
    $pLine = @(Get-Content $coverFile -ErrorAction SilentlyContinue | Where-Object { $_ -like 'P *' }) | Select-Object -First 1
    Log "R $label $readStart to $(Stamp): $errText ;; rect: $($boxLines -join ' | ') ;; probe: $(($probeOut | ForEach-Object { "$_" }) -join ' | ') ;; cover: $(($coverOut | ForEach-Object { "$_" }) -join ' ') $(if ($pLine) { ";; $pLine" })"
    return $pLine
}
function CapK([string]$label) {
    $capStart = Stamp
    $capOut = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$reqDir\taskbar-capture.ps1" -Name "$Cell-$label" -OutDir $shotDir -Out "$evDir\captures.txt" 2>&1
    $fullName = "$shotDir\$Cell-$label-full.png"; FullCopy $fullName
    Log "K $label $capStart to $(Stamp): $(($capOut | ForEach-Object { "$_" }) -join ' ') ;; full copy $Cell-$label-full.png"
}
function FullCopy([string]$outPath) {
    $bmp = New-Object System.Drawing.Bitmap 1920, 1080; $gfx = [System.Drawing.Graphics]::FromImage($bmp)
    $gfx.CopyFromScreen(0, 0, 0, 0, (New-Object System.Drawing.Size 1920, 1080)); $bmp.Save($outPath); $gfx.Dispose(); $bmp.Dispose()
}
function TaskListRect([string]$label) {
    $kLine = @(Get-Content "$evDir\rect-$Cell-$label.txt" -ErrorAction SilentlyContinue | Where-Object { $_ -match 'class=MSTaskListWClass' }) | Select-Object -First 1
    if ($kLine -match 'rect=(-?\d+),(-?\d+),(-?\d+),(-?\d+)') { return @([int]$Matches[1], [int]$Matches[2], [int]$Matches[3], [int]$Matches[4]) }
    return $null
}
function ButtonRuns([string]$label) {
    # Buttons in the cell's PrintWindow copy of the taskbar, inside the task list's rectangle: columns whose pixels differ from the
    # bar's commonest colour, in runs split by a gap of more than 6 px; each run's centre in screen physical pixels.
    $listRect = TaskListRect $label; $capLine = @(Get-Content "$evDir\captures.txt" | Where-Object { $_ -match "name=$Cell-$label " }) | Select-Object -Last 1
    if (-not $listRect -or $capLine -notmatch 'rect=(-?\d+),(-?\d+),') { return @() }
    $trayLeft = [int]$Matches[1]; $trayTop = [int]$Matches[2]
    $img = [System.Drawing.Bitmap]::FromFile("$shotDir\$Cell-$label-print.png")
    $fromX = [Math]::Max(0, $listRect[0] - $trayLeft); $toX = [Math]::Min($img.Width - 1, $listRect[2] - $trayLeft - 1); $midY = [int]($img.Height / 2)
    $colourCount = @{}
    for ($x = $fromX; $x -le $toX; $x++) { $px = $img.GetPixel($x, $midY); $key = "$($px.R),$($px.G),$($px.B)"; $colourCount[$key] = 1 + [int]$colourCount[$key] }
    $bgParts = ($colourCount.GetEnumerator() | Sort-Object Value -Descending | Select-Object -First 1).Key -split ','
    $runs = @(); $runStart = -1; $lastHit = -100
    for ($x = $fromX; $x -le $toX; $x++) {
        $hit = $false
        foreach ($dy in -10, -5, 0, 5, 10) { $px = $img.GetPixel($x, $midY + $dy); if ([Math]::Abs($px.R - [int]$bgParts[0]) + [Math]::Abs($px.G - [int]$bgParts[1]) + [Math]::Abs($px.B - [int]$bgParts[2]) -gt 60) { $hit = $true; break } }
        if ($hit) { if ($runStart -lt 0 -or ($x - $lastHit) -gt 6) { if ($runStart -ge 0) { $runs += ,@($runStart, $lastHit) }; $runStart = $x }; $lastHit = $x }
    }
    if ($runStart -ge 0) { $runs += ,@($runStart, $lastHit) }
    $img.Dispose()
    return @($runs | Where-Object { ($_[1] - $_[0]) -ge 8 } | ForEach-Object { [int](($_[0] + $_[1]) / 2) + $trayLeft })
}
function RecordConsoles { Log "recorded consoles: runner pid $PID (no window of its own: started hidden by the lab's tool); $(($script:consoleRecords | ForEach-Object { $_ }) -join '; ')" }
function ConsoleStates([string]$when) { $stateText = @(); foreach ($entry in $script:consoleHandles.GetEnumerator()) { $stateText += "$($entry.Key) $(HexOf $entry.Value) iconic=$([int][RB]::IsIconic($entry.Value)) exists=$([int][RB]::IsWindow($entry.Value))" }; Log "consoles at $when $(Stamp): $($stateText -join '; ')" }
function MinimiseConsole([string]$consoleName, [string]$why) {
    $consoleHandle = $script:consoleHandles[$consoleName]
    if (-not $consoleHandle) { Log "minimise ${consoleName}: no window recorded; nothing done"; return $false }
    if ($script:minimisedOnce.ContainsKey($consoleName)) { Log "minimise ${consoleName}: already minimised once in this cell; nothing done"; return $false }
    if ([RB]::GetForegroundWindow() -eq $consoleHandle) { Log "minimise ${consoleName}: it is the foreground window; nothing done"; return $false }
    [void][RB]::ShowWindow($consoleHandle, 6); $script:minimisedOnce[$consoleName] = 1
    Start-Sleep -Milliseconds 300; Log "MINIMISED $consoleName $(HexOf $consoleHandle) by ShowWindow(SW_MINIMIZE) $(Stamp) ($why); iconic=$([int][RB]::IsIconic($consoleHandle)); $(FgText)"; return $true
}
function WaitWord([string]$label, [string]$what, [int]$seconds = 120) {
    $readyFile = "$wordDir\$Cell-$label-ready"; [IO.File]::WriteAllText($readyFile, "$(Stamp) $what")
    Log "WAIT $label $(Stamp): $what"
    $until = (Get-Date).AddSeconds($seconds)
    while ((Get-Date) -lt $until) {
        if (Test-Path "$wordDir\$Cell-$label-go") { $word = (Get-Content "$wordDir\$Cell-$label-go" -Raw).Trim(); Log "WORD $label $(Stamp): go: $word"; return @('go', $word) }
        if (Test-Path "$wordDir\$Cell-$label-nogo") { $word = (Get-Content "$wordDir\$Cell-$label-nogo" -Raw).Trim(); Log "WORD $label $(Stamp): nogo: $word"; return @('nogo', $word) }
        Start-Sleep -Milliseconds 200
    }
    Log "WORD $label $(Stamp): none in $seconds s"; return @('none', '')
}
function LocateBoxButton([string]$label) {
    $n2Runs = @(Get-Content "$evDir\n1-buttons.txt", "$evDir\n2-buttons.txt" -ErrorAction SilentlyContinue | Where-Object { $_ -match '^\d+$' } | ForEach-Object { [int]$_ })
    $cellRuns = @(ButtonRuns $label)
    $newRuns = @($cellRuns | Where-Object { $runX = $_; -not ($n2Runs | Where-Object { [Math]::Abs($_ - $runX) -le 12 }) })
    Log "buttons $label (PrintWindow, physical x at y 1055): $($cellRuns -join ' '); N1's and N2's: $($n2Runs -join ' '); in neither N1 nor N2: $($newRuns -join ' ')"
    if ($newRuns.Count -eq 1) { return @($newRuns[0], 1055) }
    Log "the box's button is not identified ($($newRuns.Count) new runs)"; return $null
}
function Gate([string]$label, $buttonPoint) {
    $pLine = ReadR $label $buttonPoint[0] $buttonPoint[1]
    if ($pLine -match 'point_ok=0' -and $pLine -match ' root=(0x[0-9a-f]+) ') {
        $rootHex = $Matches[1]
        foreach ($entry in @($script:consoleHandles.GetEnumerator())) { if ((HexOf $entry.Value) -eq $rootHex) { if (MinimiseConsole $entry.Key "the P line names it at the button point") { $pLine = ReadR "$label-b" $buttonPoint[0] $buttonPoint[1] } } }
    }
    if ($pLine -notmatch 'point_ok=1') { Log "GATE ${label}: taskbar not reachable at the button point ($pLine)"; return $false }
    [void][RB]::SetCursorPos($buttonPoint[0], $buttonPoint[1]); Log "HOVER boxButtonPoint $(Stamp) $(Pts $buttonPoint[0] $buttonPoint[1]) (no click)"
    Start-Sleep -Milliseconds 1500
    $hoverShot = "$shotDir\$Cell-$label-hover.png"; FullCopy $hoverShot
    Log "hover copy $Cell-$label-hover.png $(Stamp); thumbnail windows: $(([RB]::Thumbs()) -join ' | ')"
    $word = WaitWord $label "gate: read the thumbnail in $hoverShot; go = a thumbnail whose title begins with ComplyEaze Bridge"
    if ($word[0] -ne 'go') { Log "GATE ${label}: no click ($($word[0]): $($word[1]))"; return $false }
    return $true
}
function BoxHandle { [RB]::ByTitle($boxTitle) }
function AnswerNo([string]$why) {
    $boxHandle = BoxHandle
    if ($boxHandle -eq [IntPtr]::Zero) { Log "no box to answer $(Stamp) ($why)"; return }
    if ([RB]::IsIconic($boxHandle)) { Log "the box is minimised $(Stamp): No cannot be clicked ($why); nothing clicked"; return }
    $noHandle = [IntPtr]::Zero; foreach ($kid in [RB]::Kids($boxHandle)) { if ([RB]::Cls($kid) -eq 'Button' -and [RB]::Txt($kid) -eq '&No') { $noHandle = $kid } }
    if ($noHandle -eq [IntPtr]::Zero) { Log "no No button found on the box $(Stamp); nothing clicked"; return }
    $boxZ = [RB]::Pos($boxHandle); $tallyZ = [RB]::Pos($script:tallyHandle); $tallyMinimised = $false
    if (-not ($boxZ -gt 0 -and $boxZ -lt $tallyZ)) { [void][RB]::ShowWindow($script:tallyHandle, 6); Start-Sleep -Milliseconds 600; $tallyMinimised = $true; Log "the box is under Tally (z box $boxZ, Tally $tallyZ): Tally MINIMISED by ShowWindow(SW_MINIMIZE) $(Stamp)" }
    $noRect = [RB]::Rect($noHandle); $noButtonPoint = @([int](($noRect.L + $noRect.R) / 2), [int](($noRect.T + $noRect.B) / 2))
    [void](ClickAt 'noButtonPoint' $noButtonPoint[0] $noButtonPoint[1]); Log "No clicked ($why)"
    if ($tallyMinimised) { Start-Sleep -Milliseconds 800; [void][RB]::ShowWindow($script:tallyHandle, 9); Log "Tally RESTORED by ShowWindow(SW_RESTORE) $(Stamp)" }
}
function ClickIntoTally {
    $boxHandle = BoxHandle
    if ($boxHandle -ne [IntPtr]::Zero) {
        $boxRect = [RB]::Rect($boxHandle)
        $gapX = [Math]::Max(0, [Math]::Max($boxRect.L - $tallyBlankPoint[0], $tallyBlankPoint[0] - $boxRect.R)); $gapY = [Math]::Max(0, [Math]::Max($boxRect.T - $tallyBlankPoint[1], $tallyBlankPoint[1] - $boxRect.B))
        $gapLogical = [Math]::Sqrt($gapX * $gapX + $gapY * $gapY) / $scaleFactor
        Log "box rect physical $($boxRect.L),$($boxRect.T),$($boxRect.R),$($boxRect.B); tallyBlankPoint $(Pts $tallyBlankPoint[0] $tallyBlankPoint[1]); gap $([Math]::Round($gapLogical, 1)) logical px"
        if ($gapLogical -lt 150) { Log "tallyBlankPoint is within 150 logical px of the box: no click into Tally"; return $false }
    }
    $coverFile = "$evDir\cover-$Cell-pre-tally.txt"; $coverStart = Stamp
    $coverOut = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$reqDir\taskbar-cover.ps1" -Out $coverFile -X $tallyBlankPoint[0] -Y $tallyBlankPoint[1] 2>&1
    $pLine = @(Get-Content $coverFile -ErrorAction SilentlyContinue | Where-Object { $_ -like 'P *' }) | Select-Object -First 1
    Log "cover check at tallyBlankPoint $coverStart to $(Stamp): $pLine"
    if ($pLine -notmatch 'image=tally\.exe') { Log "the window at tallyBlankPoint is not Tally's: no click into Tally"; return $false }
    while (((NowUtc) - $t0).TotalSeconds -lt 16) { Start-Sleep -Milliseconds 20 }
    [void](ClickAt 'tallyBlankPoint' $tallyBlankPoint[0] $tallyBlankPoint[1])
    if ([RB]::GetForegroundWindow() -ne $script:tallyHandle) { Log "click into Tally missed (Tally not in front): the cell is void"; $script:cellVoid = $true; return $false }
    return $true
}
function WaitEnd {
    $waitStart = Get-Date
    while (((Get-Date) - $waitStart).TotalSeconds -lt 150) { if ((Get-Content "$evDir\$Cell.txt" -ErrorAction SilentlyContinue) -match 'exit_code=') { break }; Start-Sleep -Milliseconds 200 }
    $waitStart = Get-Date
    while (((Get-Date) - $waitStart).TotalSeconds -lt 20) { if ((Get-Content "$evDir\trace-$Cell.txt" -ErrorAction SilentlyContinue) -match '^END ') { break }; Start-Sleep -Milliseconds 200 }
    Log "spawn file: $((Get-Content "$evDir\$Cell.txt" -ErrorAction SilentlyContinue) -join ' || ')"
    Log "stdout bytes: $(if (Test-Path "$evDir\$Cell.stdout") { (Get-Item "$evDir\$Cell.stdout").Length } else { 'no file' })"
    Log "stderr whole: $(if (Test-Path "$evDir\$Cell.stderr") { (Get-Content "$evDir\$Cell.stderr") -join ' || ' } else { 'no file' })"
    Log "trace GONE/END: $((Get-Content "$evDir\trace-$Cell.txt" -ErrorAction SilentlyContinue | Where-Object { $_ -match '^(GONE|END) ' }) -join ' | ')"
}
function AfterClickReads([string]$label) {
    [void](ReadR "$label-now"); $boxHandle = BoxHandle
    Log "after $label $(Stamp): box $(if ($boxHandle -ne [IntPtr]::Zero) { "open iconic=$([int][RB]::IsIconic($boxHandle))" } else { 'gone' }); thumbnails: $(([RB]::Thumbs()) -join ' | ')"
    Start-Sleep -Seconds 3; [void](ReadR "$label-3s"); $boxHandle = BoxHandle
    Log "after $label +3 s $(Stamp): box $(if ($boxHandle -ne [IntPtr]::Zero) { "open iconic=$([int][RB]::IsIconic($boxHandle))" } else { 'gone' })"
}

$script:tallyHandle = [RB]::ByTitle('TallyPrime:9001')
$script:consoleHandles = @{}; $script:consoleRecords = @(); $script:minimisedOnce = @{}; $script:cellVoid = $false
Log "$Cell runner pid $PID $(Stamp); dpi_context=$dpiState; Tally window $(HexOf $script:tallyHandle); $(FgText)"

if ($Cell -eq 'n1') {
    [void](ReadR 'idle'); CapK 'idle'
    $idleRuns = @(ButtonRuns 'idle'); Log "N1 buttons (PrintWindow, physical x at y 1055): $($idleRuns -join ' ')"; $idleRuns | Set-Content -Encoding ascii "$evDir\n1-buttons.txt"
    Log "$Cell DONE $(Stamp)"; exit 0
}

$minimisedWay = $Cell -in @('pmin', 'c1', 'c2', 'c3')
$traceSeconds = if ($Cell -eq 'n2') { 40 } else { 160 }
$traceArgs = "-NoProfile -ExecutionPolicy Bypass -File $reqDir\box-trace-early.ps1 -Title `"$boxTitle`" -Out $evDir\trace-$Cell.txt -Seconds $traceSeconds"
if ($minimisedWay) { $traceProc = Start-Process powershell -ArgumentList $traceArgs -WindowStyle Minimized -PassThru } else { $traceProc = Start-Process powershell -ArgumentList $traceArgs -PassThru }
$null = $traceProc.Handle
Start-Sleep -Milliseconds 1500
$traceWindow = @([RB]::OfClassAndPids('ConsoleWindowClass', [uint32[]]@($traceProc.Id))) | Select-Object -First 1
if ($traceWindow) { $script:consoleHandles['trace console'] = $traceWindow }
$script:consoleRecords += "trace console pid $($traceProc.Id) window $(if ($traceWindow) { HexOf $traceWindow } else { 'not found' })"
Log "trace pre-armed $(Stamp) pid $($traceProc.Id)$(if ($minimisedWay) { ' (started minimised)' }): powershell $traceArgs"
Start-Sleep -Milliseconds 500
[void](ClickAt 'tallyTitlePoint' $tallyTitlePoint[0] $tallyTitlePoint[1]); Log "Tally in front: $([RB]::GetForegroundWindow() -eq $script:tallyHandle)"

$cmdFile = "$wordDir\$Cell.cmd"
if ($Cell -eq 'n2') { $cmdText = "powershell -NoProfile -Command Start-Sleep -Seconds 30`r`n" }
else { $cmdText = "powershell -NoProfile -Command Start-Sleep -Seconds 8`r`npowershell -NoProfile -ExecutionPolicy Bypass -File $reqDir\spawn-parent.ps1 -Target child -ChildExe $boxExe -InputFile $reqDir\confirm-input.txt -Stdio files -Out $evDir\$Cell.txt -TimeoutSeconds 120`r`n" }
[IO.File]::WriteAllText($cmdFile, $cmdText, (New-Object Text.ASCIIEncoding))
$null = Start-Process explorer.exe -ArgumentList "`"$cmdFile`"" -PassThru; $cellStart = NowUtc
Log "opened $(Stamp): explorer.exe `"$cmdFile`"; the .cmd holds: $($cmdText.Trim() -replace "`r`n", ' | ')"
$cmdProcId = 0; $cmdWindow = $null; $findUntil = (Get-Date).AddSeconds(3)
while ((Get-Date) -lt $findUntil -and -not $cmdWindow) {
    $cmdProc = @(Get-CimInstance Win32_Process -Filter "Name='cmd.exe'" | Where-Object { $_.CommandLine -and $_.CommandLine.Contains("$Cell.cmd") }) | Select-Object -First 1
    if ($cmdProc) {
        $cmdProcId = [uint32]$cmdProc.ProcessId; $kidIds = @(Get-CimInstance Win32_Process -Filter "ParentProcessId=$cmdProcId" | ForEach-Object { [uint32]$_.ProcessId })
        $cmdWindow = @([RB]::OfClassAndPids('ConsoleWindowClass', [uint32[]](@($cmdProcId) + $kidIds))) | Select-Object -First 1
    }
    if (-not $cmdWindow) { Start-Sleep -Milliseconds 100 }
}
if ($cmdWindow) { $script:consoleHandles['cmd console'] = $cmdWindow }
$script:consoleRecords += "cmd console pid $cmdProcId window $(if ($cmdWindow) { HexOf $cmdWindow } else { 'not found' })"
RecordConsoles
if ($minimisedWay) {
    while (((NowUtc) - $cellStart).TotalSeconds -lt 2.5) { Start-Sleep -Milliseconds 50 }
    $minimisedBefore = MinimiseConsole 'cmd console' 'P-min: during the 8 s sleep, before T0 and before the second title click'
}
while (((NowUtc) - $cellStart).TotalSeconds -lt 4) { Start-Sleep -Milliseconds 50 }
[void](ClickAt 'tallyTitlePoint' $tallyTitlePoint[0] $tallyTitlePoint[1]); Log "the second title click, $([Math]::Round(((NowUtc) - $cellStart).TotalSeconds, 2)) s after the open; Tally in front: $([RB]::GetForegroundWindow() -eq $script:tallyHandle)"
if ($minimisedWay -and -not $minimisedBefore) { [void](MinimiseConsole 'cmd console' 'P-min: after the second title click, the console was in front before it') }

if ($Cell -eq 'n2') {
    while (((NowUtc) - $cellStart).TotalSeconds -lt 12) { Start-Sleep -Milliseconds 50 }
    [void](ReadR 'p12'); CapK 'p12'
    $helperRuns = @(ButtonRuns 'p12'); Log "N2 buttons (PrintWindow, physical x at y 1055): $($helperRuns -join ' ')"; $helperRuns | Set-Content -Encoding ascii "$evDir\n2-buttons.txt"
    $waitStart = Get-Date
    while (((Get-Date) - $waitStart).TotalSeconds -lt 50) { if (((Get-Content "$evDir\trace-n2.txt" -ErrorAction SilentlyContinue) -match '^END ') -and -not (Get-Process -Id $cmdProcId -ErrorAction SilentlyContinue)) { break }; Start-Sleep -Milliseconds 200 }
    Log "trace end: $((Get-Content "$evDir\trace-n2.txt" -ErrorAction SilentlyContinue | Where-Object { $_ -match '^(END|FIRST) ' }) -join ' | '); cmd process running: $([bool](Get-Process -Id $cmdProcId -ErrorAction SilentlyContinue))"
    Log "$Cell DONE $(Stamp)"; exit 0
}

$t0 = $null; $t0Until = (Get-Date).AddSeconds(60)
while ((Get-Date) -lt $t0Until -and -not $t0) {
    $spawnLine = Get-Content "$evDir\$Cell.txt" -ErrorAction SilentlyContinue | Where-Object { $_ -match 'child_pid=' } | Select-Object -First 1
    if ($spawnLine -and $spawnLine -match '^\W*SPAWN (\S+) ') { $t0 = [DateTime]::Parse($Matches[1]).ToUniversalTime(); Log "$Cell-T0 $($Matches[1]) from: $spawnLine" }
    Start-Sleep -Milliseconds 50
}
if (-not $t0) { Log "no child_pid line in 60 s: nothing more clicked"; WaitEnd; exit 20 }
function SinceT0 { [Math]::Round(((NowUtc) - $t0).TotalSeconds, 2) }
while (((NowUtc) - $t0).TotalSeconds -lt 6) { Start-Sleep -Milliseconds 20 }
[void](ReadR 't0p6'); CapK 't0p6'; ConsoleStates 'T0+6'
$boxButtonPoint = $null
if ($Cell -ne 'p') { $boxButtonPoint = LocateBoxButton 't0p6' }
$boxWait = (Get-Date).AddSeconds(15 - 6)
while ((Get-Date) -lt $boxWait -and (BoxHandle) -eq [IntPtr]::Zero) { Start-Sleep -Milliseconds 100 }
if ((BoxHandle) -eq [IntPtr]::Zero) { Log "no box by T0+15 $(Stamp): ended nothing"; WaitEnd; Log "$Cell DONE $(Stamp)"; exit 21 }

if ($Cell -eq 'c2') {
    while (((NowUtc) - $t0).TotalSeconds -lt 12) { Start-Sleep -Milliseconds 20 }
    [void](ReadR 't0p12'); $inFront = [RB]::GetForegroundWindow() -eq (BoxHandle); Log "T0+12 the box in front: $inFront"
    if (-not $inFront) { Log "C2 not reached"; AnswerNo 'C2 not reached'; WaitEnd; Log "$Cell DONE $(Stamp)"; exit 0 }
    while (((NowUtc) - $t0).TotalSeconds -lt 16) { Start-Sleep -Milliseconds 20 }
    if (-not $boxButtonPoint) { AnswerNo "the box's button is not identified"; WaitEnd; Log "$Cell DONE $(Stamp)"; exit 0 }
    if (-not (Gate 'gate1' $boxButtonPoint)) { AnswerNo 'gate not met'; WaitEnd; Log "$Cell DONE $(Stamp)"; exit 0 }
    $firstClick = ClickAt 'boxButtonPoint' $boxButtonPoint[0] $boxButtonPoint[1]; Log "first taskbar click at T0+$(SinceT0) s"
    AfterClickReads 'click1'
    $boxHandle = BoxHandle
    if ($boxHandle -ne [IntPtr]::Zero -and [RB]::IsIconic($boxHandle)) {
        Start-Sleep -Seconds 3
        if (-not (Gate 'gate2' $boxButtonPoint)) { Log "second gate not met: nothing clicked; the box stays minimised"; WaitEnd; Log "$Cell DONE $(Stamp)"; exit 0 }
        $secondClick = ClickAt 'boxButtonPoint' $boxButtonPoint[0] $boxButtonPoint[1]; Log "second taskbar click at T0+$(SinceT0) s"
        AfterClickReads 'click2'; $boxHandle = BoxHandle
        if ($boxHandle -ne [IntPtr]::Zero -and -not [RB]::IsIconic($boxHandle)) { AnswerNo 'open and visible after the second click' }
        elseif ($boxHandle -ne [IntPtr]::Zero) { Log "still minimised after the second click: nothing more clicked; the 120 s bound ends it" }
        else { Log "the box is gone after the second click" }
    } elseif ($boxHandle -ne [IntPtr]::Zero) { AnswerNo 'open and not minimised after the first click' }
    else { Log "the box is gone after the first click" }
    WaitEnd; Log "$Cell DONE $(Stamp)"; exit 0
}

# p, pmin, c1, c3: the click into Tally at T0+16 (+-2 s); its cover check starts at T0+13
while (((NowUtc) - $t0).TotalSeconds -lt 13) { Start-Sleep -Milliseconds 20 }
$intoTally = ClickIntoTally; $clickTime = NowUtc; Log "click into Tally done: $intoTally at T0+$(SinceT0) s"
if (-not $intoTally) { AnswerNo 'no click into Tally, or it missed'; WaitEnd; Log "$Cell DONE $(Stamp)"; exit 0 }
while (((NowUtc) - $clickTime).TotalSeconds -lt 3) { Start-Sleep -Milliseconds 20 }
[void](ReadR 'c3'); CapK 'c3'; ConsoleStates 'C+3'
if ($Cell -in @('p', 'pmin')) {
    if ($Cell -eq 'pmin') { [void](LocateBoxButton 'c3') }
    while (((NowUtc) - $clickTime).TotalSeconds -lt 10) { Start-Sleep -Milliseconds 20 }
    AnswerNo 'about C+10'; WaitEnd; Log "$Cell DONE $(Stamp)"; exit 0
}
while (((NowUtc) - $clickTime).TotalSeconds -lt 6) { Start-Sleep -Milliseconds 20 }
if (-not $boxButtonPoint) { AnswerNo "the box's button is not identified"; WaitEnd; Log "$Cell DONE $(Stamp)"; exit 0 }
if (-not (Gate 'gate1' $boxButtonPoint)) { AnswerNo 'gate not met'; WaitEnd; Log "$Cell DONE $(Stamp)"; exit 0 }
if ($Cell -eq 'c1') {
    $firstClick = ClickAt 'boxButtonPoint' $boxButtonPoint[0] $boxButtonPoint[1]; Log "taskbar click at T0+$(SinceT0) s"
    AfterClickReads 'click1'
    $thumbList = @([RB]::Thumbs())
    if ((BoxHandle) -ne [IntPtr]::Zero -and $thumbList.Count -gt 0 -and [RB]::GetForegroundWindow() -ne (BoxHandle)) {
        $previewShot = "$shotDir\$Cell-thumbs.png"; FullCopy $previewShot
        $word = WaitWord 'preview' "several thumbnails? give 'x y' (physical) of the preview of the ComplyEaze Bridge thumbnail, or nogo; copy $previewShot"
        if ($word[0] -eq 'go' -and $word[1] -match '^(\d+) (\d+)$') { $previewPoint = @([int]$Matches[1], [int]$Matches[2]); [void](ClickAt 'previewPoint' $previewPoint[0] $previewPoint[1]); AfterClickReads 'preview' }
    }
    if ((BoxHandle) -ne [IntPtr]::Zero) { AnswerNo 'the box still open' } else { Log "the box is gone: nothing more clicked" }
    WaitEnd; Log "$Cell DONE $(Stamp)"; exit 0
}
# c3
$fitBefore = StderrText; Log "before the right-click: $fitBefore"
[void](ClickAt 'boxButtonPoint' $boxButtonPoint[0] $boxButtonPoint[1] -RightButton); Log "right-click at T0+$(SinceT0) s"
Start-Sleep -Milliseconds 1200
$menuShot = "$shotDir\$Cell-menu.png"; FullCopy $menuShot; Log "menu copy $Cell-menu.png $(Stamp)"
$word = WaitWord 'menu' "give 'x y' (physical) of the item labelled Close window, or nogo; copy $menuShot"
$closed = $false
if ($word[0] -eq 'go' -and $word[1] -match '^(\d+) (\d+)$') {
    $menuItemPoint = @([int]$Matches[1], [int]$Matches[2])
    $pLine = ReadR 'menu-item' $menuItemPoint[0] $menuItemPoint[1]
    if ($pLine -and $pLine -notmatch 'image=(bridge_mcp|tally|powershell|cmd|conhost)\.exe') {
        $closeClick = ClickAt 'menuItemPoint' $menuItemPoint[0] $menuItemPoint[1]; $closed = $true
        AfterClickReads 'close'
    } else { Log "the window at the item's point fails the check: $pLine" }
} else { Log "no Close window item given: $($word -join ' ')" }
if (-not $closed) { [void](ClickAt 'tallyBlankPoint' $tallyBlankPoint[0] $tallyBlankPoint[1]); Log "one click on tallyBlankPoint to close the menu"; Start-Sleep -Milliseconds 800; AnswerNo 'the close was not made' }
elseif ((BoxHandle) -ne [IntPtr]::Zero) { Log "the box is still open after the close: answered No"; AnswerNo 'still open after Close window' }
WaitEnd; Log "$Cell DONE $(Stamp)"; exit 0
