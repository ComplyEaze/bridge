# Show the lines that differ between two W11 answers (UTF-16LE), plus where a search string appears.
# Usage: python -I diffans.py <old answer> <new answer> [search]
import difflib
import re
import sys

sys.stdout.reconfigure(encoding='utf-8')


def lines(p):
    t = open(p, 'rb').read().decode('utf-16-le').lstrip('﻿')
    return re.sub(r'>\s*<', '>\n<', t).splitlines()


a, b = lines(sys.argv[1]), lines(sys.argv[2])
for d in difflib.unified_diff(a, b, lineterm='', n=2):
    print(d)
if len(sys.argv) > 3:
    for i, l in enumerate(b):
        if sys.argv[3] in l:
            print('HIT', i, '\n'.join(b[max(0, i - 6):i + 4]))
