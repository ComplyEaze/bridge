# Lab helper (#1761 slice 4 row, new; not product code): writes 1761-s4-prep-2.py from 1761-s4-prep.py with the sheet folder changed
# to the extraction of lab/1761-s4-cash-paid-to-someone-sheet at 5e9580d8 (the 9 Nov prep folder, from the sheet before its build
# line was filled, was renamed aside to 1761-s4-old-1), and with the row's new empty data and backups\BK1 folders added.
# Usage: python -I -X utf8 mk-1761s4-prep2.py
import os
P = '<lab folder>\\'
assert not os.path.exists(P + '1761-s4-prep-2.py') and not os.path.exists(P + '1761-s4') and os.path.isdir(P + '1761-s4-old-1')
s = open(P + '1761-s4-prep.py', encoding='utf-8').read()
reps = [("# Lab helper (#1761 slice 4 row, not product code): stages the row folder before the sitting.",
         "# Lab helper (#1761 slice 4 row, 1761-s4-prep.py by mk-1761s4-prep2.py with the sheet at 5e9580d8 and the data and BK1 folders; not product code): stages the row folder before the sitting."),
        ("S = '<lab folder>/wt-1761-s4s/lab/1761-s4-cash-paid-to-someone'",
         "S = '<SHEET_DIR>/lab/1761-s4-cash-paid-to-someone'"),
        ("for d in ('', 'inbox', 'answers', 'step0', 'evidence', 'args', 'box', 'sheet', 'shots'): os.makedirs(f'{R}/{d}')",
         "for d in ('', 'inbox', 'answers', 'step0', 'evidence', 'args', 'box', 'sheet', 'shots', 'data', 'backups/BK1'): os.makedirs(f'{R}/{d}')")]
for a, b in reps:
    assert s.count(a) == 1, a[:70]
    s = s.replace(a, b)
compile(s, '1761-s4-prep-2.py', 'exec')
open(P + '1761-s4-prep-2.py', 'w', encoding='utf-8', newline='\n').write(s)
print('written 1761-s4-prep-2.py')
