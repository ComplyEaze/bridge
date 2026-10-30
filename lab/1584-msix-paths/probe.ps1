# probe.ps1: lab helper for item 1584-MSIX (not product code). Run with the same text on both sides of the
# package boundary; only -Side differs. Reads only, except -WriteNonce, which creates ONE new file under
# %LOCALAPPDATA%\bridge-lab-1584 (the known-answer probe). Never deletes, never overwrites.
# Existence is decided by opening the path (win32 2 or 3 = absent; any other failure = error), never by Test-Path,
# which reports a failure as False. For each path that opens it records the final path Windows reports in three forms
# (flags 0 = what Rust's std::fs::canonicalize asks for, 8 = FILE_NAME_OPENED, 2 = VOLUME_NAME_NT) and the volume
# serial and 128-bit file ID (FileIdInfo, the identity Bridge's lock set compares). It also records this process's
# package family and the known folders Bridge resolves, with and without the redirection flags.
# Output: <Root>\evidence\<Step>-<Side>.json (UTF-8, no BOM). An existing output file is a stop, not an overwrite.
param(
    [Parameter(Mandatory = $true)][ValidateSet('in', 'out')][string]$Side,
    [Parameter(Mandatory = $true)][ValidatePattern('^[A-Za-z0-9-]+$')][string]$Step,
    [Parameter(Mandatory = $true)][ValidatePattern('^[A-Za-z0-9._-]+$')][string]$Family,
    [ValidatePattern('^([0-9a-f]{8})?$')][string]$WriteNonce = '',
    [string]$ExtraFile = '',
    # The port's lock key: the hex sha256 of native-dispatch-port:<port> (default: port 9001).
    [ValidatePattern('^[0-9a-f]{64}$')][string]$Key = 'a905deaa97bc7d8acb82123c4057a3890b174daf0295ee5c10be17f6fa0f0a01',
    [string]$Root = 'C:\bridge-lab\1584'
)
$ErrorActionPreference = 'Stop'
$out = Join-Path $Root ('evidence\{0}-{1}.json' -f $Step, $Side)
if ([IO.File]::Exists($out)) { Write-Output "STOP: $out exists; nothing written"; exit 2 }

try {
Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
using System.Text;
using Microsoft.Win32.SafeHandles;

public static class PathInfo
{
    [StructLayout(LayoutKind.Sequential)]
    struct FILE_ID_INFO { public ulong VolumeSerial; public ulong IdLow; public ulong IdHigh; }

    [DllImport("kernel32.dll", CharSet = CharSet.Unicode, SetLastError = true)]
    static extern SafeFileHandle CreateFileW(string name, uint access, uint share, IntPtr security,
        uint disposition, uint flags, IntPtr template);

    [DllImport("kernel32.dll", CharSet = CharSet.Unicode, SetLastError = true)]
    static extern uint GetFinalPathNameByHandleW(SafeFileHandle handle, StringBuilder buffer, uint length, uint flags);

    [DllImport("kernel32.dll", SetLastError = true)]
    static extern bool GetFileInformationByHandleEx(SafeFileHandle handle, int infoClass, out FILE_ID_INFO info, uint size);

    [DllImport("kernel32.dll", CharSet = CharSet.Unicode)]
    static extern int GetCurrentPackageFamilyName(ref uint length, StringBuilder name);

    [DllImport("shell32.dll")]
    static extern int SHGetKnownFolderPath(ref Guid folder, uint flags, IntPtr token, out IntPtr path);

    [DllImport("ole32.dll")]
    static extern void CoTaskMemFree(IntPtr pointer);

    static string Final(SafeFileHandle handle, uint flags)
    {
        StringBuilder buffer = new StringBuilder(4096);
        uint length = GetFinalPathNameByHandleW(handle, buffer, (uint)buffer.Capacity, flags);
        if (length == 0) return "error win32=" + Marshal.GetLastWin32Error();
        if (length >= buffer.Capacity) return "error too-long";
        return buffer.ToString();
    }

    // "absent" (win32 2 or 3), "error win32=<n> at open", or
    // "final=<flags 0>|opened=<flags 8>|nt=<flags 2>|vol=<hex>|id=<hex 128-bit>".
    // Opens for attributes only, sharing read, write and delete, so a byte-range lock held by anyone is untouched.
    public static string Of(string path)
    {
        using (SafeFileHandle handle = CreateFileW(path, 0x80, 0x7, IntPtr.Zero, 3, 0x02000000, IntPtr.Zero))
        {
            if (handle.IsInvalid)
            {
                int error = Marshal.GetLastWin32Error();
                return (error == 2 || error == 3) ? "absent" : "error win32=" + error + " at open";
            }
            string text = "final=" + Final(handle, 0) + "|opened=" + Final(handle, 8) + "|nt=" + Final(handle, 2);
            FILE_ID_INFO info;
            if (!GetFileInformationByHandleEx(handle, 18, out info, (uint)Marshal.SizeOf(typeof(FILE_ID_INFO))))
                return text + "|error win32=" + Marshal.GetLastWin32Error() + " at id";
            return text + "|vol=" + info.VolumeSerial.ToString("x16") + "|id=" +
                info.IdHigh.ToString("x16") + info.IdLow.ToString("x16");
        }
    }

    // The package family of this process, or "none" (APPMODEL_ERROR_NO_PACKAGE 15700), or "error <n>".
    public static string Family()
    {
        uint length = 0;
        int rc = GetCurrentPackageFamilyName(ref length, null);
        if (rc == 15700) return "none";
        if (rc != 122) return "error " + rc;
        StringBuilder name = new StringBuilder((int)length);
        rc = GetCurrentPackageFamilyName(ref length, name);
        return rc == 0 ? name.ToString() : "error " + rc;
    }

    public static string Known(string guid, uint flags)
    {
        Guid folder = new Guid(guid);
        IntPtr path;
        int hr = SHGetKnownFolderPath(ref folder, flags, IntPtr.Zero, out path);
        if (hr != 0) return "error hr=0x" + hr.ToString("x8");
        string text = Marshal.PtrToStringUni(path);
        CoTaskMemFree(path);
        return text;
    }
}
'@
} catch {
    Write-Output ('INSTRUMENT_FAILED {0}' -f $_.Exception.Message)
    exit 5
}

