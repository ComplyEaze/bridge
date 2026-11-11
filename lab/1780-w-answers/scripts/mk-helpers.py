# Lab helper (row 1780-W, new; not product code; written whole by an editor): writes b3.ps1 and leftover.ps1 from row 1780-H's by exact
# replacements (each must match at least once). Usage: python -I -X utf8 mk-helpers.py
import os
def mk(src, dst, reps):
    assert not os.path.exists(dst), dst
    s = open(src, encoding='utf-8').read()
    for a, b in reps:
        assert s.count(a) >= 1, a
        s = s.replace(a, b)
    open(dst, 'w', encoding='utf-8', newline='\n').write(s)
    print('written', dst)
mk('<ROW_1780H_DIR>/b3.ps1', '<LAB_DIR>/b3.ps1', [
    ("(#1780-H, not product code; row 1780-G's b3.ps1 cut to B3 and B4)", "(row 1780-W, not product code; row 1780-H's b3.ps1 with the folder changed and zorder-probe-k.ps1 in place of zorder-probe.ps1)"),
    ("<ROW_1780H_DIR>", "<LAB_DIR>"), ("zorder-probe.ps1", "zorder-probe-k.ps1")])
mk('<ROW_1780H_DIR>/leftover.ps1', '<LAB_DIR>/leftover.ps1', [
    ("(#1780-H B5, not product code; row 1780-G's leftover.ps1 with the folder, markers and window title changed)", "(row 1780-W B5, not product code; row 1780-H's leftover.ps1 with the folder changed and the markers cut to the sheet's three)"),
    ("<ROW_1780H_DIR>", "<LAB_DIR>"), (", ('--confirm-' + 'masters')", ""), ("sheet's four markers", "sheet's three markers")])
