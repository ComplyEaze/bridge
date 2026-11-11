# Lab helper (row 2346-N134, new; not product code): writes run\RESULTS.md from run\RESULTS-head.md and the row's count and check
# outputs, each copied whole into a fenced block. It sends nothing and changes no other file.
# Usage: python -I -X utf8 mk-results.py
import os
R = '<LAB_DIR>\\'; E = R + 'evidence\\'
assert not os.path.exists(R + 'RESULTS.md')
def block(title, path):
    t = open(path, encoding='utf-8').read().rstrip('\n')
    assert '```' not in t, path
    return f'\n### {title}\n```\n{t}\n```\n'
out = open(R + 'RESULTS-head.md', encoding='utf-8').read().rstrip('\n') + '\n'
out += '\n## count22.py on every answer, whole (`evidence/count-all.txt`)\n```\n' + open(E + 'count-all.txt', encoding='utf-8').read().rstrip('\n') + '\n```\n'
out += '\n## The checks the steps name, each output whole\n'
for title, f in [('s2: --w-diff, row 2162-M1\'s a05a answer against s2\'s W-08s', E + 'wdiff-s2-m1.txt'),
                 ('s3m: the before-write check', E + 'beforewrite-s3m.txt'),
                 ('s3r: --type-diff s2 against s3r', E + 'typediff-s3r.txt'),
                 ('s4m: the before-write check', E + 'beforewrite-s4m.txt'),
                 ('s4: timed.py\'s times (times-s4.txt)', R + 'times-s4.txt'),
                 ('s4: --diff t00 against t60', E + 'diff-s4-t00-t60.txt'),
                 ('s4: --diff t05 against t60', E + 'diff-s4-t05-t60.txt'),
                 ('s5: --w-diff s2 against s5, W-08s', E + 'wdiff-s5-s.txt'),
                 ('s5: --w-diff s2 against s5, W-08c', E + 'wdiff-s5-c.txt'),
                 ('s5: --w-diff s2 against s5, W-08d', E + 'wdiff-s5-d.txt'),
                 ('s5: --guid, the note\'s GUID in each W answer', E + 'guid-s5.txt'),
                 ('s6: --same-row s1 against s6', E + 'samerow-s6.txt'),
                 ('s6: --w-diff s2 against s6, W-08s', E + 'wdiff-s6.txt'),
                 ('s3: the type\'s screens, as text', E + 's3-screens.txt')]:
    out += block(title, f)
open(R + 'RESULTS.md', 'w', encoding='utf-8', newline='\n').write(out)
print('written RESULTS.md', len(out.encode('utf-8')), 'B')
