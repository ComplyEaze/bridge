# Lab script (not product code): open the real approval box through the executable's child mode with prepared text, and
# record its window rectangle against the screen's work area. NO Tally, NO posting. The box is answered by MOUSE (No).
# Usage:  powershell -NoProfile -ExecutionPolicy Bypass -File run-box.ps1 -Exe <path to bridge_mcp.exe> -InputDir <folder holding inputs\> -Case win-real-10-40 -Setting "1366x768 100%" [-DryRun]
# Untested on Windows when written (written on a Mac without PowerShell): run with -DryRun first, then one real case.
param(
  [Parameter(Mandatory=$true)][string]$Exe,
  [Parameter(Mandatory=$true)][string]$InputDir,
  [Parameter(Mandatory=$true)][ValidateSet('win-1','win-10','win-real-1','win-real-10-39','win-real-10-40')][string]$Case,
  [Parameter(Mandatory=$true)][string]$Setting,
  [switch]$DryRun
)
$ErrorActionPreference = 'Stop'
$expect = @{
  'win-1'          = '3557adc4bfecdecb1f9c43b07b034d5669c050ba56324c3a6e736cbcbb77fead'
  'win-10'         = '73920de0defeb44c19f078a0398cdc65a999e5b68026a7ab2561bece1cc3fbc0'
  'win-real-1'     = '549489d6285aaefb3883e094b0ea29273d01531dd336ed97f4dd3d6b75787929'
  'win-real-10-39' = 'deded571f2a95e24fa34d1c8bbce0f54ca1280ab59843e708be445c53105fb20'
  'win-real-10-40' = '48fabd62e668e9dad56aeaa3c5f69176bacf14c2b9423dac64e39d81b8c25a74'
}
$file = Join-Path $InputDir "$Case.txt"
if (-not (Test-Path -LiteralPath $file)) { throw "missing input $file" }
$got = (Get-FileHash -Algorithm SHA256 -LiteralPath $file).Hash.ToLower()
if ($got -ne $expect[$Case]) { throw "STOP: $Case.txt hash $got differs from $($expect[$Case])" }
$exeHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $Exe).Hash.ToLower()
$count = [int](Get-Content -LiteralPath $file -TotalCount 2)[1]
$dash = [string][char]0x2014
$title = if ($count -eq 1) { "ComplyEaze Bridge $dash post this voucher?" } else { "ComplyEaze Bridge $dash post $count vouchers?" }
Add-Type -AssemblyName System.Windows.Forms
Add-Type @'
using System; using System.Runtime.InteropServices;
public static class Box {
  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern IntPtr FindWindowW(string cls, string title);
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
}
'@
[void][Box]::SetProcessDPIAware()
$wa = [System.Windows.Forms.Screen]::PrimaryScreen.WorkingArea
$bounds = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
$cmd = '"{0}" --confirm-journal < "{1}"' -f $Exe, $file
Write-Host "case=$Case setting='$Setting' exe_sha256=$exeHash"
Write-Host "screen=$($bounds.Width)x$($bounds.Height) workarea=$($wa.Width)x$($wa.Height) at ($($wa.X),$($wa.Y))"
Write-Host "title expected: $title"
Write-Host "command: cmd.exe /s /c `"$cmd`""
if ($DryRun) { Write-Host 'DRY RUN: nothing opened'; return }
Write-Host 'The box opens next. Take a WHOLE-SCREEN screenshot with the lab tool (not the keyboard), then click NO with the MOUSE. Type nothing.'
$job = Start-Job -ArgumentList $title -ScriptBlock {
  param($t)
  Add-Type @'
using System; using System.Runtime.InteropServices;
public static class Box2 {
  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern IntPtr FindWindowW(string cls, string title);
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
}
'@
  [void][Box2]::SetProcessDPIAware()
  for ($i = 0; $i -lt 100; $i++) {
    $h = [Box2]::FindWindowW($null, $t)
    if ($h -ne [IntPtr]::Zero) { $r = New-Object Box2+RECT; [void][Box2]::GetWindowRect($h, [ref]$r); return "$($r.L),$($r.T),$($r.R),$($r.B)" }
    Start-Sleep -Milliseconds 100
  }
  return 'not found in 10 s'
}
$started = Get-Date
$p = Start-Process -FilePath 'cmd.exe' -ArgumentList ('/s /c "' + $cmd + '"') -PassThru -Wait -NoNewWindow
$ended = Get-Date
$rect = Receive-Job -Job $job -Wait -AutoRemoveJob
Write-Host "exit=$($p.ExitCode) (expect 1 when answered No) rect(L,T,R,B)=$rect elapsed_s=$([int]($ended-$started).TotalSeconds)"
if ($rect -match '^-?\d+,-?\d+,-?\d+,-?\d+$') {
  $v = $rect.Split(',') | ForEach-Object { [int]$_ }
  Write-Host "box height=$($v[3]-$v[1]) width=$($v[2]-$v[0]); workarea height=$($wa.Height); top_off_screen=$($v[1] -lt $wa.Y); bottom_off_workarea=$($v[3] -gt ($wa.Y + $wa.Height))"
}
