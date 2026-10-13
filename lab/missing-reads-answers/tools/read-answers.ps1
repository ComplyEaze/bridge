# Reads the saved answers (..\answers or the folder given) with an XML parser and prints, per answer, the rows
# under COLLECTION and where the lists this item asks about sit. Windows PowerShell 5.1. Nothing here talks to
# Tally; nothing is counted by text search. Characters outside ASCII are printed as <U+XXXX>.
param([string]$Folder = (Join-Path (Split-Path $PSScriptRoot -Parent) 'answers'))
$ErrorActionPreference = 'Stop'
function Get-Sha([string]$Path) { (Get-FileHash $Path -Algorithm SHA256).Hash.ToLower() }
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
    $x = New-Object System.Xml.XmlDocument; $x.PreserveWhitespace = $true; $x.LoadXml($t); $x
}
function Asc([string]$s) { if ($null -eq $s) { return '(none)' }; -join ($s.ToCharArray() | ForEach-Object { if ([int]$_ -gt 126 -or [int]$_ -lt 32) { '<U+{0:X4}>' -f [int]$_ } else { $_ } }) }
function T($Node, [string]$Name) { $c = $Node.SelectSingleNode($Name); if ($c) { Asc $c.InnerText.Trim() } else { '(absent)' } }
function PathOf($n, $stop) { $p = @(); while ($n -and $n -ne $stop) { $p = , $n.get_LocalName() + $p; $n = $n.ParentNode }; $p -join '/' }
# Leaves that hold text under a node: "path = value", with a count when a path repeats.
function Leaves($node) {
    $h = [ordered]@{}
    foreach ($e in $node.SelectNodes('.//*')) {
        if (@($e.ChildNodes | Where-Object { $_.NodeType -eq 'Element' }).Count) { continue }
        $v = $e.InnerText.Trim(); if ($v -eq '') { continue }
        $k = "$(PathOf $e $node) = $(Asc $v)"; $h[$k] = 1 + [int]$h[$k]
    }
    foreach ($k in $h.Keys) { if ($h[$k] -gt 1) { "      $k  (x$($h[$k]))" } else { "      $k" } }
}

