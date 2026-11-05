# Lab helper (852-C2, not product code): step 4b's screen work. Waits for box.ps1's log of <Tag> to record its click; on
# "CLICKED Yes" it clicks Tally's title band once (SetCursorPos + mouse_event, physical pixels, DPI-aware), then presses
# Down once a second (Up instead after -MaxDown Downs, the end of the Day Book) by keybd_event, until the post call's answer
# file (agent-run\answers\*-<Tag>.json) exists. Before each press it checks that Tally's window is in front; if not, it
# skips that press and logs it (keys are never sent to another window). Every click, press and skip is logged, UTC with
# milliseconds, to box\<Tag>-keys.txt. On "CLICKED No", no box or any other end of box.ps1's log, it presses nothing.
param([Parameter(Mandatory = $true)][string]$RowDir, [Parameter(Mandatory = $true)][string]$Tag, [int]$MaxDown = 81, [int]$WaitClick = 200)
Add-Type @'
using System; using System.Runtime.InteropServices;
public class KX {
 [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
 [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
 [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
 [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint p);
 [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
 [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
 [DllImport("user32.dll")] public static extern void mouse_event(uint f, uint x, uint y, uint d, UIntPtr e);
 [DllImport("user32.dll")] public static extern void keybd_event(byte vk, byte scan, uint f, UIntPtr e);
 public static uint FgPid() { uint p; GetWindowThreadProcessId(GetForegroundWindow(), out p); return p; }
}
'@
[void][KX]::SetProcessDPIAware()
$boxLog = Join-Path $RowDir "box\$Tag-box.txt"; $keyLog = Join-Path $RowDir "box\$Tag-keys.txt"; $answers = Join-Path $RowDir 'agent-run\answers'
function Stamp { (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
function Log([string]$text) { Add-Content -Encoding utf8 $keyLog "$(Stamp) $text" }
function ReadBox { try { [IO.File]::ReadAllText($boxLog) } catch { '' } }
function Answered { @(Get-ChildItem $answers -Filter "*-$Tag.json" -ErrorAction SilentlyContinue).Count -gt 0 }
$tally = Get-Process tally | Where-Object { $_.MainWindowHandle -ne 0 } | Select-Object -First 1
Log "keys start: tally pid $($tally.Id); waiting for box.ps1's click (up to $WaitClick s); Down once a second, Up after $MaxDown Downs"
$t0 = Get-Date; $clicked = $null
while (((Get-Date) - $t0).TotalSeconds -lt $WaitClick) {
    $t = ReadBox
    if ($t -match 'CLICKED Yes') { $clicked = 'Yes'; break }
    if ($t -match 'CLICKED No|NO BOX seen|no longer visible|not found: no click') { $clicked = 'other'; break }
    if (Answered) { $clicked = 'answered first'; break }
    Start-Sleep -Milliseconds 100
}
if ($clicked -ne 'Yes') { Log "no Yes click seen ($clicked): no key pressed"; exit 2 }
Log "box.ps1 logged CLICKED Yes"
$r = New-Object KX+RECT; [void][KX]::GetWindowRect($tally.MainWindowHandle, [ref]$r)
$cx = [int]($r.L + ($r.R - $r.L) * 0.77); $cy = [int]($r.T + 26)
Start-Sleep -Milliseconds 300
[void][KX]::SetCursorPos($cx, $cy); [KX]::mouse_event(2, 0, 0, 0, [UIntPtr]::Zero); [KX]::mouse_event(4, 0, 0, 0, [UIntPtr]::Zero)
Start-Sleep -Milliseconds 200
Log "CLICK on Tally's title band at $cx,$cy (physical; window $($r.L),$($r.T),$($r.R),$($r.B)); foreground pid now $([KX]::FgPid())"
$downs = 0; $ups = 0; $skips = 0; $next = Get-Date
while (-not (Answered)) {
    if ((Get-Date) -lt $next) { Start-Sleep -Milliseconds 20; continue }
    $next = $next.AddSeconds(1)
    if ([KX]::FgPid() -ne [uint32]$tally.Id) { $skips++; Log "SKIP: Tally not in front (foreground pid $([KX]::FgPid()))"; continue }
    if ($downs -lt $MaxDown) { $vk = 0x28; $downs++; $name = "Down $downs" } else { $vk = 0x26; $ups++; $name = "Up $ups" }
    [KX]::keybd_event([byte]$vk, 0, 1, [UIntPtr]::Zero); [KX]::keybd_event([byte]$vk, 0, 3, [UIntPtr]::Zero)
    Log "KEY $name"
}
Log "answer file seen: stop. Downs $downs, Ups $ups, skipped $skips"
exit 0
