# class-reader.ps1 -- READ-ONLY. Reads the environment block of ONE running process from outside and prints,
# for each variable name asked, ONE class word or present/absent. It never prints a value.
# Written on a Mac, NEVER RUN: UNTESTED (U). Its first run is -SelfTest at step S5 of the sheet.
# -ExpectImage is mandatory outside -SelfTest.
# Run in 64-bit PowerShell:
#   powershell.exe -NoProfile -File class-reader.ps1 -SelfTest
#   powershell.exe -NoProfile -File class-reader.ps1 -ProcessId 1234 -ExpectImage <path of the program> -Name NAME1 -Presence NAME2,SystemRoot
# Class words for -Name (the program's own test, exact characters, nothing trimmed):
#   absent | empty | placeholder | false | on | other
# A process that could not be read, or whose image path is not the expected one before AND after the read,
# gives one line starting UNREADABLE and no class word (never 'absent').
# It opens a process only for reading (PROCESS_QUERY_INFORMATION | PROCESS_VM_READ), writes nothing to any process,
# and sets no variable anywhere. If Microsoft Defender or another protection blocks it: record 'class not read';
# never add an exclusion and never turn protection off.
param(
  [int]$ProcessId = 0,
  [string[]]$Name = @(),
  [string[]]$Presence = @(),
  [string]$ExpectImage = '',
  [switch]$SelfTest
)
$ErrorActionPreference = 'Stop'
# with -File a comma list arrives as ONE string: split it here
$Name = @($Name | ForEach-Object { $_ -split ',' } | Where-Object { $_ })
$Presence = @($Presence | ForEach-Object { $_ -split ',' } | Where-Object { $_ })
if (-not [Environment]::Is64BitProcess) { Write-Output 'UNREADABLE the reader is not a 64-bit process'; exit 3 }

Add-Type -TypeDefinition @'
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;

public static class EnvReader
{
    [DllImport("kernel32.dll", SetLastError = true)] static extern IntPtr OpenProcess(uint access, bool inherit, int pid);
    [DllImport("kernel32.dll", SetLastError = true)] static extern bool CloseHandle(IntPtr h);
    [DllImport("kernel32.dll", SetLastError = true)] static extern bool ReadProcessMemory(IntPtr h, IntPtr addr, byte[] buf, UIntPtr size, out UIntPtr read);
    [DllImport("kernel32.dll", SetLastError = true)] static extern UIntPtr VirtualQueryEx(IntPtr h, IntPtr addr, out MBI mbi, UIntPtr len);
    [DllImport("ntdll.dll")] static extern int NtQueryInformationProcess(IntPtr h, int cls, ref PBI pbi, int len, out int ret);

    [StructLayout(LayoutKind.Sequential)]
    struct PBI { public IntPtr Reserved1; public IntPtr PebBaseAddress; public IntPtr Reserved2a; public IntPtr Reserved2b; public IntPtr UniqueProcessId; public IntPtr Reserved3; }

    [StructLayout(LayoutKind.Sequential)]
    struct MBI { public IntPtr BaseAddress; public IntPtr AllocationBase; public uint AllocationProtect; public ushort PartitionId; public UIntPtr RegionSize; public uint State; public uint Protect; public uint Type; }

    static bool Rd(IntPtr h, IntPtr addr, byte[] buf, out int got)
    {
        UIntPtr n;
        bool ok = ReadProcessMemory(h, addr, buf, (UIntPtr)buf.Length, out n);
        got = (int)n.ToUInt64();
        return ok || got > 0;
    }

