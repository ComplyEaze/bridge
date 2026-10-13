# Prints the key list of one saved Tally answer: one line "GUID|ALTERID|DATE" per VOUCHER row under
# ENVELOPE/BODY/DATA/COLLECTION, in ordinal order, after a first line with the row count. The answer is parsed
# as XML (UTF-16LE, with or without a byte-order mark); nothing is counted by text search.
# Windows PowerShell 5.1. Usage: powershell -NoProfile -File list-keys.ps1 <answer.xml> > <answer>.keys.txt
param([Parameter(Mandatory)][string]$Path)
$ErrorActionPreference = 'Stop'
$b = [IO.File]::ReadAllBytes($Path)
if ($b.Length -ge 2 -and $b[0] -eq 0xFF -and $b[1] -eq 0xFE) { $t = [Text.Encoding]::Unicode.GetString($b, 2, $b.Length - 2) }
else { $t = [Text.Encoding]::Unicode.GetString($b) }
$t = [regex]::Replace($t, '[\x00-\x08\x0B\x0C\x0E-\x1F]', '')
$t = [regex]::Replace($t, '&#(x[0-9A-Fa-f]+|\d+);', {
        param($m) $v = $m.Groups[1].Value
        $n = if ($v[0] -eq 'x') { [Convert]::ToInt32($v.Substring(1), 16) } else { [int]$v }
        if (($n -ge 0 -and $n -le 8) -or $n -eq 11 -or $n -eq 12 -or ($n -ge 14 -and $n -le 31)) { '' } else { $m.Value }
    })
$x = New-Object System.Xml.XmlDocument
$x.PreserveWhitespace = $true
$x.LoadXml($t)
$keys = New-Object System.Collections.Generic.List[string]
foreach ($v in $x.SelectNodes('/ENVELOPE/BODY/DATA/COLLECTION/VOUCHER')) {
    $keys.Add(($v.SelectSingleNode('GUID').InnerText.Trim() + '|' + $v.SelectSingleNode('ALTERID').InnerText.Trim() + '|' + $v.SelectSingleNode('DATE').InnerText.Trim()))
}
$a = $keys.ToArray(); [Array]::Sort($a, [StringComparer]::Ordinal)
"# $($a.Length) VOUCHER rows of $(Split-Path $Path -Leaf): GUID|ALTERID|DATE"
$a
