# Lab helper (#2161-B, not product code): LAB-COMMON C2 d's MARKS read by C3's line, from row 2161-A's request file (its sha256
# checked first). Writes evidence\<Tag>-marks.response and .headers.txt; prints curl's line, CONTENT-LENGTH and the trimmed
# ALTVCHID and ALTMSTID of each COMPANY row.
param([Parameter(Mandatory = $true)][string]$Tag)
$rd = '<lab folder>'; $req = "$rd\marks.utf16le.xml"; $want = '539ca283638c2013df5445da0035df47eb2d7a08a4ccdae8f9858e22ed358f8d'
if ((Get-FileHash $req).Hash.ToLower() -ne $want) { 'stop: the marks request differs from its recorded sha256; nothing sent'; exit 3 }
$ans = "$rd\evidence\$Tag-marks.response"; $hdr = "$rd\evidence\$Tag-marks.headers.txt"
if (Test-Path $ans) { "stop: $ans exists"; exit 3 }
$t0 = (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ')
$o = & curl.exe -sS -w "%{http_code} %{size_download}" --data-binary "@$req" -H "Content-Type: text/xml; charset=utf-16" -D $hdr http://127.0.0.1:9001 -o $ans 2>&1; $x = $LASTEXITCODE
$cl = (Select-String -Path $hdr -Pattern 'CONTENT-LENGTH:\s*(\d+)' | ForEach-Object { $_.Matches[0].Groups[1].Value }) -join ''
$t = [IO.File]::ReadAllText($ans, [Text.Encoding]::Unicode)
$rows = [regex]::Matches($t, '<COMPANY NAME="([^"]*)"[^>]*>(.*?)</COMPANY>', 'Singleline')
"$Tag MARKS sent $t0 curl exit $x printed $o content_length $cl bytes $((Get-Item $ans).Length) sha256 $((Get-FileHash $ans).Hash.ToLower()) rows $($rows.Count)"
foreach ($r in $rows) { $b = $r.Groups[2].Value; "  $($r.Groups[1].Value) GUID $([regex]::Match($b, '<GUID[^>]*>\s*([^<]*?)\s*<').Groups[1].Value) ALTVCHID $([regex]::Match($b, '<ALTVCHID[^>]*>\s*(\d+)').Groups[1].Value) ALTMSTID $([regex]::Match($b, '<ALTMSTID[^>]*>\s*(\d+)').Groups[1].Value)" }
