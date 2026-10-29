# control-box-product.ps1 - one Windows message box with the approval box's own flags, from this process, with no Bridge code,
# adding the approval dialog process's other features one at a time (variants c to f). Written on a Mac and NOT RUN anywhere.
# Its C# part was compiled on that Mac with Mono's mcs (no errors); the PowerShell around it was not run.
# It sends nothing to Tally and reads nothing from it.
#
# The box is MessageBoxW with no owner and the flags show_review (approved_import.rs) passes for the post box:
#   MB_YESNOCANCEL 0x3 | MB_DEFBUTTON2 0x100 | MB_ICONWARNING 0x30 | MB_SETFOREGROUND 0x10000 | MB_TOPMOST 0x40000.
# Its title is "Lab topmost control", as row 1780-D's control, so the same trace and the K probe read it.
#
# Variants (each includes the one before it; the order is the order of box_fit::guarded):
#   c  the flags only.
#   d  c, after SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, i.e. -4) on this process. The thread's
#      awareness is read and written to -Out before the call (dpi_before_awareness, dpi_before_is_per_monitor_v2): a host that
#      is already aware makes the call return False (dpi_set=False) and change nothing.
#   e  d, after a WH_CBT hook on this thread that does what box_fit.rs on_activate does: on HCBT_ACTIVATE it measures the
#      window's rectangle against the work area of the monitor nearest to it and, for a box not wholly inside it, posts
#      WM_COMMAND IDCANCEL to the box. Every measurement is written, with a UTC stamp, to <Out>.fits.txt.
#   f  e, with the helper thread box_fit.rs Front starts: it waits for the thread's first visible window, then calls raise():
#      SetWindowPos(HWND_TOPMOST, NOMOVE | NOSIZE | NOACTIVATE), attaches to the foreground thread's input, SetForegroundWindow,
#      BringWindowToTop, and flashes the window if it is still not the foreground window.
# It waits -DelaySeconds first, so the person can put Tally in front and this process is in the background when the box is
# asked for. It writes the variant, the DPI and hook results, two stamps and the answer code (6 Yes, 7 No, 2 Cancel) to -Out.
param(
    [Parameter(Mandatory = $true)][ValidateSet('c', 'd', 'e', 'f')][string]$Variant,
    [int]$DelaySeconds = 8,
    [Parameter(Mandatory = $true)][string]$Out
)
Add-Type -TypeDefinition @"
using System;
using System.IO;
using System.Runtime.InteropServices;
using System.Threading;
public static class PBox {
    [StructLayout(LayoutKind.Sequential)] public struct RECT { public int Left, Top, Right, Bottom; }
    [StructLayout(LayoutKind.Sequential)] public struct MONITORINFO { public int cbSize; public RECT rcMonitor; public RECT rcWork; public uint dwFlags; }
    [StructLayout(LayoutKind.Sequential)] public struct FLASHWINFO { public uint cbSize; public IntPtr hwnd; public uint dwFlags; public uint uCount; public uint dwTimeout; }
    delegate IntPtr HookProc(int code, IntPtr wParam, IntPtr lParam);
    delegate bool EnumProc(IntPtr h, IntPtr l);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern int MessageBoxW(IntPtr owner, string text, string caption, uint type);
    [DllImport("user32.dll")] static extern bool SetProcessDpiAwarenessContext(IntPtr value);
    [DllImport("user32.dll")] static extern IntPtr GetThreadDpiAwarenessContext();
    [DllImport("user32.dll")] static extern bool AreDpiAwarenessContextsEqual(IntPtr a, IntPtr b);
    [DllImport("user32.dll")] static extern int GetAwarenessFromDpiAwarenessContext(IntPtr value);
    [DllImport("user32.dll", SetLastError = true)] static extern IntPtr SetWindowsHookEx(int idHook, HookProc proc, IntPtr module, uint threadId);
    [DllImport("user32.dll")] static extern bool UnhookWindowsHookEx(IntPtr hook);
    [DllImport("user32.dll")] static extern IntPtr CallNextHookEx(IntPtr hook, int code, IntPtr wParam, IntPtr lParam);
    [DllImport("user32.dll")] static extern bool GetWindowRect(IntPtr h, out RECT r);
    [DllImport("user32.dll")] static extern IntPtr MonitorFromWindow(IntPtr h, uint flags);
    [DllImport("user32.dll")] static extern bool GetMonitorInfo(IntPtr monitor, ref MONITORINFO info);
    [DllImport("user32.dll")] static extern bool PostMessage(IntPtr h, uint msg, IntPtr wParam, IntPtr lParam);
    [DllImport("user32.dll")] static extern bool EnumThreadWindows(uint thread, EnumProc proc, IntPtr l);
    [DllImport("user32.dll")] static extern bool IsWindowVisible(IntPtr h);
    [DllImport("user32.dll")] static extern bool SetWindowPos(IntPtr h, IntPtr after, int x, int y, int cx, int cy, uint flags);
    [DllImport("user32.dll")] static extern IntPtr GetForegroundWindow();
    [DllImport("user32.dll")] static extern uint GetWindowThreadProcessId(IntPtr h, IntPtr pid);
    [DllImport("user32.dll")] static extern bool SetForegroundWindow(IntPtr h);
    [DllImport("user32.dll")] static extern bool BringWindowToTop(IntPtr h);
    [DllImport("user32.dll")] static extern bool AttachThreadInput(uint from, uint to, bool attach);
    [DllImport("user32.dll")] static extern bool FlashWindowEx(ref FLASHWINFO info);
    [DllImport("kernel32.dll")] static extern uint GetCurrentThreadId();
    static HookProc keep;
    static IntPtr hook = IntPtr.Zero;
    static string fitsLog;
    static volatile bool stopFront;
    static Thread front;
    static string Stamp() { return DateTime.UtcNow.ToString("yyyy-MM-ddTHH:mm:ss.fffZ"); }
    static void Line(string path, string text) { File.AppendAllText(path, text + "\r\n"); }

