# Lab helper (2074-2214 W row, 18 Nov; not product code): appends one UTC-stamped line to evidence/notes.txt and prints it.
# Usage: python -I -X utf8 note.py "<text>"
import sys, time
t = time.time(); s = time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime(t)) + ' ' + sys.argv[1]
with open('<lab folder>/evidence/notes.txt', 'a', encoding='utf-8', newline='\n') as f: f.write(s + '\n')
print(s)
