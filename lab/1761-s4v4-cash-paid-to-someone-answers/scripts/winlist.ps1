# Lab helper (row 1761-S4 v4, new; not product code): records Bridge's approval window from a window listing only. It never
# clicks, presses a key, moves the mouse, focuses or captures anything, and sends nothing to any window. Every 0.25 s for
# -Seconds it lists the visible top-level windows (EnumWindows) and logs, to evidence/window-<Tag>.txt with UTC times: each
# window whose title starts with "ComplyEaze Bridge" when it first appears (handle, title, class, owning PID, that process's
# name, path and parent PID, its rectangle from GetWindowRect, the topmost bit) and when it stops being listed. Run it in the
# background before step 4.2's post. Usage: powershell -NoProfile -File winlist.ps1 -Tag s4 [-Seconds 200]
param([Parameter(Mandatory = $true)][string]$Tag, [int]$Seconds = 200)
Add-Type @'
using System; using System.Text; using System.Collections.Generic; using System.Runtime.InteropServices;
public class WL {
 public delegate bool EP(IntPtr h, IntPtr l);
 [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
 [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
 [DllImport("user32.dll")] public static extern bool EnumWindows(EP f, IntPtr l);
 [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
 [DllImport("user32.dll", CharSet = CharSet.Unicode)] public static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
 [DllImport("user32.dll", CharSet = CharSet.Unicode)] public static extern int GetClassName(IntPtr h, StringBuilder s, int n);
 [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint p);
 [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
 [DllImport("user32.dll")] public static extern int GetWindowLong(IntPtr h, int i);
 public static string Txt(IntPtr h) { var s = new StringBuilder(1024); GetWindowText(h, s, 1024); return s.ToString(); }
 public static string Cls(IntPtr h) { var s = new StringBuilder(256); GetClassName(h, s, 256); return s.ToString(); }
 public static uint PidOf(IntPtr h) { uint p; GetWindowThreadProcessId(h, out p); return p; }
 public static RECT Rect(IntPtr h) { RECT r; GetWindowRect(h, out r); return r; }
 public static List<IntPtr> Tops() { var o = new List<IntPtr>(); EnumWindows((h,l)=>{ if (IsWindowVisible(h)) o.Add(h); return true; }, IntPtr.Zero); return o; }
}
'@
[void][WL]::SetProcessDPIAware()
$log = "<LAB_DIR>\evidence\window-$Tag.txt"
if (Test-Path $log) { Write-Output "$log exists"; exit 2 }
function Stamp { (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
function Log([string]$t) { Add-Content -Encoding utf8 $log "$(Stamp) $t" }
Log "listing start: every 0.25 s for $Seconds s; listing only (no click, key, focus or capture)"
$seen = @{}; $t0 = Get-Date
while (((Get-Date) - $t0).TotalSeconds -lt $Seconds) {
    $now = @{}
    foreach ($h in [WL]::Tops()) {
        $t = [WL]::Txt($h)
        if (-not $t.StartsWith('ComplyEaze Bridge')) { continue }
        $k = $h.ToInt64(); $now[$k] = 1
        if (-not $seen.ContainsKey($k)) {
            $seen[$k] = 1; $p = [WL]::PidOf($h)
            $pr = Get-CimInstance Win32_Process -Filter "ProcessId=$p" -ErrorAction SilentlyContinue
            $r = [WL]::Rect($h); $ex = [WL]::GetWindowLong($h, -20)
            Log "LISTED 0x$($k.ToString('x')) title '$t' class $([WL]::Cls($h)) pid $p process $($pr.Name) path $($pr.ExecutablePath) parent pid $($pr.ParentProcessId); rect $($r.L),$($r.T),$($r.R),$($r.B); topmost $([int](($ex -band 8) -ne 0))"
        }
    }
    foreach ($k in @($seen.Keys)) { if ($seen[$k] -eq 1 -and -not $now.ContainsKey($k)) { $seen[$k] = 0; Log "NO LONGER LISTED 0x$($k.ToString('x'))" } }
    Start-Sleep -Milliseconds 250
}
Log "listing end"
exit 0
