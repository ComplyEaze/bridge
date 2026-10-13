# Compares two saved Tally answers: by byte, then everything under COLLECTION leaf by leaf and attribute by
# attribute with an XML parser, and lists what differs outside COLLECTION (the answer's own CMPINFO counters).
# Nothing here talks to Tally. Usage: powershell -NoProfile -File compare-collection.ps1 <first> <second>
# Exit 0: everything under COLLECTION equal. Exit 1: something under COLLECTION differs. Exit 2: could not read.
param([Parameter(Mandatory = $true)][string]$First, [Parameter(Mandatory = $true)][string]$Second)
$ErrorActionPreference = 'Stop'
function Read-Doc([string]$Path) {
    $bytes = [IO.File]::ReadAllBytes($Path)
    if ($bytes.Length -ge 2 -and $bytes[0] -eq 0xFF -and $bytes[1] -eq 0xFE) { $text = [Text.Encoding]::Unicode.GetString($bytes, 2, $bytes.Length - 2) } else { $text = [Text.Encoding]::Unicode.GetString($bytes) }
    $text = [regex]::Replace($text, '&#(?:[0-8]|1[1-2]|1[4-9]|2[0-9]|3[0-1]);', '')
    $doc = New-Object System.Xml.XmlDocument
    $doc.PreserveWhitespace = $true
    $doc.LoadXml($text)
    $doc
}
# Every element under the node: its attributes (one entry per element) and, for an element with no child
# element, its text. Keys carry a position for every element.
function Add-Entries($Node, [string]$Path, $Out) {
    $attrs = @($Node.Attributes | ForEach-Object { $_.get_LocalName() + '=' + $_.Value }) -join ' | '
    $Out[$Path + ' @'] = $attrs
    $kids = @($Node.ChildNodes | Where-Object { $_.NodeType -eq 'Element' })
    if ($kids.Count -eq 0) { $Out[$Path] = $Node.InnerText; return }
    $seen = @{}
    foreach ($kid in $kids) {
        $nm = $kid.get_LocalName()
        $seen[$nm] = 1 + [int]$seen[$nm]
        Add-Entries $kid ($Path + '/' + $nm + '[' + $seen[$nm] + ']') $Out
    }
}
function Get-Diff($One, $Two) {
    $lines = @()
    foreach ($key in $One.Keys) { if (-not $Two.Contains($key)) { $lines += "  only in first: $key = [$($One[$key])]" } elseif (-not ($One[$key] -ceq $Two[$key])) { $lines += "  differs: $key : [$($One[$key])] -> [$($Two[$key])]" } }
    foreach ($key in $Two.Keys) { if (-not $One.Contains($key)) { $lines += "  only in second: $key = [$($Two[$key])]" } }
    , $lines
}
try {
    $bytesOne = [IO.File]::ReadAllBytes($First); $bytesTwo = [IO.File]::ReadAllBytes($Second)
    $docOne = Read-Doc $First; $docTwo = Read-Doc $Second
}
catch { "could not read: $($_.Exception.Message)"; exit 2 }
$shaOne = (Get-FileHash $First -Algorithm SHA256).Hash.ToLower(); $shaTwo = (Get-FileHash $Second -Algorithm SHA256).Hash.ToLower()
"== $(Split-Path $First -Leaf) $($bytesOne.Length) B sha256 $shaOne"
"== $(Split-Path $Second -Leaf) $($bytesTwo.Length) B sha256 $shaTwo"
if ($bytesOne.Length -eq $bytesTwo.Length) {
    $offsets = @(); for ($i = 0; $i -lt $bytesOne.Length; $i++) { if ($bytesOne[$i] -ne $bytesTwo[$i]) { $offsets += $i } }
    "bytes: same length; differing bytes $($offsets.Count)$(if ($offsets.Count) { '; offsets ' + (($offsets | Select-Object -First 40) -join ', ') })"
}
else { "bytes: lengths differ ($($bytesOne.Length) / $($bytesTwo.Length))" }
$collOne = @($docOne.SelectNodes('/ENVELOPE/BODY/DATA/COLLECTION')); $collTwo = @($docTwo.SelectNodes('/ENVELOPE/BODY/DATA/COLLECTION'))
if ($collOne.Count -ne 1 -or $collTwo.Count -ne 1) { "COLLECTION elements: $($collOne.Count) / $($collTwo.Count); expected 1 each"; exit 2 }
$inOne = [ordered]@{}; $inTwo = [ordered]@{}
Add-Entries $collOne[0] 'COLLECTION' $inOne; Add-Entries $collTwo[0] 'COLLECTION' $inTwo
$inside = Get-Diff $inOne $inTwo
"under COLLECTION: entries $($inOne.Count) / $($inTwo.Count) (elements with their attributes, and leaf texts); differing or one-sided $($inside.Count)"
$inside
# outside COLLECTION: the whole document minus the COLLECTION subtree
$allOne = [ordered]@{}; $allTwo = [ordered]@{}
Add-Entries $docOne.DocumentElement 'ENVELOPE' $allOne; Add-Entries $docTwo.DocumentElement 'ENVELOPE' $allTwo
$outOne = [ordered]@{}; $outTwo = [ordered]@{}
foreach ($key in $allOne.Keys) { if ($key -notmatch '^ENVELOPE/BODY\[1\]/DATA\[1\]/COLLECTION\[1\]') { $outOne[$key] = $allOne[$key] } }
foreach ($key in $allTwo.Keys) { if ($key -notmatch '^ENVELOPE/BODY\[1\]/DATA\[1\]/COLLECTION\[1\]') { $outTwo[$key] = $allTwo[$key] } }
$outside = Get-Diff $outOne $outTwo
"outside COLLECTION: entries $($outOne.Count) / $($outTwo.Count); differing or one-sided $($outside.Count)"
$outside
if ($inside.Count -eq 0) { 'RESULT: everything under COLLECTION is equal'; exit 0 } else { 'RESULT: COLLECTION DIFFERS'; exit 1 }
