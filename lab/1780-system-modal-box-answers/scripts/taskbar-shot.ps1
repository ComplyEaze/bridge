# Lab helper (#1780-H, not product code): a DPI-aware capture of the taskbar strip (the bottom 60 physical pixels of the 1920 x 1080
# panel) to shots\<Name>.png. Reads the screen only; clicks nothing.
param([Parameter(Mandatory = $true)][string]$Name)
Add-Type -AssemblyName System.Windows.Forms, System.Drawing
Add-Type 'using System.Runtime.InteropServices; public class TBS { [DllImport("user32.dll")] public static extern bool SetProcessDPIAware(); }'
[void][TBS]::SetProcessDPIAware()
$bmp = New-Object System.Drawing.Bitmap 1920, 60; $g = [System.Drawing.Graphics]::FromImage($bmp)
$g.CopyFromScreen(0, 1020, 0, 0, (New-Object System.Drawing.Size 1920, 60)); $bmp.Save("<lab folder>\shots\$Name.png"); $g.Dispose(); $bmp.Dispose()
"$((Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ')) taskbar capture shots\$Name.png"
