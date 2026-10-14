# hold-lock.ps1: hold a Windows file lock the way ComplyEaze Bridge takes one, and report whether it was granted.
# Opens an EXISTING file (never creates one), shared for reading and writing only, then LockFileEx over the whole file
# (offset 0, length 2^64-1), failing at once if another holder has it. Holds for -Seconds or until -StopFile exists.
# Writes one line per event to -Log (UTC time, event). Exit codes: 0 held and released, 3 busy, 4 file could not be opened.
param(
    [Parameter(Mandatory = $true)][string]$Path,
    [ValidateSet('Exclusive', 'Shared')][string]$Mode = 'Exclusive',
    [ValidateRange(1, 900)][int]$Seconds = 60,
    [Parameter(Mandatory = $true)][string]$Log,
    [string]$StopFile = 'C:\bridge-lab\1641\stop.txt',
    [switch]$Probe
)

Add-Type -TypeDefinition @'
using System;
using System.IO;
using System.Runtime.InteropServices;

public static class HoldLock
{
    [StructLayout(LayoutKind.Sequential)]
    struct OVERLAPPED { public IntPtr Internal; public IntPtr InternalHigh; public uint Offset; public uint OffsetHigh; public IntPtr hEvent; }

    [DllImport("kernel32.dll", SetLastError = true)]
    static extern bool LockFileEx(IntPtr file, uint flags, uint reserved, uint lengthLow, uint lengthHigh, ref OVERLAPPED overlapped);

    [DllImport("kernel32.dll", SetLastError = true)]
    static extern bool UnlockFileEx(IntPtr file, uint reserved, uint lengthLow, uint lengthHigh, ref OVERLAPPED overlapped);

    public static FileStream Open(string path)
    {
        return new FileStream(path, FileMode.Open, FileAccess.ReadWrite, FileShare.ReadWrite);
    }

    // 0 when granted, else the Win32 error code.
    public static int TryLock(FileStream stream, bool exclusive)
    {
        var overlapped = new OVERLAPPED();
        uint flags = 0x1 | (exclusive ? 0x2u : 0x0u); // LOCKFILE_FAIL_IMMEDIATELY | LOCKFILE_EXCLUSIVE_LOCK
        if (LockFileEx(stream.SafeFileHandle.DangerousGetHandle(), flags, 0, 0xFFFFFFFF, 0xFFFFFFFF, ref overlapped)) return 0;
        return Marshal.GetLastWin32Error();
    }

    public static void Unlock(FileStream stream)
    {
        var overlapped = new OVERLAPPED();
        UnlockFileEx(stream.SafeFileHandle.DangerousGetHandle(), 0, 0xFFFFFFFF, 0xFFFFFFFF, ref overlapped);
    }
}
'@

function Say([string]$Event) {
    ('{0} {1}' -f [DateTime]::UtcNow.ToString('o'), $Event) | Add-Content -Path $Log -Encoding ASCII
}

Say ("START pid={0} mode={1} path={2}" -f $PID, $Mode, $Path)

if ($Probe) {
    # A process with a package identity has a new file under AppData written into the package's private folder.
    $probeDir = Join-Path $env:LOCALAPPDATA 'Bridge'
    New-Item -ItemType Directory -Force -Path $probeDir | Out-Null
    $probePath = Join-Path $probeDir 'probe-1641.txt'
    Set-Content -Path $probePath -Value 'probe' -Encoding ASCII
    Say ("PROBE wrote {0}" -f $probePath)
}

try {
    $stream = [HoldLock]::Open($Path)
} catch {
    Say ("OPEN_FAILED {0}" -f $_.Exception.GetType().Name)
    exit 4
}

$code = [HoldLock]::TryLock($stream, ($Mode -eq 'Exclusive'))
if ($code -ne 0) {
    Say ("BUSY win32={0}" -f $code)
    $stream.Dispose()
    exit 3
}

Say 'HELD'
$deadline = [DateTime]::UtcNow.AddSeconds($Seconds)
while ([DateTime]::UtcNow -lt $deadline -and -not (Test-Path -LiteralPath $StopFile)) {
    Start-Sleep -Milliseconds 200
}
[HoldLock]::Unlock($stream)
$stream.Dispose()
Say 'RELEASED'
exit 0