    // Returns the "NAME=VALUE" entries, or null with status "UNREADABLE ..." when the block cannot be read whole.
    public static List<string> Entries(int pid, out string status)
    {
        status = null;
        IntPtr h = OpenProcess(0x0410, false, pid); // PROCESS_QUERY_INFORMATION | PROCESS_VM_READ
        if (h == IntPtr.Zero) { status = "UNREADABLE OpenProcess error " + Marshal.GetLastWin32Error(); return null; }
        try
        {
            PBI pbi = new PBI();
            int ret;
            int nt = NtQueryInformationProcess(h, 0, ref pbi, Marshal.SizeOf(typeof(PBI)), out ret);
            if (nt != 0) { status = "UNREADABLE NtQueryInformationProcess 0x" + nt.ToString("X"); return null; }
            byte[] p8 = new byte[8];
            int got;
            // PEB.ProcessParameters is at offset 0x20 (x64)
            if (!Rd(h, IntPtr.Add(pbi.PebBaseAddress, 0x20), p8, out got) || got != 8) { status = "UNREADABLE PEB error " + Marshal.GetLastWin32Error(); return null; }
            IntPtr parms = (IntPtr)BitConverter.ToInt64(p8, 0);
            // RTL_USER_PROCESS_PARAMETERS.Environment is at offset 0x80 (x64)
            if (!Rd(h, IntPtr.Add(parms, 0x80), p8, out got) || got != 8) { status = "UNREADABLE ProcessParameters error " + Marshal.GetLastWin32Error(); return null; }
            IntPtr env = (IntPtr)BitConverter.ToInt64(p8, 0);
            if (env == IntPtr.Zero) { status = "UNREADABLE no environment pointer"; return null; }
            MBI mbi;
            if (VirtualQueryEx(h, env, out mbi, (UIntPtr)Marshal.SizeOf(typeof(MBI))) == UIntPtr.Zero) { status = "UNREADABLE VirtualQueryEx error " + Marshal.GetLastWin32Error(); return null; }
            long avail = mbi.BaseAddress.ToInt64() + (long)mbi.RegionSize.ToUInt64() - env.ToInt64();
            if (avail < 4) { status = "UNREADABLE region too small"; return null; }
            int size = (int)Math.Min(avail, 1048576L);
            byte[] buf = new byte[size];
            if (!Rd(h, env, buf, out got) || got < 4) { status = "UNREADABLE environment error " + Marshal.GetLastWin32Error(); return null; }
            List<string> list = new List<string>();
            int pos = 0;
            bool terminated = false;
            while (pos + 2 <= got)
            {
                int start = pos;
                while (pos + 2 <= got && !(buf[pos] == 0 && buf[pos + 1] == 0)) pos += 2;
                if (pos + 2 > got) break;                    // no NUL found inside the bytes read
                if (pos == start) { terminated = true; break; } // an empty entry: the closing double NUL
                list.Add(Encoding.Unicode.GetString(buf, start, pos - start));
                pos += 2;
            }
            if (!terminated) { status = "UNREADABLE block not ended by two NULs inside " + got + " bytes"; return null; }
            status = "READ " + list.Count + " variables, terminated";
            return list;
        }
        finally { CloseHandle(h); }
    }
}
'@

function Get-EntryValue([System.Collections.Generic.List[string]]$entries, [string]$name) {
    foreach ($e in $entries) {
        $i = $e.IndexOf('=', 1)   # a name may start with '=' (for example =C:); the separator is the next one
        if ($i -gt 0 -and [string]::Equals($e.Substring(0, $i), $name, [StringComparison]::OrdinalIgnoreCase)) {
            return $e.Substring($i + 1)
        }
    }
    return $null
}
function Get-Class($v) {
    if ($null -eq $v) { return 'absent' }
    if ($v -ceq '') { return 'empty' }
    if ($v -ceq '${user_config.enable_posting_2026_10}') { return 'placeholder' }
    if ($v -ceq 'false' -or $v -ceq '0') { return 'false' }
    if ($v -ceq 'true' -or $v -ceq '1') { return 'on' }
    return 'other'
}

function Get-Presence($entries, [string]$name) { if ($null -eq (Get-EntryValue $entries $name)) { return 'absent' } else { return 'present' } }

