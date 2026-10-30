# Lab helper (#1713 order rows, not product code): the operator's screenshot cadence while order.py runs (deliberate
# difference 4). Every 20 s it captures Tally's whole window by PrintWindow to shots\<Tag>-wNNN.png and compares it with the
# first capture (the Gateway with no dialog) on a grid of every 8th pixel: the share of grid points whose colour moved by more
# than 60 (sum of R, G, B differences). Above 2 % it records "changed", keeps the image, and creates the empty file stop in
# -OutDir (a dialog or another screen = the row is VOID, nothing more is sent). It presses no key and clicks nothing.
# It ends when the file -EndFile appears or after -Seconds. One line per capture in evidence\<Tag>-watch.txt.
param([Parameter(Mandatory = $true)][string]$Tag, [Parameter(Mandatory = $true)][string]$OutDir, [Parameter(Mandatory = $true)][string]$EndFile, [int]$Seconds = 3600)
$labDir = '<lab folder>'; $watchLog = "$labDir\evidence\$Tag-watch.txt"
Add-Type -AssemblyName System.Drawing
Add-Type @'
using System; using System.Runtime.InteropServices;
public class WW { [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
 [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
 [DllImport("user32.dll")] public static extern bool PrintWindow(IntPtr h, IntPtr hdc, uint f); }
'@
function Stamp { (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
$tallyHandle = [IntPtr][int64](Get-Process tally | Where-Object { $_.MainWindowTitle -like 'TallyPrime:9001*' } | Select-Object -First 1).MainWindowHandle
function Grab {
    $winRect = New-Object WW+RECT; [void][WW]::GetWindowRect($tallyHandle, [ref]$winRect)
    $bmp = New-Object System.Drawing.Bitmap ($winRect.R - $winRect.L), ($winRect.B - $winRect.T)
    $gfx = [System.Drawing.Graphics]::FromImage($bmp); $hdc = $gfx.GetHdc(); [void][WW]::PrintWindow($tallyHandle, $hdc, 2); $gfx.ReleaseHdc($hdc); $gfx.Dispose()
    return $bmp
}
$baseImage = Grab; $baseImage.Save("$labDir\shots\$Tag-w000.png"); Add-Content -Encoding utf8 $watchLog "$(Stamp) w000 baseline $($baseImage.Width)x$($baseImage.Height)"
$untilTime = (Get-Date).AddSeconds($Seconds); $shotIndex = 0
while ((Get-Date) -lt $untilTime -and -not (Test-Path $EndFile)) {
    Start-Sleep -Seconds 20; if (Test-Path $EndFile) { break }
    $shotIndex++; $nowImage = Grab; $moved = 0; $total = 0
    for ($gx = 4; $gx -lt [Math]::Min($nowImage.Width, $baseImage.Width); $gx += 8) { for ($gy = 4; $gy -lt [Math]::Min($nowImage.Height, $baseImage.Height); $gy += 8) {
        $p1 = $baseImage.GetPixel($gx, $gy); $p2 = $nowImage.GetPixel($gx, $gy); $total++
        if ([Math]::Abs($p1.R - $p2.R) + [Math]::Abs($p1.G - $p2.G) + [Math]::Abs($p1.B - $p2.B) -gt 60) { $moved++ } } }
    $share = [Math]::Round(100.0 * $moved / [Math]::Max(1, $total), 2); $shotName = "$Tag-w{0:d3}.png" -f $shotIndex
    $nowImage.Save("$labDir\shots\$shotName"); $nowImage.Dispose()
    if ($share -gt 2) { New-Item -ItemType File -Force (Join-Path $OutDir 'stop') | Out-Null; Add-Content -Encoding utf8 $watchLog "$(Stamp) $shotName changed $share % of grid points: stop created in $OutDir"; break }
    Add-Content -Encoding utf8 $watchLog "$(Stamp) $shotName unchanged ($share %)"
}
Add-Content -Encoding utf8 $watchLog "$(Stamp) watch ended"
