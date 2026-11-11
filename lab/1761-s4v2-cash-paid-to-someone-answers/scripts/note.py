# Lab helper (row 1761-S4 v2, row 2511-P's note.py with the folder changed; not product code): appends one line to
# evidence/notes.txt, the UTC time first, then the text given as one argument (so no shell reads its backslashes).
# Usage: python -I -X utf8 note.py "<text>"
import sys
from datetime import datetime, timezone
t = datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
open('<LAB_DIR>/evidence/notes.txt', 'a', encoding='utf-8', newline='\n').write(t + ' ' + sys.argv[1] + '\n')
print(t, sys.argv[1][:200])