$la = $env:LOCALAPPDATA
$up = $env:USERPROFILE
$probeDir = Join-Path $la 'bridge-lab-1584'
$packageLocal = Join-Path $la ('Packages\{0}\LocalCache\Local' -f $Family)
$parent = $null
try { $parent = (Get-CimInstance Win32_Process -Filter "ProcessId=$PID").ParentProcessId } catch { $parent = 'error ' + $_.Exception.Message }

# KF_FLAG_DONT_VERIFY (0x4000) is what Bridge passes; 0x40000 = RETURN_FILTER_REDIRECTION_TARGET,
# 0x80000 = FORCE_APP_DATA_REDIRECTION. Recorded only: nothing here depends on them.
$known = [ordered]@{}
$folders = [ordered]@{
    LocalAppData = 'f1b32785-6fba-4fcf-9d55-7b8e7f157091'
    RoamingAppData = '3eb685db-65f9-4cf6-a03a-e3ef65729f3d'
    Profile = '5e6c858f-0e22-4760-9afe-ea3317b67173'
}
foreach ($folder in $folders.Keys) {
    foreach ($flags in @(0x4000, 0x44000, 0x84000)) {
        $known[('{0}-0x{1:x}' -f $folder, $flags)] = [PathInfo]::Known($folders[$folder], [uint32]$flags)
    }
}

$record = [ordered]@{
    step = $Step
    lock_key = $Key
    side = $Side
    utc = [DateTime]::UtcNow.ToString('o')
    pid = $PID
    parent_pid = $parent
    shell_path = (Get-Process -Id $PID).Path
    package_family = [PathInfo]::Family()
    env_LOCALAPPDATA = $la
    env_APPDATA = $env:APPDATA
    env_USERPROFILE = $up
    known_folders = $known
    written = $null
    paths = [ordered]@{}
    lease_counts = [ordered]@{}
    listings = [ordered]@{}
    extra = @()
}

if ($WriteNonce -ne '') {
    $target = Join-Path $probeDir ('{0}-{1}.txt' -f $WriteNonce, $Side)
    if ([PathInfo]::Of($target) -ne 'absent') { Write-Output "STOP: $target is not absent; nothing written"; exit 2 }
    try {
        [void][IO.Directory]::CreateDirectory($probeDir)
        $fs = New-Object IO.FileStream($target, [IO.FileMode]::CreateNew, [IO.FileAccess]::Write, [IO.FileShare]::Read)
        $bytes = [Text.Encoding]::ASCII.GetBytes(('{0}-{1}' -f $WriteNonce, $Side))
        $fs.Write($bytes, 0, $bytes.Length)
        $fs.Dispose()
        $record.written = $target
    } catch {
        $record.written = 'error ' + $_.Exception.Message
    }
}

