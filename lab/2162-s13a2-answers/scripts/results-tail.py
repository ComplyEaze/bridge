# Lab helper (2162-S13a2, copied from 2162-S13's with its folder and list changed; not product code): appends count.py's whole output, as kept in evidence/, to RESULTS.md in
# the order of the steps, each in a fenced block under its file name. The diff against the control is not appended (its
# voucher keys are digit runs; RESULTS gives them whole, grouped). Usage: python -I -X utf8 results-tail.py (once)
import pathlib
E = pathlib.Path('<LAB_DIR>/evidence'); R = pathlib.Path('<LAB_DIR>/RESULTS.md')
ORDER = ['count-s1', 'count-s1bk', 'count-s2', 'count-s3m', 's3m-before-write', 'count-s3', 'count-s3r', 'count-s4m',
         's4m-before-write', 'count-s4', 'count-s4r', 's4r-diff-s13', 'count-s5m', 's5m-same-row', 'count-s5r', 's5r-diff',
         'count-s9', 's9-same-row']
assert R.read_text(encoding='utf-8').rstrip().endswith("## count.py's output, whole")
out = ['', 'Each block is the file in `evidence/` of the same name, byte for byte as kept.', '']
for n in ORDER:
    out += [f'`evidence/{n}.txt`', '```', (E / f'{n}.txt').read_text(encoding='utf-8').rstrip('\n'), '```', '']
with open(R, 'a', encoding='utf-8', newline='\n') as f: f.write('\n'.join(out))
print(len(ORDER), 'blocks appended')
