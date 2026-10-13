# Reads the saved Tally answers in ..\answers with an XML parser and prints one section per answer.
# Windows PowerShell 5.1. Usage (from this folder):
#   powershell -NoProfile -File read-answers.ps1 > ..\derived\analysis-output.txt
# Nothing here talks to Tally. Nothing is counted by text search.
$ErrorActionPreference = 'Stop'
$answers = Join-Path (Split-Path $PSScriptRoot -Parent) 'answers'

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

function Trimmed($Node, [string]$Name) { $c = $Node.SelectSingleNode($Name); if ($c) { $c.InnerText.Trim() } else { $null } }
function Raw($Node, [string]$Name) { $c = $Node.SelectSingleNode($Name); if ($c) { $c.InnerText } else { $null } }
# A name printed so that a line break shows, with its length in characters; never trimmed.
function Show([string]$s) { if ($null -eq $s) { return '(none)' }; '"' + $s.Replace("`r", '\r').Replace("`n", '\n').Replace("`t", '\t') + '" (' + $s.Length + ')' }

$flags = 'ISCANCELLED', 'ISOPTIONAL', 'ISPOSTDATED', 'VCHGSTSTATUSISINCLUDED', 'VCHGSTSTATUSISUNCERTAIN', 'VCHGSTSTATUSISEXCLUDED', 'ISGSTOVERRIDDEN'
$counters = 'CREATED', 'ALTERED', 'DELETED', 'COMBINED', 'IGNORED', 'ERRORS', 'CANCELLED', 'EXCEPTIONS', 'LASTVCHID', 'LASTMID'

foreach ($f in Get-ChildItem $answers -Filter *.xml | Sort-Object Name) {
    "== $($f.Name) | $($f.Length) bytes | sha256 $(Get-Sha $f.FullName)"
    $x = Read-TallyXml $f.FullName
    if ($x.DocumentElement.Name -eq 'RESPONSE') {
        '   import answer: ' + (($counters | ForEach-Object { $n = $x.DocumentElement.SelectSingleNode($_); "$_ $(if ($n) { $n.InnerText.Trim() } else { '(absent)' })" }) -join ', ') + "; LINEERROR elements: $(@($x.SelectNodes('//LINEERROR')).Count)"
        ''
        continue
    }
    $col = '/ENVELOPE/BODY/DATA/COLLECTION'
    $companies = @($x.SelectNodes("$col/COMPANY")); $ledgers = @($x.SelectNodes("$col/LEDGER")); $vouchers = @($x.SelectNodes("$col/VOUCHER")); $types = @($x.SelectNodes("$col/VOUCHERTYPE[@NAME]"))
    "   rows under COLLECTION: COMPANY $($companies.Count), LEDGER $($ledgers.Count), VOUCHER $($vouchers.Count), VOUCHERTYPE $($types.Count); LINEERROR elements: $(@($x.SelectNodes('//LINEERROR')).Count)"
    foreach ($vt in $types) { "   VOUCHERTYPE $(Show $vt.GetAttribute('NAME')) | ALTERID $(Trimmed $vt 'ALTERID') | series (NUMBERINGMETHOD, NUMBERINGSUBMETHOD, PREVENTDUPLICATES): " + (@($vt.SelectNodes('VOUCHERNUMBERSERIES.LIST') | ForEach-Object { "$(Trimmed $_ 'NUMBERINGMETHOD'), $(Trimmed $_ 'NUMBERINGSUBMETHOD'), $(Trimmed $_ 'PREVENTDUPLICATES')" }) -join ' ; ') }
    foreach ($c in $companies) { "   COMPANY $(Show $c.GetAttribute('NAME')) | GUID $(Trimmed $c 'GUID') | ALTVCHID $(Trimmed $c 'ALTVCHID') | ALTMSTID $(Trimmed $c 'ALTMSTID')" }
    foreach ($l in $ledgers) {
        $names = @($l.SelectNodes('.//NAME') | ForEach-Object { Show $_.InnerText }) -join ', '
        "   LEDGER $(Show $l.GetAttribute('NAME')) | NAME elements: $names | $(Trimmed $l 'GUID') | parent $(Show (Raw $l 'PARENT')) | ALTERID $(Trimmed $l 'ALTERID')"
    }
    $rows = foreach ($v in $vouchers) {
        $extra = @()
        foreach ($k in $flags) { $n = $v.SelectSingleNode($k); if ($n) { $extra += "$k $($n.InnerText.Trim())" } }
        foreach ($k in 'PARTYLEDGERNAME', 'NARRATION') { $n = $v.SelectSingleNode($k); if ($n) { $extra += "$k $(Show $n.InnerText)" } }
        $n = $v.SelectSingleNode('AMOUNT'); if ($n) { $extra += "AMOUNT $($n.InnerText.Trim())" }
        foreach ($k in 'NUMBERINGSTYLE', 'VOUCHERRETAINKEY') { $n = $v.SelectSingleNode($k); if ($n) { $extra += "$k $($n.InnerText.Trim())" } }
        $entries = @($v.SelectNodes('ALLLEDGERENTRIES.LIST') | ForEach-Object { "$(Show (Raw $_ 'LEDGERNAME')), $(Trimmed $_ 'AMOUNT'), $(Trimmed $_ 'ISDEEMEDPOSITIVE')" })
        [pscustomobject]@{
            MasterId = [int](Trimmed $v 'MASTERID')
            Line = "   VOUCHER $(Trimmed $v 'DATE'), $(Trimmed $v 'VOUCHERTYPENAME'), $(Trimmed $v 'VOUCHERNUMBER'), MASTERID $(Trimmed $v 'MASTERID'), ALTERID $(Trimmed $v 'ALTERID'), $(Trimmed $v 'GUID') | $($extra -join ' | ')$(if ($entries.Count) { ' || ' + ($entries -join ' || ') })"
        }
    }
    $rows | Sort-Object MasterId | ForEach-Object { $_.Line }
    ''
}
