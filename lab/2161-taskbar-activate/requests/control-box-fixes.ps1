# control-box-fixes.ps1 - one Windows message box with the approval box's own flags, from this process, with no Bridge code, and three
# candidate fixes for the lost WS_EX_TOPMOST bit (row 1780-G). Written on a Mac and NOT RUN anywhere. Its C# part was compiled on that
# Mac with Mono's mcs (no errors); the PowerShell around it was not run. It sends nothing to Tally and reads nothing from it.
# The box is MessageBoxW with MB_YESNOCANCEL 0x3 | MB_DEFBUTTON2 0x100 | MB_ICONWARNING 0x30 | MB_SETFOREGROUND 0x10000 | MB_TOPMOST 0x40000
# (approved_import.rs show_review), titled "Lab topmost control" so the same trace and the K probe read it. Variants:
#   c  the flags only (the control, as row 1780-E's K1c).
#   g  c, and a helper thread that waits for the box, then up to 5 times, 200 ms apart, calls SetWindowPos(HWND_TOPMOST, NOMOVE|NOSIZE|
#      NOACTIVATE) and writes to <Out>.reassert.txt the return value, GetLastError and the box's extended style (and stops at the first
#      time the style has 0x8). Candidate 1.
#   h  c with MB_SYSTEMMODAL 0x1000 added. Candidate 2.
#   i  c, owned by a hidden WS_EX_TOPMOST | WS_EX_TOOLWINDOW window this thread creates (never shown), destroyed after the answer. Candidate 3.
# It writes the variant, two stamps and the answer code (6 Yes, 7 No, 2 Cancel) to -Out.
param(
    [Parameter(Mandatory = $true)][ValidateSet('c', 'g', 'h', 'i')][string]$Variant,
    [int]$DelaySeconds = 0,
    [Parameter(Mandatory = $true)][string]$Out
)
Add-Type -TypeDefinition @"
using System;
using System.IO;
using System.Runtime.InteropServices;
using System.Threading;
public static class FBox {
    delegate bool EnumProc(IntPtr h, IntPtr l);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern int MessageBoxW(IntPtr owner, string text, string caption, uint type);
    [DllImport("user32.dll")] static extern bool EnumThreadWindows(uint thread, EnumProc proc, IntPtr l);
    [DllImport("user32.dll")] static extern bool IsWindowVisible(IntPtr h);
    [DllImport("user32.dll", SetLastError = true)] static extern bool SetWindowPos(IntPtr h, IntPtr after, int x, int y, int cx, int cy, uint flags);
    [DllImport("user32.dll")] static extern int GetWindowLong(IntPtr h, int index);
    [DllImport("user32.dll", CharSet = CharSet.Unicode, SetLastError = true)] static extern IntPtr CreateWindowEx(uint exStyle, string cls, string name, uint style, int x, int y, int w, int h, IntPtr parent, IntPtr menu, IntPtr inst, IntPtr param);
    [DllImport("user32.dll")] static extern bool DestroyWindow(IntPtr h);
    [DllImport("kernel32.dll")] static extern uint GetCurrentThreadId();
    static volatile bool stop;
    static Thread helper;
    static string Stamp() { return DateTime.UtcNow.ToString("yyyy-MM-ddTHH:mm:ss.fffZ"); }
    static void Line(string path, string text) { File.AppendAllText(path, text + "\r\n"); }
    static IntPtr FirstVisible(uint thread) {
        IntPtr slot = IntPtr.Zero;
        EnumThreadWindows(thread, delegate (IntPtr h, IntPtr l) { if (IsWindowVisible(h)) { slot = h; return false; } return true; }, IntPtr.Zero);
        return slot;
    }
    public static void StartReassert(string log) {
        uint owner = GetCurrentThreadId();
        stop = false;
        helper = new Thread(delegate () {
            IntPtr w = IntPtr.Zero;
            for (int i = 0; i < 100 && !stop && w == IntPtr.Zero; i++) { w = FirstVisible(owner); if (w == IntPtr.Zero) Thread.Sleep(100); }
            if (w == IntPtr.Zero) { Line(log, Stamp() + " reassert: no box seen"); return; }
            Line(log, Stamp() + " reassert: box seen exstyle=0x" + GetWindowLong(w, -20).ToString("x"));
            for (int n = 1; n <= 5 && !stop; n++) {
                bool ret = SetWindowPos(w, new IntPtr(-1), 0, 0, 0, 0, 0x0002 | 0x0001 | 0x0010);
                int err = Marshal.GetLastWin32Error();
                int style = GetWindowLong(w, -20);
                Line(log, Stamp() + " reassert " + n + ": SetWindowPos=" + ret + " GetLastError=" + err + " exstyle=0x" + style.ToString("x") + " topmost=" + ((style & 8) != 0 ? 1 : 0));
                if ((style & 8) != 0) return;
                Thread.Sleep(200);
            }
        });
        helper.Start();
    }
    public static void StopReassert() { stop = true; if (helper != null) helper.Join(); }
    public static IntPtr HiddenTopmostOwner() { return CreateWindowEx(0x8u | 0x80u, "Static", "", 0x80000000u, 0, 0, 0, 0, IntPtr.Zero, IntPtr.Zero, IntPtr.Zero, IntPtr.Zero); }
    public static void Destroy(IntPtr h) { if (h != IntPtr.Zero) DestroyWindow(h); }
    public static int Show(IntPtr owner, string text, string caption, uint type) { return MessageBoxW(owner, text, caption, type); }
}
"@
function Stamp { (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ss.fffZ") }
Start-Sleep -Seconds $DelaySeconds
$type = [uint32](0x3 -bor 0x100 -bor 0x30 -bor 0x10000 -bor 0x40000)
$owner = [IntPtr]::Zero
if ($Variant -eq 'h') { $type = $type -bor [uint32]0x1000 }
if ($Variant -eq 'i') { $owner = [FBox]::HiddenTopmostOwner() }
if ($Variant -eq 'g') { [FBox]::StartReassert($Out + '.reassert.txt') }
$asked = Stamp
$answer = [FBox]::Show($owner, "Lab topmost control, variant $Variant. Nothing is sent anywhere. Click No.", "Lab topmost control", $type)
$done = Stamp
if ($Variant -eq 'g') { [FBox]::StopReassert() }
if ($Variant -eq 'i') { [FBox]::Destroy($owner) }
[System.IO.File]::AppendAllText($Out, ("variant=" + $Variant + " owner_created=" + ($owner -ne [IntPtr]::Zero) + " asked=" + $asked + " answered=" + $done + " answer=" + $answer + "`r`n"), [System.Text.Encoding]::UTF8)
