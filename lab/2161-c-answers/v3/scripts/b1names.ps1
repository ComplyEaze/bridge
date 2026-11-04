# Lab helper (#2161-C v3 B1, not product code): parser errors in cell.ps1 and a case-insensitive scan of its variable names
# for any name spelled two ways. Reads only.
$p = '<LAB_DIR>\cell.ps1'; $errs = $null
[void][System.Management.Automation.Language.Parser]::ParseFile($p, [ref]$null, [ref]$errs)
"parser errors $($errs.Count)"
$names = [regex]::Matches((Get-Content -Raw $p), '\$([A-Za-z_][A-Za-z0-9_]*)') | ForEach-Object { $_.Groups[1].Value } | Sort-Object -Unique -CaseSensitive
$twice = $names | Group-Object { $_.ToLower() } | Where-Object { $_.Count -gt 1 }
"names $(@($names).Count) (case-sensitive distinct); spelled two ways: $(@($twice).Count) $(($twice | ForEach-Object { $_.Group -join '/' }) -join ', ')"
