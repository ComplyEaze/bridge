# switcher-watch.ps1 (row 2161-C) - started before an Alt+Tab, it waits for the switcher, copies the screen once while the
# switcher is up, lists its tiles by UI Automation, and times when it goes. Read-only: it changes no window, sends no key or
# click, invokes no pattern, ends nothing and sends nothing to Tally. Written on a Mac and NOT RUN anywhere; its C# part was
# compiled there with Mono's mcs (no errors), the PowerShell around it was not run.
# Per-monitor DPI aware (SetProcessDpiAwarenessContext(-4)), so every rectangle and the copy are in physical pixels.
# The switcher is a visible, uncloaked top-level window of class MultitaskingViewFrame (Windows 10), XamlExplorerHostIslandWindow
# (Windows 11) or TaskSwitcherWnd (the classic switcher). It polls every 50 ms for -Seconds (default 90). It appends to -Out (it
# refuses one that exists), each line as it happens:
#   S <stamp> dpi_context=ok|refused visible_windows=<n> screen=l,t,r,b     at the start; the windows then are the baseline
#   N <stamp> hwnd=0x.. class=.. pid=.. image=.. cloaked=0|1 rect=l,t,r,b title=...
#                       a visible top-level window not in the baseline, once each, at most 100 lines
#   A <stamp> hwnd=0x.. class=.. pid=.. rect=l,t,r,b fg=0x.. fg_class=.. fg_pid=.. fg_title=...
#                       the switcher appears; fg is the foreground window at that poll
#   C <stamp> file=<Name>-switcher.png rect=l,t,r,b                         the one screen copy, about 300 ms after the first A
#   L <stamp> n=<n> name=...                                                its tiles: every ListItem under the switcher
#   D <stamp> n=<n> type=.. name=...     only when there is no ListItem: every named element under it, at most 40 lines
#   U <stamp> uia_error=...                                                  the UI Automation read failed
#   G <stamp> fg=0x.. fg_class=.. fg_pid=.. fg_title=...                    the switcher is gone; the foreground at that poll
#   F <stamp> fg=0x.. fg_class=.. fg_pid=.. fg_title=...                    the foreground 500 ms after G
#   Z <stamp> polls=<n> new_windows=<n> appearances=<n> copies=<n>
# Only the first appearance is copied and read; a later one gets its A, G and F lines only.
param([Parameter(Mandatory = $true)][string]$Name, [Parameter(Mandatory = $true)][string]$OutDir, [Parameter(Mandatory = $true)][string]$Out, [int]$Seconds = 90)
if (Test-Path $Out) { throw ('exists already: ' + $Out) }
if (-not (Test-Path $OutDir)) { [void](New-Item -ItemType Directory -Path $OutDir) }
$png = Join-Path $OutDir ($Name + '-switcher.png')
if (Test-Path $png) { throw ('exists already: ' + $png) }
Add-Type -TypeDefinition @"
using System;
using System.Collections.Generic;
using System.Text;
using System.Runtime.InteropServices;
public static class SwW {
    delegate bool EnumProc(IntPtr h, IntPtr l);
    [StructLayout(LayoutKind.Sequential)] public struct RECT { public int Left, Top, Right, Bottom; }
    [DllImport("user32.dll")] static extern bool EnumWindows(EnumProc p, IntPtr l);
    [DllImport("user32.dll")] static extern bool IsWindowVisible(IntPtr h);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern int GetClassName(IntPtr h, StringBuilder s, int n);
    [DllImport("user32.dll")] static extern bool GetWindowRect(IntPtr h, out RECT r);
    [DllImport("user32.dll")] static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
    [DllImport("user32.dll")] static extern IntPtr GetForegroundWindow();
    [DllImport("user32.dll")] static extern int GetSystemMetrics(int index);
    [DllImport("user32.dll")] static extern bool SetProcessDpiAwarenessContext(IntPtr value);
    [DllImport("dwmapi.dll")] static extern int DwmGetWindowAttribute(IntPtr h, int attr, out int value, int size);
    [DllImport("kernel32.dll")] static extern IntPtr OpenProcess(uint access, bool inherit, uint pid);
    [DllImport("kernel32.dll", CharSet = CharSet.Unicode)] static extern bool QueryFullProcessImageName(IntPtr p, int flags, StringBuilder name, ref int size);
    [DllImport("kernel32.dll")] static extern bool CloseHandle(IntPtr h);
    static readonly string[] SwitcherClasses = { "MultitaskingViewFrame", "XamlExplorerHostIslandWindow", "TaskSwitcherWnd" };
    public static bool MakeAware() { return SetProcessDpiAwarenessContext(new IntPtr(-4)); }
    public static string Hex(IntPtr h) { return "0x" + h.ToInt64().ToString("x"); }
    public static string Text(IntPtr h) { var sb = new StringBuilder(512); GetWindowText(h, sb, sb.Capacity); return sb.ToString().Replace("\r", " ").Replace("\n", " "); }
    public static string Cls(IntPtr h) { var sb = new StringBuilder(256); GetClassName(h, sb, sb.Capacity); return sb.ToString(); }
    public static uint Pid(IntPtr h) { uint pid; GetWindowThreadProcessId(h, out pid); return pid; }
    public static int Cloaked(IntPtr h) { int v; return DwmGetWindowAttribute(h, 14, out v, 4) == 0 && v != 0 ? 1 : 0; }
    public static string RectText(IntPtr h) { RECT r; if (!GetWindowRect(h, out r)) return "unreadable"; return r.Left + "," + r.Top + "," + r.Right + "," + r.Bottom; }
    public static string Image(uint pid) {
        IntPtr p = OpenProcess(0x1000u, false, pid);
        if (p == IntPtr.Zero) return "unreadable";
        var sb = new StringBuilder(1024); int size = sb.Capacity;
        string name = QueryFullProcessImageName(p, 0, sb, ref size) ? System.IO.Path.GetFileName(sb.ToString()) : "unreadable";
        CloseHandle(p);
        return name;
    }
    public static IntPtr[] Visible() {
        var list = new List<IntPtr>();
        EnumWindows(delegate (IntPtr h, IntPtr l) { if (IsWindowVisible(h)) list.Add(h); return true; }, IntPtr.Zero);
        return list.ToArray();
    }
    public static bool IsSwitcher(IntPtr h) { return Array.IndexOf(SwitcherClasses, Cls(h)) >= 0 && Cloaked(h) == 0; }
    public static string Foreground() { IntPtr f = GetForegroundWindow(); return "fg=" + Hex(f) + " fg_class=" + Cls(f) + " fg_pid=" + Pid(f) + " fg_title=" + Text(f); }
    public static int[] Screen() { return new int[] { GetSystemMetrics(76), GetSystemMetrics(77), GetSystemMetrics(78), GetSystemMetrics(79) }; }
}
"@
$dpi = if ([SwW]::MakeAware()) { 'ok' } else { 'refused' }
Add-Type -AssemblyName System.Drawing
Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes
function Stamp { (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ss.fffZ") }
function Emit([string]$line) { [System.IO.File]::AppendAllText($Out, $line + "`r`n", [System.Text.Encoding]::UTF8); $line }
function OneLine([string]$s) { if ($null -eq $s) { return '' }; return $s.Replace("`r", ' ').Replace("`n", ' ') }
$scr = [SwW]::Screen()
$sl = $scr[0]; $st = $scr[1]; $sw = $scr[2]; $sh = $scr[3]
$seen = New-Object 'System.Collections.Generic.HashSet[long]'
foreach ($h in [SwW]::Visible()) { [void]$seen.Add($h.ToInt64()) }
Emit "S $(Stamp) dpi_context=$dpi visible_windows=$($seen.Count) screen=$sl,$st,$($sl + $sw),$($st + $sh)"
$deadline = (Get-Date).AddSeconds($Seconds)
$present = [IntPtr]::Zero; $polls = 0; $newCount = 0; $appearances = 0; $copies = 0
while ((Get-Date) -lt $deadline) {
    $polls++
    $found = [IntPtr]::Zero
    foreach ($h in [SwW]::Visible()) {
        if ($seen.Add($h.ToInt64())) {
            $newCount++
            if ($newCount -le 100) {
                $p = [SwW]::Pid($h)
                Emit "N $(Stamp) hwnd=$([SwW]::Hex($h)) class=$([SwW]::Cls($h)) pid=$p image=$([SwW]::Image($p)) cloaked=$([SwW]::Cloaked($h)) rect=$([SwW]::RectText($h)) title=$([SwW]::Text($h))"
            }
        }
        if ($found -eq [IntPtr]::Zero -and [SwW]::IsSwitcher($h)) { $found = $h }
    }
    if ($present -eq [IntPtr]::Zero -and $found -ne [IntPtr]::Zero) {
        $appearances++; $present = $found
        Emit "A $(Stamp) hwnd=$([SwW]::Hex($found)) class=$([SwW]::Cls($found)) pid=$([SwW]::Pid($found)) rect=$([SwW]::RectText($found)) $([SwW]::Foreground())"
        if ($copies -eq 0) {
            $copies = 1
            Start-Sleep -Milliseconds 300
            $bmp = New-Object System.Drawing.Bitmap $sw, $sh
            $g = [System.Drawing.Graphics]::FromImage($bmp)
            $g.CopyFromScreen($sl, $st, 0, 0, (New-Object System.Drawing.Size $sw, $sh))
            $g.Dispose(); $bmp.Save($png); $bmp.Dispose()
            Emit "C $(Stamp) file=$($Name + '-switcher.png') rect=$sl,$st,$($sl + $sw),$($st + $sh)"
            try {
                $el = [System.Windows.Automation.AutomationElement]::FromHandle($found)
                $li = [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::ListItem)
                $items = $el.FindAll([System.Windows.Automation.TreeScope]::Descendants, $li)
                $n = 0
                foreach ($it in $items) { $n++; Emit "L $(Stamp) n=$n name=$(OneLine $it.Current.Name)" }
                if ($n -eq 0) {
                    $all = $el.FindAll([System.Windows.Automation.TreeScope]::Descendants, [System.Windows.Automation.Condition]::TrueCondition)
                    $d = 0
                    foreach ($it in $all) {
                        $nm = OneLine $it.Current.Name
                        if ($nm -ne '' -and $d -lt 40) { $d++; Emit "D $(Stamp) n=$d type=$($it.Current.ControlType.ProgrammaticName) name=$nm" }
                    }
                }
            } catch { Emit "U $(Stamp) uia_error=$(OneLine $_.Exception.GetType().FullName)" }
        }
    } elseif ($present -ne [IntPtr]::Zero -and $found -eq [IntPtr]::Zero) {
        $present = [IntPtr]::Zero
        Emit "G $(Stamp) $([SwW]::Foreground())"
        Start-Sleep -Milliseconds 500
        Emit "F $(Stamp) $([SwW]::Foreground())"
    }
    Start-Sleep -Milliseconds 50
}
Emit "Z $(Stamp) polls=$polls new_windows=$newCount appearances=$appearances copies=$copies"
