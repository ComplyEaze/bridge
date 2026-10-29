# box-trace-early.ps1 - a copy of box-trace.ps1 (row 1780-D) with ONE change, made for row 1780-F: the window block is taken and written
# BEFORE the process lookup, so the first block is within about 50 ms of FIRST (box-trace.ps1 hashed the 31.8 MB exe first, which put
# its first block 1.7 to 2.3 s after FIRST). The PROC lines are written after that first block. Nothing else differs.
# box-trace.ps1 - follows one box window and every top-level window of its process, by handle.
# Read-only: it changes no window and sends no key or click. Written on a Mac and NOT RUN anywhere.
# Its C# part was compiled on that Mac with Mono's mcs (no errors); the PowerShell around it was not run.
# Run it in Windows PowerShell 5.1 opened from the Start menu, BEFORE the box is asked for:
#   powershell -NoProfile -ExecutionPolicy Bypass -File box-trace.ps1 -Title "ComplyEaze Bridge" -Out C:\bridge-lab\1780d\evidence\trace-c.txt -Seconds 90
# It waits for the first visible window whose title contains -Title, then every -IntervalMs:
#   reads every top-level window of that window's process (visible or not), by handle;
#   writes a block only when something in it changed, and otherwise one heartbeat line a second.
# One block line per window:
#   W <utc> hwnd=<hex> class=<class> visible=<0|1> exstyle=<hex> topmost=<0|1> thread=<id>
#     owner=<hex|0> owner_class=<class> owner_pid=<pid> owner_exstyle=<hex> owner_topmost=<0|1> title=<title>
# Each block starts with: T <utc> fg_hwnd=<hex> fg_pid=<pid> box_pid=<pid> windows=<n>
# When the box process is first seen it writes its image path, SHA-256, command line and parent pid.
# It ends at -Seconds, or 3 s after the last window of the box process has gone.
param(
    [string]$Title = "ComplyEaze Bridge",
    [Parameter(Mandatory = $true)][string]$Out,
    [int]$Seconds = 90,
    [int]$IntervalMs = 50
)
Add-Type -TypeDefinition @"
using System;
using System.Collections.Generic;
using System.Text;
using System.Runtime.InteropServices;
public static class BoxTrace {
    delegate bool EnumProc(IntPtr h, IntPtr l);
    [DllImport("user32.dll")] static extern bool EnumWindows(EnumProc p, IntPtr l);
    [DllImport("user32.dll")] static extern bool IsWindowVisible(IntPtr h);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern int GetClassName(IntPtr h, StringBuilder s, int n);
    [DllImport("user32.dll")] static extern int GetWindowLong(IntPtr h, int index);
    [DllImport("user32.dll")] static extern IntPtr GetWindow(IntPtr h, uint cmd);
    [DllImport("user32.dll")] static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
    [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
    const int GWL_EXSTYLE = -20;
    const int WS_EX_TOPMOST = 0x8;
    const uint GW_OWNER = 4;
    static string Text(IntPtr h) { var s = new StringBuilder(512); GetWindowText(h, s, s.Capacity); return s.ToString(); }
    static string Cls(IntPtr h) { var s = new StringBuilder(256); GetClassName(h, s, s.Capacity); return s.ToString(); }
    public static uint PidOf(IntPtr h) { uint pid; GetWindowThreadProcessId(h, out pid); return pid; }
    // The process of the first visible top-level window whose title contains part, or 0.
    public static uint FindPid(string part) {
        uint found = 0;
        EnumWindows(delegate (IntPtr h, IntPtr l) {
            if (!IsWindowVisible(h)) return true;
            if (Text(h).IndexOf(part, StringComparison.OrdinalIgnoreCase) < 0) return true;
            found = PidOf(h);
            return false;
        }, IntPtr.Zero);
        return found;
    }
    // One line per top-level window of pid, in z-order from the top.
    public static List<string> Windows(uint pid) {
        var lines = new List<string>();
        EnumWindows(delegate (IntPtr h, IntPtr l) {
            uint p; uint thread = GetWindowThreadProcessId(h, out p);
            if (p != pid) return true;
            int ex = GetWindowLong(h, GWL_EXSTYLE);
            IntPtr owner = GetWindow(h, GW_OWNER);
            string ownerPart = "owner=0";
            if (owner != IntPtr.Zero) {
                int oex = GetWindowLong(owner, GWL_EXSTYLE);
                ownerPart = "owner=0x" + owner.ToInt64().ToString("x") + " owner_class=" + Cls(owner) +
                    " owner_pid=" + PidOf(owner) + " owner_exstyle=0x" + oex.ToString("x8") +
                    " owner_topmost=" + ((oex & WS_EX_TOPMOST) != 0 ? 1 : 0);
            }
            lines.Add("hwnd=0x" + h.ToInt64().ToString("x") + " class=" + Cls(h) +
                " visible=" + (IsWindowVisible(h) ? 1 : 0) + " exstyle=0x" + ex.ToString("x8") +
                " topmost=" + ((ex & WS_EX_TOPMOST) != 0 ? 1 : 0) + " thread=" + thread + " " + ownerPart +
                " title=" + Text(h));
            return true;
        }, IntPtr.Zero);
        return lines;
    }
}
"@
function Stamp { (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ss.fffZ") }
function Emit([string]$line) { [System.IO.File]::AppendAllText($Out, $line + "`r`n", [System.Text.Encoding]::UTF8) }
Emit ("START " + (Stamp) + " title_part=" + $Title + " interval_ms=" + $IntervalMs + " seconds=" + $Seconds)
$deadline = (Get-Date).AddSeconds($Seconds)
$boxPid = [uint32]0
$previous = ""
$lastWrite = Get-Date
$goneSince = $null
$procDone = $false
while ((Get-Date) -lt $deadline) {
    if ($boxPid -eq 0) {
        $boxPid = [BoxTrace]::FindPid($Title)
        if ($boxPid -ne 0) {
            Emit ("FIRST " + (Stamp) + " box_pid=" + $boxPid)
        } else {
            Start-Sleep -Milliseconds $IntervalMs
            continue
        }
    }
    $fg = [BoxTrace]::GetForegroundWindow()
    $lines = [BoxTrace]::Windows($boxPid)
    $body = ($lines -join "`n")
    $head = "fg_hwnd=0x" + $fg.ToInt64().ToString("x") + " fg_pid=" + [BoxTrace]::PidOf($fg) + " box_pid=" + $boxPid + " windows=" + $lines.Count
    $now = Get-Date
    if (($head + $body) -ne $previous) {
        $t = Stamp
        Emit ("T " + $t + " " + $head)
        foreach ($l in $lines) { Emit ("W " + $t + " " + $l) }
        $previous = $head + $body
        $lastWrite = $now
    } elseif (($now - $lastWrite).TotalMilliseconds -ge 1000) {
        Emit ("H " + (Stamp) + " unchanged")
        $lastWrite = $now
    }
    if (-not $procDone) {
        # The process lookup, after the first block has been written.
        $procDone = $true
        try {
            $p = Get-CimInstance Win32_Process -Filter ("ProcessId=" + $boxPid)
            Emit ("PROC path=" + $p.ExecutablePath + " parent_pid=" + $p.ParentProcessId)
            Emit ("PROC command_line=" + $p.CommandLine)
            if ($p.ExecutablePath) {
                $h = Get-FileHash -Algorithm SHA256 -LiteralPath $p.ExecutablePath
                Emit ("PROC sha256=" + $h.Hash.ToLower() + " bytes=" + (Get-Item -LiteralPath $p.ExecutablePath).Length)
            }
        } catch { Emit ("PROC read_failed " + $_.Exception.Message) }
    }
    if ($lines.Count -eq 0) {
        if ($goneSince -eq $null) { $goneSince = $now }
        if (($now - $goneSince).TotalSeconds -ge 3) { Emit ("GONE " + (Stamp)); break }
    } else {
        $goneSince = $null
    }
    Start-Sleep -Milliseconds $IntervalMs
}
Emit ("END " + (Stamp))
