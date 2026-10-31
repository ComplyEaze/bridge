# Lab helper (#2057 item purchase class GST, not product code): one raw read of a committed request by LAB-COMMON C3's line
# (c3-send.ps1), its sha256 taken from the sheet's SHA256.txt, bound 2,000,000; saved as raw\<Ans>; stops on any non-zero exit.
param([Parameter(Mandatory = $true)][string]$Name, [Parameter(Mandatory = $true)][string]$Ans)
$rel = "requests/$Name.utf16le.xml"
$row = @(Get-Content <sheet checkout>\lab\2057-item-purchase-class-gst\SHA256.txt | Where-Object { ($_ -split '  ')[2] -eq $rel })
if ($row.Count -ne 1) { throw "HALT: $rel not found once in SHA256.txt" }
$reqSha = ($row[0] -split '  ')[0]
& <lab folder>\evidence-tools\c3-send.ps1 -Req "<sheet checkout>\lab\2057-item-purchase-class-gst\requests\$Name.utf16le.xml" -Sha $reqSha -AnsDir <LAB_DIR>\raw -Ans $Ans -Bound 2000000 -Log <LAB_DIR>\send-log.txt
$code = $LASTEXITCODE
$t1 = (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ')
Add-Content -Encoding ascii <LAB_DIR>\send-log.txt "$t1 answered $Name as $Ans exit=$code"
if ($code -ne 0) { throw "HALT: c3-send exit $code on $Name" }
