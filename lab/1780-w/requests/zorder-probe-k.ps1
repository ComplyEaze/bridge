# zorder-probe.ps1 - lists the visible top-level windows from the top of the stacking order down.
# Read-only: it changes no window and sends no key or click. Written on a Mac and NOT RUN anywhere.
# Run it in Windows PowerShell 5.1 opened from the Start menu. It prints one line per window:
#   position,topmost,foreground,pid,title
# position 1 is the window at the very top. topmost is 1 when the window has the TOPMOST style.
# Only windows whose title contains "ComplyEaze Bridge", "Tally" or "Lab topmost control" are printed, plus the total count.
# Lab deviation for part K of row 1780-D: the third title added to the filter; nothing else changed.
Add-Type -TypeDefinition @"
using System;
using System.Collections.Generic;
using System.Text;
using System.Runtime.InteropServices;
public static class ZProbe {
    delegate bool EnumProc(IntPtr h, IntPtr l);
    [DllImport("user32.dll")] static extern bool EnumWindows(EnumProc p, IntPtr l);
    [DllImport("user32.dll")] static extern bool IsWindowVisible(IntPtr h);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
    [DllImport("user32.dll")] static extern int GetWindowLong(IntPtr h, int index);
    [DllImport("user32.dll")] static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
    [DllImport("user32.dll")] static extern IntPtr GetForegroundWindow();
    public static List<string> Run() {
        var lines = new List<string>();
        int position = 0;
        IntPtr front = GetForegroundWindow();
        EnumWindows(delegate (IntPtr h, IntPtr l) {
            if (!IsWindowVisible(h)) return true;
            var sb = new StringBuilder(512);
            GetWindowText(h, sb, sb.Capacity);
            string title = sb.ToString();
            if (title.Length == 0) return true;
            position++;
            if (title.IndexOf("ComplyEaze Bridge", StringComparison.OrdinalIgnoreCase) >= 0 ||
                title.IndexOf("Tally", StringComparison.OrdinalIgnoreCase) >= 0 ||
                title.IndexOf("Lab topmost control", StringComparison.OrdinalIgnoreCase) >= 0) {
                uint pid; GetWindowThreadProcessId(h, out pid);
                int topmost = (GetWindowLong(h, -20) & 0x8) != 0 ? 1 : 0;
                int foreground = (h == front) ? 1 : 0;
                lines.Add(position + "," + topmost + "," + foreground + "," + pid + "," + title);
            }
            return true;
        }, IntPtr.Zero);
        lines.Add("visible_titled_windows," + position);
        return lines;
    }
}
"@
[ZProbe]::Run() | ForEach-Object { $_ }
