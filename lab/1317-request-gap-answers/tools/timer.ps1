# Lab helper (#1317 timing rule, not product code): captures Tally's window back to back with UTC millisecond times,
# and logs every change of Windows' last-input time (the moment a key or click reached the system), until -Seconds
# have passed or a file named STOP-TIMER exists in -Out. Reads the screen only; sends no key and no request.
# Usage: powershell -NoProfile -File timer.ps1 -Handle <hwnd> -Out <new folder> -Seconds <n>
param([Parameter(Mandatory = $true)][int]$Handle, [Parameter(Mandatory = $true)][string]$Out, [int]$Seconds = 120)
$ErrorActionPreference = 'Stop'
if (Test-Path $Out) { throw "folder exists: $Out" }
New-Item -ItemType Directory -Path $Out | Out-Null
Add-Type -AssemblyName System.Drawing
Add-Type @'
using System; using System.Runtime.InteropServices;
public class TimW {
  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
  [StructLayout(LayoutKind.Sequential)] public struct LASTINPUTINFO { public uint cbSize; public uint dwTime; }
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern bool PrintWindow(IntPtr h, IntPtr hdc, uint f);
  [DllImport("user32.dll")] public static extern bool GetLastInputInfo(ref LASTINPUTINFO p);
  [DllImport("kernel32.dll")] public static extern uint GetTickCount();
}
'@
$h = [IntPtr]$Handle
$r = New-Object TimW+RECT
[TimW]::GetWindowRect($h, [ref]$r) | Out-Null
$w = $r.R - $r.L; $hh = $r.B - $r.T
$jpg = [System.Drawing.Imaging.ImageCodecInfo]::GetImageEncoders() | Where-Object { $_.MimeType -eq 'image/jpeg' }
$ep = New-Object System.Drawing.Imaging.EncoderParameters 1
$ep.Param[0] = New-Object System.Drawing.Imaging.EncoderParameter ([System.Drawing.Imaging.Encoder]::Quality), 60L
$frames = Join-Path $Out 'frames.tsv'; $inputs = Join-Path $Out 'inputs.tsv'
"seq`tutc`tfile" | Set-Content $frames
"utc_input" | Set-Content $inputs
$lii = New-Object TimW+LASTINPUTINFO; $lii.cbSize = 8
$lastInput = 0
$end = (Get-Date).AddSeconds($Seconds); $seq = 0
while ((Get-Date) -lt $end -and -not (Test-Path (Join-Path $Out 'STOP-TIMER'))) {
    [void][TimW]::GetLastInputInfo([ref]$lii)
    if ($lii.dwTime -ne $lastInput) {
        $lastInput = $lii.dwTime
        $ago = [TimW]::GetTickCount() - $lii.dwTime
        (Get-Date).ToUniversalTime().AddMilliseconds(-$ago).ToString('HH:mm:ss.fff') | Add-Content $inputs
    }
    $bmp = New-Object System.Drawing.Bitmap $w, $hh
    $g = [System.Drawing.Graphics]::FromImage($bmp); $hdc = $g.GetHdc()
    [void][TimW]::PrintWindow($h, $hdc, 2)
    $g.ReleaseHdc($hdc); $g.Dispose()
    $t = (Get-Date).ToUniversalTime().ToString('HH:mm:ss.fff')
    $seq++
    $name = ('{0:D5}.jpg' -f $seq)
    $small = New-Object System.Drawing.Bitmap $bmp, ([int]($w / 2)), ([int]($hh / 2))
    $small.Save((Join-Path $Out $name), $jpg, $ep); $small.Dispose(); $bmp.Dispose()
    "$seq`t$t`t$name" | Add-Content $frames
}
"done $seq frames" | Add-Content $frames
