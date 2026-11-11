# Lab helper (row 1780-W, new; not product code; written whole by an editor): writes w-cell2.ps1 from w-cell.ps1 (which ran cells H0
# and K) by exact replacements: L writes its line to the host instead of the pipeline (in w-cell.ps1 the log lines of Probe and
# IntoTally became part of their return values, so a gate could read an array); box-rect-probe.ps1 is started hidden beside the reads
# after T0+11 instead of run between them (in cell K it held the T0+12 read until T0+13.98 s); H0's branch is rewritten whole (its
# path held two control characters). Usage: python -I -X utf8 mk-wcell2.py
import os
src = open('<LAB_DIR>/w-cell.ps1', encoding='utf-8').read()
dst = '<LAB_DIR>/w-cell2.ps1'
assert not os.path.exists(dst)
def rep(a, b):
    global src
    assert src.count(a) == 1, a
    src = src.replace(a, b)
rep("# Lab helper (row 1780-W part H, not product code;",
    "# Lab helper (row 1780-W part H, not product code; w-cell2.ps1 is w-cell.ps1 made by mk-wcell2.py with L writing to the host, box-rect-probe.ps1\n"
    "# started hidden beside the reads, and H0's branch rewritten;")
rep("function L($s) { Add-Content -Encoding utf8 $log $s; $s }", "function L($s) { Add-Content -Encoding utf8 $log $s; Write-Host $s }")
RECT_OLD = ("    $ro = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File \"$rd\\requests\\box-rect-probe.ps1\" -Out \"$ev\\rect-$c.txt\" 2>&1\n"
            "    L \"$Cell box-rect-probe ended $(Stamp) ($(Rel)): $(($ro | % { \"$_\" }) -join ' | ')\"\n    while")
RECT_NEW = ("    $rp = Start-Process powershell -ArgumentList \"-NoProfile -NonInteractive -ExecutionPolicy Bypass -File $rd\\requests\\box-rect-probe.ps1 -Out $ev\\rect-$c.txt\" -WindowStyle Hidden -PassThru\n"
            "    L \"$Cell box-rect-probe started hidden $(Stamp) ($(Rel)) pid $($rp.Id)\"\n    while")
rep(RECT_OLD, RECT_NEW)
a = src.index("if ($Cell -eq 'H0') {\n")
b = src.index("WaitT 10; FindBox; if", a)
src = src[:a] + ("if ($Cell -eq 'H0') {\n"
                 "    $ro = & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File \"$rd\\requests\\box-rect-probe.ps1\" -Out \"$ev\\rect-$c.txt\" 2>&1\n"
                 "    L \"$Cell box-rect-probe ended $(Stamp) ($(Rel)): $(($ro | % { \"$_\" }) -join ' | ')\"\n    ") + src[b:]
import re
assert not re.search('[\x00-\x08\x0b\x0c\x0e-\x1f]', src)
open(dst, 'w', encoding='utf-8', newline='\n').write(src)
print('written', dst)
