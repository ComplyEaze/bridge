# Lab helper (row 2511-P, new; not product code): evidence/bk.txt's first line was written with a shell-read escape
# (a backslash and b became one backspace byte); rewrites that one line as 'listing after: C:\lab\bk-2511p, 1 entries'
# and leaves the rest byte for byte. Checks the bad line first. Usage: python -I -X utf8 fix-bk-line.py
p = '<LAB_DIR>/evidence/bk.txt'
s = open(p, encoding='utf-8').read().split('\n', 1)
if s[0] != 'listing after: C:\\lab\bk-2511p, 1 entries': raise SystemExit('first line not as expected: ' + repr(s[0]))
open(p, 'w', encoding='utf-8', newline='\n').write('listing after: C:\\lab\\bk-2511p, 1 entries\n' + s[1])
print(open(p, encoding='utf-8').read())
