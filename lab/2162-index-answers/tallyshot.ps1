# Lab helper (#2162 index rows, not product code; 2377-b's helper): saves Tally's whole window by PrintWindow (PW_RENDERFULLCONTENT), so the
# parts that another window covers are drawn too; DPI-aware. Reads only; presses no key. Prints time, size, sha256.
# Usage: powershell -NoProfile -File tallyshot.ps1 <output png>
param([string]$OutPng)
Add-Type -AssemblyName System.Drawing
Add-Type -Namespace TS -Name U -MemberDefinition @'
[DllImport("user32.dll")] public static extern bool SetProcessDpiAwarenessContext(System.IntPtr v);
[DllImport("user32.dll")] public static extern bool GetWindowRect(System.IntPtr h, out RECT r);
[DllImport("user32.dll")] public static extern bool PrintWindow(System.IntPtr h, System.IntPtr dc, uint f);
public struct RECT { public int Left, Top, Right, Bottom; }
'@
[TS.U]::SetProcessDpiAwarenessContext([System.IntPtr](-4)) | Out-Null
$tallyProc = @(Get-Process tally -ErrorAction Stop)
if ($tallyProc.Count -ne 1) { "tally.exe processes: $($tallyProc.Count): not captured"; exit 2 }
$tallyHwnd = $tallyProc[0].MainWindowHandle
$rect = New-Object TS.U+RECT
[TS.U]::GetWindowRect($tallyHwnd, [ref]$rect) | Out-Null
$w = $rect.Right - $rect.Left; $h = $rect.Bottom - $rect.Top
$bmp = New-Object System.Drawing.Bitmap $w, $h
$gfx = [System.Drawing.Graphics]::FromImage($bmp)
$dc = $gfx.GetHdc(); $ok = [TS.U]::PrintWindow($tallyHwnd, $dc, 2); $gfx.ReleaseHdc($dc)
$bmp.Save($OutPng, [System.Drawing.Imaging.ImageFormat]::Png); $gfx.Dispose(); $bmp.Dispose()
$t = (Get-Date).ToUniversalTime().ToString('HH:mm:ssZ'); $hash = (Get-FileHash $OutPng -Algorithm SHA256).Hash.ToLower()
"$t pid $($tallyProc[0].Id) window ${w}x${h} PrintWindow $ok $((Get-Item $OutPng).Length) B $hash"
