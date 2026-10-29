# Lab helper (#2161 part C, not product code): finds the rightmost task button in a taskbar PrintWindow image (the box's button is
# the newest, so the last one drawn). The bar's background is the most common colour of the middle row between x 420 and 1250 (the
# task list ends at 1261, and the tray's icons start right of it). Scans that row band from x 1250 leftwards for pixels that differ
# from the background, takes the rightmost run, and prints its centre as "x y" in physical screen pixels (y = 1055, the taskbar's
# middle). Reads an image only; clicks nothing.
param([Parameter(Mandatory = $true)][string]$Image, [int]$Right = 1250)
Add-Type -AssemblyName System.Drawing
$b = [System.Drawing.Bitmap]::FromFile($Image)
$hist = @{}
for ($x = 420; $x -le $Right; $x++) { $p = $b.GetPixel($x, 25); $k = "$($p.R),$($p.G),$($p.B)"; $hist[$k] = 1 + [int]$hist[$k] }
$bgk = ($hist.GetEnumerator() | Sort-Object Value -Descending | Select-Object -First 1).Key -split ','
$bg = @([int]$bgk[0], [int]$bgk[1], [int]$bgk[2]); $last = -1; $first = -1
for ($x = $Right; $x -ge 420; $x--) {
    $found = $false
    foreach ($y in 15, 20, 25, 30, 35) {
        $p = $b.GetPixel($x, $y)
        if ([Math]::Abs($p.R - $bg[0]) + [Math]::Abs($p.G - $bg[1]) + [Math]::Abs($p.B - $bg[2]) -gt 90) { $found = $true; break }
    }
    if ($found -and $last -lt 0) { $last = $x }
    if ($last -ge 0 -and -not $found -and ($last - $x) -gt 6) { $first = $x + 1; break }
}
$b.Dispose()
if ($last -lt 0) { 'none'; exit 1 }
"$([int](($first + $last) / 2)) 1055"
