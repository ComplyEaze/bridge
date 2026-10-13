# Reads the saved Tally answers in ..\answers with an XML parser and prints the tables in
# ..\derived\analysis-output.txt. Windows PowerShell 5.1. Usage (from this folder):
#   powershell -NoProfile -File read-answers.ps1 > ..\derived\analysis-output.txt
# Nothing here talks to Tally.
$ErrorActionPreference = 'Stop'
$answers = Join-Path (Split-Path $PSScriptRoot -Parent) 'answers'
$n2Guid = 'c48f3906-7682-4828-9e87-2b660064b6f8'

function Get-Sha([string]$Path) { (Get-FileHash $Path -Algorithm SHA256).Hash.ToLower() }

# Answers are UTF-16LE without a byte-order mark; a mark, if present, is skipped.
function Read-TallyXml([string]$Path) {
    $b = [IO.File]::ReadAllBytes($Path)
    if ($b.Length -ge 2 -and $b[0] -eq 0xFF -and $b[1] -eq 0xFE) { $t = [Text.Encoding]::Unicode.GetString($b, 2, $b.Length - 2) }
    else { $t = [Text.Encoding]::Unicode.GetString($b) }
    $x = New-Object System.Xml.XmlDocument
    $x.PreserveWhitespace = $true
    $x.LoadXml($t)
    $x
}

# The COMPANY row whose GUID equals $Guid (any letter case); anything but one such row is an error.
function Read-Marks([string]$Path, [string]$Guid) {
    $x = Read-TallyXml $Path
    $rows = @($x.SelectNodes('//COLLECTION/COMPANY'))
    $hit = @($rows | Where-Object { $g = $_.SelectSingleNode('GUID'); $g -and $g.InnerText.Trim().ToLower() -eq $Guid.ToLower() })
    if ($hit.Count -ne 1) { throw "$($hit.Count) COMPANY rows with GUID $Guid in $Path (rows: $($rows.Count))" }
    [pscustomobject]@{
        Rows = $rows.Count
        ALTVCHID = $hit[0].SelectSingleNode('ALTVCHID').InnerText.Trim()
        ALTMSTID = $hit[0].SelectSingleNode('ALTMSTID').InnerText.Trim()
    }
}

function Get-ChildText($Node, [string]$Name) { $c = $Node.SelectSingleNode($Name); if ($c) { $c.InnerText } else { $null } }

# Every VOUCHERTYPE; each VOUCHERNUMBERSERIES.LIST with its own direct children.
function Read-Series([string]$Path) {
    $x = Read-TallyXml $Path
    foreach ($t in @($x.SelectNodes('//COLLECTION/VOUCHERTYPE'))) {
        $series = @()
        foreach ($s in @($t.SelectNodes('VOUCHERNUMBERSERIES.LIST'))) {
            $kids = [ordered]@{}
            foreach ($c in $s.ChildNodes) {
                if ($c.NodeType -eq 'Element') { $k = $c.Name; $i = 2; while ($kids.Contains($k)) { $k = "$($c.Name)#$i"; $i++ }; $kids[$k] = $c.InnerXml }
            }
            $series += [pscustomobject]@{
                NUMBERINGMETHOD = Get-ChildText $s 'NUMBERINGMETHOD'
                PREVENTDUPLICATES = Get-ChildText $s 'PREVENTDUPLICATES'
                Children = $kids
            }
        }
        $alter = Get-ChildText $t 'ALTERID'
        [pscustomobject]@{
            Name = $t.GetAttribute('NAME')
            ALTERID = if ($alter) { $alter.Trim() } else { $null }
            TypeNUMBERINGMETHOD = Get-ChildText $t 'NUMBERINGMETHOD'
            Series = $series
        }
    }
}

# One comparable line per type and series: name | index | NUMBERINGMETHOD | PREVENTDUPLICATES
function Get-SeriesLines([string]$Path) {
    foreach ($t in @(Read-Series $Path) | Sort-Object Name) {
        if ($t.Series.Count -eq 0) { "$($t.Name)|-|(no series list)|" }
        $i = 0
        foreach ($s in $t.Series) { $i++; "$($t.Name)|$i|$($s.NUMBERINGMETHOD)|$($s.PREVENTDUPLICATES)" }
    }
}

function Get-Hex([string]$Text) { ([Text.Encoding]::Unicode.GetBytes($Text) | ForEach-Object { $_.ToString('x2') }) -join '' }

