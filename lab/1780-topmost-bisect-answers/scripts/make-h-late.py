# Lab helper (#1780-E, not product code): writes h-late.ps1 from h-run.ps1 (attach to the running child by pid instead of
# starting one; a new trace; the line count without the trailing line break; its own log and capture names).
src = open(r'<lab folder>\h-run.ps1', encoding='utf-8').read()
a = src.index('$trArgs = ')
b = src.index('$box = [HW]::ByTitle')
late = r'''$p = Get-Process -Id 16104; $null = $p.Handle
L "H-late $(Stamp): attached to the running child pid $($p.Id) (started $($p.StartTime.ToUniversalTime().ToString('HH:mm:ss.fff'))Z); the first runner had stopped at its own line-count error"
$trArgs = "-NoProfile -ExecutionPolicy Bypass -File $rd\requests\box-trace.ps1 -Title `"ComplyEaze Bridge`" -Out $ev\trace-h1-late.txt -Seconds 100"
$tr = Start-Process powershell -ArgumentList $trArgs -PassThru; $null = $tr.Handle
L "H-late trace started $(Stamp) pid $($tr.Id) in its own window: powershell $trArgs"
Start-Sleep -Seconds 2
[HW]::ClickAt($tx, $ty); L "H-late Tally title clicked $(Stamp)"
Start-Sleep -Milliseconds 700
L "H-late probe before the click $(Probe 'probe-h1-late')"
$t0 = (Get-Date).AddSeconds(-16)
'''
s = src[:a] + late + src[b:]
pairs = [
    ('$boxLines = @($static -split "`r?`n").Count', '$boxLines = @(($static.TrimEnd()) -split "`r?`n").Count'),
    (r'$log = "$ev\h1-timeline.txt"', r'$log = "$ev\h1-late-timeline.txt"'),
    ("Shot 'h1-c0-screen'", "Shot 'h1-late-c0-screen'"),
    ("Shot 'h1-c6-screen'", "Shot 'h1-late-c6-screen'"),
    ("'probe-h1-c0'", "'probe-h1-late-c0'"),
    ("'probe-h1-c6'", "'probe-h1-late-c6'"),
    ('if ($pre.Count -ne 0) { L "H0 a child is already there: STOP"; exit 40 }',
     'if ($pre.Count -ne 1 -or $pre[0].ProcessId -ne 16104) { L "H0 not the one hand-run child: STOP"; exit 40 }'),
    ('# Lab helper (#1780-E part H, not product code)',
     '# Lab helper (#1780-E part H, late: attaches to the running child by pid after h-run.ps1 stopped at its own line-count error; not product code)'),
]
for old, new in pairs:
    assert s.count(old) == 1, old
    s = s.replace(old, new)
open(r'<lab folder>\h-late.ps1', 'w', encoding='utf-8').write(s)
print('written')