    // box_fit.rs make_dpi_aware: per-monitor v2, then checked.
    // The awareness is read BEFORE the call, so the lab sees whether this host was already aware (its manifest may make it so,
    // and then the call returns False and changes nothing). DPI_AWARENESS: -1 invalid, 0 unaware, 1 system aware, 2 per-monitor aware.
    public static string Dpi() {
        IntPtr v2 = new IntPtr(-4);
        IntPtr before = GetThreadDpiAwarenessContext();
        int beforeAwareness = GetAwarenessFromDpiAwarenessContext(before);
        bool beforeIsV2 = AreDpiAwarenessContextsEqual(before, v2);
        bool set = SetProcessDpiAwarenessContext(v2);
        bool same = AreDpiAwarenessContextsEqual(GetThreadDpiAwarenessContext(), v2);
        return "dpi_before_awareness=" + beforeAwareness + " dpi_before_is_per_monitor_v2=" + beforeIsV2 + " dpi_set=" + set + " thread_is_per_monitor_v2=" + same;
    }

    // box_fit.rs measure() and fits(): the window's rectangle against the work area of the monitor nearest to it.
    static bool Fits(IntPtr window) {
        RECT r;
        if (!GetWindowRect(window, out r)) { Line(fitsLog, Stamp() + " box-fit: not readable fits=False"); return false; }
        IntPtr monitor = MonitorFromWindow(window, 2);
        MONITORINFO info = new MONITORINFO();
        info.cbSize = Marshal.SizeOf(typeof(MONITORINFO));
        if (monitor == IntPtr.Zero || !GetMonitorInfo(monitor, ref info)) { Line(fitsLog, Stamp() + " box-fit: not readable fits=False"); return false; }
        RECT w = info.rcWork;
        bool ok = r.Right > r.Left && r.Bottom > r.Top && w.Right > w.Left && w.Bottom > w.Top
            && r.Left >= w.Left && r.Top >= w.Top && r.Right <= w.Right && r.Bottom <= w.Bottom;
        Line(fitsLog, Stamp() + " box-fit: box=(" + r.Left + "," + r.Top + "," + r.Right + "," + r.Bottom + ") work=(" + w.Left + "," + w.Top + "," + w.Right + "," + w.Bottom + ") fits=" + ok);
        return ok;
    }

    // box_fit.rs on_activate: HCBT_ACTIVATE is code 5; a box that does not fit gets WM_COMMAND (0x111) IDCANCEL (2).
    static IntPtr OnActivate(int code, IntPtr wParam, IntPtr lParam) {
        if (code == 5) {
            Line(fitsLog, Stamp() + " HCBT_ACTIVATE hwnd=0x" + wParam.ToInt64().ToString("x"));
            if (!Fits(wParam)) { PostMessage(wParam, 0x111, new IntPtr(2), IntPtr.Zero); Line(fitsLog, Stamp() + " IDCANCEL posted"); }
        }
        return CallNextHookEx(IntPtr.Zero, code, wParam, lParam);
    }

