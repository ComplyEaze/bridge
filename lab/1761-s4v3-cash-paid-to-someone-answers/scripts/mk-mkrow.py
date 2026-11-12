# Lab helper (row 1761-S4 v3, new; not product code): writes mk-row.py for this row from row 1761-S4 v2's
# <lab folder>/1761-s4v2-prep/mk-row.py with exact replacements, each checked to occur once: the sheet commit and folder,
# the row folder and the data folder are v3's. Stops if the new file exists. Deletes nothing.
# Usage: python -I -X utf8 mk-mkrow.py
import os
src = '<lab folder>/1761-s4v2-prep/mk-row.py'; dst = '<PREP_DIR>/mk-row.py'
if os.path.exists(dst): raise SystemExit('mk-row.py exists')
s = open(src, encoding='utf-8').read()
reps = [("# Lab helper (row 1761-S4 v2, new; not product code): makes the row folder <ROW_1761S4V2_DIR>,",
         "# Lab helper (row 1761-S4 v3, row 1761-S4 v2's mk-row.py by mk-mkrow.py with the commit and folders changed; not product code): makes the row folder <LAB_DIR>,"),
        ("# (the sheet's \"Before the sitting\" item 3). Takes the sheet's files from the commit cae1213d by git",
         "# (the sheet's \"Before the sitting\" item 3). Takes the sheet's files from the commit c00dc92d by git"),
        ("C = 'cae1213d3c49e655377c4439ded0a71ade8337fc'; P = 'lab/1761-s4v2-cash-paid-to-someone'",
         "C = 'c00dc92d1c1395e1386704fd82a367a53b511f9d'; P = 'lab/1761-s4v3-cash-paid-to-someone'"),
        ("R = '<ROW_1761S4V2_DIR>'; T = '<ROW_2511P_DIR>'", "R = '<LAB_DIR>'; T = '<ROW_2511P_DIR>'"),
        ("    '# Lab helper (row 1761-S4 v2, row 2511-P\\'s note.py with the folder changed; not product code): appends one line to\\n'",
         "    '# Lab helper (row 1761-S4 v3, row 2511-P\\'s note.py with the folder changed; not product code): appends one line to\\n'"),
        ("open('<ROW_1761S4V2_DIR>/evidence/notes.txt'", "open('<LAB_DIR>/evidence/notes.txt'"),
        ("\"# Lab helper (row 1761-S4 v2, row 2511-P's shot.sh with the folder changed; not product code)",
         "\"# Lab helper (row 1761-S4 v3, row 2511-P's shot.sh with the folder changed; not product code)"),
        ("-Out \\\"<ROW_1761S4V2_DIR>/shots/$1.png\\\"", "-Out \\\"<LAB_DIR>/shots/$1.png\\\""),
        ("'BRIDGE_AGENT_DATA_DIR': '<ROW_1761S4V2_DIR>\\\\data'", "'BRIDGE_AGENT_DATA_DIR': '<LAB_DIR>\\\\data'"),
        ("for d in ('data', 'inbox', 'answers', 'args', 'evidence', 'shots', 'backups', 'sheet'):",
         "for d in ('data', 'inbox', 'answers', 'args', 'evidence', 'shots', 'backups', 'sheet', 'step0'):")]
for a, b in reps:
    if s.count(a) != 1: raise SystemExit('not found once: ' + a[:90])
    s = s.replace(a, b)
open(dst, 'w', encoding='utf-8', newline='\n').write(s)
print('written', dst)
