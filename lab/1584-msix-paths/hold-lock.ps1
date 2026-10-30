# hold-lock.ps1: hold a Windows file lock the way ComplyEaze Bridge takes one, and report whether it was granted.
# Copied from lab/1641-rows-run-sheet for item 1584-MSIX; changed only: the default stop file, and the -Probe switch removed
# (this item decides the package boundary with probe.ps1).
# Opens an EXISTING file (never creates one), shared for reading and writing only, then LockFileEx over the whole file
# (offset 0, length 2^64-1), failing at once if another holder has it. Holds for -Seconds or until -StopFile exists.
# Writes one line per event to -Log (UTC time, event). Exit codes: 0 held and released, 3 busy (win32=33 is a held lock), 4 file could not be opened, 5 the lock code could not be loaded.
param(
    [Parameter(Mandatory = $true)][string]$Path,
    [ValidateSet('Exclusive', 'Shared')][string]$Mode = 'Exclusive',
    [ValidateRange(1, 900)][int]$Seconds = 60,
    [Parameter(Mandatory = $true)][string]$Log,
    [string]$StopFile = 'C:\bridge-lab\1584\stop.txt'
)

function Say([string]$Text) {
    ('{0} {1}' -f [DateTime]::UtcNow.ToString('o'), $Text) | Add-Content -Path $Log -Encoding ASCII
}

try {
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
} catch {
    Say ("INSTRUMENT_FAILED {0}" -f $_.Exception.Message)
    exit 5
}

Say ("START pid={0} mode={1} path={2}" -f $PID, $Mode, $Path)

try {
    $stream = [HoldLock]::Open($Path)
} catch {
    Say ("OPEN_FAILED {0}" -f $_.Exception.Message)
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
