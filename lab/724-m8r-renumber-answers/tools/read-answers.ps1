# Reads the saved Tally answers in ..\answers with an XML parser and prints derived\analysis-output.txt.
# Windows PowerShell 5.1. Usage (from this folder):
#   powershell -NoProfile -File read-answers.ps1 > ..\derived\analysis-output.txt
# Nothing here talks to Tally.
$ErrorActionPreference = 'Stop'
$answers = Join-Path (Split-Path $PSScriptRoot -Parent) 'answers'
$n2 = 'c48f3906-7682-4828-9e87-2b660064b6f8'

function Get-Sha([string]$Path) { (Get-FileHash $Path -Algorithm SHA256).Hash.ToLower() }

function Read-TallyXml([string]$Path) {
    $b = [IO.File]::ReadAllBytes($Path)
    if ($b.Length -ge 2 -and $b[0] -eq 0xFF -and $b[1] -eq 0xFE) { $t = [Text.Encoding]::Unicode.GetString($b, 2, $b.Length - 2) }
    else { $t = [Text.Encoding]::Unicode.GetString($b) }
    $x = New-Object System.Xml.XmlDocument
    $x.PreserveWhitespace = $true
    $x.LoadXml($t)
    $x
}

function Get-Text($Node, [string]$Name) { $c = $Node.SelectSingleNode($Name); if ($c) { $c.InnerText.Trim() } else { $null } }

# Marks: exactly one COMPANY row, and its GUID is the company's.
function Read-Marks([string]$Name) {
    $p = Join-Path $answers "$Name.xml"
    $rows = @((Read-TallyXml $p).SelectNodes('//COLLECTION/COMPANY'))
    if ($rows.Count -ne 1 -or (Get-Text $rows[0] 'GUID').ToLower() -ne $n2) { throw "marks answer ${Name}: $($rows.Count) COMPANY rows or another GUID" }
    "$Name.xml | $((Get-Item $p).Length) | $(Get-Sha $p) | ALTVCHID $(Get-Text $rows[0] 'ALTVCHID') | ALTMSTID $(Get-Text $rows[0] 'ALTMSTID')"
}

# Series: every VOUCHERTYPE carrying a NAME attribute; method | duplicates | sub-method of each series list.
function Read-Series([string]$Name) {
    $x = Read-TallyXml (Join-Path $answers "$Name.xml")
    foreach ($t in @($x.SelectNodes('//VOUCHERTYPE[@NAME]'))) {
        $lines = @($t.SelectNodes('VOUCHERNUMBERSERIES.LIST') | ForEach-Object { "$(Get-Text $_ 'NUMBERINGMETHOD')|$(Get-Text $_ 'PREVENTDUPLICATES')|$(Get-Text $_ 'NUMBERINGSUBMETHOD')" })
        [pscustomobject]@{ Name = $t.GetAttribute('NAME'); ALTERID = (Get-Text $t 'ALTERID'); Line = ($lines -join ' ; ') }
    }
}

# Vouchers: every VOUCHER carrying a REMOTEID attribute, sorted by MASTERID as a number.
function Read-Vch([string]$Name) {
    $x = Read-TallyXml (Join-Path $answers "$Name.xml")
    $rows = foreach ($n in @($x.SelectNodes('//VOUCHER[@REMOTEID]'))) {
        $guid = Get-Text $n 'GUID'
        if (-not $guid.ToLower().StartsWith($n2)) { throw "voucher GUID $guid in $Name is not the company's" }
        [pscustomobject]@{
            Date = (Get-Text $n 'DATE'); Type = (Get-Text $n 'VOUCHERTYPENAME'); Number = (Get-Text $n 'VOUCHERNUMBER')
            MasterId = [int](Get-Text $n 'MASTERID'); AlterId = [int](Get-Text $n 'ALTERID'); Guid = $guid
            Entries = @($n.SelectNodes('ALLLEDGERENTRIES.LIST') | ForEach-Object { "$(Get-Text $_ 'LEDGERNAME'), $(Get-Text $_ 'AMOUNT'), $(Get-Text $_ 'ISDEEMEDPOSITIVE')" })
        }
    }
    , @($rows | Sort-Object MasterId)
}

