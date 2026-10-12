# Print the last import block from Tally's tally.imp (UTF-16) and append it to lab/import-log.txt.
import sys
t = open('C:/Program Files/TallyPrime/tally.imp', 'rb').read().decode('utf-16-le', 'replace')
i = t.rfind('Importing Data from')
blk = t[i:].replace('\r', '').strip()
blk = '\n'.join(l.rstrip() for l in blk.split('\n') if l.strip())
print(blk)
open('C:/bridge-lab/demo-dt/lab/import-log.txt', 'a', encoding='utf-8', newline='\n').write(blk + '\n\n')
