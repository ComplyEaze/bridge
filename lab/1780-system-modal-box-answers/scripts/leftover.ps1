# Lab helper (#1780-H B5, not product code; row 1780-G's leftover.ps1 with the folder, markers and window title changed): the
# leftover list. Every process whose command line holds one of the sheet's four markers (pid, parent, start, image, command line),
# and every visible window whose title contains "ComplyEaze Bridge". Reads only; ends nothing. The markers are built from parts so
# that this script's own command line does not hold them.
param([Parameter(Mandatory = $true)][string]$Tag)
$out = "<lab folder>\evidence\b5-$Tag.txt"
$marks = @(('--confirm-' + 'journal'), ('--confirm-' + 'masters'), ('spawn-' + 'parent.ps1'), ('box-trace-' + 'early.ps1'))
Add-Type @'
using System; using System.Text; using System.Collections.Generic; using System.Runtime.InteropServices;
public class LW { public delegate bool P(IntPtr h, IntPtr l);
 [DllImport("user32.dll")] public static extern bool EnumWindows(P f, IntPtr l);
 [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
 [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
 public static List<string> Titled(string part) { var o = new List<string>(); EnumWindows((h,l)=>{ if (!IsWindowVisible(h)) return true; var s = new StringBuilder(512); GetWindowText(h, s, 512); var t = s.ToString(); if (t.Contains(part)) o.Add("0x" + h.ToInt64().ToString("x") + " " + t); return true; }, IntPtr.Zero); return o; } }
'@
$stamp = (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ')
$lines = @("B5 $Tag $stamp")
$procs = @(Get-CimInstance Win32_Process | Where-Object { $cl = $_.CommandLine; $cl -and ($marks | Where-Object { $cl.Contains($_) }) -and $_.ProcessId -ne $PID })
$lines += "processes: $($procs.Count)"
foreach ($p in $procs) { $lines += "  pid $($p.ProcessId) parent $($p.ParentProcessId) start $($p.CreationDate) image $($p.ExecutablePath) cmd $($p.CommandLine)" }
$w = [LW]::Titled('ComplyEaze ' + 'Bridge')
$lines += "windows: $($w.Count)"
foreach ($x in $w) { $lines += "  $x" }
$lines | Set-Content -Encoding utf8 $out
$lines
