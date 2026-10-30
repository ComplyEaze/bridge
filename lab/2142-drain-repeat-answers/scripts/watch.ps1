# Lab helper (#2142 drain repeat, not product code): the capture watcher of rule 4. It never waits more than 1 s at a time:
# each second it checks whether probe.py (the pid given) has exited, and ends within 2 s of that exit; on the way it captures
# Tally's whole window (PrintWindow) every 30 s and compares each capture with the first on a grid of every 8th pixel (a change
# over 2 % of grid points creates the stop file, which probe.py checks before each probe). It presses no key, clicks nothing
# and ends no process. One line per capture in evidence\<Tag>-watch.txt.
param([Parameter(Mandatory = $true)][string]$Tag, [Parameter(Mandatory = $true)][int]$ProbePid, [Parameter(Mandatory = $true)][string]$StopFile)
$labDir = '<lab folder>'; $watchLog = "$labDir\evidence\$Tag-watch.txt"
Add-Type -AssemblyName System.Drawing
Add-Type @'
using System; using System.Runtime.InteropServices;
public class WR { [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
 [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
 [DllImport("user32.dll")] public static extern bool PrintWindow(IntPtr h, IntPtr hdc, uint f); }
'@
function Stamp { (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
$tallyHandle = [IntPtr][int64](Get-Process tally | Select-Object -First 1).MainWindowHandle
function Grab {
    $winRect = New-Object WR+RECT; [void][WR]::GetWindowRect($tallyHandle, [ref]$winRect)
    $bmp = New-Object System.Drawing.Bitmap ($winRect.R - $winRect.L), ($winRect.B - $winRect.T)
    $gfx = [System.Drawing.Graphics]::FromImage($bmp); $hdc = $gfx.GetHdc(); [void][WR]::PrintWindow($tallyHandle, $hdc, 2); $gfx.ReleaseHdc($hdc); $gfx.Dispose()
    return $bmp
}
$baseImage = Grab; $baseImage.Save("$labDir\shots\$Tag-w000.png"); Add-Content -Encoding utf8 $watchLog "$(Stamp) w000 baseline"
$nextCapture = (Get-Date).AddSeconds(30); $shotIndex = 0
while ($true) {
    Start-Sleep -Milliseconds 1000
    if (-not (Get-Process -Id $ProbePid -ErrorAction SilentlyContinue)) { Add-Content -Encoding utf8 $watchLog "$(Stamp) probe.py pid $ProbePid has exited: watch ends"; break }
    if ((Get-Date) -ge $nextCapture) {
        $shotIndex++; $nowImage = Grab; $moved = 0; $total = 0
        for ($gx = 4; $gx -lt [Math]::Min($nowImage.Width, $baseImage.Width); $gx += 8) { for ($gy = 4; $gy -lt [Math]::Min($nowImage.Height, $baseImage.Height); $gy += 8) {
            $p1 = $baseImage.GetPixel($gx, $gy); $p2 = $nowImage.GetPixel($gx, $gy); $total++
            if ([Math]::Abs($p1.R - $p2.R) + [Math]::Abs($p1.G - $p2.G) + [Math]::Abs($p1.B - $p2.B) -gt 60) { $moved++ } } }
        $share = [Math]::Round(100.0 * $moved / [Math]::Max(1, $total), 2); $shotName = "$Tag-w{0:d3}.png" -f $shotIndex
        $nowImage.Save("$labDir\shots\$shotName"); $nowImage.Dispose()
        if ($share -gt 2) { New-Item -ItemType File -Force $StopFile | Out-Null; Add-Content -Encoding utf8 $watchLog "$(Stamp) $shotName changed $share %: stop file $StopFile created" }
        else { Add-Content -Encoding utf8 $watchLog "$(Stamp) $shotName unchanged ($share %)" }
        $nextCapture = (Get-Date).AddSeconds(30)
    }
}
