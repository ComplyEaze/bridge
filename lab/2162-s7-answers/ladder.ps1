# Lab helper (2162-S7 part C, not product code): the month ladder as the sheet's step 11 writes it.
# For each month in order: send C-Y0-<month>.xml as c<nn>a; go on only if that send's curl total <= 15 s and its
# answer <= 24 MiB; then send the same file as c<nn>b; then run count7.py and keep its whole output.
# Stops at the first timed.py failure (it stops itself) or the first ladder miss. One send at a time.
$ErrorActionPreference = 'Stop'
$R = '<LAB_DIR>'
$run = "$R\run"
$months = '2025-04','2025-05','2025-06','2025-07','2025-08','2025-09','2025-10','2025-11','2025-12','2026-01','2026-02','2026-03'
function Note($s) { Add-Content -Path "$R\notes.txt" -Encoding utf8 -Value $s }
function Send($label, $file) {
    Push-Location $run
    try { & python -I timed.py http://localhost:9001 $label $file; $code = $LASTEXITCODE } finally { Pop-Location }
    $line = (Get-Content "$run\log-$label.txt" | Select-Object -Last 1)
    return @{ code = $code; line = $line }
}
for ($i = 0; $i -lt 12; $i++) {
    $nn = '{0:D2}' -f ($i + 1); $m = $months[$i]; $file = "C-Y0-$m.xml"
    $a = Send "c${nn}a" $file
    if ($a.code -ne 0) { Note "c${nn}a ($file): timed.py stopped, exit $($a.code): $($a.line). Part C stops; nothing resent."; exit 2 }
    $f = $a.line -split "`t"; $t = ($f[5] -split ' '); $total = [double]$t[2]; $bytes = [long]$t[3]
    Note "$($f[0]) c${nn}a ($file): $($f[4]), HTTP $($t[0]), first byte $($t[1]) s, total $($t[2]) s, $bytes B."
    if ($total -gt 15 -or $bytes -gt 25165824) {
        Note "LADDER: c${nn}a total $total s, $bytes B: over 15 s or 24 MiB. Part C stops at $m; $m's second send and every later month are NOT SENT BY THE LADDER (never 0 rows)."
        exit 3
    }
    Note "LADDER: c${nn}a total $total s <= 15 s and $bytes B <= 24 MiB: go on."
    $b = Send "c${nn}b" $file
    if ($b.code -ne 0) { Note "c${nn}b ($file): timed.py stopped, exit $($b.code): $($b.line). Part C stops; nothing resent."; exit 2 }
    $g = $b.line -split "`t"; $u = ($g[5] -split ' ')
    Note "$($g[0]) c${nn}b ($file): $($g[4]), HTTP $($u[0]), first byte $($u[1]) s, total $($u[2]) s, $($u[3]) B."
    Push-Location $run
    try { & python -I count7.py > "$R\evidence\count-c$nn.txt" 2>&1; $cc = $LASTEXITCODE } finally { Pop-Location }
    Note "count7 after c$nn exit $cc (evidence/count-c$nn.txt)."
}
Note "Part C: all twelve months sent twice."
exit 0
