# taskbar-capture.ps1 (row 2161) - two captures of the taskbar at one moment, by two routes, to tell an undrawn taskbar from an
# artefact of one capture route. Read-only: it changes no window, sends no key or click, ends nothing and sends nothing to Tally.
# Written on a Mac and NOT RUN anywhere. Its C# part was compiled there with Mono's mcs (no errors); the PowerShell was not run.
# Per-monitor DPI aware, so the rectangle is in physical pixels. Writes, in -OutDir:
#   <Name>-screen.png  the screen copy of the taskbar's rectangle (CopyFromScreen, a BitBlt of the screen; row 1780-H's route)
#   <Name>-print.png   the taskbar window asked to draw itself (PrintWindow with PW_RENDERFULLCONTENT, 0x2)
# and appends one line to -Out: <stamp> name=<Name> dpi=ok|refused tray=0x.. rect=l,t,r,b print_window=True|False
# It creates -OutDir if absent and refuses a -Name whose images exist already.
param([Parameter(Mandatory = $true)][string]$Name, [Parameter(Mandatory = $true)][string]$OutDir, [Parameter(Mandatory = $true)][string]$Out)
Add-Type -AssemblyName System.Drawing
Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public static class TCap {
    [StructLayout(LayoutKind.Sequential)] public struct RECT { public int Left, Top, Right, Bottom; }
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] public static extern IntPtr FindWindow(string cls, string name);
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
    [DllImport("user32.dll")] public static extern bool PrintWindow(IntPtr h, IntPtr hdc, uint flags);
    [DllImport("user32.dll")] public static extern bool SetProcessDpiAwarenessContext(IntPtr value);
}
"@
$dpi = if ([TCap]::SetProcessDpiAwarenessContext([IntPtr](-4))) { 'ok' } else { 'refused' }
if (-not (Test-Path $OutDir)) { [void](New-Item -ItemType Directory -Path $OutDir) }
foreach ($kind in 'screen', 'print') { if (Test-Path (Join-Path $OutDir ($Name + '-' + $kind + '.png'))) { throw ('exists already: ' + $Name) } }
$tray = [TCap]::FindWindow('Shell_TrayWnd', $null)
if ($tray -eq [IntPtr]::Zero) { throw 'no Shell_TrayWnd' }
$r = New-Object TCap+RECT
if (-not [TCap]::GetWindowRect($tray, [ref]$r)) { throw 'taskbar rectangle not readable' }
$w = $r.Right - $r.Left; $h = $r.Bottom - $r.Top
$stamp = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ss.fffZ")
$screen = New-Object System.Drawing.Bitmap $w, $h
$g = [System.Drawing.Graphics]::FromImage($screen)
$g.CopyFromScreen($r.Left, $r.Top, 0, 0, (New-Object System.Drawing.Size $w, $h))
$g.Dispose(); $screen.Save((Join-Path $OutDir ($Name + '-screen.png'))); $screen.Dispose()
$print = New-Object System.Drawing.Bitmap $w, $h
$g = [System.Drawing.Graphics]::FromImage($print)
$hdc = $g.GetHdc()
$ok = [TCap]::PrintWindow($tray, $hdc, 2)
$g.ReleaseHdc($hdc); $g.Dispose(); $print.Save((Join-Path $OutDir ($Name + '-print.png'))); $print.Dispose()
$line = "$stamp name=$Name dpi=$dpi tray=0x$($tray.ToInt64().ToString('x')) rect=$($r.Left),$($r.Top),$($r.Right),$($r.Bottom) print_window=$ok"
[System.IO.File]::AppendAllText($Out, $line + "`r`n", [System.Text.Encoding]::UTF8)
$line