    public static string InstallHook(string log) {
        fitsLog = log;
        keep = new HookProc(OnActivate);
        hook = SetWindowsHookEx(5, keep, IntPtr.Zero, GetCurrentThreadId());
        return hook == IntPtr.Zero ? "hook_installed=False error=" + Marshal.GetLastWin32Error() : "hook_installed=True";
    }

    public static void RemoveHook() { if (hook != IntPtr.Zero) { UnhookWindowsHookEx(hook); hook = IntPtr.Zero; } }

    static bool FirstVisible(uint thread, ref IntPtr found) {
        IntPtr slot = IntPtr.Zero;
        EnumThreadWindows(thread, delegate (IntPtr h, IntPtr l) { if (IsWindowVisible(h)) { slot = h; return false; } return true; }, IntPtr.Zero);
        found = slot;
        return slot != IntPtr.Zero;
    }

    // box_fit.rs raise().
    static void Raise(IntPtr window, string log) {
        SetWindowPos(window, new IntPtr(-1), 0, 0, 0, 0, 0x0002 | 0x0001 | 0x0010);
        IntPtr fg = GetForegroundWindow();
        if (fg == window) { Line(log, Stamp() + " front: raise done, already the foreground window"); return; }
        uint me = GetCurrentThreadId();
        uint other = fg == IntPtr.Zero ? 0 : GetWindowThreadProcessId(fg, IntPtr.Zero);
        bool attached = other != 0 && other != me && AttachThreadInput(me, other, true);
        SetForegroundWindow(window);
        BringWindowToTop(window);
        if (attached) AttachThreadInput(me, other, false);
        bool now = GetForegroundWindow() == window;
        if (!now) { FLASHWINFO f = new FLASHWINFO(); f.cbSize = (uint)Marshal.SizeOf(typeof(FLASHWINFO)); f.hwnd = window; f.dwFlags = 3 | 0xC; FlashWindowEx(ref f); }
        Line(log, Stamp() + " front: raise done, foreground now=" + now);
    }

    // box_fit.rs Front::start: about ten seconds to see the box, then one raise.
    public static void StartFront(uint ownerThread, string log) {
        stopFront = false;
        front = new Thread(delegate () {
            for (int i = 0; i < 100 && !stopFront; i++) {
                IntPtr w = IntPtr.Zero;
                if (FirstVisible(ownerThread, ref w)) { Raise(w, log); return; }
                Thread.Sleep(100);
            }
        });
        front.Start();
    }

    public static void StopFront() { stopFront = true; if (front != null) front.Join(); }

    public static uint OwnerThread() { return GetCurrentThreadId(); }

    public static int Show(string text, string caption, uint type) { return MessageBoxW(IntPtr.Zero, text, caption, type); }
}
"@
function Stamp { (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ss.fffZ") }
Start-Sleep -Seconds $DelaySeconds
$notes = @()
if ($Variant -in 'd', 'e', 'f') { $notes += [PBox]::Dpi() }
if ($Variant -in 'e', 'f') { $notes += [PBox]::InstallHook($Out + '.fits.txt') }
if ($Variant -eq 'f') { [PBox]::StartFront([PBox]::OwnerThread(), $Out + '.fits.txt'); $notes += 'front_started=True' }
$asked = Stamp
# MB_YESNOCANCEL 0x3, MB_DEFBUTTON2 0x100, MB_ICONWARNING 0x30, MB_SETFOREGROUND 0x10000, MB_TOPMOST 0x40000.
$answer = [PBox]::Show("Lab topmost control, variant $Variant. Nothing is sent anywhere. Click No.", "Lab topmost control", [uint32](0x3 -bor 0x100 -bor 0x30 -bor 0x10000 -bor 0x40000))
$done = Stamp
if ($Variant -eq 'f') { [PBox]::StopFront() }
if ($Variant -in 'e', 'f') { [PBox]::RemoveHook() }
[System.IO.File]::AppendAllText($Out, ("variant=" + $Variant + " " + ($notes -join ' ') + " asked=" + $asked + " answered=" + $done + " answer=" + $answer + "`r`n"), [System.Text.Encoding]::UTF8)
