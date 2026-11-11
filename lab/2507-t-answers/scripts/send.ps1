# Lab helper (row 2507-T, new; not product code): sends one of the sheet's requests (t0, t1, t2, t3 or marks) by c3-send-mt.ps1, its
# sha256 taken from sheet\SHA256.txt, bound 2,000,000, answer and headers to raw\<Ans>; logs to evidence\send-log.txt; stops on a non-zero exit.
# Usage: powershell -NoProfile -File send.ps1 -Name <t0|t1|t2|t3|marks> -Ans <answer file name>
param([Parameter(Mandatory = $true)][string]$Name, [Parameter(Mandatory = $true)][string]$Ans)
$R = '<LAB_DIR>'
$row = @(Get-Content "$R\sheet\SHA256.txt" | Where-Object { ($_ -split '\s+')[-1] -eq "$Name.xml" })
if ($row.Count -ne 1) { throw "HALT: $Name.xml not found once in SHA256.txt" }
$reqSha = ($row[0] -split '\s+')[0]
& "$R\c3-send-mt.ps1" -Req "$R\requests\$Name.xml" -Sha $reqSha -AnsDir "$R\raw" -Ans $Ans -Bound 2000000 -Log "$R\evidence\send-log.txt"
$code = $LASTEXITCODE
$t1 = (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ')
Add-Content -Encoding ascii "$R\evidence\send-log.txt" "$t1 answered $Name as $Ans exit=$code"
if ($code -ne 0) { throw "HALT: c3-send-mt exit $code on $Name" }
