# Lab helper (#2057 item purchase class GST, not product code): step 12's reads in the sheet's order, one at a time: for each
# day, the raw r-a07 read (raw.ps1, C3's line) then `vouchers` for that day with goods_lines (call.py). After each raw send a
# whole-window capture (cap.py); the next request goes only if the capture's sha256 equals the quiet Gateway reference
# (a dialog or any other screen = a different image = stop, nothing further sent).
param([Parameter(Mandatory = $true)][string]$RefSha, [Parameter(Mandatory = $true)][string]$Days)
$ErrorActionPreference = 'Stop'
$companyGuid = '92a7ef97-dd28-4286-b02c-c059fcb8690e'
foreach ($dayText in ($Days -split ' ')) {
    if ($dayText -ne '20250610') {
        & <LAB_DIR>\raw.ps1 -Name "r-a07-$dayText" -Ans "r12-r-a07-$dayText.xml"
        if ($LASTEXITCODE -ne 0) { throw "HALT: raw.ps1 exit $LASTEXITCODE on $dayText" }
        Get-Content <LAB_DIR>\send-log.txt -Tail 2 | Select-Object -First 1
        $capLine = & python -I <LAB_DIR>\cap.py "r12-after-r-a07-$dayText"
        $capLine
        if ($capLine -notmatch [regex]::Escape($RefSha)) { throw "HALT: the capture after r-a07-$dayText differs from the quiet Gateway; nothing further sent" }
    }
    $argsJson = '{"company_guid": "' + $companyGuid + '", "from": "' + $dayText + '", "to": "' + $dayText + '", "goods_lines": true}'
    $callOut = & python -I -X utf8 <LAB_DIR>\call.py "s12-vouchers-$dayText" vouchers $argsJson
    if ($LASTEXITCODE -ne 0) { throw "HALT: call.py exit $LASTEXITCODE on vouchers $dayText" }
    $callOut | Select-Object -First 1
}
'group12 done'
