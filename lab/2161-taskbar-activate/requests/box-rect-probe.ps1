# box-rect-probe.ps1 (row 2161-A) - where the box and the taskbar are, in the unit the product's fit check measures in.
# Read-only: it changes no window, sends no key or click, ends nothing and sends nothing to Tally. Written on a Mac and NOT RUN
# anywhere. Its C# part was compiled on that Mac with Mono's mcs (no errors); the PowerShell around it was not run.
# It makes its own process per-monitor DPI aware (SetProcessDpiAwarenessContext(-4), as tally/box_fit.rs make_dpi_aware does for the
# box), so every rectangle is in physical pixels, as the box's own "box-fit:" line is. It writes to -Out, one line each:
#   B <stamp> hwnd=0x.. pid=.. iconic=0|1 topmost=0|1 rect=l,t,r,b title=...   every visible top-level window whose title contains
#                                                                              "ComplyEaze Bridge" or "Lab topmost control"
#   F <stamp> hwnd=0x.. pid=.. title=...                                       the foreground window
#   K <stamp> class=.. hwnd=0x.. visible=0|1 rect=l,t,r,b                      the taskbar's windows: Shell_TrayWnd, ReBarWindow32,
#                                                                              MSTaskSwWClass, MSTaskListWClass (hwnd=0x0 if not found)
#   E <stamp> dpi_context=ok|refused boxes=N
param([Parameter(Mandatory = $true)][string]$Out)
Add-Type -TypeDefinition @"
using System;
using System.Collections.Generic;
using System.Text;
using System.Runtime.InteropServices;
public static class BRProbe {
    delegate bool EnumProc(IntPtr h, IntPtr l);
    [StructLayout(LayoutKind.Sequential)] public struct RECT { public int Left, Top, Right, Bottom; }
    [DllImport("user32.dll")] static extern bool EnumWindows(EnumProc p, IntPtr l);
    [DllImport("user32.dll")] static extern bool IsWindowVisible(IntPtr h);
    [DllImport("user32.dll")] static extern bool IsIconic(IntPtr h);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
    [DllImport("user32.dll")] static extern int GetWindowLong(IntPtr h, int index);
    [DllImport("user32.dll")] static extern bool GetWindowRect(IntPtr h, out RECT r);
    [DllImport("user32.dll")] static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
    [DllImport("user32.dll")] static extern IntPtr GetForegroundWindow();
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern IntPtr FindWindowEx(IntPtr parent, IntPtr after, string cls, string name);
    [DllImport("user32.dll")] static extern bool SetProcessDpiAwarenessContext(IntPtr value);
    public static bool MakeAware() { return SetProcessDpiAwarenessContext(new IntPtr(-4)); }
    static string Hex(IntPtr h) { return "0x" + h.ToInt64().ToString("x"); }
    static string Rect(IntPtr h) {
        RECT r;
        if (!GetWindowRect(h, out r)) return "unreadable";
        return r.Left + "," + r.Top + "," + r.Right + "," + r.Bottom;
    }
    static string Title(IntPtr h) { var sb = new StringBuilder(512); GetWindowText(h, sb, sb.Capacity); return sb.ToString(); }
    public static List<string> Run(string stamp) {
        var lines = new List<string>();
        int boxes = 0;
        EnumWindows(delegate (IntPtr h, IntPtr l) {
            if (!IsWindowVisible(h)) return true;
            string title = Title(h);
            if (title.IndexOf("ComplyEaze Bridge", StringComparison.OrdinalIgnoreCase) < 0 &&
                title.IndexOf("Lab topmost control", StringComparison.OrdinalIgnoreCase) < 0) return true;
            boxes++;
            uint pid; GetWindowThreadProcessId(h, out pid);
            int topmost = (GetWindowLong(h, -20) & 0x8) != 0 ? 1 : 0;
            lines.Add("B " + stamp + " hwnd=" + Hex(h) + " pid=" + pid + " iconic=" + (IsIconic(h) ? 1 : 0) + " topmost=" + topmost +
                      " rect=" + Rect(h) + " title=" + title);
            return true;
        }, IntPtr.Zero);
        IntPtr front = GetForegroundWindow();
        uint fpid = 0;
        if (front != IntPtr.Zero) GetWindowThreadProcessId(front, out fpid);
        lines.Add("F " + stamp + " hwnd=" + Hex(front) + " pid=" + fpid + " title=" + (front == IntPtr.Zero ? "" : Title(front)));
        IntPtr parent = IntPtr.Zero;
        foreach (string cls in new string[] { "Shell_TrayWnd", "ReBarWindow32", "MSTaskSwWClass", "MSTaskListWClass" }) {
            IntPtr w = FindWindowEx(parent, IntPtr.Zero, cls, null);
            lines.Add("K " + stamp + " class=" + cls + " hwnd=" + Hex(w) + " visible=" + (w != IntPtr.Zero && IsWindowVisible(w) ? 1 : 0) +
                      " rect=" + (w == IntPtr.Zero ? "none" : Rect(w)));
            if (w == IntPtr.Zero) break;
            parent = w;
        }
        lines.Add("E " + stamp + " boxes=" + boxes);
        return lines;
    }
}
"@
$aware = [BRProbe]::MakeAware()
$stamp = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ss.fffZ")
$lines = @("E " + $stamp + " dpi_context=" + $(if ($aware) { 'ok' } else { 'refused' })) + [BRProbe]::Run($stamp)
# Written to -Out only (UTF-8): the console would mangle the box title's dash. -Out is one file per read; it is never overwritten.
if (Test-Path $Out) { throw ('exists already: ' + $Out) }
foreach ($line in $lines) { [System.IO.File]::AppendAllText($Out, $line + "`r`n", [System.Text.Encoding]::UTF8) }
