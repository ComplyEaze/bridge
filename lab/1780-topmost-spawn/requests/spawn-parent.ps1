# spawn-parent.ps1 - a small test parent that starts ONE child the way the approval dialog's parent does (tally/approved_import.rs
# nonce_bound_dialog), or with plain file redirects, so that row 1780-F can change one thing at a time. Written on a Mac and NOT RUN
# anywhere. Its C# part was compiled on that Mac with Mono's mcs (no errors); the PowerShell around it was not run.
# It sends nothing to Tally and reads nothing from it.
#
#   -Target child    the dialog child: <ChildExe> --confirm-journal, fed -InputFile (a nonce line, the count line 1 and the box text).
#   -Target control  the 1780-E control box: powershell.exe -NoProfile -ExecutionPolicy Bypass -File <ControlScript> -Variant c
#                    -DelaySeconds 0 -Out <stem>.control.txt (its box has the approval box's flags; stdin is empty).
#   -Stdio tokio     stdin an anonymous pipe, stdout an anonymous pipe, stderr the NUL device; CreateProcess with handle inheritance,
#                    STARTF_USESTDHANDLES, and the creation flag CREATE_UNICODE_ENVIRONMENT (0x400) only: no CREATE_NO_WINDOW, no
#                    DETACHED_PROCESS, no breakaway, no STARTF_USESHOWWINDOW. That is what tokio's Command does with piped, piped, null.
#   -Stdio files     stdin the -InputFile, stdout <stem>.stdout, stderr <stem>.stderr, each opened with share mode READ|WRITE so that they can be read while the child runs (what Start-Process -RedirectStandard* gives).
# Like the product's parent it writes the input, closes stdin, reads stdout to its end, waits for the child, and ends the child
# (TerminateProcess) if it is still running after -TimeoutSeconds (kill_on_drop's effect). That is the ONE process this script ends:
# the child it started itself, by the pid and handle its own CreateProcess returned (the named carve-out of the run sheet from
# LAB-COMMON C2). It ends nothing else. It writes to -Out: its own pid, the command line (with the user's name scrubbed), the stdio
# kind, the creation flags, the child's pid, and at the end the exit code and the bytes read from stdout.
# <stem> is -Out without a trailing .txt: -Out C:\...\f1.txt gives f1.txt, f1.stdout, f1.stderr and f1.control.txt.
param(
    [Parameter(Mandatory = $true)][ValidateSet('child', 'control')][string]$Target,
    [Parameter(Mandatory = $true)][ValidateSet('tokio', 'files')][string]$Stdio,
    [Parameter(Mandatory = $true)][string]$Out,
    [string]$ChildExe,
    [string]$InputFile,
    [string]$ControlScript,
    [int]$TimeoutSeconds = 60
)
Add-Type -TypeDefinition @"
using System;
using System.Text;
using System.Threading;
using System.Runtime.InteropServices;
public static class Spawn {
    [StructLayout(LayoutKind.Sequential)] struct SECURITY_ATTRIBUTES { public int nLength; public IntPtr lpSecurityDescriptor; public int bInheritHandle; }
    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Unicode)] struct STARTUPINFO {
        public int cb; public string lpReserved; public string lpDesktop; public string lpTitle;
        public int dwX, dwY, dwXSize, dwYSize, dwXCountChars, dwYCountChars, dwFillAttribute, dwFlags;
        public short wShowWindow, cbReserved2; public IntPtr lpReserved2, hStdInput, hStdOutput, hStdError; }
    [StructLayout(LayoutKind.Sequential)] struct PROCESS_INFORMATION { public IntPtr hProcess, hThread; public int dwProcessId, dwThreadId; }
    [DllImport("kernel32.dll", SetLastError = true)] static extern bool CreatePipe(out IntPtr read, out IntPtr write, ref SECURITY_ATTRIBUTES sa, int size);
    [DllImport("kernel32.dll", SetLastError = true)] static extern bool SetHandleInformation(IntPtr h, int mask, int flags);
    [DllImport("kernel32.dll", CharSet = CharSet.Unicode, SetLastError = true)] static extern IntPtr CreateFile(string name, uint access, uint share, ref SECURITY_ATTRIBUTES sa, uint disposition, uint flags, IntPtr template);
    [DllImport("kernel32.dll", CharSet = CharSet.Unicode, SetLastError = true)] static extern bool CreateProcess(string app, StringBuilder cmd, IntPtr pa, IntPtr ta, bool inherit, uint flags, IntPtr env, string cwd, ref STARTUPINFO si, out PROCESS_INFORMATION pi);
    [DllImport("kernel32.dll", SetLastError = true)] static extern bool WriteFile(IntPtr h, byte[] buffer, int count, out int written, IntPtr overlapped);
    [DllImport("kernel32.dll", SetLastError = true)] static extern bool ReadFile(IntPtr h, byte[] buffer, int count, out int read, IntPtr overlapped);
    [DllImport("kernel32.dll")] static extern uint WaitForSingleObject(IntPtr h, uint ms);
    [DllImport("kernel32.dll")] static extern bool GetExitCodeProcess(IntPtr h, out uint code);
    [DllImport("kernel32.dll")] static extern bool TerminateProcess(IntPtr h, uint code);
    [DllImport("kernel32.dll")] static extern bool CloseHandle(IntPtr h);
    static SECURITY_ATTRIBUTES Inheritable() { SECURITY_ATTRIBUTES sa = new SECURITY_ATTRIBUTES(); sa.nLength = Marshal.SizeOf(typeof(SECURITY_ATTRIBUTES)); sa.bInheritHandle = 1; return sa; }
    static IntPtr Open(string path, bool write) {
        SECURITY_ATTRIBUTES sa = Inheritable();
        IntPtr h = CreateFile(path, write ? 0x40000000u : 0x80000000u, 3u, ref sa, write ? 2u : 3u, 0x80u, IntPtr.Zero);
        if (h == new IntPtr(-1)) throw new Exception("CreateFile failed for a path: " + Marshal.GetLastWin32Error());
        return h;
    }
    // Returns "pid=<n> exit_code=<n|killed> stdout_bytes=<n>" after the child has ended.
    public static string Run(string exe, string commandLine, string stdio, string inputPath, string stdoutPath, string stderrPath, byte[] input, int timeoutSeconds, Action<int> onStart) {
        SECURITY_ATTRIBUTES sa = Inheritable();
        STARTUPINFO si = new STARTUPINFO(); si.cb = Marshal.SizeOf(typeof(STARTUPINFO)); si.dwFlags = 0x100; // STARTF_USESTDHANDLES only
        IntPtr inRead = IntPtr.Zero, inWrite = IntPtr.Zero, outRead = IntPtr.Zero, outWrite = IntPtr.Zero, err = IntPtr.Zero;
        if (stdio == "tokio") {
            if (!CreatePipe(out inRead, out inWrite, ref sa, 0)) throw new Exception("CreatePipe failed");
            if (!CreatePipe(out outRead, out outWrite, ref sa, 0)) throw new Exception("CreatePipe failed");
            SetHandleInformation(inWrite, 1, 0); SetHandleInformation(outRead, 1, 0); // the parent's ends are not inherited
            err = Open("NUL", true);
            si.hStdInput = inRead; si.hStdOutput = outWrite; si.hStdError = err;
        } else {
            inRead = Open(inputPath, false); outWrite = Open(stdoutPath, true); err = Open(stderrPath, true);
            si.hStdInput = inRead; si.hStdOutput = outWrite; si.hStdError = err;
        }
        PROCESS_INFORMATION pi;
        StringBuilder cmd = new StringBuilder(commandLine);
        if (!CreateProcess(exe, cmd, IntPtr.Zero, IntPtr.Zero, true, 0x400u, IntPtr.Zero, null, ref si, out pi)) throw new Exception("CreateProcess failed: " + Marshal.GetLastWin32Error());
        CloseHandle(pi.hThread);
        if (inRead != IntPtr.Zero) CloseHandle(inRead);
        if (outWrite != IntPtr.Zero) CloseHandle(outWrite);
        if (err != IntPtr.Zero) CloseHandle(err);
        onStart(pi.dwProcessId);
        bool killed = false;
        IntPtr process = pi.hProcess;
        Thread watchdog = new Thread(delegate () {
            if (WaitForSingleObject(process, (uint)(timeoutSeconds * 1000)) == 0x102u) { killed = true; TerminateProcess(process, 1); }
        });
        watchdog.Start();
        long total = 0;
        if (stdio == "tokio") {
            int written;
            if (input != null && input.Length > 0) WriteFile(inWrite, input, input.Length, out written, IntPtr.Zero);
            CloseHandle(inWrite);
            byte[] buffer = new byte[4096]; int n;
            while (ReadFile(outRead, buffer, buffer.Length, out n, IntPtr.Zero) && n > 0) total += n;
            CloseHandle(outRead);
        }
        WaitForSingleObject(process, 0xFFFFFFFFu);
        uint code; GetExitCodeProcess(process, out code);
        watchdog.Join();
        CloseHandle(process);
        return "pid=" + pi.dwProcessId + " exit_code=" + (killed ? "killed_after_timeout" : code.ToString()) + " stdout_bytes_read=" + total;
    }
}
"@
function Stamp { (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ss.fffZ") }
function Scrub([string]$s) {
    if ($env:USERNAME -and $env:USERNAME.Length -ge 3) { $s = $s -replace [regex]::Escape($env:USERNAME), '<user>' }
    $s -replace '(?i)([A-Z]:\\Users\\)[^\\]+', '${1}<user>'
}
function Emit([string]$line) { [System.IO.File]::AppendAllText($Out, (Scrub $line) + "`r`n", [System.Text.Encoding]::UTF8) }
$stem = $Out -replace '\.txt$', ''
if ($Target -eq 'child') {
    if (-not $ChildExe -or -not $InputFile) { throw 'child needs -ChildExe and -InputFile' }
    $exe = $ChildExe
    $cmdline = '"' + $ChildExe + '" --confirm-journal'
    $bytes = [System.IO.File]::ReadAllBytes($InputFile)
} else {
    if (-not $ControlScript) { throw 'control needs -ControlScript' }
    $exe = (Join-Path $env:SystemRoot 'System32\WindowsPowerShell\v1.0\powershell.exe')
    $cmdline = '"' + $exe + '" -NoProfile -ExecutionPolicy Bypass -File "' + $ControlScript + '" -Variant c -DelaySeconds 0 -Out "' + $stem + '.control.txt"'
    $bytes = New-Object byte[] 0
}
Emit ("SPAWN " + (Stamp) + " parent_pid=" + $PID + " target=" + $Target + " stdio=" + $Stdio + " creation_flags=0x400 startf=0x100 timeout_s=" + $TimeoutSeconds)
Emit ("SPAWN command_line=" + $cmdline)
$started = Get-Date
$result = [Spawn]::Run($exe, $cmdline, $Stdio, $InputFile, ($stem + '.stdout'), ($stem + '.stderr'), $bytes, $TimeoutSeconds, [Action[int]]{ param($childPid) Emit ("SPAWN " + (Stamp) + " child_pid=" + $childPid) })
Emit ("SPAWN " + (Stamp) + " " + $result + " elapsed_s=" + [Math]::Round(((Get-Date) - $started).TotalSeconds, 1))
