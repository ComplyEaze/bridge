# ctx-reader.ps1 - reads, by pid, the context a process was started in. READ ONLY: it opens each process for query and memory read,
# changes nothing, sends no key or click, and sends nothing to Tally. Written on a Mac and NOT RUN anywhere.
# Its C# part was compiled on that Mac with Mono's mcs (no errors); the PowerShell around it was not run, and the PEB offsets below
# (64-bit Windows, a 64-bit PowerShell) are from the published RTL_USER_PROCESS_PARAMETERS layout and are CHECKED by -Check first.
#
#   powershell -NoProfile -ExecutionPolicy Bypass -File ctx-reader.ps1 -Check -Out <file>
#   powershell -NoProfile -ExecutionPolicy Bypass -File ctx-reader.ps1 -Pids '1234 5678' -Out <file>
#
# -Pids is one quoted string of pids separated by spaces (a comma is a digit separator on the lab PC; never write a list with one).
# Per pid it writes one CTX line: the parent pid and name, the session id, the token's integrity level (0x1000 low is 4096, 0x2000
# medium is 8192, 0x3000 high is 12288), whether the token is an AppContainer and whether it is elevated, IsProcessInJob (in ANY job:
# the job's own UI limits cannot be read from outside for an unnamed job), and from the process's startup information in its PEB:
# the console handle, the three standard handles, STARTF_* flags (0x1 USESHOWWINDOW, 0x100 USESTDHANDLES), the show-window value
# (0 is SW_HIDE), the desktop string and the window title. A CHAIN line gives the process's ancestors (up to 6), and a CONHOST
# line lists conhost.exe processes with their parents (the owner of a console).
# -Check reads this very process and a hidden PowerShell it starts, and prints "reader check ok" only if the command line it reads
# from the PEB equals CIM's, and the hidden one reads USESHOWWINDOW set with show-window 0. Otherwise "reader check FAILED": STOP.
param(
    [string]$Pids = '',
    [Parameter(Mandatory = $true)][string]$Out,
    [switch]$Check
)
Add-Type -TypeDefinition @"
using System;
using System.Text;
using System.Runtime.InteropServices;
public static class Ctx {
    [StructLayout(LayoutKind.Sequential)] struct PBI { public IntPtr ExitStatus, PebBaseAddress, AffinityMask, BasePriority, UniqueProcessId, InheritedFromUniqueProcessId; }
    [DllImport("kernel32.dll", SetLastError = true)] static extern IntPtr OpenProcess(uint access, bool inherit, int pid);
    [DllImport("kernel32.dll")] static extern bool CloseHandle(IntPtr h);
    [DllImport("kernel32.dll")] static extern bool IsProcessInJob(IntPtr h, IntPtr job, out bool result);
    [DllImport("kernel32.dll")] static extern bool ProcessIdToSessionId(uint pid, out uint session);
    [DllImport("kernel32.dll", SetLastError = true)] static extern bool ReadProcessMemory(IntPtr h, IntPtr address, byte[] buffer, IntPtr size, out IntPtr read);
    [DllImport("ntdll.dll")] static extern int NtQueryInformationProcess(IntPtr h, int cls, ref PBI info, int size, out int ret);
    [DllImport("advapi32.dll", SetLastError = true)] static extern bool OpenProcessToken(IntPtr h, uint access, out IntPtr token);
    [DllImport("advapi32.dll", SetLastError = true)] static extern bool GetTokenInformation(IntPtr token, int cls, IntPtr buffer, int length, out int ret);
    [DllImport("advapi32.dll")] static extern IntPtr GetSidSubAuthorityCount(IntPtr sid);
    [DllImport("advapi32.dll")] static extern IntPtr GetSidSubAuthority(IntPtr sid, uint n);

