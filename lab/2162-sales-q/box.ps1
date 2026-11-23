# Lab helper (0.7.0 RC sitting, not product code): waits for one ComplyEaze Bridge box, records it, and clicks the button
# the lab names. DPI-aware. From the start it polls every 0.25 s for a visible top-level window whose title starts with
# "ComplyEaze Bridge" owned by a bridge_mcp.exe; when seen it logs the time, its rect, its extended style (topmost bit),
# its z-order place against Tally's window, the foreground window, every child control's class and text, and takes one
# whole-screen capture. It then waits for <RowDir>\box\<Tag>.answer holding Yes or No and clicks that button at the
# centre of its rectangle by SetCursorPos + mouse_event (no key is pressed). With no answer file by -AnswerBy seconds after
# the box was seen it clicks No. A box not seen within -SeeWithin seconds: one whole-screen capture, no click, exit 3.
param([Parameter(Mandatory = $true)][string]$RowDir, [Parameter(Mandatory = $true)][string]$Tag,
      [int]$SeeWithin = 20, [int]$AnswerBy = 70)
Add-Type -AssemblyName System.Drawing
Add-Type @'
using System; using System.Text; using System.Collections.Generic; using System.Runtime.InteropServices;
public class BX {
 public delegate bool EP(IntPtr h, IntPtr l);
 [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
 [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
 [DllImport("user32.dll")] public static extern bool EnumWindows(EP f, IntPtr l);
 [DllImport("user32.dll")] public static extern bool EnumChildWindows(IntPtr p, EP f, IntPtr l);
 [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
 [DllImport("user32.dll", CharSet = CharSet.Unicode)] public static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
 [DllImport("user32.dll", CharSet = CharSet.Unicode)] public static extern int GetClassName(IntPtr h, StringBuilder s, int n);
 [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint p);
 [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
 [DllImport("user32.dll")] public static extern int GetWindowLong(IntPtr h, int i);
 [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
 [DllImport("user32.dll")] public static extern int GetSystemMetrics(int i);
 [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
 [DllImport("user32.dll")] public static extern void mouse_event(uint f, uint x, uint y, uint d, UIntPtr e);
 public static string Txt(IntPtr h) { var s = new StringBuilder(4096); GetWindowText(h, s, 4096); return s.ToString(); }
 public static string Cls(IntPtr h) { var s = new StringBuilder(256); GetClassName(h, s, 256); return s.ToString(); }
 public static uint PidOf(IntPtr h) { uint p; GetWindowThreadProcessId(h, out p); return p; }
 public static RECT Rect(IntPtr h) { RECT r; GetWindowRect(h, out r); return r; }
 public static List<IntPtr> Tops() { var o = new List<IntPtr>(); EnumWindows((h,l)=>{ if (IsWindowVisible(h)) o.Add(h); return true; }, IntPtr.Zero); return o; }
 public static List<IntPtr> Kids(IntPtr p) { var o = new List<IntPtr>(); EnumChildWindows(p, (h,l)=>{ o.Add(h); return true; }, IntPtr.Zero); return o; }
 public static void LeftAt(int x, int y) { SetCursorPos(x, y); mouse_event(2,0,0,0,UIntPtr.Zero); mouse_event(4,0,0,0,UIntPtr.Zero); }
}
'@
[void][BX]::SetProcessDPIAware()
$boxDir = Join-Path $RowDir 'box'; if (-not (Test-Path $boxDir)) { New-Item -ItemType Directory $boxDir | Out-Null }
$boxLog = Join-Path $boxDir "$Tag-box.txt"; $answerFile = Join-Path $boxDir "$Tag.answer"
function Stamp { (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
function Log([string]$text) { Add-Content -Encoding utf8 $boxLog "$(Stamp) $text" }
function Shot([string]$name) {
    $sw = [BX]::GetSystemMetrics(0); $sh = [BX]::GetSystemMetrics(1)
    $bmp = New-Object System.Drawing.Bitmap $sw, $sh; $gfx = [System.Drawing.Graphics]::FromImage($bmp)
    $gfx.CopyFromScreen(0, 0, 0, 0, $bmp.Size); $gfx.Dispose()
    $out = Join-Path $boxDir "$Tag-$name.png"; $bmp.Save($out, [System.Drawing.Imaging.ImageFormat]::Png); $bmp.Dispose()
    $hash = (Get-FileHash $out).Hash.ToLower(); Log "capture $Tag-$name.png ${sw}x${sh} $((Get-Item $out).Length) B sha256 $hash"
}
function FgText { $fg = [BX]::GetForegroundWindow(); $fgPid = [BX]::PidOf($fg); $fgName = (Get-Process -Id $fgPid -ErrorAction SilentlyContinue).ProcessName; "foreground 0x$($fg.ToInt64().ToString('x')) pid $fgPid $fgName title '$([BX]::Txt($fg))'" }
$bridgePids = @(); $tallyHandle = [IntPtr][int64](Get-Process tally | Select-Object -First 1).MainWindowHandle
Log "watch start: tally window 0x$($tallyHandle.ToInt64().ToString('x')); see within $SeeWithin s; answer by $AnswerBy s"
$t0 = Get-Date; $boxHandle = [IntPtr]::Zero
while (((Get-Date) - $t0).TotalSeconds -lt $SeeWithin) {
    $bridgePids = @(Get-Process bridge_mcp -ErrorAction SilentlyContinue | ForEach-Object { [uint32]$_.Id })
    foreach ($h in [BX]::Tops()) { if ([BX]::Txt($h).StartsWith('ComplyEaze Bridge') -and ($bridgePids -contains [BX]::PidOf($h))) { $boxHandle = $h; break } }
    if ($boxHandle -ne [IntPtr]::Zero) { break }
    Start-Sleep -Milliseconds 250
}
if ($boxHandle -eq [IntPtr]::Zero) { Log "NO BOX seen within $SeeWithin s"; Shot 'nobox'; exit 3 }
$seenAt = Get-Date; Log "BOX SEEN 0x$($boxHandle.ToInt64().ToString('x')) pid $([BX]::PidOf($boxHandle)) title '$([BX]::Txt($boxHandle))' class $([BX]::Cls($boxHandle))"
$r = [BX]::Rect($boxHandle); $ex = [BX]::GetWindowLong($boxHandle, -20)
$tops = [BX]::Tops(); $boxPlace = $tops.IndexOf($boxHandle); $tallyPlace = $tops.IndexOf($tallyHandle)
Log "rect $($r.L),$($r.T),$($r.R),$($r.B) (screen $([BX]::GetSystemMetrics(0))x$([BX]::GetSystemMetrics(1))); exstyle 0x$($ex.ToString('x')) topmost $([int](($ex -band 8) -ne 0)); z-order place box $boxPlace tally $tallyPlace (lower = nearer the top); above tally $($boxPlace -lt $tallyPlace)"
Log (FgText)
foreach ($kid in [BX]::Kids($boxHandle)) { $kr = [BX]::Rect($kid); Log "child $([BX]::Cls($kid)) rect $($kr.L),$($kr.T),$($kr.R),$($kr.B) text: $([BX]::Txt($kid))" }
Shot 'open'
Log "WAITING for $answerFile"
$choice = $null
while (((Get-Date) - $seenAt).TotalSeconds -lt $AnswerBy) {
    if (-not [BX]::IsWindowVisible($boxHandle)) { Log "box no longer visible before any click"; Shot 'gone'; exit 4 }
    if (Test-Path $answerFile) { $choice = (Get-Content $answerFile -Raw).Trim(); break }
    Start-Sleep -Milliseconds 250
}
if ($choice -ne 'Yes' -and $choice -ne 'No') { Log "no valid answer file by $AnswerBy s (read '$choice'): No"; $choice = 'No' }
$btn = [IntPtr]::Zero; foreach ($kid in [BX]::Kids($boxHandle)) { if ([BX]::Cls($kid) -eq 'Button' -and ([BX]::Txt($kid) -replace '&', '') -eq $choice) { $btn = $kid } }
if ($btn -eq [IntPtr]::Zero) { Log "button $choice not found: no click"; Shot 'nobutton'; exit 5 }
$br = [BX]::Rect($btn); $cx = [int](($br.L + $br.R) / 2); $cy = [int](($br.T + $br.B) / 2)
Log "before click: $(FgText)"
[BX]::LeftAt($cx, $cy); $clickAt = Get-Date
Log "CLICKED $choice at $cx,$cy (physical) $(Stamp); seen-to-click $([Math]::Round(($clickAt - $seenAt).TotalSeconds, 1)) s"
Start-Sleep -Milliseconds 800
Log "after click: box visible $([BX]::IsWindowVisible($boxHandle)); $(FgText)"
exit 0
