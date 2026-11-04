# taskbar-cover.ps1 (row 2161-B) - what lies over the taskbar, and what a click at one point would hit. Read-only: it changes no
# window, sends no key or click, ends nothing and sends nothing to Tally. Written on a Mac and NOT RUN anywhere; its C# part was
# compiled there with Mono's mcs (no errors), the PowerShell around it was not run.
# Per-monitor DPI aware (SetProcessDpiAwarenessContext(-4)), so every rectangle and the point are in physical pixels.
# Writes to -Out (a new file each time; it refuses one that exists), one line each:
#   E <stamp> dpi_context=ok|refused
#   T <stamp> tray=0x.. z=<n> rect=l,t,r,b                     the taskbar (Shell_TrayWnd) and its place in the stacking order
#   W <stamp> z=<n> above_tray=0|1 hwnd=0x.. class=.. pid=.. image=.. topmost=0|1 exstyle=0x........ cloaked=0|1 rect=l,t,r,b title=...
#                                                              every visible top-level window whose rectangle meets the taskbar's,
#                                                              from the top of the stacking order down (z 1 is the top)
#   P <stamp> x=.. y=.. hwnd=0x.. class=.. root=0x.. root_class=.. pid=.. image=.. point_ok=0|1
#                                                              only with -X and -Y: WindowFromPoint at that physical point;
#                                                              point_ok=1 only when the window is MSTaskListWClass and its
#                                                              root window is the taskbar (Shell_TrayWnd)
#   Z <stamp> visible_windows=<n> meeting_tray=<n>
param([Parameter(Mandatory = $true)][string]$Out, [int]$X = -1, [int]$Y = -1)
if (Test-Path $Out) { throw ('exists already: ' + $Out) }
Add-Type -TypeDefinition @"
using System;
using System.Collections.Generic;
using System.Text;
using System.Runtime.InteropServices;
public static class TCover {
    delegate bool EnumProc(IntPtr h, IntPtr l);
    [StructLayout(LayoutKind.Sequential)] public struct RECT { public int Left, Top, Right, Bottom; }
    [StructLayout(LayoutKind.Sequential)] public struct POINT { public int X, Y; }
    [DllImport("user32.dll")] static extern bool EnumWindows(EnumProc p, IntPtr l);
    [DllImport("user32.dll")] static extern bool IsWindowVisible(IntPtr h);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern int GetClassName(IntPtr h, StringBuilder s, int n);
    [DllImport("user32.dll")] static extern int GetWindowLong(IntPtr h, int index);
    [DllImport("user32.dll")] static extern bool GetWindowRect(IntPtr h, out RECT r);
    [DllImport("user32.dll")] static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern IntPtr FindWindow(string cls, string name);
    [DllImport("user32.dll")] static extern IntPtr WindowFromPoint(POINT p);
    [DllImport("user32.dll")] static extern IntPtr GetAncestor(IntPtr h, uint flags);
    [DllImport("user32.dll")] static extern bool SetProcessDpiAwarenessContext(IntPtr value);
    [DllImport("dwmapi.dll")] static extern int DwmGetWindowAttribute(IntPtr h, int attr, out int value, int size);
    [DllImport("kernel32.dll")] static extern IntPtr OpenProcess(uint access, bool inherit, uint pid);
    [DllImport("kernel32.dll", CharSet = CharSet.Unicode)] static extern bool QueryFullProcessImageName(IntPtr p, int flags, StringBuilder name, ref int size);
    [DllImport("kernel32.dll")] static extern bool CloseHandle(IntPtr h);
    public static bool MakeAware() { return SetProcessDpiAwarenessContext(new IntPtr(-4)); }
    static string Hex(IntPtr h) { return "0x" + h.ToInt64().ToString("x"); }
    static string Text(IntPtr h) { var sb = new StringBuilder(512); GetWindowText(h, sb, sb.Capacity); return sb.ToString().Replace("\r", " ").Replace("\n", " "); }
    static string Cls(IntPtr h) { var sb = new StringBuilder(256); GetClassName(h, sb, sb.Capacity); return sb.ToString(); }
    static string RectText(RECT r) { return r.Left + "," + r.Top + "," + r.Right + "," + r.Bottom; }
    static string Image(uint pid) {
        IntPtr p = OpenProcess(0x1000u, false, pid);
        if (p == IntPtr.Zero) return "unreadable";
        var sb = new StringBuilder(1024); int size = sb.Capacity;
        string name = QueryFullProcessImageName(p, 0, sb, ref size) ? System.IO.Path.GetFileName(sb.ToString()) : "unreadable";
        CloseHandle(p);
        return name;
    }
    static string Owner(IntPtr h) { uint pid; GetWindowThreadProcessId(h, out pid); return "pid=" + pid + " image=" + Image(pid); }
    public static List<string> Run(string stamp, int x, int y) {
        var lines = new List<string>();
        IntPtr tray = FindWindow("Shell_TrayWnd", null);
        RECT tr = new RECT();
        bool haveTray = tray != IntPtr.Zero && GetWindowRect(tray, out tr);
        int z = 0, trayZ = -1, meeting = 0;
        var found = new List<string>();
        EnumWindows(delegate (IntPtr h, IntPtr l) {
            if (!IsWindowVisible(h)) return true;
            z++;
            if (h == tray) { trayZ = z; return true; }
            RECT r;
            if (!haveTray || !GetWindowRect(h, out r)) return true;
            if (r.Right <= tr.Left || r.Left >= tr.Right || r.Bottom <= tr.Top || r.Top >= tr.Bottom) return true;
            meeting++;
            int cloaked = 0;
            DwmGetWindowAttribute(h, 14, out cloaked, 4);
            int ex = GetWindowLong(h, -20);
            int topmost = (ex & 0x8) != 0 ? 1 : 0;
            found.Add(z + "|" + Hex(h) + " class=" + Cls(h) + " " + Owner(h) + " topmost=" + topmost + " exstyle=0x" + ex.ToString("x8") + " cloaked=" + (cloaked != 0 ? 1 : 0) +
                      " rect=" + RectText(r) + " title=" + Text(h));
            return true;
        }, IntPtr.Zero);
        lines.Add("T " + stamp + " tray=" + Hex(tray) + " z=" + trayZ + " rect=" + (haveTray ? RectText(tr) : "none"));
        foreach (string f in found) {
            int bar = f.IndexOf('|');
            int wz = int.Parse(f.Substring(0, bar));
            lines.Add("W " + stamp + " z=" + wz + " above_tray=" + (trayZ > 0 && wz < trayZ ? 1 : 0) + " hwnd=" + f.Substring(bar + 1));
        }
        if (x >= 0 && y >= 0) {
            POINT pt; pt.X = x; pt.Y = y;
            IntPtr w = WindowFromPoint(pt);
            IntPtr root = w == IntPtr.Zero ? IntPtr.Zero : GetAncestor(w, 2);
            bool ok = w != IntPtr.Zero && Cls(w) == "MSTaskListWClass" && root == tray && tray != IntPtr.Zero;
            lines.Add("P " + stamp + " x=" + x + " y=" + y + " hwnd=" + Hex(w) + " class=" + (w == IntPtr.Zero ? "none" : Cls(w)) +
                      " root=" + Hex(root) + " root_class=" + (root == IntPtr.Zero ? "none" : Cls(root)) + " " +
                      (w == IntPtr.Zero ? "pid=0 image=none" : Owner(w)) + " point_ok=" + (ok ? 1 : 0));
        }
        lines.Add("Z " + stamp + " visible_windows=" + z + " meeting_tray=" + meeting);
        return lines;
    }
}
"@
$aware = [TCover]::MakeAware()
$stamp = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ss.fffZ")
$lines = @("E " + $stamp + " dpi_context=" + $(if ($aware) { 'ok' } else { 'refused' })) + [TCover]::Run($stamp, $X, $Y)
foreach ($line in $lines) { [System.IO.File]::AppendAllText($Out, $line + "`r`n", [System.Text.Encoding]::UTF8) }
($lines | Where-Object { $_ -like 'P *' }) -replace '.* point_ok=', 'point_ok='
