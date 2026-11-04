# Lab helper (2057-B, not product code): C6's directory listing (names, sizes, modified times) of THE COMPANY's own data
# folder under the Restore Data Path. The folder is found from the BK3 file name, so its number is never typed; the listing
# is written with the number shown as <company number>. Usage: python -I -X utf8 listdir.py <out file>
import datetime, os, sys
R = '<LAB_DIR>'
NUM = os.listdir(R + '/backups/bk3')[0].replace('TDBK1800_', '').replace('.001', '')
D = '<Tally data folder>/' + NUM
rows = []
for n in sorted(os.listdir(D)):
    st = os.stat(os.path.join(D, n))
    t = datetime.datetime.fromtimestamp(st.st_mtime).strftime('%Y-%m-%d %H:%M:%S')
    rows.append(f'{n.replace(NUM, "<company number>")}  {st.st_size}  {t}')
now = datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
open(sys.argv[1], 'w', encoding='utf-8', newline='\n').write(f'{now} <Tally data folder>/<company number>: {len(rows)} entries\n' + '\n'.join(rows) + '\n')
print(len(rows), 'entries')