foreach ($f in Get-ChildItem $Folder -Filter 'ans-*.xml' | Sort-Object Name) {
    "== $($f.Name) | $($f.Length) bytes | sha256 $(Get-Sha $f.FullName)"
    $x = Read-TallyXml $f.FullName
    $col = $x.SelectSingleNode('/ENVELOPE/BODY/DATA/COLLECTION')
    if (-not $col) { '   no COLLECTION'; ''; continue }
    $kinds = [ordered]@{}; foreach ($r in $col.ChildNodes | Where-Object { $_.NodeType -eq 'Element' }) { $kinds[$r.get_LocalName()] = 1 + [int]$kinds[$r.get_LocalName()] }
    "   rows under COLLECTION: $((@($kinds.Keys | ForEach-Object { "$_ $($kinds[$_])" }) -join ', ')); LINEERROR elements: $(@($x.SelectNodes('//LINEERROR')).Count)"
    foreach ($c in $col.SelectNodes('COMPANY')) { "   COMPANY $(Asc $c.GetAttribute('NAME')) | NAME $(T $c 'NAME') | GUID $(T $c 'GUID') | ALTVCHID $(T $c 'ALTVCHID') | ALTMSTID $(T $c 'ALTMSTID') | LASTVOUCHERDATE $(T $c 'LASTVOUCHERDATE') | BOOKSFROM $(T $c 'BOOKSFROM') | EDUMODE $(T $c 'EDUMODE')" }
    foreach ($v in $col.SelectNodes('VOUCHER')) {
        "   VOUCHER $(T $v 'DATE') | $(T $v 'VOUCHERTYPENAME') | no. $(T $v 'VOUCHERNUMBER') | MASTERID $(T $v 'MASTERID') | GUID $(T $v 'GUID') | narration $(T $v 'NARRATION')"
        foreach ($name in 'ALLLEDGERENTRIES.LIST', 'LEDGERENTRIES.LIST', 'ALLINVENTORYENTRIES.LIST', 'INVENTORYENTRIES.LIST') {
            $n = @($v.SelectNodes($name)); if ($n.Count) { "      $name directly under VOUCHER: $($n.Count)" }
        }
        foreach ($name in 'CATEGORYALLOCATIONS.LIST', 'COSTCENTREALLOCATIONS.LIST', 'BATCHALLOCATIONS.LIST', 'ACCOUNTINGALLOCATIONS.LIST', 'BILLALLOCATIONS.LIST') {
            foreach ($e in $v.SelectNodes(".//$name")) {
                if (-not (@($e.SelectNodes('.//*')) | Where-Object { $_.InnerText.Trim() -ne '' })) { continue }
                "      $(PathOf $e $v):"
                $own = [ordered]@{}
                foreach ($k in $e.ChildNodes | Where-Object { $_.NodeType -eq 'Element' -and -not @($_.ChildNodes | Where-Object { $_.NodeType -eq 'Element' }).Count -and $_.InnerText.Trim() -ne '' }) { $own[$k.get_LocalName()] = Asc $k.InnerText.Trim() }
                "         " + ((@($own.Keys | ForEach-Object { "$_ $($own[$_])" })) -join ' | ')
            }
        }
        foreach ($e in $v.SelectNodes('ALLLEDGERENTRIES.LIST | LEDGERENTRIES.LIST')) { "      ledger entry: $(T $e 'LEDGERNAME') | AMOUNT $(T $e 'AMOUNT') | ISDEEMEDPOSITIVE $(T $e 'ISDEEMEDPOSITIVE')" }
        foreach ($e in $v.SelectNodes('ALLINVENTORYENTRIES.LIST | INVENTORYENTRIES.LIST')) { "      inventory entry: $(T $e 'STOCKITEMNAME') | RATE $(T $e 'RATE') | AMOUNT $(T $e 'AMOUNT') | ACTUALQTY $(T $e 'ACTUALQTY') | BILLEDQTY $(T $e 'BILLEDQTY') | ISDEEMEDPOSITIVE $(T $e 'ISDEEMEDPOSITIVE')" }
    }
    foreach ($tag in 'STOCKITEM', 'STOCKGROUP') {
        foreach ($r in $col.SelectNodes($tag)) {
            $hsn = @($r.SelectNodes('.//HSNDETAILS.LIST')); $gst = @($r.SelectNodes('.//GSTDETAILS.LIST'))
            $hsnText = @($hsn | Where-Object { $_.InnerText.Trim() -ne '' }).Count; $gstText = @($gst | Where-Object { $_.InnerText.Trim() -ne '' }).Count
            "   $tag $(Asc $r.GetAttribute('NAME')) | GUID $(T $r 'GUID') | PARENT $(T $r 'PARENT') | HSNDETAILS.LIST $($hsn.Count) (with content $hsnText) | GSTDETAILS.LIST $($gst.Count) (with content $gstText)"
            foreach ($e in @($hsn) + @($gst)) { if ($e.InnerText.Trim() -ne '') { "      $($e.get_LocalName()):"; Leaves $e } }
        }
    }
    foreach ($r in $col.SelectNodes('TAXUNIT')) {
        $leaf = @($r.SelectNodes('.//*') | Where-Object { -not @($_.ChildNodes | Where-Object { $_.NodeType -eq 'Element' }).Count })
        "   TAXUNIT $(Asc $r.GetAttribute('NAME')) | GUID $(T $r 'GUID') | leaf elements $($leaf.Count), with text $(@($leaf | Where-Object { $_.InnerText.Trim() -ne '' }).Count)"
        Leaves $r
    }
    foreach ($r in $col.SelectNodes('CURRENCY')) {
        "   CURRENCY $(Asc $r.GetAttribute('NAME')) | GUID $(T $r 'GUID')"
        "      child element names: $((@($r.ChildNodes | Where-Object { $_.NodeType -eq 'Element' } | ForEach-Object { $_.get_LocalName() } | Sort-Object -Unique)) -join ', ')"
        Leaves $r
    }
    ''
}