'== Marks (file | bytes | sha256 | ALTVCHID | ALTMSTID)'
foreach ($n in 'm8r-s0-x0-marks', 'm8r-x0-marks-after-backup', 'm8r-x1-marks', 'm8r-x2-marks', 'm8r-x3-marks', 'm8r-s0-x4-marks', 'm8r-x4-marks', 'm8r-x5-marks', 'm8r-x6-marks', 'm8r-x7-marks', 'm8r-x8-marks', 'm8r-end-marks') { Read-Marks $n }

''
'== Series (file | bytes | sha256 | named types | Receipt ALTERID | Receipt NUMBERINGMETHOD|PREVENTDUPLICATES|NUMBERINGSUBMETHOD | other 23 types against x0)'
$base = @(Read-Series 'm8r-x0-series')
foreach ($n in 'm8r-x0-series', 'm8r-x4-series', 'm8r-x7-series', 'm8r-end-series') {
    $p = Join-Path $answers "$n.xml"; $all = @(Read-Series $n); $r = $all | Where-Object Name -eq 'Receipt'
    $d = Compare-Object @($base | Where-Object Name -ne 'Receipt' | ForEach-Object { "$($_.Name)|$($_.Line)" }) @($all | Where-Object Name -ne 'Receipt' | ForEach-Object { "$($_.Name)|$($_.Line)" })
    "$n.xml | $((Get-Item $p).Length) | $(Get-Sha $p) | $($all.Count) | $($r.ALTERID) | $($r.Line) | $(if ($d) { 'DIFFER' } else { 'equal' })"
}

''
'== Vouchers of 1 to 31 Jul 2026 at each step, and each Receipt against the step before'
$prev = $null
foreach ($s in 'x0', 'x1', 'x2', 'x3', 'x4', 'x5', 'x6', 'x7', 'x8', 'end') {
    $p = Join-Path $answers "m8r-$s-vch.xml"; $rows = Read-Vch "m8r-$s-vch"
    "-- $s : m8r-$s-vch.xml | $((Get-Item $p).Length) | $(Get-Sha $p) | vouchers $($rows.Count) | Receipts $(@($rows | Where-Object Type -eq 'Receipt').Count)"
    foreach ($r in $rows) { "   $($r.Date), $($r.Type), $($r.Number), $($r.MasterId), $($r.AlterId), $($r.Guid) || $($r.Entries -join ' || ')" }
    if ($prev) {
        foreach ($o in ($prev | Where-Object Type -eq 'Receipt')) {
            $n = $rows | Where-Object { $_.MasterId -eq $o.MasterId }
            $mark = if ($n.Number -ne $o.Number -and $n.AlterId -eq $o.AlterId) { 'number changed, ALTERID unchanged' } elseif ($n.Number -ne $o.Number) { 'number changed, ALTERID changed' } elseif ($n.AlterId -ne $o.AlterId) { 'number same, ALTERID changed' } else { 'unchanged' }
            "   Receipt MASTERID $($o.MasterId): number $($o.Number) -> $($n.Number); ALTERID $($o.AlterId) -> $($n.AlterId); $mark"
        }
        $nonRec = Compare-Object @($prev | Where-Object Type -ne 'Receipt' | ForEach-Object { "$($_.Date)|$($_.Type)|$($_.Number)|$($_.MasterId)|$($_.AlterId)|$($_.Guid)" }) @($rows | Where-Object Type -ne 'Receipt' | ForEach-Object { "$($_.Date)|$($_.Type)|$($_.Number)|$($_.MasterId)|$($_.AlterId)|$($_.Guid)" })
        "   rows that are not Receipts: $(if ($nonRec) { 'CHANGED' } else { 'unchanged' })"
    }
    $prev = $rows
}
