# Lab helper (#1211, not product code): one DPI-aware capture of the whole screen (Tally's ledger display totals sit below
# what the window capture keeps) to shots\<Name>.png; prints size and sha256. Usage: -Name <name>
param([Parameter(Mandatory = $true)][string]$Name)
Add-Type -AssemblyName System.Drawing
Add-Type @'
using System; using System.Runtime.InteropServices;
public class FS { [DllImport("user32.dll")] public static extern bool SetProcessDPIAware(); [DllImport("user32.dll")] public static extern int GetSystemMetrics(int i); }
'@
[void][FS]::SetProcessDPIAware()
$w = [FS]::GetSystemMetrics(0); $h = [FS]::GetSystemMetrics(1)
$bmp = New-Object System.Drawing.Bitmap $w, $h
$g = [System.Drawing.Graphics]::FromImage($bmp); $g.CopyFromScreen(0, 0, 0, 0, $bmp.Size); $g.Dispose()
$out = Join-Path '<LAB_DIR>\shots' ($Name + '.png')
$bmp.Save($out, [System.Drawing.Imaging.ImageFormat]::Png); $bmp.Dispose()
$f = Get-Item $out
"capture $Name.png $($f.Length) B sha256 $((Get-FileHash $out -Algorithm SHA256).Hash.ToLower()) ($w x $h)"
