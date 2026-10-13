# PowerShell variable names ignore case: no two variables here differ only by case, and an element's name
# is read as LocalName (Name would return a child element called NAME).
# Compares the VOUCHER rows of two saved Tally answers leaf by leaf with an XML parser, and looks for given
# texts in the leaves of each. Windows PowerShell 5.1. Usage, from the answers folder:
#   powershell -NoProfile -File ..\tools\leaf-compare.ps1 <first answer> <second answer> [<text to look for> ...]
# Nothing here talks to Tally. Nothing is counted by text search over the raw file.
# A leaf is an element with no child element, keyed by its path from the VOUCHER row with a position for
# every element, e.g. VOUCHER[1]/LEDGERENTRIES.LIST[2]/AMOUNT[1]. Attributes are not leaves. Values are
# trimmed. A leaf's value is compared in the exact (case-sensitive) way.
param([Parameter(Mandatory = $true)][string]$FirstFile, [Parameter(Mandatory = $true)][string]$SecondFile, [Parameter(ValueFromRemainingArguments = $true)][string[]]$Texts)
$ErrorActionPreference = 'Stop'

function Get-Sha([string]$Path) { (Get-FileHash $Path -Algorithm SHA256).Hash.ToLower() }

# UTF-16LE with or without a byte-order mark; only the characters XML forbids are dropped (raw or as
# references); tab, line feed and carriage return are kept.
function Read-TallyXml([string]$Path) {
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
    $x
}

function Add-Leaves($Node, [string]$Path, $Out) {
    $kids = @($Node.ChildNodes | Where-Object { $_.NodeType -eq 'Element' })
    if ($kids.Count -eq 0) { $Out[$Path] = $Node.InnerText.Trim(); return }
    $seen = @{}
    foreach ($k in $kids) {
        if ($seen.ContainsKey($k.LocalName)) { $seen[$k.LocalName] = $seen[$k.LocalName] + 1 } else { $seen[$k.LocalName] = 1 }
        Add-Leaves $k ($Path + '/' + $k.LocalName + '[' + $seen[$k.LocalName] + ']') $Out
    }
}

function Show([string]$s) { if ($null -eq $s) { return '(none)' }; '"' + $s.Replace("`r", '\r').Replace("`n", '\n').Replace("`t", '\t') + '" (' + $s.Length + ')' }

function Read-Leaves([string]$Path) {
    $x = Read-TallyXml $Path
    $rows = @($x.SelectNodes('/ENVELOPE/BODY/DATA/COLLECTION/VOUCHER'))
    $leaves = [ordered]@{}
    $i = 0
    foreach ($r in $rows) { $i++; Add-Leaves $r ('VOUCHER[' + $i + ']') $leaves }
    $lineErrors = @($x.SelectNodes('//LINEERROR')).Count
    [pscustomobject]@{ Path = $Path; Bytes = (Get-Item $Path).Length; Sha = (Get-Sha $Path); Rows = $rows.Count; LineErrors = $lineErrors; Leaves = $leaves }
}

$sides = @((Read-Leaves $FirstFile), (Read-Leaves $SecondFile))
foreach ($s in $sides) {
    "== $(Split-Path $s.Path -Leaf) | $($s.Bytes) bytes | sha256 $($s.Sha) | VOUCHER rows $($s.Rows) | LINEERROR elements $($s.LineErrors) | leaves $($s.Leaves.Count)"
}
$leavesFirst = $sides[0].Leaves; $leavesSecond = $sides[1].Leaves
$onlyA = @($leavesFirst.Keys | Where-Object { -not $leavesSecond.Contains($_) })
$onlyB = @($leavesSecond.Keys | Where-Object { -not $leavesFirst.Contains($_) })
$differ = @($leavesFirst.Keys | Where-Object { $leavesSecond.Contains($_) -and -not ($leavesFirst[$_] -ceq $leavesSecond[$_]) })
"compare: leaves in A only $($onlyA.Count), in B only $($onlyB.Count), in both with a different value $($differ.Count), in both and equal $(@($leavesFirst.Keys | Where-Object { $leavesSecond.Contains($_) -and ($leavesFirst[$_] -ceq $leavesSecond[$_]) }).Count)"
foreach ($k in $onlyA) { "  only in A: $k = $(Show $leavesFirst[$k])" }
foreach ($k in $onlyB) { "  only in B: $k = $(Show $leavesSecond[$k])" }
foreach ($k in $differ) { "  differs: $k | A $(Show $leavesFirst[$k]) | B $(Show $leavesSecond[$k])" }
foreach ($t in $Texts) {
    foreach ($s in $sides) {
        $hits = @($s.Leaves.Keys | Where-Object { $s.Leaves[$_].Contains($t) })
        "text $(Show $t) in $(Split-Path $s.Path -Leaf): $($hits.Count) leaf(s)"
        foreach ($k in $hits) { "  $k = $(Show $s.Leaves[$k])" }
    }
}