    static byte[] Read(IntPtr h, IntPtr address, int size) {
        byte[] buffer = new byte[size]; IntPtr read;
        if (!ReadProcessMemory(h, address, buffer, new IntPtr(size), out read)) return null;
        return buffer;
    }
    static string Str(IntPtr h, byte[] block, int offset) {
        int length = BitConverter.ToUInt16(block, offset);
        long pointer = BitConverter.ToInt64(block, offset + 8);
        if (length == 0 || pointer == 0 || length > 8192) return "";
        byte[] data = Read(h, new IntPtr(pointer), length);
        return data == null ? "<unreadable>" : Encoding.Unicode.GetString(data);
    }
    static string Token(IntPtr process) {
        IntPtr token;
        if (!OpenProcessToken(process, 8u, out token)) return "token=<unreadable>";
        try {
            string result = "";
            int ret; IntPtr buffer = Marshal.AllocHGlobal(256);
            try {
                if (GetTokenInformation(token, 25, buffer, 256, out ret)) {
                    IntPtr sid = Marshal.ReadIntPtr(buffer);
                    int count = Marshal.ReadByte(GetSidSubAuthorityCount(sid));
                    uint rid = (uint)Marshal.ReadInt32(GetSidSubAuthority(sid, (uint)(count - 1)));
                    result += "integrity_rid=0x" + rid.ToString("x");
                } else result += "integrity_rid=<unreadable>";
                if (GetTokenInformation(token, 29, buffer, 256, out ret)) result += " appcontainer=" + Marshal.ReadInt32(buffer); else result += " appcontainer=<unreadable>";
                if (GetTokenInformation(token, 20, buffer, 256, out ret)) result += " elevated=" + Marshal.ReadInt32(buffer); else result += " elevated=<unreadable>";
            } finally { Marshal.FreeHGlobal(buffer); }
            return result;
        } finally { CloseHandle(token); }
    }
    // The startup information and context of one process, one line. "cmd=" is the command line read from the PEB.
    public static string Describe(int pid) {
        if (IntPtr.Size != 8) return "CTX pid=" + pid + " <this reader needs a 64-bit PowerShell>";
        IntPtr h = OpenProcess(0x400u | 0x10u, false, pid);
        if (h == IntPtr.Zero) return "CTX pid=" + pid + " <cannot open: " + Marshal.GetLastWin32Error() + ">";
        try {
            uint session; ProcessIdToSessionId((uint)pid, out session);
            bool inJob; IsProcessInJob(h, IntPtr.Zero, out inJob);
            string line = "CTX pid=" + pid + " session=" + session + " in_job=" + inJob + " " + Token(h);
            PBI pbi = new PBI(); int ret;
            if (NtQueryInformationProcess(h, 0, ref pbi, Marshal.SizeOf(typeof(PBI)), out ret) != 0) return line + " peb=<unreadable>";
            byte[] peb = Read(h, new IntPtr(pbi.PebBaseAddress.ToInt64() + 0x20), 8);
            if (peb == null) return line + " peb=<unreadable>";
            long parameters = BitConverter.ToInt64(peb, 0);
            byte[] p = Read(h, new IntPtr(parameters), 0xF0);
            if (p == null) return line + " params=<unreadable>";
            line += " console_handle=0x" + BitConverter.ToInt64(p, 0x10).ToString("x");
            line += " std_in=0x" + BitConverter.ToInt64(p, 0x20).ToString("x") + " std_out=0x" + BitConverter.ToInt64(p, 0x28).ToString("x") + " std_err=0x" + BitConverter.ToInt64(p, 0x30).ToString("x");
            line += " startf=0x" + BitConverter.ToInt32(p, 0xA4).ToString("x") + " show_window=" + BitConverter.ToInt32(p, 0xA8);
            line += " desktop=" + Str(h, p, 0xC0);
            line += " title_chars=" + Str(h, p, 0xB0).Length;
            line += " cmd=" + Str(h, p, 0x70);
            return line;
        } finally { CloseHandle(h); }
    }
}
"@
function Emit([string]$line) { [System.IO.File]::AppendAllText($Out, $line + "`r`n", [System.Text.Encoding]::UTF8) }
function Chain([int]$id) {
    $line = "CHAIN pid=" + $id
    $current = $id
    for ($i = 0; $i -lt 6; $i++) {
        $p = Get-CimInstance Win32_Process -Filter ("ProcessId=" + $current) -ErrorAction SilentlyContinue
        if (-not $p) { break }
        $line += " <- " + $p.ParentProcessId + ":" + $p.Name
        $current = $p.ParentProcessId
        if ($current -eq 0) { break }
    }
    $line
}
if ($Check) {
    $mine = [Ctx]::Describe($PID)
    $cim = (Get-CimInstance Win32_Process -Filter ("ProcessId=" + $PID)).CommandLine
    $okSelf = $mine.Contains("cmd=" + $cim)
    $hidden = Start-Process powershell -WindowStyle Hidden -ArgumentList '-NoProfile -Command Start-Sleep -Seconds 20' -PassThru
    Start-Sleep -Milliseconds 1500
    $theirs = [Ctx]::Describe($hidden.Id)
    Stop-Process -Id $hidden.Id -ErrorAction SilentlyContinue
    $okHidden = $false
    if ($theirs -match 'startf=0x([0-9a-f]+) show_window=([0-9]+) ') { $okHidden = ((([Convert]::ToInt32($Matches[1], 16)) -band 1) -eq 1) -and ($Matches[2] -eq '0') }
    Emit $mine; Emit $theirs
    Emit ("reader check " + $(if ($okSelf -and $okHidden) { 'ok' } else { 'FAILED' }) + " (own command line equal to CIM's: " + $okSelf + "; hidden PowerShell reads USESHOWWINDOW with show-window 0: " + $okHidden + ")")
    exit 0
}
foreach ($token in ($Pids -split ' ')) {
    if ($token -match '^[0-9]+$') {
        Emit ([Ctx]::Describe([int]$token))
        Emit (Chain ([int]$token))
    }
}
foreach ($c in (Get-CimInstance Win32_Process -Filter "Name='conhost.exe'" -ErrorAction SilentlyContinue)) { Emit ("CONHOST pid=" + $c.ProcessId + " parent=" + $c.ParentProcessId) }
try { Emit ("NAMED_JOB_OBJECTS count=" + @(Get-CimInstance Win32_NamedJobObject -ErrorAction Stop).Count) } catch { Emit "NAMED_JOB_OBJECTS <not available>" }
