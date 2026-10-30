# Lab helper (#2119 B row restore, not product code): puts the owner's Desktop configuration and the ComplyEaze Bridge extension
# setting back as they were before the row (desktop-config-before.json, sha256 65a4e505...; extension-settings-before.json,
# isEnabled true), records the hashes, then quits and restarts Desktop through restart-desktop.ps1 (detached). On the owner's word
# of 29 Oct 2026. Run only after the row's evidence is pushed and END is written.
$E = '<lab folder>'; $cfg = "<desktop config file>"
$ext = "<desktop extension settings folder>\local.mcpb.complyeaze-contributors.bridge-tally.json"
$log = "$E\restart-log.txt"
function L($s) { Add-Content -Encoding utf8 $log "$((Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ')) restore $s"; $s }
L "live config before restore sha256 $((Get-FileHash $cfg).Hash.ToLower())"
[IO.File]::WriteAllBytes($cfg, [IO.File]::ReadAllBytes("$E\desktop-config-before.json"))
[IO.File]::WriteAllBytes($ext, [IO.File]::ReadAllBytes("$E\extension-settings-before.json"))
L "config restored sha256 $((Get-FileHash $cfg).Hash.ToLower()) (expect 65a4e505...); extension settings sha256 $((Get-FileHash $ext).Hash.ToLower()) (expect 9ced1574...)"
Start-Process powershell -WindowStyle Hidden -ArgumentList "-NoProfile -File $E\restart-desktop.ps1 -DelaySeconds 20 -Tag restore"
L 'restart-desktop.ps1 started detached (20 s delay)'
