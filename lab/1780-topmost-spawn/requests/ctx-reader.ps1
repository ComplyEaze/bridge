# ctx-reader.ps1 - reads, by pid, the context a process was started in. READ ONLY: it opens each process for query and memory read,
# changes nothing, sends no key or click, ends no process and sends nothing to Tally. Written on a Mac and NOT RUN anywhere.
# Its C# part was compiled on that Mac with Mono's mcs (no errors); the PowerShell around it was not run, and the PEB offsets below
# (64-bit Windows, a 64-bit PowerShell) are from the published RTL_USER_PROCESS_PARAMETERS layout and are CHECKED by -Check first.
#
#   powershell -NoProfile -ExecutionPolicy Bypass -File ctx-reader.ps1 -Check -Out <file>
#   powershell -NoProfile -ExecutionPolicy Bypass -File ctx-reader.ps1 -Pids '1234 5678' -Out <file>
#   powershell -NoProfile -ExecutionPolicy Bypass -File ctx-reader.ps1 -SpawnFile <cell>.txt -DelaySeconds 6 -Out <file>
#   powershell -NoProfile -ExecutionPolicy Bypass -File ctx-reader.ps1 -WaitForConfirmChild -DelaySeconds 6 -Out <file>
#
# The last two are PRE-ARMED: start them BEFORE the box exists and they wait by themselves, so that no key has to be typed while a box
# has the keyboard. -SpawnFile waits (up to -WaitSeconds) for the `SPAWN ... child_pid=` line that spawn-parent.ps1 writes, then reads
# the spawn-parent process and its child. -WaitForConfirmChild waits for a bridge_mcp.exe whose command line holds --confirm-journal
# (the approval dialog child), then reads it, its parent and its parent's parent. Both wait -DelaySeconds after the process is seen.
# -Pids is one quoted string of pids separated by spaces (a comma is a digit separator on the lab PC; never write a list with one).
# Per pid it writes one CTX line: the session id, whether the process is 32-bit (wow64=True: its PEB is then read as the native 64-bit
# one and the line says so), the token's integrity level (0x1000 low is 4096, 0x2000 medium is 8192, 0x3000 high is 12288), whether the
# token is an AppContainer and whether it is elevated, IsProcessInJob (in ANY job: the job's own UI limits cannot be read from outside
# for an unnamed job), and from the process's startup information in its PEB: the console handle, the three standard handles,
# STARTF_* flags (0x1 USESHOWWINDOW, 0x100 USESTDHANDLES), the show-window value (0 is SW_HIDE), the desktop string and the length of
# the window title. A CHAIN line gives the process's ancestors (up to 6), and a CONHOST line lists conhost.exe processes with their
# parents (the owner of a console). The user's name is replaced by <user> in every line before it is written.
# -Check reads this very process and a hidden PowerShell it starts (which ends by itself after 20 s; nothing is ended). It prints
# "reader check ok" only if ALL of these hold: CIM returned a command line and the one read from the PEB equals it; the standard
# handles read from the PEB equal GetStdHandle's; the desktop reads WinSta0\Default; the console handle is non-zero and equals none
# of the three standard handles; this reader is 64-bit and not wow64; and the hidden one reads USESHOWWINDOW set with show-window 0.
# Otherwise "reader check FAILED" with each result: STOP.
param(
    [string]$Pids = '',
    [Parameter(Mandatory = $true)][string]$Out,
    [switch]$Check,
    [string]$SpawnFile = '',
    [switch]$WaitForConfirmChild,
    [int]$DelaySeconds = 6,
    [int]$WaitSeconds = 180
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
    [DllImport("kernel32.dll")] static extern IntPtr GetStdHandle(int which);
    [DllImport("kernel32.dll")] static extern bool IsWow64Process(IntPtr h, out bool wow64);
    // This process's own standard handles as Windows reports them, in the same form as the CTX line.
    public static string OwnStd() {
        return "std_in=0x" + GetStdHandle(-10).ToInt64().ToString("x") + " std_out=0x" + GetStdHandle(-11).ToInt64().ToString("x") + " std_err=0x" + GetStdHandle(-12).ToInt64().ToString("x");
    }

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
            bool wow64; IsWow64Process(h, out wow64);
            string line = "CTX pid=" + pid + " session=" + session + " wow64=" + wow64 + " in_job=" + inJob + " " + Token(h);
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
function Scrub([string]$s) {
    if ($env:USERNAME -and $env:USERNAME.Length -ge 3) { $s = $s -replace [regex]::Escape($env:USERNAME), '<user>' }
    $s -replace '(?i)([A-Z]:\\Users\\)[^\\]+', '${1}<user>'
}
function Emit([string]$line) { [System.IO.File]::AppendAllText($Out, (Scrub $line) + "`r`n", [System.Text.Encoding]::UTF8) }
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
    $okCim = [bool]$cim
    $okSelf = $okCim -and $mine.Contains("cmd=" + $cim)
    $okStd = $mine.Contains([Ctx]::OwnStd())
    $okDesktop = $mine -match 'desktop=WinSta0\\Default( |$)'
    $okConsole = $false
    if ($mine -match 'console_handle=(0x[0-9a-f]+) std_in=(0x[0-9a-f]+) std_out=(0x[0-9a-f]+) std_err=(0x[0-9a-f]+) ') {
        $okConsole = ($Matches[1] -ne '0x0') -and ($Matches[1] -ne $Matches[2]) -and ($Matches[1] -ne $Matches[3]) -and ($Matches[1] -ne $Matches[4])
    }
    $okBits = $mine.Contains('wow64=False')
    $hidden = Start-Process powershell -WindowStyle Hidden -ArgumentList '-NoProfile -Command Start-Sleep -Seconds 20' -PassThru
    Start-Sleep -Milliseconds 1500
    $theirs = [Ctx]::Describe($hidden.Id)
    $okHidden = $false
    if ($theirs -match 'startf=0x([0-9a-f]+) show_window=([0-9]+) ') { $okHidden = ((([Convert]::ToInt32($Matches[1], 16)) -band 1) -eq 1) -and ($Matches[2] -eq '0') }
    Emit $mine; Emit $theirs
    $all = $okSelf -and $okStd -and $okDesktop -and $okConsole -and $okBits -and $okHidden
    Emit ("reader check " + $(if ($all) { 'ok' } else { 'FAILED' }) + " (CIM gave a command line: " + $okCim + "; command line from the PEB equals CIM's: " + $okSelf + "; std handles equal GetStdHandle's: " + $okStd + "; desktop is WinSta0\Default: " + $okDesktop + "; console handle non-zero and not a std handle: " + $okConsole + "; reader is 64-bit and not wow64: " + $okBits + "; hidden PowerShell reads USESHOWWINDOW with show-window 0: " + $okHidden + ")")
    exit 0
}
$bad = 0
$ids = @()
if ($SpawnFile) {
    $deadline = (Get-Date).AddSeconds($WaitSeconds)
    $parentPid = $null; $childPid = $null
    while ((Get-Date) -lt $deadline -and -not $childPid) {
        if (Test-Path -LiteralPath $SpawnFile) {
            $text = (Get-Content -LiteralPath $SpawnFile -Raw -ErrorAction SilentlyContinue)
            if ($text -match 'parent_pid=([0-9]+)') { $parentPid = $Matches[1] }
            if ($text -match 'child_pid=([0-9]+)') { $childPid = $Matches[1] }
        }
        if (-not $childPid) { Start-Sleep -Milliseconds 100 }
    }
    if (-not $childPid) { Emit ("ERROR no child_pid line in the spawn file within " + $WaitSeconds + " s"); exit 3 }
    Emit ("WAIT seen child_pid=" + $childPid + " at " + (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ss.fffZ"))
    Start-Sleep -Seconds $DelaySeconds
    $ids = @($parentPid, $childPid)
} elseif ($WaitForConfirmChild) {
    $deadline = (Get-Date).AddSeconds($WaitSeconds)
    $found = $null
    while ((Get-Date) -lt $deadline -and -not $found) {
        $found = Get-CimInstance Win32_Process -Filter "Name='bridge_mcp.exe'" -ErrorAction SilentlyContinue | Where-Object { $_.CommandLine -like '*--confirm-journal*' } | Select-Object -First 1
        if (-not $found) { Start-Sleep -Milliseconds 250 }
    }
    if (-not $found) { Emit ("ERROR no --confirm-journal child within " + $WaitSeconds + " s"); exit 3 }
    Emit ("WAIT seen child pid=" + $found.ProcessId + " at " + (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ss.fffZ"))
    Start-Sleep -Seconds $DelaySeconds
    $parent = Get-CimInstance Win32_Process -Filter ("ProcessId=" + $found.ParentProcessId) -ErrorAction SilentlyContinue
    $ids = @($found.ProcessId, $found.ParentProcessId)
    if ($parent) { $ids += $parent.ParentProcessId }
} else {
    foreach ($token in ($Pids -split ' ')) {
        if ($token -eq '') { continue }
        if ($token -match '^[0-9]+$') { $ids += $token } else { Emit ("ERROR not a pid: " + $token); $bad = 2 }
    }
}
foreach ($id in $ids) {
    Emit ([Ctx]::Describe([int]$id))
    Emit (Chain ([int]$id))
}
foreach ($c in (Get-CimInstance Win32_Process -Filter "Name='conhost.exe'" -ErrorAction SilentlyContinue)) { Emit ("CONHOST pid=" + $c.ProcessId + " parent=" + $c.ParentProcessId) }
try { Emit ("NAMED_JOB_OBJECTS count=" + @(Get-CimInstance Win32_NamedJobObject -ErrorAction Stop).Count) } catch { Emit "NAMED_JOB_OBJECTS <not available>" }
exit $bad
