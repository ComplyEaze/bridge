# tally-window-capture.ps1 (row 1780-W) - one copy of Tally's own top-level window, drawn by the window itself, to read which
# Tally screen is showing while an approval box is open (the screen tool takes no capture while a box exists).
# Read-only: it changes no window, sends no key or click, ends nothing and sends nothing to Tally.
# Written on a Mac and NOT RUN anywhere: B3c below is its known answer. Adapted from taskbar-capture.ps1 (row 2161), whose
# PrintWindow route it keeps; only the window it copies differs.
# Per-monitor DPI aware, so the rectangle is in physical pixels. Writes <Name>-tally.png in -OutDir and appends one line to -Out:
#   <stamp> name=<Name> dpi=ok|refused tally_pid=<pid> hwnd=0x.. rect=l,t,r,b print_window=True|False
# It refuses when no tally.exe, or more than one, has a main window, and refuses a -Name whose image exists already.
param([Parameter(Mandatory = $true)][string]$Name, [Parameter(Mandatory = $true)][string]$OutDir, [Parameter(Mandatory = $true)][string]$Out)
Add-Type -AssemblyName System.Drawing
Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public static class TWin {
    [StructLayout(LayoutKind.Sequential)] public struct RECT { public int Left, Top, Right, Bottom; }
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
    [DllImport("user32.dll")] public static extern bool PrintWindow(IntPtr h, IntPtr hdc, uint flags);
    [DllImport("user32.dll")] public static extern bool SetProcessDpiAwarenessContext(IntPtr value);
}
"@
$dpi = if ([TWin]::SetProcessDpiAwarenessContext([IntPtr](-4))) { 'ok' } else { 'refused' }
if (-not (Test-Path $OutDir)) { [void](New-Item -ItemType Directory -Path $OutDir) }
$png = Join-Path $OutDir ($Name + '-tally.png')
if (Test-Path $png) { throw ('exists already: ' + $Name) }
$procs = @(Get-Process -Name tally -ErrorAction SilentlyContinue | Where-Object { $_.MainWindowHandle -ne [IntPtr]::Zero })
if ($procs.Count -ne 1) { throw ('tally.exe with a main window: ' + $procs.Count + ', expected 1') }
$hwnd = $procs[0].MainWindowHandle
$r = New-Object TWin+RECT
if (-not [TWin]::GetWindowRect($hwnd, [ref]$r)) { throw 'Tally window rectangle not readable' }
$w = $r.Right - $r.Left; $h = $r.Bottom - $r.Top
$stamp = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ss.fffZ")
$print = New-Object System.Drawing.Bitmap $w, $h
$g = [System.Drawing.Graphics]::FromImage($print)
$hdc = $g.GetHdc()
$ok = [TWin]::PrintWindow($hwnd, $hdc, 2)
$g.ReleaseHdc($hdc); $g.Dispose(); $print.Save($png); $print.Dispose()
$line = "$stamp name=$Name dpi=$dpi tally_pid=$($procs[0].Id) hwnd=0x$($hwnd.ToInt64().ToString('x')) rect=$($r.Left),$($r.Top),$($r.Right),$($r.Bottom) print_window=$ok"
[System.IO.File]::AppendAllText($Out, $line + "`r`n", [System.Text.Encoding]::UTF8)
$line
