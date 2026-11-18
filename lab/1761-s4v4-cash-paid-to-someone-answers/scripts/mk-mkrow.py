# Lab helper (row 1761-S4 v4, new; not product code): writes mk-row.py for this row from row 1761-S4 v3's
# <lab folder>/1761-s4v3-prep/mk-row.py with exact replacements, each checked to occur once: the sheet commit and folder,
# the row folder and the data folder are v4's. Stops if the new file exists. Deletes nothing.
# Usage: python -I -X utf8 mk-mkrow.py
import os
src = '<lab folder>/1761-s4v3-prep/mk-row.py'; dst = '<PREP_DIR>/mk-row.py'
if os.path.exists(dst): raise SystemExit('mk-row.py exists')
s = open(src, encoding='utf-8').read()
reps = [("# Lab helper (row 1761-S4 v3, row 1761-S4 v2's mk-row.py by mk-mkrow.py with the commit and folders changed; not product code): makes the row folder <ROW_1761S4V3_DIR>,",
         "# Lab helper (row 1761-S4 v4, row 1761-S4 v3's mk-row.py by mk-mkrow.py with the commit and folders changed; not product code): makes the row folder <LAB_DIR>,"),
        ("# (the sheet's \"Before the sitting\" item 3). Takes the sheet's files from the commit c00dc92d by git",
         "# (the sheet's \"Before the sitting\" item 3). Takes the sheet's files from the commit e85b1d75 by git"),
        ("C = 'c00dc92d1c1395e1386704fd82a367a53b511f9d'; P = 'lab/1761-s4v3-cash-paid-to-someone'",
         "C = 'e85b1d759ff41949040ec88d238dfc772d42531e'; P = 'lab/1761-s4v4-cash-paid-to-someone'"),
        ("R = '<ROW_1761S4V3_DIR>'; T = '<lab folder>/2511-p'", "R = '<LAB_DIR>'; T = '<lab folder>/2511-p'"),
        ("    '# Lab helper (row 1761-S4 v3, row 2511-P\\'s note.py with the folder changed; not product code): appends one line to\\n'",
         "    '# Lab helper (row 1761-S4 v4, row 2511-P\\'s note.py with the folder changed; not product code): appends one line to\\n'"),
        ("open('<ROW_1761S4V3_DIR>/evidence/notes.txt'", "open('<LAB_DIR>/evidence/notes.txt'"),
        ("\"# Lab helper (row 1761-S4 v3, row 2511-P's shot.sh with the folder changed; not product code)",
         "\"# Lab helper (row 1761-S4 v4, row 2511-P's shot.sh with the folder changed; not product code)"),
        ("-Out \\\"<ROW_1761S4V3_DIR>/shots/$1.png\\\"", "-Out \\\"<LAB_DIR>/shots/$1.png\\\""),
        ("'BRIDGE_AGENT_DATA_DIR': '<ROW_1761S4V3_DIR>\\\\data'", "'BRIDGE_AGENT_DATA_DIR': '<LAB_DIR>\\\\data'")]
for a, b in reps:
    if s.count(a) != 1: raise SystemExit('not found once: ' + a[:90])
    s = s.replace(a, b)
open(dst, 'w', encoding='utf-8', newline='\n').write(s)
print('written', dst)