'== Marks of the numbering-lab company (COMPANY row with GUID ' + $n2Guid + ')'
'file | bytes | sha256 | COMPANY rows | ALTVCHID | ALTMSTID | ALTMSTID moved since the row above'
$prev = $null
foreach ($n in 'n2-identity-marks', 'm6r-u0-marks', 'm6r-u0-marks-after-backup', 'm6r-u1-marks', 'm6r-u2-marks', 'm6r-u3-marks', 'm6r-u4-marks', 'm6r-u4b-marks', 'm6r-u5-marks', 'm6r-u5b-marks', 'm6r-u6-marks', 'm6r-u6b-marks', 'm6r-end-marks') {
    $p = Join-Path $answers "$n.xml"
    $m = Read-Marks $p $n2Guid
    $moved = if ($null -eq $prev) { '-' } elseif ($m.ALTMSTID -ne $prev) { 'yes' } else { 'no' }
    "$n.xml | $((Get-Item $p).Length) | $(Get-Sha $p) | $($m.Rows) | $($m.ALTVCHID) | $($m.ALTMSTID) | $moved"
    $prev = $m.ALTMSTID
}

''
'== Contra in each series answer'
'file | bytes | sha256 | VOUCHERTYPE count | Contra ALTERID | Contra type-level NUMBERINGMETHOD | series lists | series NUMBERINGMETHOD (UTF-16LE hex) | PREVENTDUPLICATES | NUMBERINGSUBMETHOD'
$steps = 'm6r-u0-series', 'm6r-u2-series', 'm6r-u3-series', 'm6r-u4-series', 'm6r-u5-series', 'm6r-u6-series', 'm6r-end-series'
foreach ($n in $steps) {
    $p = Join-Path $answers "$n.xml"
    $all = @(Read-Series $p)
    $c = @($all | Where-Object { $_.Name -eq 'Contra' })
    if ($c.Count -ne 1) { throw "Contra rows: $($c.Count) in $n" }
    $s = $c[0].Series
    "$n.xml | $((Get-Item $p).Length) | $(Get-Sha $p) | $($all.Count) | $($c[0].ALTERID) | $($c[0].TypeNUMBERINGMETHOD) | $($s.Count) | $($s[0].NUMBERINGMETHOD) ($(Get-Hex $s[0].NUMBERINGMETHOD)) | $($s[0].PREVENTDUPLICATES) | $($s[0].Children['NUMBERINGSUBMETHOD'])"
}

''
'== Against m6r-u0-series.xml: types whose series NUMBERINGMETHOD or PREVENTDUPLICATES differ, children of the Contra series list whose text differs, and types whose ALTERID differs'
$u0 = Join-Path $answers 'm6r-u0-series.xml'
$u0Lines = @(Get-SeriesLines $u0)
$u0Contra = (@(Read-Series $u0) | Where-Object { $_.Name -eq 'Contra' }).Series[0].Children
$u0Alter = @{}; Read-Series $u0 | ForEach-Object { $u0Alter[$_.Name] = $_.ALTERID }
foreach ($n in $steps | Select-Object -Skip 1) {
    $p = Join-Path $answers "$n.xml"
    "-- $n.xml"
    $d = Compare-Object $u0Lines @(Get-SeriesLines $p)
    if ($d) { foreach ($x in $d) { "   series values: $(if ($x.SideIndicator -eq '<=') { 'u0 ' } else { 'now' }) $($x.InputObject)" } } else { '   series values: every type equal to u0' }
    $now = (@(Read-Series $p) | Where-Object { $_.Name -eq 'Contra' }).Series[0].Children
    $kd = foreach ($k in (@($u0Contra.Keys) + @($now.Keys) | Select-Object -Unique)) { if ("$($u0Contra[$k])" -ne "$($now[$k])") { "   Contra series child $k : u0 [$($u0Contra[$k])] now [$($now[$k])]" } }
    if ($kd) { $kd } else { '   Contra series children: all equal to u0' }
    $ad = foreach ($t in Read-Series $p) { if ($u0Alter[$t.Name] -ne $t.ALTERID) { "   ALTERID $($t.Name): $($u0Alter[$t.Name]) -> $($t.ALTERID)" } }
    if ($ad) { $ad } else { '   ALTERID: no type differs' }
}

''
'== All types in m6r-u0-series.xml (name | series index | NUMBERINGMETHOD | PREVENTDUPLICATES)'
$u0Lines

''
'== Restart check on the pilot-lab company (GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85)'
foreach ($n in 'restart-pilot-marks-before', 'restart-pilot-marks-after') {
    $p = Join-Path $answers "$n.xml"
    $m = Read-Marks $p '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    "$n.xml | $((Get-Item $p).Length) | $(Get-Sha $p) | rows $($m.Rows) | ALTVCHID $($m.ALTVCHID) | ALTMSTID $($m.ALTMSTID)"
}
$p = Join-Path $answers 'restart-pilot-series.xml'
"restart-pilot-series.xml | $((Get-Item $p).Length) | $(Get-Sha $p) | VOUCHERTYPE count $(@(Read-Series $p).Count)"
Get-SeriesLines $p
