# Lab helper (row 1761-S4 v3, new; not product code): writes stage-1761s4v3.py from row 1761-S4 v2's stage-1761s4v2.py with
# exact replacements, each checked to occur once: this row's folders and branch, its extra scripts, the prep scripts, its own
# BK1 and restore plan, and this sheet's commit. Stops if the new file exists. Deletes nothing.
# Usage: python -I -X utf8 mk-stage.py
import os
src = '<ROW_1761S4V2_DIR>/stage-1761s4v2.py'; dst = '<LAB_DIR>/stage-1761s4v3.py'
if os.path.exists(dst): raise SystemExit('stage-1761s4v3.py exists')
s = open(src, encoding='utf-8').read()
reps = [
 ("# Lab helper (row 1761-S4 v2, new; not product code; adapted from row 1761-S4's stage-1761s4-2.py for this row's folders, with no\n# BK1 of its own):",
  "# Lab helper (row 1761-S4 v3, row 1761-S4 v2's stage-1761s4v2.py by mk-stage.py with this row's folders, its own BK1 and\n# restore plan; not product code):"),
 ("# Usage: python -I -X utf8 stage-1761s4v2.py", "# Usage: python -I -X utf8 stage-1761s4v3.py"),
 ("R = '<ROW_1761S4V2_DIR>'; S = R + '/stage/lab/1761-s4v2-cash-paid-to-someone-answers';",
  "R = '<LAB_DIR>'; S = R + '/stage/lab/1761-s4v3-cash-paid-to-someone-answers';"),
 ("+ lab('1761-s4v2-build', '<BUILD_DIR>') + lab('1761-s4v2-prep', '<PREP_DIR>') + lab('1761-s4v2', '<LAB_DIR>')",
  "+ lab('1761-s4v2-build', '<BUILD_DIR>') + lab('1761-s4v3-prep', '<PREP_DIR>') + lab('1761-s4v3', '<LAB_DIR>') + lab('1761-s4v2', '<ROW_1761S4V2_DIR>')"),
 ("SCRIPTS = ['client.py', 'call.py', 'show.py', 'step0.py', 'shot.sh', 'note.py', 'mk-args.py', 'fallback-check.py']",
  "SCRIPTS = ['client.py', 'call.py', 'show.py', 'step0.py', 'shot.sh', 'note.py', 'mk-args.py', 'mk-args4.py', 'mk-restore-plan.py', 'mk-stage.py']"),
 ("text(R + '/RESULTS.md', 'RESULTS.md')", "text(R + '/RESULTS.md', 'RESULTS.md')\ntext(R + '/restore-plan.txt', 'restore-plan.txt')"),
 ("text('<lab folder>/1761-s4v2-prep/mk-row.py', 'scripts/mk-row.py')",
  "text('<PREP_DIR>/mk-row.py', 'scripts/mk-row.py')\ntext('<PREP_DIR>/mk-mkrow.py', 'scripts/mk-mkrow.py')"),
 ("(the fallback, row 1761-S4\\'s BK1: checked at step 0.6, not restored; this row took no backup of its own)\\n')",
  "(the fallback, row 1761-S4\\'s BK1: checked at step 0.6, not restored)\\n')\n    for n in sorted(os.listdir(R + '/backups/BK1')):\n        d = open(f'{R}/backups/BK1/{n}', 'rb').read(); f.write(f'<LAB_DIR>/backups/BK1/{n.replace(NUM, \"<company number>\")} {len(d)} B {sha(d)} (this row\\'s BK1, taken at step 0.10 and restored at step 6)\\n')"),
 ("    for n in ('stage-1761s4v2.py',):", "    for n in ('stage-1761s4v3.py',):"),
 ("on the sheet branch at cae1213d, equal to its SHA256.txt)", "on the sheet branch at c00dc92d, equal to its SHA256.txt)"),
]
for a, b in reps:
    if s.count(a) != 1: raise SystemExit('not found once: ' + a[:90])
    s = s.replace(a, b)
open(dst, 'w', encoding='utf-8', newline='\n').write(s)
print('written', dst)