if ($SelfTest) {
    $exe = [System.Diagnostics.Process]::GetCurrentProcess().MainModule.FileName
    $cases = New-Object System.Collections.Generic.List[object]
    foreach ($c in @(@($null, 'absent'), @('', 'empty'), @('${user_config.enable_posting_2026_10}', 'placeholder'), @('false', 'false'), @('0', 'false'),
                     @('true', 'on'), @('1', 'on'), @('TRUE', 'other'), @(' true', 'other'), @('maybe', 'other'))) {
        $cases.Add(@{ Probe = 'LAB060S_CLASS_PROBE'; Value = $c[0]; Ask = 'LAB060S_CLASS_PROBE'; Kind = 'class'; Expect = $c[1] })
    }
    $cases.Add(@{ Probe = 'LAB060S_PRESENCE_PROBE'; Value = ''; Ask = 'LAB060S_PRESENCE_PROBE'; Kind = 'presence'; Expect = 'present' })
    $cases.Add(@{ Probe = 'LAB060S_PRESENCE_PROBE'; Value = 'x'; Ask = 'LAB060S_PRESENCE_PROBE'; Kind = 'presence'; Expect = 'present' })
    $cases.Add(@{ Probe = 'LAB060S_CLASS_PROBE'; Value = 'true'; Ask = 'lab060s_class_probe'; Kind = 'class'; Expect = 'on' })
    $fail = 0; $n = 0
    foreach ($c in $cases) {
        $n++
        $psi = New-Object System.Diagnostics.ProcessStartInfo
        $psi.FileName = $exe; $psi.Arguments = '-NoProfile -Command Start-Sleep -Seconds 600'
        $psi.UseShellExecute = $false; $psi.CreateNoWindow = $true
        if ($null -ne $c.Value) { $psi.EnvironmentVariables[$c.Probe] = $c.Value }   # only the lab's own probe names
        $child = [System.Diagnostics.Process]::Start($psi)
        $got = ''
        try {
            Start-Sleep -Milliseconds 1500
            $st = $null; $ent = [EnvReader]::Entries($child.Id, [ref]$st)
            if ($null -eq $ent) { $got = $st }
            else {
                if ($c.Kind -eq 'class') { $got = Get-Class (Get-EntryValue $ent $c.Ask) } else { $got = Get-Presence $ent $c.Ask }
                if ((Get-Presence $ent 'SystemRoot') -ne 'present') { $got = $got + ' (positive control SystemRoot absent)' }
            }
        } finally { Stop-Process -Id $child.Id -Force }   # by process id, never by name
        $ok = ($got -ceq $c.Expect)
        if (-not $ok) { $fail++ }
        Write-Output ("case " + $n + " " + $c.Kind + " expect " + $c.Expect + " got " + $got + " " + $(if ($ok) { 'PASS' } else { 'FAIL' }))
    }
    if ($fail -eq 0) { Write-Output 'SELFTEST PASS' } else { Write-Output ('SELFTEST FAIL ' + $fail); exit 5 }
    exit 0
}

if ($ProcessId -le 0) { Write-Output 'UNREADABLE no process id'; exit 2 }
if ($ExpectImage -eq '') { Write-Output 'UNREADABLE no expected image'; exit 2 }
function Image-Path([int]$id) { try { return (Get-Process -Id $id -ErrorAction Stop).Path } catch { return $null } }
if ($ExpectImage -ne '') {
    $before = Image-Path $ProcessId
    if ($null -eq $before -or -not [string]::Equals($before, $ExpectImage, [StringComparison]::OrdinalIgnoreCase)) { Write-Output 'UNREADABLE image path is not the expected one (before the read)'; exit 4 }
}
$status = $null
$entries = [EnvReader]::Entries($ProcessId, [ref]$status)
if ($ExpectImage -ne '') {
    $after = Image-Path $ProcessId
    if ($null -eq $after -or -not [string]::Equals($after, $ExpectImage, [StringComparison]::OrdinalIgnoreCase)) { Write-Output 'UNREADABLE image path is not the expected one (after the read)'; exit 4 }
}
Write-Output ("pid " + $ProcessId + " " + $status)
if ($null -eq $entries) { exit 2 }
foreach ($n in $Name)     { Write-Output ($n + " " + (Get-Class (Get-EntryValue $entries $n))) }
foreach ($n in $Presence) { Write-Output ($n + " " + (Get-Presence $entries $n)) }
exit 0
