# Row 1381-BX's copy of row 2569-R's starth.ps1 (blob aef601eb), with one change, marked 1381-BX on each line it touches: the one argument is
# chosen by -Mode (stock: --confirm-stock-item; masters: --confirm-masters) in place of the fixed --confirm-journal; the
# mode is named by a word so that PowerShell never reads a value that begins with a dash as a parameter name.
# Lab helper (row 2569-R; not product code): starts `<exe> <mode>` with an explicit first show state
# (STARTF_USESHOWWINDOW, wShowWindow = -Show: 1 normal, 7 minimised without activation) and no console window, its
# standard input read from -In and its standard output and error written to new files. Prints the child's pid at
# once, then waits up to 600 s for it to exit and prints its exit code. Answers nothing and clicks nothing.
# Usage: powershell -NoProfile -File startc.ps1 -Exe <exe> -Mode <stock|masters> -Show <1|7> -In <input> -OutFile <new> -ErrFile <new>
param([string]$Exe, [string]$Mode, [int]$Show, [string]$In, [string]$OutFile, [string]$ErrFile)
if ($Show -ne 1 -and $Show -ne 7) { throw "Show must be 1 or 7" }
if ($Mode -cne 'stock' -and $Mode -cne 'masters') { throw "Mode must be stock or masters" }  # 1381-BX
$ModeArg = if ($Mode -ceq 'stock') { '--confirm-stock-item' } else { '--confirm-masters' }  # 1381-BX
# Resolve every path against this shell's location once, so .NET and Test-Path read the same files.
$Exe = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($Exe)
$In = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($In)
$OutFile = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($OutFile)
$ErrFile = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($ErrFile)
if (-not (Test-Path -LiteralPath $In)) { throw "missing: $In" }
foreach ($f in $OutFile, $ErrFile) { if (Test-Path -LiteralPath $f) { throw "exists: $f" } }
Add-Type @"
using System;
using System.Runtime.InteropServices;
public static class StartH {
  [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Unicode)]
  public struct StartupInfo {
    public int cb; public string lpReserved; public string lpDesktop; public string lpTitle;
    public int dwX, dwY, dwXSize, dwYSize, dwXCountChars, dwYCountChars, dwFillAttribute, dwFlags;
    public short wShowWindow, cbReserved2; public IntPtr lpReserved2, hStdInput, hStdOutput, hStdError;
  }
  [StructLayout(LayoutKind.Sequential)]
  public struct ProcessInfo { public IntPtr hProcess, hThread; public int dwProcessId, dwThreadId; }
  [DllImport("kernel32.dll", SetLastError = true, CharSet = CharSet.Unicode)]
  public static extern bool CreateProcessW(string app, System.Text.StringBuilder cmd, IntPtr pa, IntPtr ta,
    bool inherit, uint flags, IntPtr env, string dir, ref StartupInfo si, out ProcessInfo pi);
  [DllImport("kernel32.dll", SetLastError = true)]
  public static extern bool SetHandleInformation(IntPtr h, uint mask, uint flags);
  [DllImport("kernel32.dll")] public static extern uint WaitForSingleObject(IntPtr h, uint ms);
  [DllImport("kernel32.dll")] public static extern bool GetExitCodeProcess(IntPtr h, out uint code);
  [DllImport("kernel32.dll")] public static extern bool CloseHandle(IntPtr h);
}
"@
$inStream = [System.IO.File]::Open($In, 'Open', 'Read', 'Read')
$outStream = [System.IO.File]::Open($OutFile, 'CreateNew', 'Write', 'None')
$errStream = [System.IO.File]::Open($ErrFile, 'CreateNew', 'Write', 'None')
foreach ($s in $inStream, $outStream, $errStream) {
  if (-not [StartH]::SetHandleInformation($s.SafeFileHandle.DangerousGetHandle(), 1, 1)) { throw "handle not inheritable" }
}
$si = New-Object StartH+StartupInfo
$si.cb = [System.Runtime.InteropServices.Marshal]::SizeOf($si)
$si.dwFlags = 0x101  # STARTF_USESHOWWINDOW | STARTF_USESTDHANDLES
$si.wShowWindow = [int16]$Show
$si.hStdInput = $inStream.SafeFileHandle.DangerousGetHandle()
$si.hStdOutput = $outStream.SafeFileHandle.DangerousGetHandle()
$si.hStdError = $errStream.SafeFileHandle.DangerousGetHandle()
$cmd = New-Object System.Text.StringBuilder ('"' + $Exe + '" ' + $ModeArg)  # 1381-BX: one argument, the mode's
$pi = New-Object StartH+ProcessInfo
# CREATE_NO_WINDOW (0x08000000): no console window, so no window but the box can take the foreground.
# lpCurrentDirectory must be a real null (inherit this shell's folder): PowerShell passes $null to a string
# parameter as "", which CreateProcessW refuses with error 123.
if (-not [StartH]::CreateProcessW($Exe, $cmd, [IntPtr]::Zero, [IntPtr]::Zero, $true, 0x08000000, [IntPtr]::Zero, [NullString]::Value, [ref]$si, [ref]$pi)) {
  throw ("CreateProcess failed: " + [System.Runtime.InteropServices.Marshal]::GetLastWin32Error())
}
"STARTED pid=" + $pi.dwProcessId + " show=" + $Show + " arg=" + $ModeArg + " at " + [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ss.fffZ')  # 1381-BX: the argument printed
$waited = [StartH]::WaitForSingleObject($pi.hProcess, 600000)
$code = [uint32]0
[void][StartH]::GetExitCodeProcess($pi.hProcess, [ref]$code)
if ($waited -eq 0) { "EXITED code=" + $code + " at " + [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
else { "NOT EXITED within 600 s (wait result " + $waited + ")" }
[void][StartH]::CloseHandle($pi.hThread)
[void][StartH]::CloseHandle($pi.hProcess)
$inStream.Close(); $outStream.Close(); $errStream.Close()
