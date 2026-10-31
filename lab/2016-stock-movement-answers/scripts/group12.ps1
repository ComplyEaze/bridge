# Lab helper (#2016 stock movement, not product code): step 12's remaining raw reads, one at a time, by raw.ps1 (C3's line).
# After each send: a whole-window capture (cap.py); the next read is sent only if that capture's sha256 equals the
# reference capture of the quiet Gateway taken after the group's first read (any dialog or other screen = a different
# image = stop, nothing further sent).
param([Parameter(Mandatory = $true)][string]$RefSha)
$ErrorActionPreference = 'Stop'
$readNames = @('r-m-20260706', 'r-a07-20260707', 'r-m-20260707', 'r-a07-20260708', 'r-m-20260708')
foreach ($readName in $readNames) {
    & <LAB_DIR>\raw.ps1 -Name $readName -Ans "r12-$readName.xml"
    if ($LASTEXITCODE -ne 0) { throw "HALT: raw.ps1 exit $LASTEXITCODE on $readName" }
    Get-Content <LAB_DIR>\send-log.txt -Tail 2 | Select-Object -First 1
    $capLine = & python -I <LAB_DIR>\cap.py "r12-after-$readName"
    $capLine
    if ($capLine -notmatch [regex]::Escape($RefSha)) { throw "HALT: the capture after $readName differs from the quiet Gateway; nothing further sent" }
}
'group12 done'