$named = [ordered]@{
    R = Join-Path $la 'Bridge\agent'
    Q = Join-Path $packageLocal 'Bridge\agent'
    P = Join-Path $up '.complyeaze-bridge\coordination'
    X = $probeDir
    XQ = Join-Path $packageLocal 'bridge-lab-1584'
    KA = Join-Path $Root 'ka.txt'
}
foreach ($name in @('R', 'Q', 'P')) {
    $named["$name-imports"] = Join-Path $named[$name] 'imports'
    $named["$name-leases"] = Join-Path $named[$name] 'native-dispatch-leases'
    $named["$name-journal"] = Join-Path $named[$name] 'agent-import-ledger.jsonl'
    foreach ($suffix in @('lock', 'snapshot.lock', 'wire.lock')) {
        $named["$name-K.$suffix"] = Join-Path $named["$name-leases"] ('{0}.{1}' -f $Key, $suffix)
    }
}
foreach ($name in $named.Keys) {
    $entry = [ordered]@{ path = $named[$name]; info = [PathInfo]::Of($named[$name]); bytes = $null }
    if ($entry.info -like 'final=*' -and [IO.File]::Exists($named[$name])) { $entry.bytes = (New-Object IO.FileInfo($named[$name])).Length }
    $record.paths[$name] = $entry
}

# Each leases folder by name suffix: counts only (a folder can hold over a thousand lock files).
foreach ($name in @('R-leases', 'Q-leases', 'P-leases')) {
    $counts = [ordered]@{ total = $null; dot_lock = 0; snapshot_lock = 0; wire_lock = 0; other = 0; error = $null }
    if ($record.paths[$name].info -like 'final=*') {
        try {
            $all = @([IO.Directory]::EnumerateFileSystemEntries($named[$name]))
            $counts.total = $all.Count
            foreach ($child in $all) {
                if ($child.EndsWith('.snapshot.lock')) { $counts.snapshot_lock++ }
                elseif ($child.EndsWith('.wire.lock')) { $counts.wire_lock++ }
                elseif ($child.EndsWith('.lock')) { $counts.dot_lock++ }
                else { $counts.other++ }
            }
        } catch { $counts.error = $_.Exception.Message }
    } else { $counts.error = 'not listed: ' + $record.paths[$name].info }
    $record.lease_counts[$name] = $counts
}

# Children of each folder, at most 200 per folder (the total is always recorded), each opened as above.
foreach ($name in @('R', 'Q', 'R-imports', 'Q-imports', 'X', 'XQ')) {
    $folder = $named[$name]
    $entry = [ordered]@{ folder = $folder; total = $null; shown = @(); error = $null }
    if ($record.paths[$name].info -like 'final=*') {
        try {
            $children = @([IO.Directory]::EnumerateFileSystemEntries($folder) | Sort-Object)
            $entry.total = $children.Count
            $entry.shown = @($children | Select-Object -First 200 | ForEach-Object {
                $child = $_
                $isFile = [IO.File]::Exists($child)
                [ordered]@{
                    name = [IO.Path]::GetFileName($child)
                    kind = $(if ($isFile) { 'file' } else { 'dir' })
                    bytes = $(if ($isFile) { (New-Object IO.FileInfo($child)).Length } else { $null })
                    info = [PathInfo]::Of($child)
                }
            })
        } catch {
            $entry.error = $_.Exception.Message
        }
    } else {
        $entry.error = 'not listed: ' + $record.paths[$name].info
    }
    $record.listings[$name] = $entry
}

# -ExtraFile names a UTF-8 text file holding one full path per line: each is opened as above and, when it opens as
# a file, hashed.
if ($ExtraFile -ne '') {
    $record.extra = @([IO.File]::ReadAllLines($ExtraFile) | Where-Object { $_ -ne '' } | ForEach-Object {
        $item = $_
        $info = [PathInfo]::Of($item)
        $hash = $null
        if ($info -like 'final=*' -and [IO.File]::Exists($item)) {
            try { $hash = (Get-FileHash -LiteralPath $item -Algorithm SHA256).Hash.ToLowerInvariant() }
            catch { $hash = 'error ' + $_.Exception.Message }
        }
        [ordered]@{ path = $item; info = $info; sha256 = $hash }
    })
}

$json = $record | ConvertTo-Json -Depth 8
[void][IO.Directory]::CreateDirectory((Split-Path $out))
[IO.File]::WriteAllText($out, $json, (New-Object Text.UTF8Encoding($false)))
Write-Output ('WROTE {0} {1} bytes' -f $out, (New-Object IO.FileInfo($out)).Length)
exit 0
