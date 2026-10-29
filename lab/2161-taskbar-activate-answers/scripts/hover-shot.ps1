# Lab helper (#2161, not product code): stands in for the screen tool's screenshot and the lab's hover while a box is open (the
# screen tool hides windows of apps not granted to it, which minimised the B-h box). Per-monitor DPI aware (physical pixels). Takes
# a whole-screen screen copy to shots\<Name>-full.png; then for each hover point (physical x,y) moves the pointer there (no click),
# waits 1.5 s and copies the bottom 420 px of the screen to shots\<Name>-hover<k>.png. Appends one line per image to
# evidence\hover.txt. Clicks nothing; sends no key.
param([Parameter(Mandatory = $true)][string]$Name, [string]$Points = '885 1055 633 1055')
Add-Type -AssemblyName System.Windows.Forms, System.Drawing
Add-Type 'using System; using System.Runtime.InteropServices; public class HS { [DllImport("user32.dll")] public static extern bool SetProcessDPIAware(); [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y); }'
[void][HS]::SetProcessDPIAware()
$rd = '<lab folder>'
function Stamp { (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
function Save-ScreenPart([int]$top, [int]$h, [string]$out) {
    $bmp = New-Object System.Drawing.Bitmap 1920, $h; $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.CopyFromScreen(0, $top, 0, 0, (New-Object System.Drawing.Size 1920, $h)); $bmp.Save($out); $g.Dispose(); $bmp.Dispose()
}
Save-ScreenPart 0 1080 "$rd\shots\$Name-full.png"; Add-Content -Encoding utf8 "$rd\evidence\hover.txt" "$(Stamp) $Name full screen copy shots\$Name-full.png"
$p = $Points -split '\s+'; $k = 1
for ($i = 0; $i + 1 -lt $p.Count; $i += 2) {
    $x = [int]$p[$i]; $y = [int]$p[$i + 1]; [void][HS]::SetCursorPos($x, $y); Start-Sleep -Milliseconds 1500
    Save-ScreenPart 660 420 "$rd\shots\$Name-hover$k.png"; Add-Content -Encoding utf8 "$rd\evidence\hover.txt" "$(Stamp) $Name hover $k at physical ($x,$y): shots\$Name-hover$k.png"; $k++
}
[void][HS]::SetCursorPos(1700, 600)
"$(Stamp) $Name done"
